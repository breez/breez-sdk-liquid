use bip39::rand::{self, RngCore};
use sdk_common::{
    bitcoin::{
        hashes::{sha256, Hash},
        secp256k1::{Secp256k1, SecretKey},
    },
    lightning::ln::PaymentSecret,
    lightning_invoice::{Currency, InvoiceBuilder},
    prelude::invoice_pubkey,
};
use std::time::SystemTime;

use crate::{
    model::{LiquidNetwork, PaymentState, PaymentTxData, PaymentType, ReceiveSwap, SendSwap},
    test_utils::generate_random_string,
    utils,
};

fn new_secret_key() -> SecretKey {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 32];
    rng.fill_bytes(&mut buf);
    SecretKey::from_slice(&buf).expect("Expected valid secret key")
}

pub fn new_send_swap(
    payment_state: Option<PaymentState>,
    receiver_amount_sat: Option<u64>,
) -> SendSwap {
    let private_key = new_secret_key();

    let payment_hash = sha256::Hash::from_slice(&[0; 32][..]).expect("Expecting valid hash");
    let invoice = InvoiceBuilder::new(Currency::BitcoinTestnet)
        .description("Test invoice".into())
        .payment_hash(payment_hash)
        .payment_secret(PaymentSecret([42u8; 32]))
        .timestamp(SystemTime::UNIX_EPOCH)
        .min_final_cltv_expiry_delta(144)
        .build_signed(|hash| Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key))
        .expect("Expected valid invoice");
    let destination_pubkey = invoice_pubkey(&invoice);

    SendSwap {
        id: generate_random_string(4),
        invoice: invoice.to_string(),
        bolt12_offer: None,
        payment_hash: Some(payment_hash.to_string()),
        destination_pubkey: Some(destination_pubkey),
        timeout_block_height: 1459611,
        description: Some("Send to BTC lightning".to_string()),
        preimage: None,
        payer_amount_sat: 1149,
        receiver_amount_sat: receiver_amount_sat.unwrap_or(1000),
        pair_fees_json: r#"{
            "hash": "b53c0ac3da051a78f67f6dd25f2ab0858492dc6881015b236d554227c85fda7d",
            "rate": 1,
            "limits": {
                "maximal": 25000000,
                "minimal": 1000,
                "maximalZeroConf": 100000
            },
            "fees": {
                "percentage": 0.1,
                "minerFees": 148
            }
        }"#.to_string(),
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
        refund_address: None,
        refund_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
        refund_private_key: "945affeef55f12227f1d4a3f80a17062a05b229ddc5a01591eb5ddf882df92e3".to_string(),
        metadata: Default::default(),
    }
}

