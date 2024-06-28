#![cfg(test)]

use anyhow::{anyhow, Result};
use sdk_common::prelude::{BreezServer, STAGING_BREEZSERVER_URL};
use std::sync::Arc;

use tokio::sync::{watch, Mutex, RwLock};

use crate::{
    chain_swap::ChainSwapStateHandler, event::EventManager, model::Config, persist::Persister,
    receive_swap::ReceiveSwapStateHandler, sdk::LiquidSdk, send_swap::SendSwapStateHandler,
};

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    status_stream::MockStatusStream,
    swapper::MockSwapper,
    wallet::new_onchain_wallet,
};

pub(crate) fn new_liquid_sdk(
    persister: Arc<Persister>,
    status_stream: Arc<MockStatusStream>,
) -> Result<LiquidSdk> {
    let mut config = Config::testnet();
    config.working_dir = persister
        .get_database_dir()
        .to_str()
        .ok_or(anyhow!("An invalid SDK directory was specified"))?
        .to_string();

    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(MockSwapper::new());

    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    let send_swap_state_handler = SendSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let receive_swap_state_handler = ReceiveSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let chain_swap_state_handler = Arc::new(ChainSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
    )?);

    let event_manager = Arc::new(EventManager::new());
    let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

    let fiat_api = Arc::new(BreezServer::new(STAGING_BREEZSERVER_URL.into(), None)?);

    Ok(LiquidSdk {
        config,
        onchain_wallet,
        persister,
        event_manager,
        status_stream,
        swapper,
        liquid_chain_service,
        bitcoin_chain_service,
        fiat_api,
        is_started: RwLock::new(true),
        shutdown_sender,
        shutdown_receiver,
        send_swap_state_handler,
        receive_swap_state_handler,
        chain_swap_state_handler,
    })
}
