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
    SendSwap {
        id: generate_random_string(4),
        invoice: generate_random_string(4),
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
    ReceiveSwap {
        id: generate_random_string(4),
        preimage: "49ef4cb865d78519e5b3cf6aae6b409e1b471fe8ddbda744582e23665a2252cf".to_string(),
        create_response_json: r#"{
            "swap_tree": {
                "claim_leaf": {
                    "output": "82012088a9140383457bbf2cec402b74a408fdfc43a800ee9a0088206a3c0b798ae842c0b54d8de3610ebcb4221574d0dc0be44547cf0a2acf860474ac",
                    "version": 196
                },
                "refund_leaf": {
                    "output": "20c95af4b20b6146d86487389306445ffb8893af21bbcad7fedfa2223df16bc190ad039b4516b1",
                    "version": 196
                }
            },
            "lockup_address": "tlq1pqdgzxrqac50pmn40f46alyuc9n90zafdu2x2r7ks5lmdu2n8u4tlh5nrnxv7nvdqjyehm3fqkzv5g0e2plxc0u3zj304hva3usshjf6ev9ezza8p5gsc",
            "refund_public_key": "02c95af4b20b6146d86487389306445ffb8893af21bbcad7fedfa2223df16bc190",
            "timeout_block_height": 1459611,
            "onchain_amount": 721,
            "blinding_key": "303a4865fa98083afb34d474db04d4dc45d122105aa0cffad1b97af81496e6d8"
        }"#.to_string(),
        claim_private_key: "179dc5137d2c211fb84e2159252832658afb6d03e095fb5cf324a2b782d2a5ca".to_string(),
        invoice: "lntb10u1pngqdj3pp5ujsq2txha9nnjwm3sql0t3g8hy67d6qvrr0ykygtycej44jvdljqdpz2djkuepqw3hjqnpdgf2yxgrpv3j8yetnwvcqz95xqyp2xqrzjqf4rczme3t5y9s94fkx7xcgwhj6zy9t56rwqhez9gl8s52k0scz8gzzxeyqq28qqqqqqqqqqqqqqq9gq2ysp5fmynazrpmuz05vp8r5dxpu9cupkaus7hcd258saklp3v79azt6qs9qxpqysgq5sxknac9fwe69q5vzffgayjddskzhjeyu6h8vx45m4svchsy2e3rv6yc3puht7pjzvhwfl7ljamkzfy2dsa75fxd5j82ug0ty0y4xhgq82gc9k".to_string(),
        payer_amount_sat: 1000,
        receiver_amount_sat: 587,
        claim_fees_sat: 200,
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
    PaymentTxData {
        tx_id: generate_random_string(4),
        timestamp: None,
        amount_sat: 0,
        fees_sat: 0,
        payment_type,
        is_confirmed: false,
    }
}
