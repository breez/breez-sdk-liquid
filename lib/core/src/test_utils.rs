#![cfg(test)]
use std::sync::Arc;

use crate::{
    model::{Config, Network, PaymentState, PaymentTxData, PaymentType, ReceiveSwap, SendSwap},
    persist::Persister,
    receive_swap::ReceiveSwapStateHandler,
    send_swap::SendSwapStateHandler,
    swapper::BoltzSwapper,
    utils,
    wallet::test_utils::new_onchain_wallet,
};

use anyhow::{anyhow, Result};
use bip39::rand::{self, distributions::Alphanumeric, Rng};
use lwk_wollet::{ElectrumClient, ElectrumUrl};
use tempdir::TempDir;

pub(crate) fn new_send_swap_state_handler(
    persister: Arc<Persister>,
) -> Result<SendSwapStateHandler> {
    let config = Config::testnet();
    let onchain_wallet = Arc::new(new_onchain_wallet(&config)?);
    let swapper = Arc::new(BoltzSwapper::new(config.clone()));
    let chain_service = Arc::new(ElectrumClient::new(&ElectrumUrl::new(
        &config.electrum_url,
        true,
        true,
    ))?);

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
    let swapper = Arc::new(BoltzSwapper::new(config.clone()));

    Ok(ReceiveSwapStateHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
    ))
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

pub(crate) struct TempPersister {
    pub(crate) persister: Persister,
    #[allow(dead_code)]
    pub(crate) temp_dir: TempDir,
}

pub(crate) fn new_temp_persister() -> Result<TempPersister> {
    let temp_dir = TempDir::new("liquid-sdk")?;
    let persister = Persister::new(
        temp_dir
            .path()
            .to_str()
            .ok_or(anyhow!("Could not create temporary directory"))?,
        Network::Testnet,
    )?;
    persister.init()?;
    Ok(TempPersister {
        persister,
        temp_dir,
    })
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
