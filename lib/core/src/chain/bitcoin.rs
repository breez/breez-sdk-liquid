use std::collections::HashMap;

use anyhow::Result;
use electrum_client::{Client, ElectrumApi, HeaderNotification, ListUnspentRes};
use lwk_wollet::{
    bitcoin::{
        block::Header,
        consensus::{deserialize, serialize},
        BlockHash, Script, Transaction, Txid,
    },
    ElectrumOptions, ElectrumUrl, Error, History,
};

type Height = u32;

#[allow(dead_code)]
pub struct Unspent {
    /// Confirmation height of the transaction that created this output.
    pub height: usize,
    /// Txid of the transaction
    pub tx_hash: Txid,
    /// Index of the output in the transaction.
    pub tx_pos: usize,
    /// Value of the output.
    pub value: u64,
}
impl From<ListUnspentRes> for Unspent {
    fn from(value: ListUnspentRes) -> Self {
        Unspent {
            height: value.height,
            tx_hash: value.tx_hash,
            tx_pos: value.tx_pos,
            value: value.value,
        }
    }
}

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

    /// Get a list of unspend outputs
    fn script_list_unspent(&self, script: &Script) -> Result<Vec<Unspent>>;
}

pub(crate) struct ElectrumClient {
    client: Client,
    tip: HeaderNotification,
}
impl ElectrumClient {
    pub fn new(url: &ElectrumUrl) -> Result<Self, Error> {
        Self::with_options(url, ElectrumOptions::default())
    }

    /// Creates an Electrum client specifying non default options like timeout
    pub fn with_options(url: &ElectrumUrl, options: ElectrumOptions) -> Result<Self, Error> {
        let client = url.build_client(&options)?;
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

        match maybe_popped_header {
            Some(popped_header) => {
                let tip: HeaderNotification = popped_header.try_into()?;
                self.tip = tip;
            }
            None => {
                // https://github.com/bitcoindevkit/rust-electrum-client/issues/124
                // It might be that the client has reconnected and subscriptions don't persist
                // across connections. Calling `client.ping()` won't help here because the
                // successful retry will prevent us knowing about the reconnect.
                if let Ok(header) = self.client.block_headers_subscribe_raw() {
                    let tip: HeaderNotification = header.try_into()?;
                    self.tip = tip;
                }
            }
        }

        Ok(self.tip.clone())
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.transaction_broadcast_raw(&serialize(tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let mut result = vec![];
        for tx in self.client.batch_transaction_get_raw(txids)? {
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
        Ok(self
            .client
            .batch_script_get_history(scripts)?
            .into_iter()
            .map(|e| e.into_iter().map(Into::into).collect())
            .collect())
    }

    fn script_list_unspent(&self, script: &Script) -> Result<Vec<Unspent>> {
        Ok(self
            .client
            .script_list_unspent(script)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
