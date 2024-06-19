use std::collections::HashMap;

use anyhow::Result;
use electrum_client::{Client, ElectrumApi, HeaderNotification};
use lwk_wollet::{
    bitcoin::{
        self,
        block::Header,
        consensus::{deserialize, serialize},
        BlockHash, Script, Transaction, Txid,
    },
    ElectrumUrl, History,
};

type Height = u32;

/// Trait implemented by types that can fetch data from a blockchain data source.
#[allow(dead_code)]
pub trait BitcoinChainService: Send + Sync {
    /// Get the blockchain latest block
    fn tip(&mut self) -> Result<HeaderNotification>;

    /// Broadcast a transaction
    fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a list of transactions
    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get a list of block headers
    ///
    /// Optionally pass the blockhash if already known
    fn get_headers(
        &self,
        heights: &[Height],
        height_blockhash: &HashMap<Height, BlockHash>,
    ) -> Result<Vec<Header>>;

    /// Get the transactions involved in a list of scripts
    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>>;
}

pub(crate) struct ElectrumClient {
    client: Client,
    tip: HeaderNotification,
}
impl ElectrumClient {
    pub fn new(url: &ElectrumUrl) -> Result<Self> {
        let client = url.build_client()?;
        let header = client.block_headers_subscribe_raw()?;
        let tip: HeaderNotification = header.try_into()?;

        Ok(Self { client, tip })
    }
}

impl BitcoinChainService for ElectrumClient {
    fn tip(&mut self) -> Result<HeaderNotification> {
        let mut maybe_popped_header = None;
        while let Some(header) = self.client.block_headers_pop_raw()? {
            maybe_popped_header = Some(header)
        }

        if let Some(popped_header) = maybe_popped_header {
            let tip: HeaderNotification = popped_header.try_into()?;
            self.tip = tip;
        }

        Ok(self.tip.clone())
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.transaction_broadcast_raw(&serialize(tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let txids: Vec<bitcoin::Txid> = txids
            .iter()
            .map(|t| bitcoin::Txid::from_raw_hash(t.to_raw_hash()))
            .collect();

        let mut result = vec![];
        for tx in self.client.batch_transaction_get_raw(&txids)? {
            let tx: Transaction = deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn get_headers(
        &self,
        heights: &[Height],
        _: &HashMap<Height, BlockHash>,
    ) -> Result<Vec<Header>> {
        let mut result = vec![];
        for header in self.client.batch_block_header_raw(heights)? {
            let header: Header = deserialize(&header)?;
            result.push(header);
        }
        Ok(result)
    }

    fn get_scripts_history(&self, scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        let scripts: Vec<&bitcoin::Script> = scripts
            .iter()
            .map(|t| bitcoin::Script::from_bytes(t.as_bytes()))
            .collect();

        Ok(self
            .client
            .batch_script_get_history(&scripts)?
            .into_iter()
            .map(|e| e.into_iter().map(Into::into).collect())
            .collect())
    }
}
