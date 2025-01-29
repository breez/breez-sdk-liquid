use std::sync::{Mutex, RwLock, RwLockReadGuard};
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use boltz_client::ToHex;
use electrum_client::{Client, ElectrumApi};
use elements::encode::serialize as elements_serialize;
use log::info;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::{bitcoin, elements, ElectrumOptions};
use lwk_wollet::{
    elements::{Address, OutPoint, Script, Transaction, Txid},
    hashes::{sha256, Hash},
    ElectrumUrl, History,
};

use crate::model::LiquidNetwork;
use crate::prelude::Utxo;
use crate::{model::Config, utils};

#[async_trait]
pub trait LiquidChainService: Send + Sync {
    /// Get the blockchain latest block
    async fn tip(&self) -> Result<u32>;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a single transaction from its raw hash
    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>>;

    /// Get a list of transactions
    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved in a script
    async fn get_script_history(&self, scripts: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts.
    ///
    /// The data is fetched in a single call from the Electrum endpoint.
    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>>;

    /// Get the transactions involved in a list of scripts
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

    /// Get the utxos associated with a script pubkey
    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;
}

macro_rules! get_client {
    ($chain_service:ident,$client:ident) => {
        let lock = $chain_service.get_client()?;
        let Some($client) = lock.as_ref() else {
            bail!("Could not read Liquid electrum client");
        };
    };
}

pub(crate) struct HybridLiquidChainService {
    client: RwLock<Option<Client>>,
    config: Config,
    last_known_tip: Mutex<Option<u32>>,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            client: RwLock::new(None),
            last_known_tip: Mutex::new(None),
        })
    }

    fn build_client(&self, url: &str, options: &ElectrumOptions) -> Result<Client> {
        let (tls, validate_domain) = match self.config.network {
            LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
            LiquidNetwork::Regtest => (false, false),
        };
        let electrum_url = ElectrumUrl::new(url, tls, validate_domain)?;
        Ok(electrum_url.build_client(options)?)
    }

    fn get_client(&self) -> Result<RwLockReadGuard<Option<Client>>> {
        let lock = self.client.read().unwrap();
        if let Some(client) = lock.as_ref() {
            if client.ping().is_ok() {
                return Ok(lock);
            }
        }

        let mut lock = self.client.write().unwrap();
        for electrum_url in &self.config.liquid_electrum_explorers() {
            if let Ok(electrum_client) =
                self.build_client(electrum_url, &ElectrumOptions { timeout: Some(3) })
            {
                if electrum_client.ping().is_ok() {
                    *lock = Some(electrum_client);
                    drop(lock);
                    return Ok(self.client.read().unwrap());
                }
            }
        }

        bail!("Could not create Liquid electrum client: no url specified in the configuration");
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        get_client!(self, client);
        let mut maybe_popped_header = None;
        while let Some(header) = client.block_headers_pop_raw()? {
            maybe_popped_header = Some(header)
        }

        let new_tip: Option<u32> = match maybe_popped_header {
            Some(popped_header) => Some(popped_header.height.try_into()?),
            None => {
                // https://github.com/bitcoindevkit/rust-electrum-client/issues/124
                // It might be that the client has reconnected and subscriptions don't persist
                // across connections. Calling `client.ping()` won't help here because the
                // successful retry will prevent us knowing about the reconnect.
                if let Ok(header) = client.block_headers_subscribe_raw() {
                    Some(header.height.try_into()?)
                } else {
                    None
                }
            }
        };

        let mut last_tip: std::sync::MutexGuard<'_, Option<u32>> =
            self.last_known_tip.lock().unwrap();
        match new_tip {
            Some(height) => {
                *last_tip = Some(height);
                Ok(height)
            }
            None => last_tip.ok_or_else(|| anyhow!("Failed to get tip")),
        }
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        get_client!(self, client);
        let txid = client.transaction_broadcast_raw(&elements_serialize(tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>> {
        Ok(self.get_transactions(&[*txid]).await?.first().cloned())
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        get_client!(self, client);
        let txids: Vec<bitcoin::Txid> = txids
            .iter()
            .map(|t| bitcoin::Txid::from_raw_hash(t.to_raw_hash()))
            .collect();

        let mut result = vec![];
        for tx in client.batch_transaction_get_raw(&txids)? {
            let tx: Transaction = elements::encode::deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        get_client!(self, client);
        let scripts = &[script];
        let scripts: Vec<&bitcoin::Script> = scripts
            .iter()
            .map(|t| bitcoin::Script::from_bytes(t.as_bytes()))
            .collect();

        let mut history_vec: Vec<Vec<History>> = client
            .batch_script_get_history(&scripts)?
            .into_iter()
            .map(|e| e.into_iter().map(Into::into).collect())
            .collect();
        let h = history_vec.pop();
        Ok(h.unwrap_or_default())
    }

    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        get_client!(self, client);
        let scripts: Vec<&bitcoin::Script> = scripts
            .iter()
            .map(|t| bitcoin::Script::from_bytes(t.as_bytes()))
            .collect();

        Ok(client
            .batch_script_get_history(&scripts)?
            .into_iter()
            .map(|e| e.into_iter().map(Into::into).collect())
            .collect())
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>> {
        let script_hash = sha256::Hash::hash(script.as_bytes())
            .to_byte_array()
            .to_hex();
        info!("Fetching script history for {}", script_hash);
        let mut script_history = vec![];

        let mut retry = 0;
        while retry <= retries {
            script_history = self.get_script_history(script).await?;
            match script_history.is_empty() {
                true => {
                    retry += 1;
                    info!("Script history for {script_hash} is empty, retrying in 1 second... ({retry} of {retries})");
                    // Waiting 1s between retries, so we detect the new tx as soon as possible
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                false => break,
            }
        }
        Ok(script_history)
    }

    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>> {
        let history = self.get_script_history_with_retry(script, 10).await?;

        let mut utxos: Vec<Utxo> = vec![];
        for history_item in history {
            match self.get_transaction_hex(&history_item.txid).await {
                Ok(Some(tx)) => {
                    let mut new_utxos = tx
                        .output
                        .iter()
                        .enumerate()
                        .map(|(vout, output)| {
                            Utxo::Liquid(Box::new((
                                OutPoint::new(history_item.txid, vout as u32),
                                output.clone(),
                            )))
                        })
                        .collect();
                    utxos.append(&mut new_utxos);
                }
                _ => {
                    log::warn!("Could not retrieve transaction from history item");
                    continue;
                }
            }
        }

        return Ok(utxos);
    }

    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction> {
        let script = Script::from_hex(
            hex::encode(address.to_unconfidential().script_pubkey().as_bytes()).as_str(),
        )
        .map_err(|e| anyhow!("Failed to get script from address {e:?}"))?;

        let script_history = self.get_script_history_with_retry(&script, 30).await?;
        let lockup_tx_history = script_history.iter().find(|h| h.txid.to_hex().eq(tx_id));

        match lockup_tx_history {
            Some(history) => {
                info!("Liquid transaction found, verifying transaction content...");
                let tx: Transaction = utils::deserialize_tx_hex(tx_hex)?;
                if !tx.txid().to_hex().eq(&history.txid.to_hex()) {
                    return Err(anyhow!(
                        "Liquid transaction id and hex do not match: {} vs {}",
                        tx_id,
                        tx.txid().to_hex()
                    ));
                }

                if verify_confirmation && history.height <= 0 {
                    return Err(anyhow!(
                        "Liquid transaction was not confirmed, txid={} waiting for confirmation",
                        tx_id,
                    ));
                }
                Ok(tx)
            }
            None => Err(anyhow!(
                "Liquid transaction was not found, txid={} waiting for broadcast",
                tx_id,
            )),
        }
    }
}
