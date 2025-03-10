#![cfg(test)]

use std::sync::Mutex;

use crate::{
    bitcoin, elements,
    model::{BtcScriptBalance, History},
};
use anyhow::Result;
use async_trait::async_trait;
use bitcoin::{consensus::deserialize, OutPoint, Script, TxOut};
use boltz_client::Amount;
use elements::{
    hex::FromHex, OutPoint as ElementsOutPoint, Script as ElementsScript, TxOut as ElementsTxOut,
    Txid as ElementsTxid,
};

use crate::{
    chain::{bitcoin::service::BitcoinChainService, liquid::service::LiquidChainService},
    prelude::{RecommendedFees, Utxo},
    utils,
};

#[derive(Default)]
pub(crate) struct MockLiquidChainService {
    history: Mutex<Vec<History<ElementsTxid>>>,
}

impl MockLiquidChainService {
    pub(crate) fn new() -> Self {
        MockLiquidChainService::default()
    }

    pub(crate) fn set_history(&self, history: Vec<History<ElementsTxid>>) -> &Self {
        *self.history.lock().unwrap() = history;
        self
    }

    pub(crate) fn get_history(&self) -> Vec<History<ElementsTxid>> {
        self.history.lock().unwrap().clone()
    }
}

#[async_trait]
impl LiquidChainService for MockLiquidChainService {
    async fn tip(&self) -> Result<u32> {
        Ok(0)
    }

    async fn broadcast(&self, tx: &elements::Transaction) -> Result<elements::Txid> {
        Ok(tx.txid())
    }

    async fn get_transaction_hex(
        &self,
        _txid: &elements::Txid,
    ) -> Result<Option<elements::Transaction>> {
        unimplemented!()
    }

    async fn get_transactions(
        &self,
        _txids: &[elements::Txid],
    ) -> Result<Vec<elements::Transaction>> {
        Ok(vec![])
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &ElementsScript,
        _retries: u64,
    ) -> Result<Vec<History<ElementsTxid>>> {
        Ok(self.get_history().into_iter().map(Into::into).collect())
    }

    async fn get_scripts_history(
        &self,
        _scripts: &[&ElementsScript],
    ) -> Result<Vec<Vec<History<elements::Txid>>>> {
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
        _address: &elements::Address,
        _tx_id: &str,
        tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<elements::Transaction> {
        utils::deserialize_tx_hex(tx_hex)
    }
}

pub(crate) struct MockBitcoinChainService {
    history: Mutex<Vec<History<bitcoin::Txid>>>,
    txs: Mutex<Vec<bitcoin::Transaction>>,
    script_balance_sat: Mutex<u64>,
}

impl MockBitcoinChainService {
    pub(crate) fn new() -> Self {
        MockBitcoinChainService {
            history: Mutex::new(vec![]),
            txs: Mutex::new(vec![]),
            script_balance_sat: Mutex::new(0),
        }
    }

    pub(crate) fn set_history(&self, history: Vec<History<bitcoin::Txid>>) -> &Self {
        *self.history.lock().unwrap() = history;
        self
    }

    pub(crate) fn set_transactions(&self, txs: &[&str]) -> &Self {
        *self.txs.lock().unwrap() = txs
            .iter()
            .map(|tx_hex| deserialize(&Vec::<u8>::from_hex(tx_hex).unwrap()).unwrap())
            .collect();
        self
    }

    pub(crate) fn set_script_balance_sat(&self, script_balance_sat: u64) -> &Self {
        *self.script_balance_sat.lock().unwrap() = script_balance_sat;
        self
    }
}

#[async_trait]
impl BitcoinChainService for MockBitcoinChainService {
    async fn tip(&self) -> Result<u32> {
        Ok(0)
    }

    async fn broadcast(&self, tx: &bitcoin::Transaction) -> Result<bitcoin::Txid, anyhow::Error> {
        Ok(tx.compute_txid())
    }

    async fn get_transactions(
        &self,
        _txids: &[bitcoin::Txid],
    ) -> Result<Vec<bitcoin::Transaction>> {
        Ok(self.txs.lock().unwrap().clone())
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &Script,
        _retries: u64,
    ) -> Result<Vec<History<bitcoin::Txid>>> {
        Ok(self
            .history
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn get_scripts_history(
        &self,
        _scripts: &[&Script],
    ) -> Result<Vec<Vec<History<bitcoin::Txid>>>> {
        Ok(vec![])
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
        let scripts_utxos = scripts
            .iter()
            .map(|s| {
                vec![Utxo::Bitcoin((
                    OutPoint::default(),
                    TxOut {
                        value: Amount::from_sat(1000),
                        script_pubkey: s.to_p2sh(),
                    },
                ))]
            })
            .collect();
        Ok(scripts_utxos)
    }

    async fn script_get_balance(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<BtcScriptBalance> {
        Ok(BtcScriptBalance {
            confirmed: 0,
            unconfirmed: 0,
        })
    }

    async fn scripts_get_balance(&self, _scripts: &[&Script]) -> Result<Vec<BtcScriptBalance>> {
        Ok(vec![])
    }

    async fn script_get_balance_with_retry(
        &self,
        _script: &boltz_client::bitcoin::Script,
        _retries: u64,
    ) -> Result<BtcScriptBalance> {
        Ok(BtcScriptBalance {
            confirmed: *self.script_balance_sat.lock().unwrap(),
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
