use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use electrum_client::bitcoin::{Script, Transaction, Txid};
use electrum_client::{ElectrumApi as _, GetBalanceRes, GetHistoryRes};
use lwk_wollet::ElectrumOptions;

use crate::model::{BlockchainExplorer, LiquidNetwork, RecommendedFees};

pub(crate) enum BitcoinClient {
    Electrum {
        inner: Box<electrum_client::Client>,
    },
    Esplora {
        inner: Box<esplora_client::BlockingClient>,
    },
}

impl BitcoinClient {
    pub(crate) fn is_available(&self) -> bool {
        match self {
            BitcoinClient::Electrum { inner } => inner.ping().is_ok(),
            BitcoinClient::Esplora { .. } => true,
        }
    }

    pub(crate) fn try_from_explorer(
        exp: &BlockchainExplorer,
        network: LiquidNetwork,
    ) -> Result<Self> {
        match exp {
            BlockchainExplorer::Electrum { url } => {
                let (tls, validate_domain) = match network {
                    LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
                    LiquidNetwork::Regtest => (false, false),
                };
                let url = lwk_wollet::ElectrumUrl::new(url, tls, validate_domain)?;
                let client = url.build_client(&ElectrumOptions { timeout: Some(3) })?;
                // Ensure we ping so we know the client is working
                client.ping()?;
                Ok(BitcoinClient::Electrum {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::Esplora { url, .. } => {
                let client = esplora_client::Builder::new(url)
                    .timeout(3)
                    .max_retries(5)
                    .build_blocking();
                Ok(BitcoinClient::Esplora {
                    inner: Box::new(client),
                })
            }
        }
    }

    pub(crate) fn tip(&mut self) -> Result<u32> {
        match self {
            BitcoinClient::Electrum { inner } => {
                let mut maybe_popped_header = None;
                while let Some(header) = inner.block_headers_pop_raw()? {
                    maybe_popped_header = Some(header)
                }

                match maybe_popped_header {
                    Some(popped_header) => Ok(popped_header.height as u32),
                    None => {
                        // https://github.com/bitcoindevkit/rust-electrum-client/issues/124
                        // It might be that the client has reconnected and subscriptions don't persist
                        // across connections. Calling `client.ping()` won't help here because the
                        // successful retry will prevent us knowing about the reconnect.
                        if let Ok(header) = inner.block_headers_subscribe_raw() {
                            return Ok(header.height as u32);
                        }

                        bail!("No new tip found")
                    }
                }
            }
            BitcoinClient::Esplora { inner } => Ok(inner.get_height()?),
        }
    }

    pub(crate) fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        Ok(match self {
            BitcoinClient::Electrum { inner } => inner.transaction_broadcast(tx)?,
            BitcoinClient::Esplora { inner } => {
                inner.broadcast(tx)?;
                tx.compute_txid()
            }
        })
    }

    pub(crate) fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(match self {
            BitcoinClient::Electrum { inner } => inner.batch_transaction_get(txids)?,
            BitcoinClient::Esplora { inner } => {
                // TODO Add support for batch search
                let mut result = vec![];
                for txid in txids {
                    result.push(inner.get_tx(txid)?.context("Transaction not found")?);
                }
                result
            }
        })
    }

    pub(crate) fn get_scripts_history(
        &self,
        scripts: &[&Script],
    ) -> Result<Vec<Vec<GetHistoryRes>>> {
        Ok(match self {
            BitcoinClient::Electrum { inner } => inner.batch_script_get_history(scripts)?,
            BitcoinClient::Esplora { inner } => {
                // TODO Add support for batch search
                let mut histories = vec![];
                for script in scripts {
                    let txs = inner.scripthash_txs(script, None)?;
                    let history = txs
                        .into_iter()
                        .map(|tx| GetHistoryRes {
                            height: match tx.vin.iter().any(|input| input.prevout.is_none()) {
                                true => -1,
                                false => 0,
                            },
                            tx_hash: tx.txid,
                            fee: Some(tx.fee),
                        })
                        .collect();
                    histories.push(history);
                }
                histories
            }
        })
    }

    pub(crate) fn get_scripts_balance(&self, scripts: &[&Script]) -> Result<Vec<GetBalanceRes>> {
        Ok(match self {
            BitcoinClient::Electrum { inner } => inner.batch_script_get_balance(scripts)?,
            BitcoinClient::Esplora { .. } => {
                // TODO Add support for get balance
                unimplemented!()
            }
        })
    }

    pub(crate) fn get_recommended_fees(&self) -> Result<RecommendedFees> {
        Ok(match self {
            BitcoinClient::Electrum { .. } => unreachable!(),
            BitcoinClient::Esplora { inner } => {
                let fee_estimates: HashMap<u16, u64> = inner
                    .get_fee_estimates()?
                    .into_iter()
                    .map(|(k, v)| (k, v as u64))
                    .collect();

                let get_fee_estimate =
                    |block: &u16| fee_estimates.get(block).cloned().unwrap_or_default();

                // See https://github.com/Blockstream/esplora/blob/master/API.md#fee-estimates
                RecommendedFees {
                    fastest_fee: get_fee_estimate(&1),
                    half_hour_fee: get_fee_estimate(&3),
                    hour_fee: get_fee_estimate(&6),
                    economy_fee: get_fee_estimate(&144),
                    minimum_fee: get_fee_estimate(&1008),
                }
            }
        })
    }
}
