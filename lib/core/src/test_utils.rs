#![cfg(test)]
use std::sync::Arc;

use crate::{
    chain::{bitcoin, liquid::HybridLiquidChainService},
    chain_swap::ChainSwapStateHandler,
    model::{
        ChainSwap, Config, Direction, LiquidNetwork, PaymentState, PaymentTxData, PaymentType,
        ReceiveSwap, SendSwap,
    },
    persist::Persister,
    receive_swap::ReceiveSwapStateHandler,
    send_swap::SendSwapStateHandler,
    swapper::BoltzSwapper,
    utils,
    wallet::LiquidOnchainWallet,
};

use anyhow::{anyhow, Result};
use lwk_wollet::secp256k1::rand::{self, distributions::Alphanumeric, Rng};
use lwk_wollet::ElectrumUrl;
use tempdir::TempDir;
use tokio::sync::Mutex;

pub(crate) const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

pub(crate) fn new_send_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<SendSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(BoltzSwapper::new(config.clone(), None));
    let chain_service = Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));

    Ok(SendSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        chain_service,
    ))
}

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

pub(crate) fn new_send_swap(payment_state: Option<PaymentState>) -> SendSwap {
    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    let invoice = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    SendSwap {
        id,
        invoice,
        preimage: None,
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        create_response_json: "{}".to_string(),
        lockup_tx_id: None,
        refund_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
        refund_private_key: "".to_string(),
    }
}

pub(crate) fn new_receive_swap(payment_state: Option<PaymentState>) -> ReceiveSwap {
    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    ReceiveSwap {
        id,
        preimage: "".to_string(),
        create_response_json: "{}".to_string(),
        claim_private_key: "".to_string(),
        invoice: "".to_string(),
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        claim_fees_sat: 0,
        claim_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
    }
}

pub(crate) fn new_chain_swap(payment_state: Option<PaymentState>) -> ChainSwap {
    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
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

pub(crate) fn new_persister() -> Result<(TempDir, Persister)> {
    let temp_dir = TempDir::new("liquid-sdk")?;
    let persister = Persister::new(
        temp_dir
            .path()
            .to_str()
            .ok_or(anyhow!("Could not create temporary directory"))?,
        LiquidNetwork::Testnet,
    )?;
    persister.init()?;
    Ok((temp_dir, persister))
}

pub(crate) fn new_payment_tx_data(payment_type: PaymentType) -> PaymentTxData {
    let tx_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    PaymentTxData {
        tx_id,
        timestamp: None,
        amount_sat: 0,
        fees_sat: 0,
        payment_type,
        is_confirmed: false,
    }
}

pub(crate) fn new_onchain_wallet(config: &Config) -> Result<LiquidOnchainWallet> {
    LiquidOnchainWallet::new(TEST_MNEMONIC.to_string(), config.clone())
}
