use std::sync::Arc;

use crate::{
    model::{Config, Signer},
    persist::Persister,
    recover::recoverer::Recoverer,
    send_swap::SendSwapHandler,
    utils,
};
use anyhow::Result;

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    swapper::MockSwapper,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_mock_wallet() -> Result<Arc<MockWallet>> {
    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    Ok(Arc::new(MockWallet::new(signer)?))
}

pub(crate) fn new_send_swap_handler(
    persister: std::sync::Arc<Persister>,
) -> Result<SendSwapHandler> {
    new_send_swap_handler_with_mocks(
        persister,
        Arc::new(MockLiquidChainService::new()),
        new_mock_wallet()?,
    )
}

/// Same as [`new_send_swap_handler`], but lets the caller keep handles on the chain service used
/// for broadcasting and on the wallet, so their behaviour can be scripted and asserted on.
pub(crate) fn new_send_swap_handler_with_mocks(
    persister: std::sync::Arc<Persister>,
    chain_service: Arc<MockLiquidChainService>,
    onchain_wallet: Arc<MockWallet>,
) -> Result<SendSwapHandler> {
    let config = Config::regtest_esplora();
    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let swapper = Arc::new(MockSwapper::default());
    let liquid_chain_service = Arc::new(MockLiquidChainService::new());
    let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());
    let recoverer = Arc::new(Recoverer::new(
        signer.slip77_master_blinding_key()?,
        utils::lbtc_asset_id(config.network),
        swapper.clone(),
        onchain_wallet.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
        persister.clone(),
    )?);

    Ok(SendSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
        recoverer,
    ))
}
