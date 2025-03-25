use std::{sync::OnceLock, time::Duration};

use anyhow::{anyhow, bail, Context as _, Result};
use tokio::sync::RwLock;

use crate::{
    elements::{Address, OutPoint, Script, Transaction, Txid},
    model::{BlockchainExplorer, Config, Utxo},
    utils,
};

use log::info;
use lwk_wollet::{
    asyncr::EsploraClientBuilder, clients::asyncr::EsploraClient, elements::hex::FromHex as _,
};
use sdk_common::bitcoin::hashes::hex::ToHex as _;

use super::{History, LiquidChainService};

pub(crate) struct EsploraLiquidChainService {
    config: Config,
    client: OnceLock<RwLock<EsploraClient>>,
}

impl EsploraLiquidChainService {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            client: OnceLock::new(),
        }
    }

    fn get_client(&self) -> Result<&RwLock<EsploraClient>> {
        if let Some(c) = self.client.get() {
            return Ok(c);
        }

        #[allow(unreachable_patterns)]
        let client = match &self.config.liquid_explorer {
            BlockchainExplorer::Esplora {
                url,
                use_waterfalls,
            } => EsploraClientBuilder::new(url, self.config.network.into())
                .timeout(3)
                .waterfalls(*use_waterfalls)
                .build(),
            _ => bail!("Cannot start Liquid Esplora chain service without an Esplora url"),
        };

        let client = self.client.get_or_init(|| RwLock::new(client));
        Ok(client)
    }
}

#[sdk_macros::async_trait]
impl LiquidChainService for EsploraLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        Ok(self
            .get_client()?
            .write()
            .await
            .tip()
            .await
            .map(|header| header.height)?)
    }

    async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        Ok(self.get_client()?.read().await.broadcast(tx).await?)
    }

    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>> {
        Ok(self.get_transactions(&[*txid]).await?.first().cloned())
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(self
            .get_client()?
            .read()
            .await
            .get_transactions(txids)
            .await?)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        self.get_scripts_history(&[script.clone()])
            .await?
            .into_iter()
            .nth(0)
            .context("History not found")
    }

    async fn get_scripts_history(&self, scripts: &[Script]) -> Result<Vec<Vec<History>>> {
        let scripts: Vec<&Script> = scripts.iter().collect();
        Ok(self
            .get_client()?
            .read()
            .await
            .get_scripts_history(&scripts)
            .await?
            .into_iter()
            .map(|h| h.into_iter().map(Into::into).collect())
            .collect())
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>> {
        info!("Fetching script history for {script:x}");
        let mut script_history = vec![];

        let mut retry = 0;
        while retry <= retries {
            script_history = self.get_script_history(script).await?;
            match script_history.is_empty() {
                true => {
                    retry += 1;
                    info!("Script history for {script:x} is empty, retrying in 1 second... ({retry} of {retries})");
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
