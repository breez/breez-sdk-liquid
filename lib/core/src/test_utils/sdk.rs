#![cfg(test)]

use anyhow::{anyhow, Result};
use sdk_common::prelude::{MockRestClient, RestClient, STAGING_BREEZSERVER_URL};
use std::sync::Arc;

use crate::{
    model::{Config, Signer},
    persist::Persister,
    recover::recoverer::Recoverer,
    sdk::{LiquidSdk, LiquidSdkBuilder},
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
) -> Result<Arc<LiquidSdk>> {
    let liquid_chain_service = Arc::new(MockLiquidChainService::new());
    let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());

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
    liquid_chain_service: Arc<MockLiquidChainService>,
    bitcoin_chain_service: Arc<MockBitcoinChainService>,
    onchain_fee_rate_leeway_sat_per_vbyte: Option<u32>,
) -> Result<Arc<LiquidSdk>> {
    let mut config = Config::testnet(None);
    config.working_dir = persister
        .get_database_dir()
        .to_str()
        .ok_or(anyhow!("An invalid SDK directory was specified"))?
        .to_string();
    config.onchain_fee_rate_leeway_sat_per_vbyte = onchain_fee_rate_leeway_sat_per_vbyte;

    let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
    let rest_client: Arc<dyn RestClient> = Arc::new(MockRestClient::new());
    let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
    let recoverer = Arc::new(Recoverer::new(
        signer.slip77_master_blinding_key()?,
        swapper.clone(),
        onchain_wallet.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
    )?);

    let (_incoming_tx, _outgoing_records, sync_service) =
        new_sync_service(persister.clone(), recoverer.clone(), signer.clone())?;
    let sync_service = Arc::new(sync_service);

    LiquidSdkBuilder::new(config, STAGING_BREEZSERVER_URL.into(), signer)?
        .bitcoin_chain_service(bitcoin_chain_service)
        .liquid_chain_service(liquid_chain_service)
        .onchain_wallet(onchain_wallet)
        .persister(persister)
        .recoverer(recoverer)
        .rest_client(rest_client)
        .status_stream(status_stream)
        .swapper(swapper)
        .sync_service(sync_service)
        .build()
}
