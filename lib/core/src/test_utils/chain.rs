#![cfg(test)]

use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::{
    elements::{
        hex::FromHex, OutPoint as ElementsOutPoint, Script as ElementsScript,
        TxOut as ElementsTxOut,
    },
    Amount,
};
use electrum_client::GetBalanceRes;
use electrum_client::{
    bitcoin::{consensus::deserialize, OutPoint, Script, TxOut},
    HeaderNotification,
};
use lwk_wollet::{
    bitcoin::constants::genesis_block,
    elements::{BlockHash, Txid as ElementsTxid},
    History,
};

use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    prelude::{RecommendedFees, Utxo},
    utils,
};

#[derive(Clone)]
pub(crate) struct MockHistory {
    pub txid: ElementsTxid,
    pub height: i32,
    pub block_hash: Option<BlockHash>,
    pub block_timestamp: Option<u32>,
}

impl From<MockHistory> for lwk_wollet::History {
    fn from(h: MockHistory) -> Self {
        lwk_wollet::History {
            txid: h.txid,
            height: h.height,
            block_hash: h.block_hash,
            block_timestamp: h.block_timestamp,
        }
    }
}

#[derive(Default)]
pub(crate) struct MockLiquidChainService {
    history: Mutex<Vec<MockHistory>>,
}

impl MockLiquidChainService {
    pub(crate) fn new() -> Self {
        MockLiquidChainService::default()
    }

    pub(crate) fn set_history(&self, history: Vec<MockHistory>) -> &Self {
        *self.history.lock().unwrap() = history;
        self
    }

    pub(crate) fn get_history(&self) -> Vec<MockHistory> {
        self.history.lock().unwrap().clone()
    }
}

#[async_trait]
impl LiquidChainService for MockLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        Ok(0)
    }

    async fn broadcast(
        &self,
        tx: &lwk_wollet::elements::Transaction,
    ) -> Result<lwk_wollet::elements::Txid> {
        Ok(tx.txid())
    }

    async fn get_transaction_hex(
        &self,
        _txid: &lwk_wollet::elements::Txid,
    ) -> Result<Option<lwk_wollet::elements::Transaction>> {
        unimplemented!()
    }

    async fn get_transactions(
        &self,
        _txids: &[lwk_wollet::elements::Txid],
    ) -> Result<Vec<lwk_wollet::elements::Transaction>> {
        Ok(vec![])
    }

    async fn get_script_history(
        &self,
        _scripts: &ElementsScript,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.get_history().into_iter().map(Into::into).collect())
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &ElementsScript,
        _retries: u64,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.get_history().into_iter().map(Into::into).collect())
    }

    async fn get_scripts_history(&self, _scripts: &[&ElementsScript]) -> Result<Vec<Vec<History>>> {
        Ok(vec![])
    }

    async fn get_script_utxos(&self, _script: &ElementsScript) -> Result<Vec<Utxo>> {
        Ok(vec![Utxo::Liquid(Box::new((
            ElementsOutPoint::default(),
            ElementsTxOut::default(),
        )))])
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::ElementsAddress,
        _tx_id: &str,
        tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<lwk_wollet::elements::Transaction> {
        utils::deserialize_tx_hex(tx_hex)
    }
}

pub(crate) struct MockBitcoinChainService {
    history: Vec<MockHistory>,
    txs: Vec<boltz_client::bitcoin::Transaction>,
    script_balance_sat: u64,
}

impl MockBitcoinChainService {
    pub(crate) fn new() -> Self {
        MockBitcoinChainService {
            history: vec![],
            txs: vec![],
            script_balance_sat: 0,
        }
    }

    pub(crate) fn set_history(&mut self, history: Vec<MockHistory>) -> &mut Self {
        self.history = history;
        self
    }

    pub(crate) fn set_transactions(&mut self, txs: &[&str]) -> &mut Self {
        self.txs = txs
            .iter()
            .map(|tx_hex| deserialize(&Vec::<u8>::from_hex(tx_hex).unwrap()).unwrap())
            .collect();
        self
    }

    pub(crate) fn set_script_balance_sat(&mut self, script_balance_sat: u64) -> &mut Self {
        self.script_balance_sat = script_balance_sat;
        self
    }
}

#[async_trait]
impl BitcoinChainService for MockBitcoinChainService {
    fn tip(&mut self) -> Result<HeaderNotification> {
        Ok(HeaderNotification {
            height: 0,
            header: genesis_block(lwk_wollet::bitcoin::Network::Testnet).header,
        })
    }

    fn broadcast(
        &self,
        tx: &boltz_client::bitcoin::Transaction,
    ) -> Result<boltz_client::bitcoin::Txid, anyhow::Error> {
        Ok(tx.compute_txid())
    }

    fn get_transactions(
        &self,
        _txids: &[boltz_client::bitcoin::Txid],
    ) -> Result<Vec<boltz_client::bitcoin::Transaction>> {
        Ok(self.txs.clone())
    }

    fn get_script_history(&self, _script: &Script) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &Script,
        _retries: u64,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    fn get_scripts_history(&self, _scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        Ok(vec![])
    }

    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>> {
        let out_point = OutPoint::default();
        let tx_out = TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: script.to_p2sh(),
        };
        Ok(vec![Utxo::Bitcoin((out_point, tx_out))])
    }

    fn script_get_balance(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<electrum_client::GetBalanceRes> {
        Ok(GetBalanceRes {
            confirmed: 0,
            unconfirmed: 0,
        })
    }

    fn scripts_get_balance(&self, _scripts: &[&Script]) -> Result<Vec<GetBalanceRes>> {
        Ok(vec![])
    }

    async fn script_get_balance_with_retry(
        &self,
        _script: &boltz_client::bitcoin::Script,
        _retries: u64,
    ) -> Result<electrum_client::GetBalanceRes> {
        Ok(GetBalanceRes {
            confirmed: self.script_balance_sat,
            unconfirmed: 0,
        })
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::Address,
        _tx_id: &str,
        tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<boltz_client::bitcoin::Transaction> {
        Ok(deserialize(&Vec::<u8>::from_hex(tx_hex).map_err(
            |err| anyhow::anyhow!("Could not deserialize transaction: {err:?}"),
        )?)?)
    }

    async fn recommended_fees(&self) -> Result<RecommendedFees> {
        unimplemented!()
    }
}
