use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
    time::Duration,
};

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use electrum_client::{
    bitcoin::{
        consensus::deserialize,
        hashes::{sha256, Hash},
        Address, OutPoint, Script, ScriptBuf, Transaction, Txid,
    },
    GetBalanceRes, GetHistoryRes,
};
use log::info;
use sdk_common::bitcoin::hashes::hex::ToHex;

use crate::{
    get_client,
    model::{BlockchainExplorer, Config, RecommendedFees},
    prelude::Utxo,
};

use super::client::BitcoinClient;

/// Trait implemented by types that can fetch data from a blockchain data source.
#[allow(dead_code)]
#[async_trait]
pub trait BitcoinChainService: Send + Sync {
    /// Get the blockchain latest block
    fn tip(&self) -> Result<u32>;

    /// Broadcast a transaction
    fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a list of transactions
    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved in a list of scripts.
    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<GetHistoryRes>>>;

    /// Get the transactions involved for a script
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<GetHistoryRes>>;

    /// Get the utxos associated with a script pubkey
    fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>>;

    /// Get the utxos associated with a list of scripts
    fn get_scripts_utxos(&self, scripts: &[&Script]) -> Result<Vec<Vec<Utxo>>>;

    /// Return the confirmed and unconfirmed balances of a script hash
    fn script_get_balance(&self, script: &Script) -> Result<GetBalanceRes>;

    /// Return the confirmed and unconfirmed balances of a list of script hashes
    fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<GetBalanceRes>>;

    /// Return the confirmed and unconfirmed balances of a script hash
    async fn script_get_balance_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<GetBalanceRes>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;

    /// Get the recommended fees, in sat/vbyte
    async fn recommended_fees(&self) -> Result<RecommendedFees>;
}

pub(crate) struct HybridBitcoinChainService {
    config: Config,
    client: RwLock<Option<BitcoinClient>>,
    last_known_tip: Mutex<Option<u32>>,
}
impl HybridBitcoinChainService {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            client: RwLock::new(None),
            last_known_tip: Mutex::new(None),
        })
    }

    fn init_client(&self) -> Result<BitcoinClient> {
        for explorer in &self.config.bitcoin_explorers {
            if let Ok(client) = BitcoinClient::try_from_explorer(explorer, self.config.network) {
                if client.is_available() {
                    return Ok(client);
                }
            }
        }

        bail!("Could not create Bitcoin chain service client: no working clients found");
    }

    fn set_client(&self) -> Result<()> {
        let lock = self.client.read().unwrap();
        if let Some(client) = lock.as_ref() {
            if client.is_available() {
                return Ok(());
            }
        }

        let mut lock = self.client.write().unwrap();
        *lock = Some(self.init_client()?);
        Ok(())
    }
}

