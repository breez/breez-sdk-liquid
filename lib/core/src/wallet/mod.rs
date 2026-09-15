pub(crate) mod network_fee;
pub mod persister;
pub(crate) mod utxo_select;

use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use boltz_client::ElementsAddress;
use log::{debug, error, info, warn};
use lwk_common::Signer as LwkSigner;
use lwk_common::{singlesig_desc, Singlesig};
use lwk_wollet::asyncr::{EsploraClient, EsploraClientBuilder};
use lwk_wollet::elements::hex::ToHex;
use lwk_wollet::elements::pset::PartiallySignedTransaction;
use lwk_wollet::elements::{Address, AssetId, OutPoint, Transaction, TxOut, Txid};
use lwk_wollet::secp256k1::Message;
use lwk_wollet::{Network, WalletTx, WalletTxOut, Wollet, WolletDescriptor};
use persister::SqliteWalletCachePersister;
use sdk_common::bitcoin::hashes::{sha256, Hash};
use sdk_common::bitcoin::secp256k1::PublicKey;
use sdk_common::lightning::util::message_signing::verify;
use tokio::sync::Mutex;
use utxo_select::{InOut, WalletUtxoSelectRequest};
use web_time::Instant;

use crate::model::{BlockchainExplorer, Signer, BREEZ_LIQUID_ESPLORA_URL};
use crate::persist::Persister;
use crate::signer::SdkLwkSigner;
use crate::{ensure_sdk, error::PaymentError, model::Config};

use crate::wallet::persister::WalletCachePersister;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use lwk_wollet::blocking::BlockchainBackend;

static LN_MESSAGE_PREFIX: &[u8] = b"Lightning Signed Message:";

#[sdk_macros::async_trait]
pub trait OnchainWallet: Send + Sync {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError>;

    /// List all transactions in the wallet mapped by tx id
    async fn transactions_by_tx_id(&self) -> Result<HashMap<Txid, WalletTx>, PaymentError>;

    /// List all utxos in the wallet for a given asset
    async fn asset_utxos(&self, asset: &AssetId) -> Result<Vec<WalletTxOut>, PaymentError>;

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Builds a drain tx.
    ///
    /// ### Arguments
    /// - `fee_rate_sats_per_kvb`: custom drain tx feerate
    /// - `recipient_address`: drain tx recipient
    /// - `enforce_amount_sat`: if set, the drain tx will only be built if the amount transferred is
    ///   this amount, otherwise it will fail with a validation error
    async fn build_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        enforce_amount_sat: Option<u64>,
    ) -> Result<Transaction, PaymentError>;

    /// Build a transaction to send funds to a recipient. If building a transaction
    /// results in an InsufficientFunds error, attempt to build a drain transaction
    /// validating that the `amount_sat` matches the drain output.
    async fn build_tx_or_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Sign a partially signed transaction
    async fn sign_pset(&self, pset: &mut PartiallySignedTransaction) -> Result<(), PaymentError>;

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError>;

    /// Get the next unused change address in the wallet
    async fn next_unused_change_address(&self) -> Result<Address, PaymentError>;

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> u32;

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String>;

    /// Get the fingerprint of the wallet
    fn fingerprint(&self) -> Result<String>;

    /// Sign given message with the wallet private key. Returns a zbase
    /// encoded signature.
    fn sign_message(&self, msg: &str) -> Result<String>;

    /// Check whether given message was signed by the given
    /// pubkey and the signature (zbase encoded) is valid.
    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool>;

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError>;

    /// Records a just-broadcast tx so the coins it spends stop being selectable before the next
    /// scan sees them; without this a second payment re-selects them and is rejected.
    async fn apply_broadcast_tx(&self, tx: &Transaction);

    /// Repairs the cached unspent set locally, with no network access, by re-applying the txs
    /// the wallet already holds that spend outputs still listed as unspent.
    ///
    /// Returns whether the cache is consistent afterwards. If not, the wallet flags itself so the
    /// next scan wipes and rescans.
    async fn repair_cache(&self) -> Result<bool, PaymentError>;
}

/// Maps a stale-cache broadcast rejection to an actionable error, repairing the cache on the way.
///
/// The repair is local and instant, so a retry by the caller succeeds. If it cannot resolve the
/// drift it schedules a wipe for the next scan, which runs in the background.
pub(crate) async fn handle_stale_cache_broadcast_error(
    onchain_wallet: &dyn OnchainWallet,
    err: anyhow::Error,
) -> PaymentError {
    if !crate::error::is_txn_inputs_missing_or_spent_error(&err) {
        return err.into();
    }
    warn!("Broadcast rejected for spending inputs the node does not have, repairing the cache");
    if let Err(e) = onchain_wallet.repair_cache().await {
        warn!("Could not repair the wallet cache: {e}");
    }
    PaymentError::Generic {
        err: format!("Wallet state was out of date, please retry shortly: {err}"),
    }
}

pub enum WalletClient {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    Electrum(Box<lwk_wollet::ElectrumClient>),
    Esplora(Box<EsploraClient>),
}

impl WalletClient {
    pub(crate) fn from_config(config: &Config) -> Result<Self> {
        match &config.liquid_explorer {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { url } => {
                let client = Box::new(config.electrum_client(url)?);
                Ok(Self::Electrum(client))
            }
            BlockchainExplorer::Esplora {
                url,
                use_waterfalls,
                authorization,
            } => {
                let waterfalls = *use_waterfalls;
                let mut builder = EsploraClientBuilder::new(url, config.network.into());
                if url == BREEZ_LIQUID_ESPLORA_URL {
                    // Breez API key takes precedence for the Breez URL
                    match &config.breez_api_key {
                        Some(api_key) => {
                            builder = builder
                                .header("authorization".to_string(), format!("Bearer {api_key}"));
                        }
                        None => {
                            let err = "Cannot start Breez Esplora client: Breez API key is not set";
                            error!("{err}");
                            bail!(err)
                        }
                    };
                } else if let Some(auth) = authorization {
                    // Apply custom authorization for non-Breez URLs
                    builder = builder.header("authorization".to_string(), auth.to_header_value());
                }
                let client = builder
                    .timeout(config.onchain_sync_request_timeout_sec as u8)
                    .waterfalls(waterfalls)
                    .build()?;
                Ok(Self::Esplora(Box::new(client)))
            }
        }
    }

    pub(crate) async fn full_scan_to_index(
        &mut self,
        wallet: &mut Wollet,
        index: u32,
    ) -> Result<(), lwk_wollet::Error> {
        let maybe_update = match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            WalletClient::Electrum(electrum_client) => {
                electrum_client.full_scan_to_index(&wallet.state(), index)?
            }
            WalletClient::Esplora(esplora_client) => {
                esplora_client.full_scan_to_index(wallet, index).await?
            }
        };

        if let Some(update) = maybe_update {
            debug!(
                "WalletClient::full_scan_to_index: applying update {}",
                update.version
            );
            wallet.apply_update(update)?;
        }

        Ok(())
    }
}

pub struct LiquidOnchainWallet {
    config: Config,
    persister: std::sync::Arc<Persister>,
    wallet: Arc<Mutex<Wollet>>,
    client: Mutex<Option<WalletClient>>,
    pub(crate) signer: SdkLwkSigner,
    wallet_cache_persister: Arc<dyn WalletCachePersister>,
    /// Whether the next scan should verify the cached unspent set.
    needs_cache_check: AtomicBool,
    /// Whether the next scan must wipe the cache first, because a local repair could not fix it.
    needs_cache_clear: AtomicBool,
    /// Whether a wipe-and-rescan is running. It holds the wallet lock for a cold rescan, so tx
    /// building checks this first to fail fast rather than block for minutes.
    recovery_scan_in_progress: AtomicBool,
    /// Whether a wipe already ran this session. A cold rescan costs minutes, and if one did not
    /// produce a clean unspent set a second will not either, so this bounds it to one attempt.
    cache_wiped: AtomicBool,
}

