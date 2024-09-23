use std::io::Write;
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::ElementsAddress;
use lwk_common::Signer as LwkSigner;
use lwk_common::{singlesig_desc, Singlesig};
use lwk_wollet::{
    elements::{Address, Transaction},
    ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister, Tip, WalletTx, Wollet,
    WolletDescriptor,
};
use sdk_common::bitcoin::hashes::{sha256, Hash};
use sdk_common::bitcoin::secp256k1::PublicKey;
use sdk_common::lightning::util::message_signing::verify;
use tokio::sync::Mutex;

use crate::model::Signer;
use crate::signer::SdkLwkSigner;
use crate::{
    ensure_sdk,
    error::PaymentError,
    model::{Config, LiquidNetwork},
};
use lwk_wollet::secp256k1::Message;

static LN_MESSAGE_PREFIX: &[u8] = b"Lightning Signed Message:";

#[async_trait]
pub trait OnchainWallet: Send + Sync {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError>;

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
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

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError>;

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> Tip;

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String>;

    /// Sign given message with the wallet private key. Returns a zbase
    /// encoded signature.
    fn sign_message(&self, msg: &str) -> Result<String>;

    /// Check whether given message was signed by the given
    /// pubkey and the signature (zbase encoded) is valid.
    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool>;

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError>;
}

pub(crate) struct LiquidOnchainWallet {
    wallet: Arc<Mutex<Wollet>>,
    config: Config,
    pub(crate) signer: SdkLwkSigner,
}

impl LiquidOnchainWallet {
    pub(crate) fn new(
        user_signer: Arc<Box<dyn Signer>>,
        config: Config,
        working_dir: &String,
    ) -> Result<Self> {
        let signer = crate::signer::SdkLwkSigner::new(user_signer)?;
        let descriptor = LiquidOnchainWallet::get_descriptor(&signer, config.network)?;
        let elements_network: ElementsNetwork = config.network.into();

        let lwk_persister = FsPersister::new(working_dir, elements_network, &descriptor)?;
        let wollet = Wollet::new(elements_network, lwk_persister, descriptor)?;
        Ok(Self {
            wallet: Arc::new(Mutex::new(wollet)),
            signer,
            config,
        })
    }

    fn get_descriptor(
        signer: &SdkLwkSigner,
        network: LiquidNetwork,
    ) -> Result<WolletDescriptor, PaymentError> {
        let is_mainnet = network == LiquidNetwork::Mainnet;
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
    }
}

#[async_trait]
impl OnchainWallet for LiquidOnchainWallet {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError> {
        let wallet = self.wallet.lock().await;
        wallet.transactions().map_err(|e| PaymentError::Generic {
            err: format!("Failed to fetch wallet transactions: {e:?}"),
        })
    }

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.wallet.lock().await;
        let mut pset = lwk_wollet::TxBuilder::new(self.config.network.into())
            .add_lbtc_recipient(
                &ElementsAddress::from_str(recipient_address).map_err(|e| {
                    PaymentError::Generic {
                        err: format!(
                      "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                  ),
                    }
                })?,
                amount_sat,
            )?
            .fee_rate(fee_rate_sats_per_kvb)
            .finish(&lwk_wollet)?;
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
            .finish()?;

        if let Some(enforce_amount_sat) = enforce_amount_sat {
            let pset_details = lwk_wollet.get_details(&pset)?;
            let pset_balance_sat = pset_details
                .balance
                .balances
                .get(&lwk_wollet.policy_asset())
                .unwrap_or(&0);
            let pset_fees = pset_details.balance.fee;

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

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError> {
        Ok(self.wallet.lock().await.address(None)?.address().clone())
    }

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> Tip {
        self.wallet.lock().await.tip()
    }

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String> {
        Ok(self.signer.xpub()?.public_key.to_string())
    }

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError> {
        let mut wallet = self.wallet.lock().await;
        let mut electrum_client = ElectrumClient::new(&ElectrumUrl::new(
            &self.config.liquid_electrum_url,
            true,
            true,
        ))?;
        lwk_wollet::full_scan_with_electrum_client(&mut wallet, &mut electrum_client)?;
        Ok(())
    }

    fn sign_message(&self, message: &str) -> Result<String> {
        // Prefix and double hash message
        let mut engine = sha256::HashEngine::default();
        engine.write_all(LN_MESSAGE_PREFIX)?;
        engine.write_all(message.as_bytes())?;
        let hashed_msg = sha256::Hash::from_engine(engine);
        let double_hashed_msg = Message::from_digest(hashed_msg.into_inner());
        // Get message signature and encode to zbase32
        let recoverable_sig = self.signer.sign_ecdsa_recoverable(&double_hashed_msg)?;
        Ok(zbase32::encode_full_bytes(recoverable_sig.as_slice()))
    }

    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool> {
        let pk = PublicKey::from_str(pubkey)?;
        Ok(verify(message.as_bytes(), signature, &pk))
    }
}
