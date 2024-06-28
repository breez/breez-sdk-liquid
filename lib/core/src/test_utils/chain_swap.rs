#![cfg(test)]

use anyhow::Result;
use std::sync::Arc;

use lwk_wollet::ElectrumUrl;
use tokio::sync::Mutex;

use crate::{
    chain::{bitcoin, liquid::HybridLiquidChainService},
    chain_swap::ChainSwapStateHandler,
    model::{ChainSwap, Config, Direction, PaymentState},
    persist::Persister,
    swapper::BoltzSwapper,
    utils,
};

use super::{generate_random_string, wallet::new_onchain_wallet};

pub(crate) fn new_chain_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<ChainSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(BoltzSwapper::new(config.clone(), None));
    let liquid_chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));
    let bitcoin_chain_service = Arc::new(Mutex::new(bitcoin::ElectrumClient::new(
        &ElectrumUrl::new(&config.bitcoin_electrum_url, true, true),
    )?));

    ChainSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        liquid_chain_service,
        bitcoin_chain_service,
    )
}

pub(crate) fn new_chain_swap(payment_state: Option<PaymentState>) -> ChainSwap {
    let id = generate_random_string(4);
    ChainSwap {
        id,
        direction: Direction::Incoming,
        claim_address: "".to_string(),
        lockup_address: "".to_string(),
        timeout_block_height: 0,
        preimage: "".to_string(),
        create_response_json: "{}".to_string(),
        claim_private_key: "".to_string(),
        refund_private_key: "".to_string(),
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        claim_fees_sat: 0,
        server_lockup_tx_id: None,
        user_lockup_tx_id: None,
        claim_tx_id: None,
        refund_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
        accept_zero_conf: false,
    }
}
