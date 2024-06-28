#![cfg(test)]

use anyhow::Result;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    chain::liquid::HybridLiquidChainService, model::Config, persist::Persister,
    receive_swap::ReceiveSwapStateHandler, swapper::BoltzSwapper,
};

use super::wallet::new_onchain_wallet;

pub(crate) fn new_receive_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<ReceiveSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(BoltzSwapper::new(config.clone(), None));
    let liquid_chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));

    Ok(ReceiveSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        liquid_chain_service,
    ))
}
