#![cfg(test)]

use std::sync::Arc;

use crate::{
    chain::liquid::HybridLiquidChainService, model::Config, persist::Persister,
    send_swap::SendSwapStateHandler, swapper::BoltzSwapper,
};
use anyhow::Result;
use tokio::sync::Mutex;

use super::wallet::new_onchain_wallet;

pub(crate) fn new_send_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<SendSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(BoltzSwapper::new(config.clone(), None));
    let chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));

    Ok(SendSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
    ))
}
