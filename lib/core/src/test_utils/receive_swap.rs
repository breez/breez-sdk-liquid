#![cfg(test)]

use anyhow::Result;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    model::{Config, Signer},
    persist::Persister,
    receive_swap::ReceiveSwapHandler,
};

use super::{
    chain::MockLiquidChainService,
    swapper::MockSwapper,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_receive_swap_handler(persister: Arc<Persister>) -> Result<ReceiveSwapHandler> {
    let config = Config::testnet(None);
    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let onchain_wallet = Arc::new(MockWallet::new(signer)?);
    let swapper = Arc::new(MockSwapper::new());
    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));

    Ok(ReceiveSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        liquid_chain_service,
    ))
}
