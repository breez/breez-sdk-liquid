#![cfg(test)]

use anyhow::{anyhow, Result};
use tempdir::TempDir;

use crate::{
    model::{LiquidNetwork, PaymentState, PaymentTxData, PaymentType, ReceiveSwap, SendSwap},
    persist::Persister,
    test_utils::generate_random_string,
    utils,
};

pub(crate) fn new_send_swap(payment_state: Option<PaymentState>) -> SendSwap {
    let id = generate_random_string(4);
    let invoice = generate_random_string(4);
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
    let id = generate_random_string(4);
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
    let tx_id = generate_random_string(4);
    PaymentTxData {
        tx_id,
        timestamp: None,
        amount_sat: 0,
        fees_sat: 0,
        payment_type,
        is_confirmed: false,
    }
}