pub fn new_receive_swap(
    payment_state: Option<PaymentState>,
    receiver_amount_sat: Option<u64>,
) -> ReceiveSwap {
    ReceiveSwap {
        id: generate_random_string(4),
        preimage: "7d3ef1b83ea380e570100c54efc164cd258f5014568eed41780b0561997c4f9f".to_string(),
        timeout_block_height: 1459611,
        description: Some("Send to L-BTC address".to_string()),
        create_response_json: r#"{"swap_tree":{"claim_leaf":{"output":"82012088a91476089e96a323d103b4d9546ab0b64505672197f58820d5272b21c51e7fe6a2e0d6b3ddafde514ff2b31ca70399a3a0960f19f3b1853dac","version":196},"refund_leaf":{"output":"20859b5e5e3b66c76e0920a21a41f4a64246caf2cf0084307c447ba94a2e3a483dad03842231b1","version":196}},"lockup_address":"lq1pq0ka6jmyx62herardll0ccu3zze4qvmh04vnzdw4c5338rp3yquggh47wr29jh6akr6mtw2zzrgn6nuv68setq76d2uk9fqs0l84z7t2jhw58m0crqu4","refund_public_key":"03859b5e5e3b66c76e0920a21a41f4a64246caf2cf0084307c447ba94a2e3a483d","timeout_block_height":3220100,"onchain_amount":971,"blinding_key":"ef121ccd2906a4cc80f8a9b33b18fa2ba4de7e9032b7b143bfc816494d46dc66"}"#.to_string(),
        claim_private_key: "08e4555d4388552fe6a72a89953b3d333ddbb66b7ae2167f5f66327ec66cede1".to_string(),
        invoice: "lnbc10u1pnez5ulsp5szkn8zq25p99m3kkhcyv5xfaszvya80gca2efduhp9v0g3qy9spqpp52m3vvah5xj8mzu6knwl4gtcymzg9w7lmm90yctwr39kae36sjpsqdpz2djkuepqw3hjqnpdgf2yxgrpv3j8yetnwvxqyp2xqcqz95rzjqt2jw2epc508le4zurtt8hd0meg5lu4nrjns8xdr5ztq7x0nkxzn6zzxeyqq28qqqqqqqqqqqqqqq9gq2y9qyysgq4kue7c5mrla8cxgzlpddvl62a3quzpkhlza84tkrxea3hmvq4zcnn2rcve7l9cu5xdxglflerp5rcyeyc88j33mht4fea60jj9e7cqspe058nk".to_string(),
        payment_hash: Some("56e2c676f4348fb173569bbf542f04d890577bfbd95e4c2dc3896ddcc7509060".to_string()),
        destination_pubkey: Some("02d96eadea3d780104449aca5c93461ce67c1564e2e1d73225fa67dd3b997a6018".to_string()),
        payer_amount_sat: 1000,
        receiver_amount_sat: receiver_amount_sat.unwrap_or(957),
        pair_fees_json: r#"{"hash":"b32246ad7d9c9b1ff499a36e226b5e2fb5b83d78cbc8a70b6d3429b80bfc5876","rate":1.0,"limits":{"maximal":25000000,"minimal":1000},"fees":{"percentage":0.25,"minerFees":{"lockup":26,"claim":14}}}"#.to_string(),
        claim_fees_sat: 14,
        claim_address: None,
        claim_tx_id: None,
        lockup_tx_id: None,
        mrh_address: "lq1qqdgpjf28g2r27urtan4grfr9206adax5jm94uv68mvpe40lye6aa36x99kklezup4tcs5fvm8sgaz329stru560s8tz65fruz".to_string(),
        mrh_tx_id: None,
        created_at: utils::now(),
        state: payment_state.unwrap_or(PaymentState::Created),
        metadata: Default::default(),
    }
}

#[macro_export]
macro_rules! create_persister {
    ($name:ident) => {
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        let $name = {
            let db_id = {
                use rand::Rng;
                let res: String = rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();
                res
            };
            sdk_common::utils::Arc::new($crate::persist::Persister::new_in_memory(
                &db_id,
                $crate::model::LiquidNetwork::Testnet,
                true,
                None,
                None,
            )?)
        };
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        let $name = {
            let temp_dir_path = tempdir::TempDir::new("liquid-sdk")?
                .path()
                .to_str()
                .ok_or(anyhow::anyhow!("Could not create temporary directory"))?
                .to_string();
            sdk_common::utils::Arc::new($crate::persist::Persister::new_using_fs(
                &temp_dir_path,
                $crate::model::LiquidNetwork::Testnet,
                true,
                None,
            )?)
        };
    };
}
pub use create_persister;

pub(crate) fn new_payment_tx_data(
    network: LiquidNetwork,
    payment_type: PaymentType,
) -> PaymentTxData {
    PaymentTxData {
        tx_id: generate_random_string(4),
        timestamp: None,
        asset_id: utils::lbtc_asset_id(network).to_string(),
        amount: 0,
        fees_sat: 0,
        payment_type,
        is_confirmed: false,
        unblinding_data: None,
    }
}
