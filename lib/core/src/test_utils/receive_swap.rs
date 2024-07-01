#![cfg(test)]

use anyhow::Result;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    chain::liquid::HybridLiquidChainService, model::Config, persist::Persister,
    receive_swap::ReceiveSwapStateHandler,
};

use super::{swapper::MockSwapper, wallet::MockWallet};

pub(crate) fn new_receive_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<ReceiveSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(MockWallet::new());
    let swapper = Arc::new(MockSwapper::new());
    let liquid_chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));

    Ok(ReceiveSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        liquid_chain_service,
    ))
}
