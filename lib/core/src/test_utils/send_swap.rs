use crate::{
    model::{Config, Signer},
    persist::Persister,
    recover::recoverer::Recoverer,
    send_swap::SendSwapHandler,
};
use anyhow::Result;
use sdk_common::utils::Arc;

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    swapper::MockSwapper,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_send_swap_handler(persister: Arc<Persister>) -> Result<SendSwapHandler> {
    let config = Config::testnet_esplora(None);
    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
    let swapper = Arc::new(MockSwapper::default());
    let chain_service = Arc::new(MockLiquidChainService::new());
    let liquid_chain_service = Arc::new(MockLiquidChainService::new());
    let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());
    let recoverer = Arc::new(Recoverer::new(
        signer.slip77_master_blinding_key()?,
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
