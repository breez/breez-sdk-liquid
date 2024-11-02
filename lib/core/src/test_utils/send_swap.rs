#![cfg(test)]

use std::sync::Arc;

use crate::{
    model::{Config, Signer},
    persist::Persister,
    send_swap::SendSwapHandler,
    sync::SyncService,
};
use anyhow::Result;
use tokio::sync::Mutex;

use super::{
    chain::MockLiquidChainService,
    swapper::MockSwapper,
    sync::MockSyncerClient,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_send_swap_handler(persister: Arc<Persister>) -> Result<SendSwapHandler> {
    let config = Config::testnet(None);
    let onchain_wallet = Arc::new(MockWallet::new());
    let swapper = Arc::new(MockSwapper::new());
    let chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));

    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()));
    let syncer_client = Box::new(MockSyncerClient::new());
    let sync_service = Arc::new(SyncService::new(
        "".to_string(),
        persister.clone(),
        signer.clone(),
        syncer_client,
    ));

    Ok(SendSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
        sync_service,
    ))
}
