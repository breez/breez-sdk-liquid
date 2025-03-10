use std::time::Duration;

use crate::prelude::*;

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use elements::{hex::FromHex, Address, OutPoint, Script, Transaction, Txid};
use log::info;
use lwk_wollet::hashes::{sha256, Hash};
use sdk_common::bitcoin::hashes::hex::ToHex;
use tokio::sync::{Mutex, RwLock};

use crate::get_client;
use crate::prelude::Utxo;
use crate::{model::Config, utils};

use super::client::LiquidClient;

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

    /// Get the transactions involved in a list of scripts.
    ///
    /// The data is fetched in a single call from the Electrum endpoint.
    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History<Txid>>>>;

    /// Get the transactions involved in a list of scripts
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History<Txid>>>;

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

pub(crate) struct HybridLiquidChainService {
    config: Config,
    last_known_tip: Mutex<Option<u32>>,
    client: RwLock<Option<LiquidClient>>,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            client: RwLock::new(None),
            last_known_tip: Mutex::new(None),
        })
    }

    fn init_client(&self) -> Result<LiquidClient> {
        for explorer in &self.config.liquid_explorers {
            if let Ok(client) = LiquidClient::try_from_explorer(explorer, self.config.network) {
                if client.is_available() {
                    return Ok(client);
                }
            }
        }

        bail!("Could not create Liquid chain service client: no working clients found");
    }

    async fn set_client(&self) -> Result<()> {
        let lock = self.client.read().await;
        if let Some(client) = lock.as_ref() {
            if client.is_available() {
                return Ok(());
            }
        }

        let mut lock = self.client.write().await;
        *lock = Some(self.init_client()?);
        Ok(())
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        self.set_client().await?;

        let mut lock = self.client.write().await;
        let Some(client) = lock.as_mut() else {
            bail!("Liquid client not set"); // unreachable
        };

        let new_tip = client.tip().await.ok().map(|t| t.height);
        let mut last_tip = self.last_known_tip.lock().await;
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
        client.broadcast(tx).await
    }

    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>> {
        Ok(self.get_transactions(&[*txid]).await?.first().cloned())
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        get_client!(self, client);
        client.get_transactions(txids).await
    }

    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History<Txid>>>> {
        get_client!(self, client);
        Ok(client
            .get_scripts_history(scripts)
            .await?
            .into_iter()
            .map(|h| h.into_iter().map(Into::into).collect())
            .collect())
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History<Txid>>> {
        let script_hash = sha256::Hash::hash(script.as_bytes())
            .to_byte_array()
            .to_hex();
        info!("Fetching script history for {}", script_hash);

        let mut retry = 0;
        while retry <= retries {
            let history = self
                .get_scripts_history(&[script])
                .await?
                .into_iter()
                .nth(0);

            match history.as_ref().is_some_and(|h| !h.is_empty()) {
                true => return Ok(history.unwrap()),
                false => {
                    retry += 1;
                    info!("Script history for {script_hash} is empty, retrying in 1 second... ({retry} of {retries})");
                    // Waiting 1s between retries, so we detect the new tx as soon as possible
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Ok(vec![])
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