/// Clears an [`AtomicBool`] on drop, so the flag is released even on an early return.
struct FlagGuard<'a>(&'a AtomicBool);

impl Drop for FlagGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Relaxed);
    }
}

/// Outpoints still listed as unspent despite being spent by a tx the wallet already knows about.
///
/// lwk enforced this on every `utxos()` call before the unspent set became materialised state
/// (`RawCache::spent()`). A violation is permanent without a wipe: selection is deterministic, so
/// it keeps picking the same spent coin and every broadcast is rejected.
pub(crate) fn find_spent_utxos(txs: &[WalletTx], utxos: &[WalletTxOut]) -> Vec<OutPoint> {
    let spent: std::collections::HashSet<OutPoint> = txs
        .iter()
        .flat_map(|wtx| wtx.tx.input.iter().map(|i| i.previous_output))
        .collect();
    utxos
        .iter()
        .map(|u| u.outpoint)
        .filter(|o| spent.contains(o))
        .collect()
}

impl LiquidOnchainWallet {
    /// Creates a new LiquidOnchainWallet that caches data on the provided `working_dir`.
    pub(crate) async fn new(
        config: Config,
        persister: std::sync::Arc<Persister>,
        user_signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self> {
        let signer = SdkLwkSigner::new(user_signer.clone())?;

        let wallet_cache_persister: Arc<dyn WalletCachePersister> = Arc::new(
            SqliteWalletCachePersister::new(std::sync::Arc::clone(&persister))?,
        );

        let wollet = Self::create_wallet(&config, &signer, wallet_cache_persister.clone()).await?;

        Ok(Self {
            config,
            persister,
            wallet: Arc::new(Mutex::new(wollet)),
            client: Mutex::new(None),
            signer,
            wallet_cache_persister,
            // Check on startup, so a cache corrupted in a previous session is repaired before a
            // payment discovers it.
            needs_cache_check: AtomicBool::new(true),
            needs_cache_clear: AtomicBool::new(false),
            cache_wiped: AtomicBool::new(false),
            recovery_scan_in_progress: AtomicBool::new(false),
        })
    }

    /// Verifies the cached unspent set against the tx set and repairs any drift. Cheap and
    /// local; only a repair costs anything. Runs after a scan, never during one.
    /// Schedules a wipe-and-rescan for the next scan, unless one already ran this session.
    ///
    /// A cold rescan costs minutes and rebuilds the unspent set from an empty cache, so if it did
    /// not produce a clean one, repeating it will not either. Bounding it keeps a wallet whose
    /// drift the rescan cannot resolve from wiping on every scan for the rest of the session.
    fn schedule_cache_wipe(&self) -> bool {
        if self.cache_wiped.load(Ordering::Relaxed) {
            error!(
                "Wallet cache is still inconsistent after a full rescan this session, not wiping \
                 again. Coin selection may keep offering spent utxos until the next restart."
            );
            return false;
        }
        warn!("Flagging the wallet cache to be wiped and rebuilt on the next scan");
        self.needs_cache_clear.store(true, Ordering::Relaxed);
        true
    }

    async fn check_and_repair_cache(&self) -> Result<(), PaymentError> {
        if !self.needs_cache_check.swap(false, Ordering::Relaxed) {
            return Ok(());
        }

        let spent = {
            let wallet = self.wallet.lock().await;
            let txs = wallet.transactions()?;
            let utxos = wallet.utxos()?;
            find_spent_utxos(&txs, &utxos)
        };

        if spent.is_empty() {
            if self.cache_wiped.load(Ordering::Relaxed) {
                info!("Wallet cache verified clean after the rescan");
            } else {
                debug!("Wallet cache check: no utxo is spent by a known tx");
            }
            return Ok(());
        }

        error!(
            "Wallet cache is inconsistent: {} utxo(s) are already spent by known txs. {spent:?}",
            spent.len()
        );

        // The local repair usually suffices; if not it schedules a wipe for the next scan.
        self.repair_cache().await?;
        Ok(())
    }

    async fn create_wallet(
        config: &Config,
        signer: &SdkLwkSigner,
        wallet_cache_persister: Arc<dyn WalletCachePersister>,
    ) -> Result<Wollet> {
        let network: Network = config.network.into();
        let descriptor = get_descriptor(signer)?;
        let build_wollet = |persister: persister::LwkPersister| {
            lwk_wollet::WolletBuilder::new(network, descriptor.clone())
                .with_updates_store(persister)
                .build()
        };
        let wollet_res = build_wollet(wallet_cache_persister.get_lwk_persister()?);
        match wollet_res {
            Ok(wollet) => Ok(wollet),
            res @ Err(
                lwk_wollet::Error::UpdateHeightTooOld { .. }
                | lwk_wollet::Error::UpdateOnDifferentStatus { .. }
                | lwk_wollet::Error::StoreError(_),
            ) => {
                warn!("Update error initialising wollet, wiping cache and retrying: {res:?}");
                wallet_cache_persister.clear_cache().await?;
                Ok(build_wollet(wallet_cache_persister.get_lwk_persister()?)?)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn get_txout(&self, wallet: &Wollet, outpoint: &OutPoint) -> Result<TxOut> {
        let wallet_tx = wallet
            .transaction(&outpoint.txid)?
            .ok_or(anyhow!("Transaction not found"))?;
        let tx_out = wallet_tx
            .tx
            .output
            .get(outpoint.vout as usize)
            .ok_or(anyhow!("Output not found"))?;
        Ok(tx_out.clone())
    }

    fn select_wallet_utxos(
        &self,
        wallet: &Wollet,
        policy_asset: AssetId,
        selection_asset: AssetId,
        recipient_outputs: Vec<InOut>,
        fee_rate_sats_per_kvb: Option<f32>,
    ) -> Result<Vec<OutPoint>, PaymentError> {
        let mut wallet_utxos = wallet.utxos()?;
        debug!(
            "Wallet utxos: {:?}",
            wallet_utxos
                .iter()
                .map(|tx_out| format!(
                    "{}:{}, value: {}",
                    tx_out.outpoint.txid, tx_out.outpoint.vout, tx_out.unblinded.value
                ))
                .collect::<Vec<_>>()
        );
        let fee_rate = fee_rate_sats_per_kvb.map(|rate| rate as f64 / 1000.0);
        let selected_in_outs = utxo_select::utxo_select(WalletUtxoSelectRequest {
            policy_asset,
            selection_asset,
            wallet_utxos: wallet_utxos.iter().map(Into::into).collect(),
            recipient_outputs,
            fee_rate,
        })?;
        let selected_utxos = Self::resolve_selected_utxos(&mut wallet_utxos, &selected_in_outs)?;
        debug!(
            "Selected wallet outputs: {:?}",
            selected_utxos
                .iter()
                .map(|outpoint| format!("{}:{}", outpoint.txid, outpoint.vout))
                .collect::<Vec<_>>()
        );
        Ok(selected_utxos)
    }

    /// Selects wallet utxos for a non-L-BTC asset send: enough utxos of `asset`
    /// to cover `amount_sat`, plus a bounded set of L-BTC utxos to cover the fee.
    ///
    /// Without an explicit selection, lwk falls into its "always add all L-BTC
    /// inputs" path, which can exceed the 256-input surjection-proof limit (and
    /// fail with `TooManyInputs`) for wallets with many small L-BTC utxos, even
    /// though an asset send only needs a couple of inputs to pay the fee.
    fn select_asset_and_fee_utxos(
        &self,
        wallet: &Wollet,
        asset: AssetId,
        amount_sat: u64,
        fee_rate_sats_per_kvb: Option<f32>,
    ) -> Result<Vec<OutPoint>, PaymentError> {
        let policy_asset = wallet.policy_asset();
        ensure_sdk!(
            asset != policy_asset,
            PaymentError::generic("select_asset_and_fee_utxos called for the policy asset")
        );

        let mut wallet_utxos = wallet.utxos()?;

        // Select asset utxos to cover the amount being sent.
        let asset_values = wallet_utxos
            .iter()
            .filter(|tx_out| tx_out.unblinded.asset == asset)
            .map(|tx_out| tx_out.unblinded.value)
            .collect::<Vec<_>>();
        let selected_asset_values = utxo_select::utxo_select_best(amount_sat, &asset_values)
            .ok_or_else(|| PaymentError::generic("Failed to select asset utxos"))?;
        let asset_input_count = selected_asset_values.len();

        // Select a bounded set of L-BTC utxos to cover the fee. The fee depends on
        // the total input count, so seed the estimate with the asset inputs above.
        let fee_rate = fee_rate_sats_per_kvb.map(|rate| rate as f64 / 1000.0);
        let policy_values = wallet_utxos
            .iter()
            .filter(|tx_out| tx_out.unblinded.asset == policy_asset)
            .map(|tx_out| tx_out.unblinded.value)
            .collect::<Vec<_>>();
        let selected_fee_values = utxo_select::utxo_select_dynamic(
            0,
            &policy_values,
            |lbtc_input_count, change_count| {
                network_fee::TxFee {
                    native_inputs: asset_input_count + lbtc_input_count,
                    nested_inputs: 0,
                    // asset recipient + asset change + L-BTC change
                    outputs: 2 + change_count,
                }
                .fee(fee_rate)
            },
        )
        .ok_or_else(|| PaymentError::generic("Failed to select L-BTC utxos for fee"))?;

        // Resolve the selected asset and fee values to their wallet outpoints.
        let selected = selected_asset_values
            .into_iter()
            .map(|value| InOut {
                asset_id: asset,
                value,
            })
            .chain(selected_fee_values.into_iter().map(|value| InOut {
                asset_id: policy_asset,
                value,
            }))
            .collect::<Vec<_>>();
        Self::resolve_selected_utxos(&mut wallet_utxos, &selected)
    }

    /// Resolves selected `(asset, value)` pairs to their wallet outpoints,
    /// removing each match as it is found so that duplicate values resolve to
    /// distinct utxos. Errors if any selected value has no matching utxo.
    fn resolve_selected_utxos(
        wallet_utxos: &mut Vec<WalletTxOut>,
        selected: &[InOut],
    ) -> Result<Vec<OutPoint>, PaymentError> {
        let selected_utxos = selected
            .iter()
            .filter_map(|in_out| {
                wallet_utxos
                    .iter()
                    .position(|tx_out| {
                        tx_out.unblinded.asset == in_out.asset_id
                            && tx_out.unblinded.value == in_out.value
                    })
                    .map(|index| wallet_utxos.remove(index).outpoint)
            })
            .collect::<Vec<_>>();
        ensure_sdk!(
            selected_utxos.len() == selected.len(),
            PaymentError::generic("Failed to resolve selected wallet utxos to outpoints")
        );
        Ok(selected_utxos)
    }
}

pub fn get_descriptor(signer: &SdkLwkSigner) -> Result<WolletDescriptor, PaymentError> {
    let descriptor_str = singlesig_desc(
        signer,
        Singlesig::Wpkh,
        lwk_common::DescriptorBlindingKey::Slip77,
    )
    .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
    Ok(descriptor_str.parse()?)
}

#[sdk_macros::async_trait]
impl OnchainWallet for LiquidOnchainWallet {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError> {
        let wallet = self.wallet.lock().await;
        wallet.transactions().map_err(|e| PaymentError::Generic {
            err: format!("Failed to fetch wallet transactions: {e:?}"),
        })
    }

    /// List all transactions in the wallet mapped by tx id
    async fn transactions_by_tx_id(&self) -> Result<HashMap<Txid, WalletTx>, PaymentError> {
        let tx_map: HashMap<Txid, WalletTx> = self
            .transactions()
            .await?
            .iter()
            .map(|tx| (tx.txid, tx.clone()))
            .collect();
        Ok(tx_map)
    }

    async fn asset_utxos(&self, asset: &AssetId) -> Result<Vec<WalletTxOut>, PaymentError> {
        Ok(self
            .wallet
            .lock()
            .await
            .utxos()?
            .into_iter()
            .filter(|utxo| &utxo.unblinded.asset == asset)
            .collect())
    }

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        ensure_sdk!(
            !self.recovery_scan_in_progress.load(Ordering::Relaxed),
            PaymentError::Generic {
                err: "Wallet state is being repaired, please retry shortly".to_string()
            }
        );
        let lwk_wollet = self.wallet.lock().await;
        let address =
            ElementsAddress::from_str(recipient_address).map_err(|e| PaymentError::Generic {
                err: format!(
                    "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                ),
            })?;
        let mut tx_builder = lwk_wollet::TxBuilder::new(self.config.network.into())
            .fee_rate(fee_rate_sats_per_kvb)
            .enable_ct_discount();
        if asset_id.eq(&self.config.lbtc_asset_id()) {
            // If the asset is L-BTC, try to select wallet utxos for the recipient amount.
            // If it fails to select utxos, the LWK wallet will select the utxos for us.
            let policy_asset = lwk_wollet.policy_asset();
            // TODO: LWK only supports selecting utxos for the policy asset, in the future
            // we should be able to select utxos for any asset.
            match self.select_wallet_utxos(
                &lwk_wollet,
                policy_asset,
                policy_asset,
                vec![InOut {
                    asset_id: policy_asset,
                    value: amount_sat,
                }],
                fee_rate_sats_per_kvb,
            ) {
                Ok(wallet_utxos) => {
                    tx_builder = tx_builder.set_wallet_utxos(wallet_utxos);
                }
                Err(e) => warn!("Failed to select wallet utxos: {e:?}"),
            }
            // Add the L-BTC recipient
            tx_builder = tx_builder.add_lbtc_recipient(&address, amount_sat)?;
        } else {
            // Add the asset recipient
            let asset = AssetId::from_str(asset_id)?;
            // Explicitly select the asset utxos plus a bounded set of L-BTC utxos
            // for the fee. If selection fails, fall back to letting lwk select the
            // utxos (which adds all L-BTC inputs).
            match self.select_asset_and_fee_utxos(
                &lwk_wollet,
                asset,
                amount_sat,
                fee_rate_sats_per_kvb,
            ) {
                Ok(wallet_utxos) => {
                    tx_builder = tx_builder.set_wallet_utxos(wallet_utxos);
                }
                Err(e) => warn!("Failed to select asset and fee wallet utxos: {e:?}"),
            }
            tx_builder = tx_builder.add_recipient(&address, amount_sat, asset)?;
        }
        let mut pset = tx_builder.finish(&lwk_wollet)?;
        self.signer
            .sign(&mut pset)
            .map_err(|e| PaymentError::Generic {
                err: format!("Failed to sign transaction: {e:?}"),
            })?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    async fn build_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        enforce_amount_sat: Option<u64>,
    ) -> Result<Transaction, PaymentError> {
        ensure_sdk!(
            !self.recovery_scan_in_progress.load(Ordering::Relaxed),
            PaymentError::Generic {
                err: "Wallet state is being repaired, please retry shortly".to_string()
            }
        );
        let lwk_wollet = self.wallet.lock().await;

        let address =
            ElementsAddress::from_str(recipient_address).map_err(|e| PaymentError::Generic {
                err: format!(
                    "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                ),
            })?;
        let mut pset = lwk_wollet
            .tx_builder()
            .drain_lbtc_wallet()
            .drain_lbtc_to(address)
            .fee_rate(fee_rate_sats_per_kvb)
            .enable_ct_discount()
            .finish()?;

        if let Some(enforce_amount_sat) = enforce_amount_sat {
            let pset_details = lwk_wollet.get_details(&pset)?;
            let pset_balance_sat = pset_details
                .balance
                .balances
                .get(&lwk_wollet.policy_asset())
                .unwrap_or(&0);
            let pset_fees = pset_details.balance.fees_in(&lwk_wollet.policy_asset());

            ensure_sdk!(
                (*pset_balance_sat * -1) as u64 - pset_fees == enforce_amount_sat,
                PaymentError::Generic {
                    err: format!("Drain tx amount {pset_balance_sat} sat doesn't match enforce_amount_sat {enforce_amount_sat} sat")
                }
            );
        }

        self.signer
            .sign(&mut pset)
            .map_err(|e| PaymentError::Generic {
                err: format!("Failed to sign transaction: {e:?}"),
            })?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    async fn build_tx_or_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        match self
            .build_tx(
                fee_rate_sats_per_kvb,
                recipient_address,
                asset_id,
                amount_sat,
            )
            .await
        {
            Ok(tx) => Ok(tx),
            Err(PaymentError::InsufficientFunds) if asset_id.eq(&self.config.lbtc_asset_id()) => {
                warn!("Cannot build tx due to insufficient funds, attempting to build drain tx");
                self.build_drain_tx(fee_rate_sats_per_kvb, recipient_address, Some(amount_sat))
                    .await
            }
            Err(e) => Err(e),
        }
    }

    async fn sign_pset(&self, pset: &mut PartiallySignedTransaction) -> Result<(), PaymentError> {
        let lwk_wollet = self.wallet.lock().await;

        // Get the tx_out for each input and add the rangeproof/witness utxo
        for input in pset.inputs_mut().iter_mut() {
            let tx_out_res = self
                .get_txout(
                    &lwk_wollet,
                    &OutPoint {
                        txid: input.previous_txid,
                        vout: input.previous_output_index,
                    },
                )
                .await;
            if let Ok(mut tx_out) = tx_out_res {
                input.in_utxo_rangeproof = tx_out.witness.rangeproof.take();
                input.witness_utxo = Some(tx_out);
            }
        }

        lwk_wollet.add_details(pset)?;

        self.signer.sign(pset).map_err(|e| PaymentError::Generic {
            err: format!("Failed to sign transaction: {e:?}"),
        })?;

        // Set the final script witness for each input adding the signature and any missing public key
        for input in pset.inputs_mut() {
            if let Some((public_key, input_sign)) = input.partial_sigs.iter().next() {
                input.final_script_witness = Some(vec![input_sign.clone(), public_key.to_bytes()]);
            }
        }

        Ok(())
    }

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError> {
        let tip = self.tip().await;
        let address = match self.persister.next_expired_reserved_address(tip)? {
            Some(reserved_address) => {
                debug!(
                    "Got reserved address {} that expired on block height {}",
                    reserved_address.address, reserved_address.expiry_block_height
                );
                ElementsAddress::from_str(&reserved_address.address)
                    .map_err(|e| PaymentError::Generic { err: e.to_string() })?
            }
            None => {
                let next_index = self.persister.next_derivation_index()?;
                let address_result = self.wallet.lock().await.address(next_index)?;
                let address = address_result.address().clone();
                let index = address_result.index();
                debug!("Got unused address {address} with derivation index {index}");
                if next_index.is_none() {
                    self.persister.set_last_derivation_index(index)?;
                }
                address
            }
        };

        Ok(address)
    }

    /// Get the next unused change address in the wallet
    async fn next_unused_change_address(&self) -> Result<Address, PaymentError> {
        let address = self.wallet.lock().await.change(None)?.address().clone();

        Ok(address)
    }

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> u32 {
        self.wallet.lock().await.tip().height()
    }

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String> {
        Ok(self.signer.xpub()?.public_key.to_string())
    }

    /// Get the fingerprint of the wallet
    fn fingerprint(&self) -> Result<String> {
        Ok(self.signer.fingerprint()?.to_hex())
    }

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError> {
        // Scoped so the wallet and client locks drop before the repair re-acquires them.
        {
            debug!("LiquidOnchainWallet::full_scan: start");
            let full_scan_started = Instant::now();

            // create electrum client if doesn't already exist
            let mut client = self.client.lock().await;
            if client.is_none() {
                *client = Some(WalletClient::from_config(&self.config)?);
            }
            let client = client.as_mut().ok_or_else(|| PaymentError::Generic {
                err: "Wallet client not initialized".to_string(),
            })?;

            // Use the cached derivation index with a buffer of 5 to perform the scan
            let last_derivation_index = self
                .persister
                .get_last_derivation_index()?
                .unwrap_or_default();
            let index_with_buffer = last_derivation_index + 5;
            let mut wallet = self.wallet.lock().await;

            // Wipe at the *start* of a scan so this same scan repopulates before the sync runs
            // `update_wallet_info`; wiping after one would persist an empty balance.
            let clearing = self.needs_cache_clear.load(Ordering::Relaxed);
            // Guard before anything fallible: an early return here must still release the flag,
            // or tx building fails fast forever.
            let _recovery_guard = clearing.then(|| {
                warn!("Wiping the wallet cache; this scan will rebuild it from scratch");
                self.recovery_scan_in_progress
                    .store(true, Ordering::Relaxed);
                FlagGuard(&self.recovery_scan_in_progress)
            });
            if clearing {
                self.wallet_cache_persister.clear_cache().await?;
                *wallet = Self::create_wallet(
                    &self.config,
                    &self.signer,
                    self.wallet_cache_persister.clone(),
                )
                .await?;
                // Only now is the wipe complete. Clearing earlier would strand a wiped store
                // beside the old in-memory wallet if `create_wallet` failed.
                self.needs_cache_clear.store(false, Ordering::Relaxed);
                self.cache_wiped.store(true, Ordering::Relaxed);
                // Re-arm the check so this scan verifies its own result. Without it a wipe that
                // failed to produce a clean set would look identical to one that succeeded.
                self.needs_cache_check.store(true, Ordering::Relaxed);
            }

            // Reunblind the wallet txs if there has been a change in the derivation index since the
            // last full scan
            if self
                .persister
                .get_last_scanned_derivation_index()?
                .is_some_and(|index| index != last_derivation_index)
            {
                debug!("LiquidOnchainWallet::full_scan: reunblinding all transactions");
                wallet.reunblind()?;
            }

            let res: Result<(), PaymentError> = match client
                .full_scan_to_index(&mut wallet, index_with_buffer)
                .await
            {
                Ok(()) => Ok(()),
                Err(e)
                    if matches!(
                        e,
                        lwk_wollet::Error::UpdateHeightTooOld { .. }
                            | lwk_wollet::Error::UpdateOnDifferentStatus { .. }
                            | lwk_wollet::Error::StoreError(_)
                    ) =>
                {
                    warn!("Full scan failed due to {e}, reloading wallet and retrying");
                    let mut new_wallet = Self::create_wallet(
                        &self.config,
                        &self.signer,
                        self.wallet_cache_persister.clone(),
                    )
                    .await?;
                    let rescan_res = client
                        .full_scan_to_index(&mut new_wallet, index_with_buffer)
                        .await;
                    // Adopt the reloaded wallet even if the rescan failed: `create_wallet` may
                    // have wiped the cache, and a stale in-memory wallet would then persist a
                    // delta that cannot reconstruct it.
                    *wallet = new_wallet;
                    rescan_res?;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            };

            self.persister
                .set_last_scanned_derivation_index(last_derivation_index)?;

            let duration_ms = Instant::now().duration_since(full_scan_started).as_millis();
            info!("lwk wallet full_scan duration: ({duration_ms} ms)");
            debug!("LiquidOnchainWallet::full_scan: end");
            res?;
        }

        // Best-effort: a failed check must not abort the scan, or `sync_inner` returns before
        // syncing payments.
        if let Err(e) = self.check_and_repair_cache().await {
            error!("Wallet cache check failed, continuing: {e}");
        }
        Ok(())
    }

    async fn apply_broadcast_tx(&self, tx: &Transaction) {
        // Same mutex `full_scan` holds across scan + apply_update, which is what stops this
        // racing an update into UpdateOnDifferentStatus.
        let mut wallet = self.wallet.lock().await;
        match wallet.apply_transaction(tx.clone()) {
            Ok(_) => debug!("Applied broadcast tx {} to the wallet state", tx.txid()),
            Err(e) => warn!(
                "Could not apply broadcast tx {} to the wallet state: {e}",
                tx.txid()
            ),
        }
    }

    async fn repair_cache(&self) -> Result<bool, PaymentError> {
        use std::collections::HashSet;

        let mut wallet = self.wallet.lock().await;
        let txs = wallet.transactions()?;
        let stale: HashSet<OutPoint> = find_spent_utxos(&txs, &wallet.utxos()?)
            .into_iter()
            .collect();
        if stale.is_empty() {
            debug!("Wallet cache repair found nothing to do; the unspent set is unchanged");
            return Ok(true);
        }

        // Re-applying a spender drops its stale inputs but re-adds its own outputs, so if one of
        // those was itself already spent the drift just moves a hop down the chain. Walk the whole
        // chain first and re-apply it in one go, otherwise a chain of N spends needs N passes.
        let mut to_apply: Vec<&WalletTx> = vec![];
        let mut queued: HashSet<Txid> = HashSet::new();
        let mut frontier = stale.clone();
        while !frontier.is_empty() {
            let spenders: Vec<&WalletTx> = txs
                .iter()
                .filter(|wtx| !queued.contains(&wtx.txid))
                .filter(|wtx| {
                    wtx.tx
                        .input
                        .iter()
                        .any(|i| frontier.contains(&i.previous_output))
                })
                .collect();
            if spenders.is_empty() {
                break;
            }
            // The next hop is whatever those spenders paid back to us.
            frontier = spenders
                .iter()
                .flat_map(|wtx| wtx.outputs.iter().flatten().map(|o| o.outpoint))
                .collect();
            for wtx in spenders {
                queued.insert(wtx.txid);
                to_apply.push(wtx);
            }
        }

        if to_apply.is_empty() {
            warn!(
                "Wallet cache has {} stale utxo(s) with no known spender",
                stale.len()
            );
            self.schedule_cache_wipe();
            return Ok(false);
        }

        // Oldest first, so a child removes what its parent re-adds rather than the reverse.
        to_apply.sort_by_key(|wtx| wtx.height.unwrap_or(u32::MAX));

        warn!(
            "Repairing wallet cache locally: re-applying {} tx(s) that spend {} stale utxo(s)",
            to_apply.len(),
            stale.len()
        );
        for wtx in to_apply {
            let txid = wtx.txid;
            // Note this records the tx as unconfirmed; the next scan restores its real height.
            if let Err(e) = wallet.apply_transaction(wtx.tx.clone()) {
                warn!("Could not re-apply tx {txid} while repairing the wallet cache: {e}");
            }
        }

        let remaining = find_spent_utxos(&wallet.transactions()?, &wallet.utxos()?);
        if remaining.is_empty() {
            info!("Wallet cache repaired locally, no rescan needed");
            return Ok(true);
        }
        warn!(
            "Local wallet cache repair left {} utxo(s) unresolved",
            remaining.len()
        );
        self.schedule_cache_wipe();
        Ok(false)
    }

    fn sign_message(&self, message: &str) -> Result<String> {
        // Prefix and double hash message
        let mut engine = sha256::HashEngine::default();
        engine.write_all(LN_MESSAGE_PREFIX)?;
        engine.write_all(message.as_bytes())?;
        let hashed_msg = sha256::Hash::from_engine(engine);
        let double_hashed_msg = Message::from_digest(sha256::Hash::hash(&hashed_msg).into_inner());
        // Get message signature and encode to zbase32
        let recoverable_sig = self.signer.sign_ecdsa_recoverable(&double_hashed_msg)?;
        Ok(zbase32::encode_full_bytes(recoverable_sig.as_slice()))
    }

    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool> {
        let pk = PublicKey::from_str(pubkey)?;
        Ok(verify(message.as_bytes(), signature, &pk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Config;
    use crate::signer::SdkSigner;
    use crate::test_utils::persist::create_persister;
    use crate::wallet::LiquidOnchainWallet;
    use anyhow::Result;
    use lwk_common::SignedBalance;
    use lwk_wollet::elements::confidential::{AssetBlindingFactor, ValueBlindingFactor};
    use lwk_wollet::elements::{AssetId, TxIn, TxOutSecrets, Txid};
    use lwk_wollet::Chain;
    use std::collections::BTreeMap;

    fn test_asset() -> AssetId {
        AssetId::from_str("6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d")
            .unwrap()
    }

    /// A transaction spending `spends`, wrapped as a wallet tx.
    fn wallet_tx(spends: &[OutPoint]) -> WalletTx {
        let tx = Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: spends
                .iter()
                .map(|o| TxIn {
                    previous_output: *o,
                    ..Default::default()
                })
                .collect(),
            output: vec![],
        };
        WalletTx {
            txid: tx.txid(),
            tx,
            height: Some(1),
            balance: SignedBalance::from(BTreeMap::new()),
            fee: 0,
            type_: "outgoing".to_string(),
            timestamp: None,
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn wallet_utxo(outpoint: OutPoint) -> WalletTxOut {
        // p2wpkh so it renders to an address; neither is read by the invariant.
        let mut bytes = vec![0x00, 0x14];
        bytes.extend_from_slice(&[7u8; 20]);
        let script = lwk_wollet::elements::Script::from(bytes);
        let address =
            Address::from_script(&script, None, &lwk_wollet::elements::AddressParams::LIQUID)
                .expect("p2wpkh script should render to an address");
        WalletTxOut {
            outpoint,
            script_pubkey: script,
            height: Some(1),
            unblinded: TxOutSecrets::new(
                test_asset(),
                AssetBlindingFactor::zero(),
                1000,
                ValueBlindingFactor::zero(),
            ),
            wildcard_index: 0,
            ext_int: Chain::External,
            is_spent: false,
            address,
        }
    }

    /// `repair_cache` rests on this: applying a tx must remove the inputs it spends from
    /// lwk's unspent set. Against a real `Wollet`, not a mock.
    #[sdk_macros::async_test_all]
    async fn test_apply_transaction_drops_spent_inputs() -> Result<()> {
        use lwk_wollet::clients::LastUnused;
        use lwk_wollet::elements::bitcoin::bip32::ChildNumber;
        use lwk_wollet::elements::{BlockExtData, BlockHash, BlockHeader, TxMerkleNode};
        use lwk_wollet::hashes::Hash as _;
        use lwk_wollet::{DownloadTxResult, Update};

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(mnemonic, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;
        let mut w = wallet.wallet.lock().await;

        // Derive a real script from the wallet descriptor so it resolves back to an index.
        // `None` for the blinding pubkey: lwk derives it from the wallet descriptor.
        let script: lwk_wollet::elements::Script = {
            let desc = w.wollet_descriptor();
            desc.definite_descriptor(Chain::External, 0)?
                .script_pubkey()
        };
        let blinding_pubkey = None;

        // Non-zero blinding factors, or the output counts as explicit and `utxos()` drops it.
        let secrets = TxOutSecrets::new(
            test_asset(),
            AssetBlindingFactor::from_slice(&[3u8; 32])?,
            1000,
            ValueBlindingFactor::from_slice(&[4u8; 32])?,
        );

        let funding = Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: vec![],
            output: vec![lwk_wollet::elements::TxOut {
                script_pubkey: script.clone(),
                ..Default::default()
            }],
        };
        let funding_txid = funding.txid();
        let outpoint = OutPoint::new(funding_txid, 0);

        let wollet_status = w.status();
        w.apply_update(Update {
            version: 4,
            wollet_status,
            new_txs: DownloadTxResult {
                txs: vec![(funding_txid, funding)],
                unblinds: vec![(outpoint, secrets)],
            },
            txid_height_new: vec![(funding_txid, Some(1))],
            txid_height_delete: vec![],
            timestamps: vec![(1, 1)],
            scripts_with_blinding_pubkey: vec![(
                Chain::External,
                ChildNumber::from_normal_idx(0)?,
                script,
                blinding_pubkey,
            )],
            tip: BlockHeader {
                version: 0,
                prev_blockhash: BlockHash::all_zeros(),
                merkle_root: TxMerkleNode::all_zeros(),
                time: 0,
                height: 0,
                ext: BlockExtData::default(),
            },
            unspent: vec![],
            last_unused: LastUnused {
                internal: 0,
                external: 1,
            },
        })?;

        assert!(
            w.utxos()?.iter().any(|u| u.outpoint == outpoint),
            "the funding output should be unspent"
        );

        let spend = Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: outpoint,
                ..Default::default()
            }],
            output: vec![],
        };
        w.apply_transaction(spend)?;

        assert!(
            !w.utxos()?.iter().any(|u| u.outpoint == outpoint),
            "apply_transaction must drop the input it spends - repair_cache depends on it"
        );

        Ok(())
    }

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn descriptor_script(w: &Wollet) -> Result<lwk_wollet::elements::Script> {
        Ok(w.wollet_descriptor()
            .definite_descriptor(Chain::External, 0)?
            .script_pubkey())
    }

    fn default_tip() -> lwk_wollet::elements::BlockHeader {
        use lwk_wollet::elements::{BlockExtData, BlockHash, TxMerkleNode};
        use lwk_wollet::hashes::Hash as _;
        lwk_wollet::elements::BlockHeader {
            version: 0,
            prev_blockhash: BlockHash::all_zeros(),
            merkle_root: TxMerkleNode::all_zeros(),
            time: 0,
            height: 0,
            ext: BlockExtData::default(),
        }
    }

    fn tx_paying(script: &lwk_wollet::elements::Script) -> Transaction {
        Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: vec![],
            output: vec![lwk_wollet::elements::TxOut {
                script_pubkey: script.clone(),
                ..Default::default()
            }],
        }
    }

    /// A tx paying `sats` of the policy asset to `script`, plus the secrets it unblinds with.
    ///
    /// Unlike [`tx_paying`], the commitments are real and consistent with the secrets. The cache
    /// tests can leave them `Default::default()` because `extend_unblinded` does no crypto, but
    /// anything that *builds* a tx blinds and proves against them, so they have to be genuine.
    fn confidential_tx_paying(
        script: &lwk_wollet::elements::Script,
        policy_asset: AssetId,
        sats: u64,
        seed: u8,
    ) -> (Transaction, TxOutSecrets) {
        use lwk_wollet::elements::confidential::{Asset, Nonce, Value};
        use lwk_wollet::elements::secp256k1_zkp;

        let secp = secp256k1_zkp::Secp256k1::new();
        let asset_bf = AssetBlindingFactor::from_slice(&[seed | 0x01; 32]).unwrap();
        let value_bf = ValueBlindingFactor::from_slice(&[seed | 0x40; 32]).unwrap();
        let asset_gen = secp256k1_zkp::Generator::new_blinded(
            &secp,
            policy_asset.into_tag(),
            asset_bf.into_inner(),
        );
        let value_commit =
            secp256k1_zkp::PedersenCommitment::new(&secp, sats, value_bf.into_inner(), asset_gen);

        let tx = Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: vec![],
            output: vec![lwk_wollet::elements::TxOut {
                asset: Asset::Confidential(asset_gen),
                value: Value::Confidential(value_commit),
                nonce: Nonce::Null,
                script_pubkey: script.clone(),
                witness: Default::default(),
            }],
        };
        let secrets = TxOutSecrets {
            asset: policy_asset,
            value: sats,
            asset_bf,
            value_bf,
        };
        (tx, secrets)
    }

    /// Credits the wallet a spendable confirmed utxo of `sats`, returning its outpoint.
    fn fund(
        w: &mut Wollet,
        script: &lwk_wollet::elements::Script,
        sats: u64,
        seed: u8,
    ) -> Result<OutPoint> {
        use lwk_wollet::clients::LastUnused;
        use lwk_wollet::elements::bitcoin::bip32::ChildNumber;
        use lwk_wollet::{DownloadTxResult, Update};

        let (tx, secrets) = confidential_tx_paying(script, w.policy_asset(), sats, seed);
        let txid = tx.txid();
        let outpoint = OutPoint::new(txid, 0);
        let wollet_status = w.status();
        w.apply_update(Update {
            version: 4,
            wollet_status,
            new_txs: DownloadTxResult {
                txs: vec![(txid, tx)],
                unblinds: vec![(outpoint, secrets)],
            },
            txid_height_new: vec![(txid, Some(1))],
            txid_height_delete: vec![],
            timestamps: vec![(1, 1)],
            scripts_with_blinding_pubkey: vec![(
                Chain::External,
                ChildNumber::from_normal_idx(0)?,
                script.clone(),
                None,
            )],
            tip: default_tip(),
            unspent: vec![],
            last_unused: LastUnused {
                internal: 0,
                external: 1,
            },
        })?;
        Ok(outpoint)
    }

    fn tx_spending(outpoint: OutPoint) -> Transaction {
        Transaction {
            version: 2,
            lock_time: lwk_wollet::elements::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: outpoint,
                ..Default::default()
            }],
            output: vec![],
        }
    }

    fn has_utxo(w: &Wollet, outpoint: OutPoint) -> Result<bool> {
        Ok(w.utxos()?.iter().any(|u| u.outpoint == outpoint))
    }

    /// Applies `tx` with the given heights, unblinding any output paying `script`.
    fn apply(
        w: &mut Wollet,
        tx: &Transaction,
        heights: &[(Txid, Option<u32>)],
        script: &lwk_wollet::elements::Script,
    ) -> Result<()> {
        use lwk_wollet::clients::LastUnused;
        use lwk_wollet::elements::bitcoin::bip32::ChildNumber;
        use lwk_wollet::{DownloadTxResult, Update};

        let txid = tx.txid();
        let unblinds = tx
            .output
            .iter()
            .enumerate()
            .filter(|(_, o)| &o.script_pubkey == script)
            .map(|(vout, _)| {
                (
                    OutPoint::new(txid, vout as u32),
                    TxOutSecrets::new(
                        test_asset(),
                        AssetBlindingFactor::from_slice(&[3u8; 32]).unwrap(),
                        1000,
                        ValueBlindingFactor::from_slice(&[4u8; 32]).unwrap(),
                    ),
                )
            })
            .collect();

        let wollet_status = w.status();
        w.apply_update(Update {
            version: 4,
            wollet_status,
            new_txs: DownloadTxResult {
                txs: vec![(txid, tx.clone())],
                unblinds,
            },
            txid_height_new: heights.to_vec(),
            txid_height_delete: vec![],
            timestamps: vec![(1, 1)],
            scripts_with_blinding_pubkey: vec![(
                Chain::External,
                ChildNumber::from_normal_idx(0)?,
                script.clone(),
                None,
            )],
            tip: default_tip(),
            unspent: vec![],
            last_unused: LastUnused {
                internal: 0,
                external: 1,
            },
        })?;
        Ok(())
    }

    /// A height-only delta, as a scan produces when a known tx confirms.
    fn apply_heights_only(w: &mut Wollet, heights: &[(Txid, Option<u32>)]) -> Result<()> {
        use lwk_wollet::clients::LastUnused;
        use lwk_wollet::{DownloadTxResult, Update};
        let wollet_status = w.status();
        w.apply_update(Update {
            version: 4,
            wollet_status,
            new_txs: DownloadTxResult {
                txs: vec![],
                unblinds: vec![],
            },
            txid_height_new: heights.to_vec(),
            txid_height_delete: vec![],
            timestamps: vec![(1, 1)],
            scripts_with_blinding_pubkey: vec![],
            tip: default_tip(),
            unspent: vec![],
            last_unused: LastUnused {
                internal: 0,
                external: 1,
            },
        })?;
        Ok(())
    }

    /// Re-applying a spender re-adds its own outputs, so if one of those was already spent the
    /// drift moves one hop down the chain. Raised in review: the repair must iterate, not give up
    /// after a single pass and schedule a wipe.
    #[sdk_macros::async_test_all]
    async fn test_repair_cache_resolves_a_chain_of_spends() -> Result<()> {
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;

        let (a0, b0) = {
            let mut w = wallet.wallet.lock().await;
            let script = descriptor_script(&w)?;

            // A funds us; B spends A:0 and pays us change; C spends that change.
            let a = tx_paying(&script);
            let a0 = OutPoint::new(a.txid(), 0);
            apply(&mut w, &a, &[(a.txid(), None)], &script)?;

            let mut b = tx_paying(&script);
            b.input = vec![TxIn {
                previous_output: a0,
                ..Default::default()
            }];
            let b0 = OutPoint::new(b.txid(), 0);
            apply(&mut w, &b, &[(b.txid(), None)], &script)?;

            let c = tx_spending(b0);
            apply(&mut w, &c, &[(c.txid(), None)], &script)?;

            assert!(!has_utxo(&w, a0)?, "A:0 is spent by B");
            assert!(!has_utxo(&w, b0)?, "B:0 is spent by C");

            // A confirms, resurrecting A:0 (the lwk bug).
            apply_heights_only(&mut w, &[(a.txid(), Some(1))])?;
            assert!(has_utxo(&w, a0)?, "A:0 resurrected");
            (a0, b0)
        };

        // One pass would drop A:0 but re-add B:0, which C spends. Only iterating resolves both.
        assert!(
            wallet.repair_cache().await?,
            "the repair should resolve the chain without needing a wipe"
        );

        let w = wallet.wallet.lock().await;
        assert!(!has_utxo(&w, a0)?, "A:0 must not survive the repair");
        assert!(
            !has_utxo(&w, b0)?,
            "B:0 must not be left behind by the repair"
        );
        assert!(find_spent_utxos(&w.transactions()?, &w.utxos()?).is_empty());
        Ok(())
    }

    /// A long chain of change spends. Each re-applied spender re-adds its own change, which the
    /// next spender consumes, so a hop-by-hop repair needs one pass per hop and stalls on chains
    /// longer than its pass limit. Seen in the field as 16 identical passes over 34 stale utxos
    /// before falling back to a wipe. The repair must walk the whole chain in one go.
    #[sdk_macros::async_test_all]
    async fn test_repair_cache_resolves_a_long_chain_in_one_pass() -> Result<()> {
        const CHAIN: usize = 25; // longer than any sane pass limit

        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;

        let first_outpoint = {
            let mut w = wallet.wallet.lock().await;
            let script = descriptor_script(&w)?;

            // tx0 pays us; each following tx spends the previous change and pays us again.
            let tx0 = tx_paying(&script);
            let first = OutPoint::new(tx0.txid(), 0);
            apply(&mut w, &tx0, &[(tx0.txid(), None)], &script)?;

            let mut prev = first;
            for _ in 0..CHAIN {
                let mut next = tx_paying(&script);
                next.input = vec![TxIn {
                    previous_output: prev,
                    ..Default::default()
                }];
                apply(&mut w, &next, &[(next.txid(), None)], &script)?;
                prev = OutPoint::new(next.txid(), 0);
            }

            // Only the tip of the chain should be unspent.
            assert!(!has_utxo(&w, first)?, "the head of the chain is spent");
            assert!(has_utxo(&w, prev)?, "the tip of the chain is unspent");

            // Confirm tx0, resurrecting its already-spent output at the head of the chain.
            apply_heights_only(&mut w, &[(tx0.txid(), Some(1))])?;
            assert!(has_utxo(&w, first)?, "head resurrected");
            first
        };

        assert!(
            wallet.repair_cache().await?,
            "a {CHAIN}-hop chain must resolve without falling back to a wipe"
        );

        let w = wallet.wallet.lock().await;
        assert!(!has_utxo(&w, first_outpoint)?);
        assert!(find_spent_utxos(&w.transactions()?, &w.utxos()?).is_empty());
        Ok(())
    }

    /// Drain hands the whole job to lwk's `drain_lbtc_wallet()` and never consults
    /// `select_wallet_utxos`, so it must keep spending every utxo regardless of what our own
    /// coin selection would have picked.
    #[sdk_macros::async_test_all]
    async fn test_build_drain_tx_spends_every_utxo() -> Result<()> {
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;

        let (script, recipient) = {
            let w = wallet.wallet.lock().await;
            (
                descriptor_script(&w)?,
                w.address(Some(1))?.address().clone(),
            )
        };

        // A spread that our own selector would never take whole: a 21 sat send would pick one.
        let funded = {
            let mut w = wallet.wallet.lock().await;
            let outpoints = [
                fund(&mut w, &script, 100_000, 1)?,
                fund(&mut w, &script, 50_000, 2)?,
                fund(&mut w, &script, 25_000, 3)?,
            ];
            assert_eq!(w.utxos()?.len(), 3, "wallet should hold the 3 funded utxos");
            outpoints
        };

        let tx = wallet
            .build_drain_tx(None, &recipient.to_string(), None)
            .await?;

        assert_eq!(
            tx.input.len(),
            funded.len(),
            "a drain must spend every utxo, not a selected subset"
        );
        for outpoint in funded {
            assert!(
                tx.input.iter().any(|i| i.previous_output == outpoint),
                "drain tx is missing utxo {outpoint}"
            );
        }
        // The drain output reuses the change slot, so there is no third output to change into.
        assert_eq!(tx.output.len(), 2, "expected the drain output and the fee");
        let fee = tx.output.iter().find(|o| o.is_fee()).expect("a fee output");
        assert!(
            fee.value.explicit().is_some_and(|sats| sats > 0),
            "the fee must be explicit and non-zero, so the tx actually balanced"
        );

        // Contrast: an ordinary send of the same funds goes through `select_wallet_utxos` and
        // takes a subset. Without this the drain assertion would pass on any built tx.
        let ordinary = wallet
            .build_tx(
                None,
                &recipient.to_string(),
                &wallet.config.lbtc_asset_id(),
                1_000,
            )
            .await?;
        assert!(
            ordinary.input.len() < funded.len(),
            "a 1000 sat send should select a subset, got {} of {} inputs",
            ordinary.input.len(),
            funded.len()
        );
        Ok(())
    }

    /// `enforce_amount_sat` is the branch `build_tx_or_drain_tx` falls back to when an ordinary
    /// send cannot be built, and it only passes when the drained amount matches exactly. Getting
    /// the arithmetic wrong turns an unbuildable tx into a bare "not enough funds".
    #[sdk_macros::async_test_all]
    async fn test_build_drain_tx_enforces_the_exact_amount() -> Result<()> {
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;

        let (script, recipient) = {
            let w = wallet.wallet.lock().await;
            (
                descriptor_script(&w)?,
                w.address(Some(1))?.address().clone(),
            )
        };
        let total = 175_000;
        {
            let mut w = wallet.wallet.lock().await;
            fund(&mut w, &script, 100_000, 1)?;
            fund(&mut w, &script, 50_000, 2)?;
            fund(&mut w, &script, 25_000, 3)?;
        }
        let recipient = recipient.to_string();

        // Learn the fee from an unconstrained drain, so the expected amount is derived rather
        // than hardcoded to a fee model that may change.
        let fee = wallet
            .build_drain_tx(None, &recipient, None)
            .await?
            .output
            .iter()
            .find(|o| o.is_fee())
            .and_then(|o| o.value.explicit())
            .expect("an explicit fee output");
        let drained = total - fee;

        wallet
            .build_drain_tx(None, &recipient, Some(drained))
            .await
            .unwrap_or_else(|e| {
                panic!("enforcing the actual drained amount {drained} failed: {e}")
            });

        // Off by one in either direction must be rejected, not silently drained.
        for wrong in [drained - 1, drained + 1] {
            let err = match wallet.build_drain_tx(None, &recipient, Some(wrong)).await {
                // Report the txid rather than the tx, which Debug-prints every rangeproof.
                Ok(tx) => panic!(
                    "enforcing {wrong} must fail, but it built {} draining {drained}",
                    tx.txid()
                ),
                Err(e) => e,
            };
            assert!(
                err.to_string().contains("doesn't match enforce_amount_sat"),
                "expected an enforce mismatch for {wrong}, got: {err}"
            );
        }
        Ok(())
    }

    /// A wipe costs a cold rescan (minutes on a large wallet, 221s when measured against the
    /// affected one), so drift a rescan cannot resolve must not re-trigger it on every scan.
    #[sdk_macros::async_test_all]
    async fn test_cache_wipe_is_bounded_to_once_per_session() -> Result<()> {
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;

        assert!(wallet.schedule_cache_wipe(), "the first wipe is allowed");
        assert!(wallet.needs_cache_clear.load(Ordering::Relaxed));

        // Stand in for `full_scan` performing the wipe.
        wallet.needs_cache_clear.store(false, Ordering::Relaxed);
        wallet.cache_wiped.store(true, Ordering::Relaxed);

        assert!(
            !wallet.schedule_cache_wipe(),
            "a second wipe in the same session must be refused"
        );
        assert!(
            !wallet.needs_cache_clear.load(Ordering::Relaxed),
            "a refused wipe must not leave the scan flagged, or every scan would rescan cold"
        );
        Ok(())
    }

    /// `Cache::update_unspent` re-adds the outputs of every tx in `txid_height_new`
    /// filtered only by txid. It never checks whether a tx already in the cache
    /// spends them. So a funding tx confirming *after* the tx spending its output was recorded
    /// resurrects that output. Which is what happens whenever unconfirmed change is spent.
    #[sdk_macros::async_test_all]
    async fn test_lwk_resurrects_outputs_of_reannounced_funding_tx() -> Result<()> {
        let signer: Arc<Box<dyn Signer>> =
            Arc::new(Box::new(SdkSigner::new(TEST_MNEMONIC, "", false).unwrap()));
        create_persister!(storage);
        let wallet =
            LiquidOnchainWallet::new(Config::regtest_esplora(), storage, signer.clone()).await?;
        let mut w = wallet.wallet.lock().await;
        let script = descriptor_script(&w)?;

        // 1. Funding tx A arrives unconfirmed, paying us.
        let funding = tx_paying(&script);
        let a0 = OutPoint::new(funding.txid(), 0);
        apply(&mut w, &funding, &[(funding.txid(), None)], &script)?;
        assert!(
            has_utxo(&w, a0)?,
            "A:0 should be spendable while unconfirmed"
        );

        // 2. We spend that unconfirmed output. B is recorded, A:0 correctly leaves the set.
        let spend = tx_spending(a0);
        apply(&mut w, &spend, &[(spend.txid(), None)], &script)?;
        assert!(!has_utxo(&w, a0)?, "A:0 should be spent by B");

        // 3. A confirms. Only A is in this delta, so B is never consulted.
        apply_heights_only(&mut w, &[(funding.txid(), Some(1))])?;

        let resurrected = has_utxo(&w, a0)?;
        let spender_known = w.transactions()?.iter().any(|t| t.txid == spend.txid());

        // Asserts the bug is still PRESENT, so this fails if lwk ever fixes it. That is the
        // point: it is the signal to revisit the recovery code, not a regression in this crate.
        assert!(
            resurrected && spender_known,
            "lwk no longer resurrects outputs of a re-announced funding tx \
             (resurrected={resurrected}, spender_known={spender_known}). If this failed after an \
             lwk bump the upstream bug is likely fixed, so reassess whether repair_cache and \
             check_and_repair_cache are still needed."
        );
        assert_eq!(
            find_spent_utxos(&w.transactions()?, &w.utxos()?),
            vec![a0],
            "and our invariant should catch it"
        );
        Ok(())
    }

    /// The invariant that drives both detection and repair: a utxo the wallet still lists as
    /// unspent, but which a transaction it already knows about spends, is stale.
    #[sdk_macros::test_all]
    fn test_find_spent_utxos() {
        let a = OutPoint::new(Txid::from_str(&"a".repeat(64)).unwrap(), 0);
        let b = OutPoint::new(Txid::from_str(&"b".repeat(64)).unwrap(), 1);

        // Healthy: nothing the wallet holds spends either utxo.
        let unrelated = wallet_tx(&[OutPoint::new(Txid::from_str(&"c".repeat(64)).unwrap(), 0)]);
        assert!(find_spent_utxos(
            std::slice::from_ref(&unrelated),
            &[wallet_utxo(a), wallet_utxo(b)]
        )
        .is_empty());

        // Corrupt: `a` is still listed as unspent although a known tx spends it.
        let spender = wallet_tx(&[a]);
        assert_eq!(
            find_spent_utxos(
                &[unrelated.clone(), spender.clone()],
                &[wallet_utxo(a), wallet_utxo(b)]
            ),
            vec![a],
            "a utxo spent by a known tx must be reported"
        );

        // An empty utxo set cannot be corrupt, however many spends are known.
        assert!(find_spent_utxos(&[spender], &[]).is_empty());
    }

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_sign_and_check_message() -> Result<()> {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let sdk_signer: Box<dyn Signer> = Box::new(SdkSigner::new(mnemonic, "", false).unwrap());
        let sdk_signer = Arc::new(sdk_signer);

        let config = Config::regtest_esplora();

        create_persister!(storage);

        let wallet: Arc<dyn OnchainWallet> = Arc::new(
            LiquidOnchainWallet::new(config, storage, sdk_signer.clone())
                .await
                .unwrap(),
        );

        // Test message
        let message = "Hello, Liquid!";

        // Sign the message
        let signature = wallet.sign_message(message).unwrap();

        // Get the public key
        let pubkey = wallet.pubkey().unwrap();

        // Check the message
        let is_valid = wallet.check_message(message, &pubkey, &signature).unwrap();
        assert!(is_valid, "Message signature should be valid");

        // Check with an incorrect message
        let incorrect_message = "Wrong message";
        let is_invalid = wallet
            .check_message(incorrect_message, &pubkey, &signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect message"
        );

        // Check with an incorrect public key
        let incorrect_pubkey = "02a1633cafcc01ebfb6d78e39f687a1f0995c62fc95f51ead10a02ee0be551b5dc";
        let is_invalid = wallet
            .check_message(message, incorrect_pubkey, &signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect public key"
        );

        // Check with an incorrect signature
        let incorrect_signature = zbase32::encode_full_bytes(&[0; 65]);
        let is_invalid = wallet
            .check_message(message, &pubkey, &incorrect_signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect signature"
        );

        // The temporary directory will be automatically deleted when temp_dir goes out of scope
        Ok(())
    }
}
