use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{anyhow, Result};
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

pub(crate) struct HybridLiquidChainService {
    client: OnceLock<Client>,
    config: Config,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            client: OnceLock::new(),
        })
    }

    fn get_client(&self) -> Result<&Client> {
        if let Some(c) = self.client.get() {
            return Ok(c);
        }
        let electrum_url = ElectrumUrl::new(&self.config.liquid_electrum_url, true, true)?;
        let client = electrum_url.build_client(&ElectrumOptions { timeout: Some(3) })?;

        let client = self.client.get_or_init(|| client);
        Ok(client)
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        let client = self.get_client()?;
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

        let tip: u32 = new_tip.ok_or_else(|| anyhow!("Failed to get tip"))?;
        Ok(tip)
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self
            .get_client()?
            .transaction_broadcast_raw(&elements_serialize(tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>> {
        Ok(self.get_transactions(&[*txid]).await?.first().cloned())
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let txids: Vec<bitcoin::Txid> = txids
            .iter()
            .map(|t| bitcoin::Txid::from_raw_hash(t.to_raw_hash()))
            .collect();

        let mut result = vec![];
        for tx in self.get_client()?.batch_transaction_get_raw(&txids)? {
            let tx: Transaction = elements::encode::deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        let scripts = &[script];
        let scripts: Vec<&bitcoin::Script> = scripts
            .iter()
            .map(|t| bitcoin::Script::from_bytes(t.as_bytes()))
            .collect();

        let mut history_vec: Vec<Vec<History>> = self
            .get_client()?
            .batch_script_get_history(&scripts)?
            .into_iter()
            .map(|e| e.into_iter().map(Into::into).collect())
            .collect();
        let h = history_vec.pop();
        Ok(h.unwrap_or_default())
    }

    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        let scripts: Vec<&bitcoin::Script> = scripts
            .iter()
            .map(|t| bitcoin::Script::from_bytes(t.as_bytes()))
            .collect();

        Ok(self
            .get_client()?
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
