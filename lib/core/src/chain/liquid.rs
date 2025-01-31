use std::sync::Mutex;
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::ToHex;
use log::info;
use lwk_wollet::clients::blocking::BlockchainBackend;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::ElectrumOptions;
use lwk_wollet::{
    elements::{Address, OutPoint, Script, Transaction, Txid},
    hashes::{sha256, Hash},
    ElectrumClient, ElectrumUrl, History,
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

pub(crate) struct HybridLiquidChainService {
    electrum_client: ElectrumClient,
    tip_client: Mutex<ElectrumClient>,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        let (tls, validate_domain) = match config.network {
            LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
            LiquidNetwork::Regtest => (false, false),
        };
        let electrum_url = ElectrumUrl::new(&config.liquid_electrum_url, tls, validate_domain)?;
        let electrum_client =
            ElectrumClient::with_options(&electrum_url, ElectrumOptions { timeout: Some(3) })?;
        let tip_client =
            ElectrumClient::with_options(&electrum_url, ElectrumOptions { timeout: Some(3) })?;
        Ok(Self {
            electrum_client,
            tip_client: Mutex::new(tip_client),
        })
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        Ok(self.tip_client.lock().unwrap().tip()?.height)
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        Ok(self.electrum_client.broadcast(tx)?)
    }

    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>> {
        Ok(self.get_transactions(&[*txid]).await?.first().cloned())
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(self.electrum_client.get_transactions(txids)?)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        let mut history_vec = self.electrum_client.get_scripts_history(&[script])?;
        let h = history_vec.pop();
        Ok(h.unwrap_or(vec![]))
    }

    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        self.electrum_client
            .get_scripts_history(scripts)
            .map_err(Into::into)
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
