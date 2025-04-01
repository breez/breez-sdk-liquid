#![cfg(not(all(target_family = "wasm", target_os = "unknown")))]

use std::{collections::HashMap, sync::OnceLock, time::Duration};

use anyhow::{anyhow, bail, Result};
use tokio::sync::Mutex;

use crate::{
    bitcoin::{
        consensus::{deserialize, serialize},
        hashes::{sha256, Hash},
        Address, OutPoint, Script, ScriptBuf, Transaction, Txid,
    },
    model::{BlockchainExplorer, Config, RecommendedFees, Utxo},
};

use electrum_client::{Client, ElectrumApi, HeaderNotification};
use log::info;
use lwk_wollet::{ElectrumOptions, ElectrumUrl};
use sdk_common::bitcoin::hashes::hex::ToHex as _;

use super::{BitcoinChainService, BtcScriptBalance, History};

pub(crate) struct ElectrumBitcoinChainService {
    config: Config,
    client: OnceLock<Client>,
    last_known_tip: Mutex<Option<u32>>,
}

impl ElectrumBitcoinChainService {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            client: OnceLock::new(),
            last_known_tip: Mutex::new(None),
        }
    }

    fn get_client(&self) -> Result<&Client> {
        if let Some(c) = self.client.get() {
            return Ok(c);
        }

        let (tls, validate_domain) = self.config.electrum_tls_options();
        let electrum_url = match &self.config.bitcoin_explorer {
            BlockchainExplorer::Electrum { url } => ElectrumUrl::new(url, tls, validate_domain)?,
            _ => bail!("Cannot start Bitcoin Electrum chain service without an Electrum url"),
        };
        let client = electrum_url.build_client(&ElectrumOptions { timeout: Some(3) })?;

        let client = self.client.get_or_init(|| client);
        Ok(client)
    }
}

#[sdk_macros::async_trait]
impl BitcoinChainService for ElectrumBitcoinChainService {
    async fn tip(&self) -> Result<u32> {
        let client = self.get_client()?;
        let mut maybe_popped_header = None;
        while let Some(header) = client.block_headers_pop_raw()? {
            maybe_popped_header = Some(header)
        }

        let new_tip: Option<HeaderNotification> = match maybe_popped_header {
            Some(popped_header) => Some(popped_header.try_into()?),
            None => {
                // https://github.com/bitcoindevkit/rust-electrum-client/issues/124
                // It might be that the client has reconnected and subscriptions don't persist
                // across connections. Calling `client.ping()` won't help here because the
                // successful retry will prevent us knowing about the reconnect.
                if let Ok(header) = client.block_headers_subscribe_raw() {
                    Some(header.try_into()?)
                } else {
                    None
                }
            }
        };

        let mut last_tip = self.last_known_tip.lock().await;
        match new_tip {
            Some(header) => {
                let height = header.height as u32;
                *last_tip = Some(height);
                Ok(height)
            }
            None => (*last_tip).ok_or_else(|| anyhow!("Failed to get tip")),
        }
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self
            .get_client()?
            .transaction_broadcast_raw(&serialize(&tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let mut result = vec![];
        for tx in self.get_client()?.batch_transaction_get_raw(txids)? {
            let tx: Transaction = deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        Ok(self
            .get_client()?
            .script_get_history(script)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        Ok(self
            .get_client()?
            .batch_script_get_history(scripts)?
            .into_iter()
            .map(|v| v.into_iter().map(Into::into).collect())
            .collect())
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
                        "Script history for {} got zero transactions, retrying in {} seconds...",
                        script_hash, retry
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
            .map(|h| (Txid::from_raw_hash(h.txid.to_raw_hash()), h.height > 0))
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
        Ok(self.get_client()?.script_get_balance(script)?.into())
    }

    async fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<BtcScriptBalance>> {
        Ok(self
            .get_client()?
            .batch_script_get_balance(scripts)?
            .into_iter()
            .map(Into::into)
            .collect())
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
        let fees: Vec<u64> = self
            .get_client()?
            .batch_estimate_fee([1, 3, 6, 25, 1008])?
            .into_iter()
            .map(|v| v.ceil() as u64)
            .collect();
        Ok(RecommendedFees {
            fastest_fee: fees[0],
            half_hour_fee: fees[1],
            hour_fee: fees[2],
            economy_fee: fees[3],
            minimum_fee: fees[4],
        })
    }
}
