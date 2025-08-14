use std::{collections::HashMap, sync::OnceLock, time::Duration};

use esplora_client::{AsyncClient, Builder};
use tokio::sync::Mutex;
use tokio_with_wasm::alias as tokio;

use crate::{
    bitcoin::{
        consensus::deserialize,
        hashes::{sha256, Hash},
        Address, OutPoint, Script, ScriptBuf, Transaction, Txid,
    },
    model::{BlockchainExplorer, Config},
};

use anyhow::{anyhow, Context, Result};

use crate::model::{RecommendedFees, Utxo};
use log::info;
use sdk_common::bitcoin::hashes::hex::ToHex as _;

use super::{BitcoinChainService, BtcScriptBalance, History};

pub(crate) struct EsploraBitcoinChainService {
    config: Config,
    client: OnceLock<AsyncClient>,
    last_known_tip: Mutex<Option<u32>>,
}

impl EsploraBitcoinChainService {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            client: OnceLock::new(),
            last_known_tip: Mutex::new(None),
        }
    }

    fn get_client(&self) -> Result<&AsyncClient> {
        if let Some(c) = self.client.get() {
            return Ok(c);
        }

        let esplora_url = match &self.config.bitcoin_explorer {
            BlockchainExplorer::Esplora { url, .. } => url,
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { .. } => {
                anyhow::bail!("Cannot start Bitcoin Esplora chain service without an Esplora url")
            }
        };
        let client = Builder::new(esplora_url)
            .connect_timeout(10)
            .timeout(3)
            .max_retries(10)
            .build_async()?;
        let client = self.client.get_or_init(|| client);
        Ok(client)
    }
}

#[sdk_macros::async_trait]
impl BitcoinChainService for EsploraBitcoinChainService {
    async fn tip(&self) -> Result<u32> {
        let client = self.get_client()?;
        let new_tip = client.get_height().await.ok();

        let mut last_tip = self.last_known_tip.lock().await;
        match new_tip {
            Some(height) => {
                *last_tip = Some(height);
                Ok(height)
            }
            None => (*last_tip).ok_or_else(|| anyhow!("Failed to get tip")),
        }
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        self.get_client()?.broadcast(tx).await?;
        Ok(tx.compute_txid())
    }

    // TODO Switch to batch search
    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let client = self.get_client()?;
        let mut result = vec![];
        for txid in txids {
            let tx = client
                .get_tx(txid)
                .await?
                .context("Transaction not found")?;
            result.push(tx);
        }
        Ok(result)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        let client = self.get_client()?;
        let history = client
            .scripthash_txs(script, None)
            .await?
            .into_iter()
            .map(|tx| History {
                txid: tx.txid,
                height: tx.status.block_height.map(|h| h as i32).unwrap_or(-1),
            })
            .collect();
        Ok(history)
    }

    // TODO Switch to batch search
    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        let mut result = vec![];
        for script in scripts {
            let history = self.get_script_history(script).await?;
            result.push(history);
        }
        Ok(result)
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>> {
        let script_hash = sha256::Hash::hash(script.as_bytes()).to_hex();
        info!("Fetching script history for {}", script_hash);
        let mut script_history = vec![];

        let mut retry = 0;
        while retry <= retries {
            script_history = self.get_script_history(script).await?;
            match script_history.is_empty() {
                true => {
                    retry += 1;
                    info!(
                        "Script history for {script_hash} got zero transactions, retrying in {retry} seconds..."
                    );
                    tokio::time::sleep(Duration::from_secs(retry)).await;
                }
                false => break,
            }
        }
        Ok(script_history)
    }

    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>> {
        Ok(self
            .get_scripts_utxos(&[script])
            .await?
            .first()
            .cloned()
            .unwrap_or_default())
    }

    async fn get_scripts_utxos(&self, scripts: &[&Script]) -> Result<Vec<Vec<Utxo>>> {
        let scripts_history = self.get_scripts_history(scripts).await?;
        let tx_confirmed_map: HashMap<_, _> = scripts_history
            .iter()
            .flatten()
            .map(|h| (h.txid, h.height > 0))
            .collect();
        let txs = self
            .get_transactions(&tx_confirmed_map.keys().cloned().collect::<Vec<_>>())
            .await?;
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
                                .find(|tx| tx.compute_txid().as_raw_hash() == h.txid.as_raw_hash())
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

    async fn script_get_balance(&self, script: &Script) -> Result<BtcScriptBalance> {
        let client = self.get_client()?;
        let utxos = client.scripthash_utxos(script).await?;
        let mut balance = BtcScriptBalance {
            confirmed: 0,
            unconfirmed: 0,
        };
        for utxo in utxos {
            match utxo.status.confirmed {
                true => balance.confirmed += utxo.value,
                false => balance.unconfirmed += utxo.value as i64,
            };
        }
        Ok(balance)
    }

    // TODO Switch to batch search
    async fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<BtcScriptBalance>> {
        let mut result = vec![];
        for script in scripts {
            let balance = self.script_get_balance(script).await?;
            result.push(balance);
        }
        Ok(result)
    }

    async fn script_get_balance_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<BtcScriptBalance> {
        let script_hash = sha256::Hash::hash(script.as_bytes()).to_hex();
        info!("Fetching script balance for {}", script_hash);
        let mut script_balance = BtcScriptBalance {
            confirmed: 0,
            unconfirmed: 0,
        };

        let mut retry = 0;
        while retry <= retries {
            script_balance = self.script_get_balance(script).await?;
            match script_balance {
                BtcScriptBalance {
                    confirmed: 0,
                    unconfirmed: 0,
                } => {
                    retry += 1;
                    info!(
                        "Got zero balance for script {script_hash}, retrying in {retry} seconds..."
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
        let lockup_tx_history = script_history.iter().find(|h| h.txid.to_hex().eq(tx_id));

        match lockup_tx_history {
            Some(history) => {
                info!("Bitcoin transaction found, verifying transaction content...");
                let tx: Transaction = deserialize(&hex::decode(tx_hex)?)?;
                let tx_hex = tx.compute_txid().to_hex();
                if !tx_hex.eq(&history.txid.to_hex()) {
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
        let client = self.get_client()?;
        let fees = client.get_fee_estimates().await?;
        let get_fees = |block: &u16| fees.get(block).map(|fee| fee.ceil() as u64).unwrap_or(0);

        Ok(RecommendedFees {
            fastest_fee: get_fees(&1),
            half_hour_fee: get_fees(&3),
            hour_fee: get_fees(&6),
            economy_fee: get_fees(&25),
            minimum_fee: get_fees(&1008),
        })
    }
}
