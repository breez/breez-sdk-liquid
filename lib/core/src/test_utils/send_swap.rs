#![cfg(test)]

use std::sync::Arc;

use crate::{
    model::{Config, Signer},
    persist::Persister,
    send_swap::SendSwapHandler,
};
use anyhow::Result;

use super::{
    chain::MockLiquidChainService,
    swapper::MockSwapper,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_send_swap_handler(persister: Arc<Persister>) -> Result<SendSwapHandler> {
    let config = Config::testnet(None);
    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let onchain_wallet = Arc::new(MockWallet::new(signer)?);
    let swapper = Arc::new(MockSwapper::default());
    let chain_service = Arc::new(MockLiquidChainService::new());

    Ok(SendSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
    ))
}
