#![cfg(test)]

use anyhow::{anyhow, Result};
use sdk_common::prelude::{BreezServer, STAGING_BREEZSERVER_URL};
use std::sync::Arc;

use tokio::sync::{watch, Mutex, RwLock};

use crate::{
    buy::BuyBitcoinService,
    chain_swap::ChainSwapHandler,
    event::EventManager,
    model::{Config, Signer},
    persist::Persister,
    receive_swap::ReceiveSwapHandler,
    recover::recoverer::Recoverer,
    sdk::LiquidSdk,
    send_swap::SendSwapHandler,
};

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    status_stream::MockStatusStream,
    swapper::MockSwapper,
    sync::new_sync_service,
    wallet::{MockSigner, MockWallet},
};

pub(crate) fn new_liquid_sdk(
    persister: Arc<Persister>,
    swapper: Arc<MockSwapper>,
    status_stream: Arc<MockStatusStream>,
) -> Result<LiquidSdk> {
    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    new_liquid_sdk_with_chain_services(
        persister,
        swapper,
        status_stream,
        liquid_chain_service,
        bitcoin_chain_service,
        None,
    )
}

pub(crate) fn new_liquid_sdk_with_chain_services(
    persister: Arc<Persister>,
    swapper: Arc<MockSwapper>,
    status_stream: Arc<MockStatusStream>,
    liquid_chain_service: Arc<Mutex<MockLiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<MockBitcoinChainService>>,
    onchain_fee_rate_leeway_sat_per_vbyte: Option<u32>,
) -> Result<LiquidSdk> {
    let mut config = Config::testnet(None);
    config.working_dir = persister
        .get_database_dir()
        .to_str()
        .ok_or(anyhow!("An invalid SDK directory was specified"))?
        .to_string();
    config.onchain_fee_rate_leeway_sat_per_vbyte = onchain_fee_rate_leeway_sat_per_vbyte;

    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);

    let send_swap_handler = SendSwapHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let receive_swap_handler = ReceiveSwapHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let chain_swap_handler = Arc::new(ChainSwapHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
    )?);

    let recoverer = Arc::new(Recoverer::new(
        signer.slip77_master_blinding_key()?,
        swapper.clone(),
        onchain_wallet.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
    )?);

    let event_manager = Arc::new(EventManager::new());
    let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

    let breez_server = Arc::new(BreezServer::new(STAGING_BREEZSERVER_URL.into(), None)?);

    let buy_bitcoin_service =
        Arc::new(BuyBitcoinService::new(config.clone(), breez_server.clone()));

    let (_incoming_tx, _outgoing_records, sync_service) =
        new_sync_service(persister.clone(), recoverer.clone(), signer.clone())?;
    let sync_service = Some(Arc::new(sync_service));

    Ok(LiquidSdk {
        config,
        onchain_wallet,
        signer,
        persister,
        event_manager,
        status_stream,
        swapper,
        recoverer,
        liquid_chain_service,
        bitcoin_chain_service,
        fiat_api: breez_server,
        is_started: RwLock::new(true),
        shutdown_sender,
        shutdown_receiver,
        send_swap_handler,
        receive_swap_handler,
        sync_service,
        chain_swap_handler,
        buy_bitcoin_service,
        external_input_parsers: Vec::new(),
    })
}
