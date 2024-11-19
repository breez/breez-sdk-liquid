#![cfg(test)]

use anyhow::{anyhow, Result};
use bip39::rand::{self, RngCore};
use sdk_common::{
    bitcoin::{
        hashes::{sha256, Hash},
        secp256k1::{Secp256k1, SecretKey},
    },
    lightning::ln::PaymentSecret,
    lightning_invoice::{Currency, InvoiceBuilder},
};
use tempdir::TempDir;

use crate::{
    model::{LiquidNetwork, PaymentState, PaymentTxData, PaymentType, ReceiveSwap, SendSwap},
    persist::Persister,
    test_utils::generate_random_string,
    utils,
};

fn new_secret_key() -> SecretKey {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 32];
    rng.fill_bytes(&mut buf);
    SecretKey::from_slice(&buf).expect("Expected valid secret key")
}

pub(crate) fn new_send_swap(payment_state: Option<PaymentState>) -> SendSwap {
    let private_key = new_secret_key();

    let payment_hash = sha256::Hash::from_slice(&[0; 32][..]).expect("Expecting valid hash");
    let invoice = InvoiceBuilder::new(Currency::BitcoinTestnet)
        .description("Test invoice".into())
        .payment_hash(payment_hash)
        .payment_secret(PaymentSecret([42u8; 32]))
        .current_timestamp()
        .min_final_cltv_expiry_delta(144)
        .build_signed(|hash| Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key))
        .expect("Expected valid invoice");

    SendSwap {
        id: generate_random_string(4),
        invoice: invoice.to_string(),
        bolt12_offer: None,
        payment_hash: Some(payment_hash.to_string()),
        description: Some("Send to BTC lightning".to_string()),
        preimage: None,
        payer_amount_sat: 1149,
        receiver_amount_sat: 1000,
        create_response_json: r#"{
            "accept_zero_conf": true,
            "address": "tlq1pqwq5ft2l0khw7fr2f0fzfz5c00lku06sy9sgqlzhuj8y5vgslfx6y2pffw53ksu76uv25zkss8vpam96y8n2ke826mfmklaeg057guneaf8hr0ckqh0z",
            "bip21": "liquidtestnet:tlq1pqwq5ft2l0khw7fr2f0fzfz5c00lku06sy9sgqlzhuj8y5vgslfx6y2pffw53ksu76uv25zkss8vpam96y8n2ke826mfmklaeg057guneaf8hr0ckqh0z?amount=0.00001149&label=Send%20to%20BTC%20lightning&assetid=144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49",
            "claim_public_key": "023bb9487e9b3faebad3d358b1c24ca91a6ce9ed8417ac5e0b65fa4918f644b08b",
            "expected_amount": 1149,
            "swap_tree": {
                "claim_leaf": {
                    "output": "a9144d716c8c50228c1fc07a2e354bfa51899ded90f088203bb9487e9b3faebad3d358b1c24ca91a6ce9ed8417ac5e0b65fa4918f644b08bac",
                    "version": 196
                },
                "refund_leaf": {
                    "output": "20a1d004d26c27c219fd005212596a0c0eb3be3a48f84443a13199c26568624634ad03f56c16b1",
                    "version": 196
                }
            },
            "timeout_block_height": 1459611,
            "blinding_key": "1eabe70f75a3c92e1ce1e4108a014a275a4b03415234c87d8670e29d70059326"
        }"#.to_string(),
        lockup_tx_id: None,
        refund_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
        refund_private_key: "945affeef55f12227f1d4a3f80a17062a05b229ddc5a01591eb5ddf882df92e3".to_string(),
    }
}

pub(crate) fn new_receive_swap(payment_state: Option<PaymentState>) -> ReceiveSwap {
    ReceiveSwap {
        id: generate_random_string(4),
        preimage: "49ef4cb865d78519e5b3cf6aae6b409e1b471fe8ddbda744582e23665a2252cf".to_string(),
        description: Some("Send to L-BTC address".to_string()),
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
        payment_hash: Some("e4a0052cd7e967393b71803ef5c507b935e6e80c18de4b110b26332ad64c6fe4".to_string()),
        payer_amount_sat: 1000,
        receiver_amount_sat: 587,
        claim_fees_sat: 200,
        claim_tx_id: None,
        lockup_tx_id: None,
        mrh_address: "tlq1pq2amlulhea6ltq7x3eu9atsc2nnrer7yt7xve363zxedqwu2mk6ctcyv9awl8xf28cythreqklt5q0qqwsxzlm6wu4z6d574adl9zh2zmr0h85gt534n".to_string(),
        mrh_script_pubkey: "tex1qnkznyyxwnxnkk0j94cnvq27h24jk6sqf0te55x".to_string(),
        mrh_tx_id: None,
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
