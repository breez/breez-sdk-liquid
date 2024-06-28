#![cfg(test)]

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;

use crate::chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService};

use super::TEST_TX_TXID;

pub(crate) struct MockLiquidChainService {}

impl MockLiquidChainService {
    pub(crate) fn new() -> Self {
        MockLiquidChainService {}
    }
}

#[async_trait]
impl LiquidChainService for MockLiquidChainService {
    async fn tip(&mut self) -> Result<u32> {
        todo!()
    }

    async fn broadcast(
        &self,
        _tx: &lwk_wollet::elements::Transaction,
        _swap_id: Option<&str>,
    ) -> Result<lwk_wollet::elements::Txid> {
        Ok(lwk_wollet::elements::Txid::from_str(TEST_TX_TXID)?)
    }

    async fn get_transactions(
        &self,
        _txids: &[lwk_wollet::elements::Txid],
    ) -> Result<Vec<lwk_wollet::elements::Transaction>> {
        todo!()
    }

    async fn get_script_history(
        &self,
        _scripts: &lwk_wollet::elements::Script,
    ) -> Result<Vec<lwk_wollet::History>> {
        todo!()
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::ElementsAddress,
        _tx_id: &str,
        _tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<lwk_wollet::elements::Transaction> {
        todo!()
    }
}

pub(crate) struct MockBitcoinChainService {}

impl MockBitcoinChainService {
    pub(crate) fn new() -> Self {
        MockBitcoinChainService {}
    }
}

#[async_trait]
impl BitcoinChainService for MockBitcoinChainService {
    fn tip(&mut self) -> Result<electrum_client::HeaderNotification> {
        todo!()
    }

    fn broadcast(
        &self,
        _tx: &boltz_client::bitcoin::Transaction,
    ) -> Result<boltz_client::bitcoin::Txid, anyhow::Error> {
        todo!()
    }

    fn get_transactions(
        &self,
        _txids: &[boltz_client::bitcoin::Txid],
    ) -> Result<Vec<boltz_client::bitcoin::Transaction>> {
        todo!()
    }

    fn get_script_history(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<Vec<lwk_wollet::History>> {
        todo!()
    }

    fn script_get_balance(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<electrum_client::GetBalanceRes> {
        todo!()
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::Address,
        _tx_id: &str,
        _tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<boltz_client::bitcoin::Transaction> {
        todo!()
    }
}