#[async_trait]
impl BitcoinChainService for HybridBitcoinChainService {
    fn tip(&self) -> Result<u32> {
        self.set_client()?;

        let mut lock = self
            .client
            .write()
            .map_err(|err| anyhow!("Could not write to client lock: {err:?}"))?;
        let Some(client) = lock.as_mut() else {
            bail!("Bitcoin client not set"); // unreachable
        };

        let new_tip = client.tip().ok();
        let mut last_tip = self.last_known_tip.lock().unwrap();
        match new_tip {
            Some(height) => {
                *last_tip = Some(height);
                Ok(height)
            }
            None => (*last_tip).ok_or_else(|| anyhow!("Failed to get tip")),
        }
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        get_client!(self, client);
        client.broadcast(tx)
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        get_client!(self, client);
        client.get_transactions(txids)
    }

    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<GetHistoryRes>>> {
        get_client!(self, client);
        client.get_scripts_history(scripts)
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<GetHistoryRes>> {
        let script_hash = sha256::Hash::hash(script.as_bytes()).to_hex();
        info!("Fetching script history for {}", script_hash);

        let mut retry = 0;
        while retry <= retries {
            let history = self.get_scripts_history(&[script])?.into_iter().nth(0);

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

    fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>> {
        Ok(self
            .get_scripts_utxos(&[script])?
            .first()
            .cloned()
            .unwrap_or_default())
    }

    fn get_scripts_utxos(&self, scripts: &[&Script]) -> Result<Vec<Vec<Utxo>>> {
        let scripts_history = self.get_scripts_history(scripts)?;
        let tx_confirmed_map: HashMap<_, _> = scripts_history
            .iter()
            .flatten()
            .map(|h| (Txid::from_raw_hash(h.tx_hash.to_raw_hash()), h.height > 0))
            .collect();
        let txs = self.get_transactions(&tx_confirmed_map.keys().cloned().collect::<Vec<_>>())?;
        let script_txs_map: HashMap<ScriptBuf, Vec<Transaction>> = scripts
            .iter()
            .map(|script| ScriptBuf::from_bytes(script.to_bytes().to_vec()))
            .zip(scripts_history)
            .map(|(script_buf, script_history)| {
                (
                    script_buf,
                    script_history
                        .iter()
                        .filter_map(|h| {
                            txs.iter()
                                .find(|tx| {
                                    tx.compute_txid().as_raw_hash() == h.tx_hash.as_raw_hash()
                                })
                                .cloned()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let scripts_utxos = script_txs_map
            .iter()
            .map(|(script_buf, txs)| {
                txs.iter()
                    .flat_map(|tx| {
                        tx.output
                            .iter()
                            .enumerate()
                            .filter(|(_, output)| output.script_pubkey == *script_buf)
                            .filter(|(vout, _)| {
                                // Check if output is unspent (only consider confirmed spending txs)
                                !txs.iter().any(|spending_tx| {
                                    let spends_our_output = spending_tx.input.iter().any(|input| {
                                        input.previous_output.txid == tx.compute_txid()
                                            && input.previous_output.vout == *vout as u32
                                    });

                                    if spends_our_output {
                                        // If it does spend our output, check if it's confirmed
                                        let spending_tx_hash = spending_tx.compute_txid();
                                        tx_confirmed_map
                                            .get(&spending_tx_hash)
                                            .copied()
                                            .unwrap_or(false)
                                    } else {
                                        false
                                    }
                                })
                            })
                            .map(|(vout, output)| {
                                Utxo::Bitcoin((
                                    OutPoint::new(tx.compute_txid(), vout as u32),
                                    output.clone(),
                                ))
                            })
                    })
                    .collect()
            })
            .collect();
        Ok(scripts_utxos)
    }

    fn script_get_balance(&self, script: &Script) -> Result<GetBalanceRes> {
        self.scripts_get_balance(&[script])?
            .into_iter()
            .nth(0)
            .context("Script balance not found")
    }

    fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<GetBalanceRes>> {
        get_client!(self, client);
        client.get_scripts_balance(scripts)
    }

    async fn script_get_balance_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<GetBalanceRes> {
        let script_hash = sha256::Hash::hash(script.as_bytes()).to_hex();
        info!("Fetching script balance for {}", script_hash);
        let mut script_balance = GetBalanceRes {
            confirmed: 0,
            unconfirmed: 0,
        };

        let mut retry = 0;
        while retry <= retries {
            script_balance = self.script_get_balance(script)?;
            match script_balance {
                GetBalanceRes {
                    confirmed: 0,
                    unconfirmed: 0,
                } => {
                    retry += 1;
                    info!(
                        "Got zero balance for script {}, retrying in {} seconds...",
                        script_hash, retry
                    );
                    tokio::time::sleep(Duration::from_secs(retry)).await;
                }
                _ => break,
            }
        }
        Ok(script_balance)
    }

    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction> {
        let script = address.script_pubkey();
        let script_history = self.get_script_history_with_retry(&script, 10).await?;
        let lockup_tx_history = script_history.iter().find(|h| h.tx_hash.to_hex().eq(tx_id));

        match lockup_tx_history {
            Some(history) => {
                info!("Bitcoin transaction found, verifying transaction content...");
                let tx: Transaction = deserialize(&hex::decode(tx_hex)?)?;
                let tx_hex = tx.compute_txid().to_hex();
                if !tx_hex.eq(&history.tx_hash.to_hex()) {
                    return Err(anyhow!(
                        "Bitcoin transaction id and hex do not match: {} vs {}",
                        tx_id,
                        tx_hex
                    ));
                }

                if verify_confirmation && history.height <= 0 {
                    return Err(anyhow!(
                        "Bitcoin transaction was not confirmed, txid={} waiting for confirmation",
                        tx_id,
                    ));
                }
                Ok(tx)
            }
            None => Err(anyhow!(
                "Bitcoin transaction was not found, txid={} waiting for broadcast",
                tx_id,
            )),
        }
    }

    async fn recommended_fees(&self) -> Result<RecommendedFees> {
        if self.config.bitcoin_esplora_explorers().is_empty() {
            bail!("Cannot fetch recommended fees without specifying a Bitcoin Esplora backend.");
        }

        for explorer in &self.config.bitcoin_explorers {
            match explorer {
                BlockchainExplorer::Electrum { .. } => continue,
                BlockchainExplorer::Esplora { .. } => {
                    let Ok(client) =
                        BitcoinClient::try_from_explorer(explorer, self.config.network)
                    else {
                        continue;
                    };
                    if let Ok(fees) = client.get_recommended_fees() {
                        return Ok(fees);
                    }
                }
            }
        }

        bail!("Could not fetch recommended fees: request failed on all specified clients")
    }
}
