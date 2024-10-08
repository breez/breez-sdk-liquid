#![cfg(test)]

use std::sync::Arc;

use crate::{model::Config, persist::Persister, send_swap::SendSwapHandler};
use anyhow::Result;
use tokio::sync::Mutex;

use super::{chain::MockLiquidChainService, swapper::MockSwapper, wallet::MockWallet};

pub(crate) fn new_send_swap_handler(persister: Arc<Persister>) -> Result<SendSwapHandler> {
    let config = Config::testnet(None);
    let onchain_wallet = Arc::new(MockWallet::new());
    let swapper = Arc::new(MockSwapper::new());
    let chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));

    Ok(SendSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
    ))
}
