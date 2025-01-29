use std::{
    collections::HashMap,
    sync::{Mutex, RwLock, RwLockReadGuard},
    time::Duration,
};

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use electrum_client::{
    bitcoin::{
        consensus::{deserialize, serialize},
        hashes::{sha256, Hash},
        Address, OutPoint, Script, Transaction, Txid,
    },
    Client, ElectrumApi, GetBalanceRes, HeaderNotification,
};
use log::info;
use lwk_wollet::{bitcoin::ScriptBuf, ElectrumOptions, ElectrumUrl, Error, History};
use sdk_common::{bitcoin::hashes::hex::ToHex, prelude::get_parse_and_log_response};

use crate::{
    model::{Config, LiquidNetwork, RecommendedFees},
    prelude::Utxo,
};

/// Trait implemented by types that can fetch data from a blockchain data source.
#[allow(dead_code)]
#[async_trait]
pub trait BitcoinChainService: Send + Sync {
    /// Get the blockchain latest block
    fn tip(&self) -> Result<HeaderNotification>;

    /// Broadcast a transaction
    fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a list of transactions
    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved for a script
    fn get_script_history(&self, script: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts.
    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>>;

    /// Get the transactions involved for a script
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

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

macro_rules! get_client {
    ($chain_service:ident,$client:ident) => {
        let lock = $chain_service.get_client()?;
        let Some($client) = lock.as_ref() else {
            bail!("Could not read Bitcoin electrum client");
        };
    };
}

pub(crate) struct HybridBitcoinChainService {
    config: Config,
    client: RwLock<Option<Client>>,
    last_known_tip: Mutex<Option<HeaderNotification>>,
}
impl HybridBitcoinChainService {
    pub fn new(config: Config) -> Result<Self, Error> {
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
        for electrum_url in &self.config.bitcoin_electrum_explorers() {
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

        bail!("Could not create Bitcoin electrum client: no url specified in the configuration");
    }
}

#[async_trait]
impl BitcoinChainService for HybridBitcoinChainService {
    fn tip(&self) -> Result<HeaderNotification> {
        get_client!(self, client);
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

        let mut last_tip = self.last_known_tip.lock().unwrap();
        match new_tip {
            Some(header) => {
                *last_tip = Some(header.clone());
                Ok(header)
            }
            None => last_tip.clone().ok_or_else(|| anyhow!("Failed to get tip")),
        }
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        get_client!(self, client);
        let txid = client.transaction_broadcast_raw(&serialize(&tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        get_client!(self, client);
        let mut result = vec![];
        for tx in client.batch_transaction_get_raw(txids)? {
            let tx: Transaction = deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        get_client!(self, client);
        Ok(client
            .script_get_history(script)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        get_client!(self, client);
        Ok(client
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
            script_history = self.get_script_history(script)?;
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
            .map(|h| (Txid::from_raw_hash(h.txid.to_raw_hash()), h.height > 0))
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

    fn script_get_balance(&self, script: &Script) -> Result<GetBalanceRes> {
        get_client!(self, client);
        Ok(client.script_get_balance(script)?)
    }

    fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<GetBalanceRes>> {
        get_client!(self, client);
        Ok(client.batch_script_get_balance(scripts)?)
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
        if self.config.bitcoin_esplora_explorers().is_empty() {
            bail!("Cannot fetch recommended fees without specifying a Bitcoin Esplora backend.");
        }

        for mempool_space_url in self.config.bitcoin_mempool_space_explorers() {
            let res = get_parse_and_log_response(
                &format!("{mempool_space_url}/v1/fees/recommended"),
                true,
            )
            .await
            .map_err(Into::<anyhow::Error>::into);

            match res {
                Ok(fees) => return Ok(fees),
                Err(err) => {
                    log::warn!("Could not fetch recommended fees from {mempool_space_url}: {err:?}")
                }
            }
        }

        bail!("Could not fetch recommended fees: request failed on all specified clients")
    }
}
