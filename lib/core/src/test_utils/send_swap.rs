#![cfg(test)]

use std::sync::Arc;

use crate::{
    chain::liquid::HybridLiquidChainService, model::Config, persist::Persister,
    send_swap::SendSwapStateHandler,
};
use anyhow::Result;
use tokio::sync::Mutex;

use super::{swapper::MockSwapper, wallet::MockWallet};

pub(crate) fn new_send_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<SendSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(MockWallet::new());
    let swapper = Arc::new(MockSwapper::new());
    let chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));

    Ok(SendSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
    ))
}
