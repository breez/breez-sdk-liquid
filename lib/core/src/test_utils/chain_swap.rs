#![cfg(test)]

use anyhow::Result;
use hex::FromHex;
use lazy_static::lazy_static;
use sdk_common::bitcoin::{consensus::deserialize, Transaction};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    chain_swap::ChainSwapHandler,
    model::{ChainSwap, Config, Direction, PaymentState},
    persist::Persister,
    swapper::boltz::BoltzSwapper,
    utils,
};

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    generate_random_string,
    wallet::MockWallet,
};

lazy_static! {
    pub(crate) static ref TEST_BITCOIN_TX: Transaction = deserialize(&Vec::<u8>::from_hex("01000000000101da6af195321dfa98218c7deafa2da6d39d8d4a809a811de87269ddc4c4d28c810100000000ffffffff0c30c80700000000002251200894aacf46d0eed22594ed328b1e6806e94e662a4494f07cbca80720c3435e4130c807000000000022512098a3a5a9d34ebf22ced8f0056457164c9a9ee6c6eaef110c1a0cb465ac541d9130c807000000000022512050e1a1af89928af930b3bd0b826b40b2f3072c0009cd3186ae3ae23d0504f97930c80700000000002251201a55eb37d4331f8f367c0d4c727b565da089b8ab3d10e7079f1e3c2ae3b1123a30c8070000000000225120247e5ea29cb7bcec21b1bea1ed1f778adf887ab1bb04faaa014afd4ec8a2c0bb30c8070000000000225120330280e4540a00dace540ec2119d608024c11024a2bc8c7c8b652998fa42248330c80700000000002251202645e1ea344306e9068f1d086a08d22a30b0e5f839f790e924df081255b0a9c930c80700000000002251200b541effc0522207e5284a26071741e2b8302a964cb8b078d24540d73b9ec59430c807000000000022512041a1883aa113fbc5bf69387e0d599eef86181f0c916bdb55378d907291702f5530c80700000000002251206c2eb1f12ce37b57524337c6de7a55ee420edd100d6cb3ea50f512ef68d2075b30c807000000000022512090ea942f7c3eed7eb073682d38202bcae9df106034a3bd406dac13371bf18a0987e2d21f0000000016001471c1c386a4772bbc7f39dc7c7e75a17ff5d1e92402483045022100cbf19c0563a70378e26b5c9c1e2a77e4783f8926717457899efc4491bf3402c4022078e1b5e4d759eea100b3659f8a421866e96ff7e23e161c56dd979961e7b6d205012103bbcd5914f15887ed609c6278c077241cd95f80dc199989f89f968ff007fe8c0000000000").unwrap()).unwrap();
}

pub(crate) fn new_chain_swap_handler(persister: Arc<Persister>) -> Result<ChainSwapHandler> {
    let config = Config::testnet("".to_string());
    let onchain_wallet = Arc::new(MockWallet::new());
    let swapper = Arc::new(BoltzSwapper::new(config.clone(), None));
    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    ChainSwapHandler::new(
        config,
        onchain_wallet,
        persister,
        swapper,
        liquid_chain_service,
        bitcoin_chain_service,
    )
}

pub(crate) fn new_chain_swap(
    direction: Direction,
    payment_state: Option<PaymentState>,
    accept_zero_conf: bool,
    user_lockup_tx_id: Option<String>,
) -> ChainSwap {
    match direction {
        Direction::Incoming => ChainSwap {
            id: generate_random_string(4),
            direction,
            claim_address: "tlq1qq0nn497zr4l6nfq84pxzqwme87n7kz09lvnx94t7ecw045dvjr09s9s6ens46nt7qcrmx673vq6gkss50qhpcxywt3r5a44j2".to_string(),
            lockup_address: "tb1p7cftn5u3ndt8ln0m6hruwyhsz8kc5sxt557ua03qcew0z29u5paqh8f7uu".to_string(),
            timeout_block_height: 2868778,
            preimage: "bbce422d96c0386c3a6c1b1fe11fc7be3fdd871c6855db6ab2e319e96ec19c78".to_string(),
            description: Some("Bitcoin transfer".to_string()),
            create_response_json: r#"{
              "claim_details": {
                "swapTree": {
                  "claimLeaf": {
                    "output": "82012088a914e5ec6c5b814b2d8616c1a0da0acc8b3388cf80d78820e5f32fc89e6947ca08a7855a99ac145f7de599446a0cc0ff4c9aa2694baa1138ac",
                    "version": 196
                  },
                  "refundLeaf": {
                    "output": "20692bbff63e48c1c05c5efeb7080f7c2416d2f9ecb79d217410eabc125f4d2ff0ad0312a716b1",
                    "version": 196
                  }
                },
                "lockupAddress": "tlq1pq0gfse32q454tmr30t7yl6lx2sv5sswdzh3j0zygz9v5jwwdq6deaec8ntnjq55yrx300u9ts5ykqnfcpuzrypmtda9yuszq0zpl6j8l9tunvqjyrdm3",
                "serverPublicKey": "02692bbff63e48c1c05c5efeb7080f7c2416d2f9ecb79d217410eabc125f4d2ff0",
                "timeoutBlockHeight": 1484562,
                "amount": 0,
                "blindingKey": "ebdd91bb06b2282e879256ff1c1a976016a582fea5418188799b1598281b0a5b",
                "refundAddress": null,
                "claimAddress": null,
                "bip21": null
              },
              "lockup_details": {
                "swapTree": {
                  "claimLeaf": {
                    "output": "82012088a914e5ec6c5b814b2d8616c1a0da0acc8b3388cf80d7882039688adbf0625672ec56e713e65ce809ee84e96525a13a68fe521588bf41628cac",
                    "version": 192
                  },
                  "refundLeaf": {
                    "output": "20edf1db3da18ad19962c8dfd7566048c7dc2e11f3d6580cbfed8f9a1321ffe4c7ad032ac62bb1",
                    "version": 192
                  }
                },
                "lockupAddress": "tb1p7cftn5u3ndt8ln0m6hruwyhsz8kc5sxt557ua03qcew0z29u5paqh8f7uu",
                "serverPublicKey": "0239688adbf0625672ec56e713e65ce809ee84e96525a13a68fe521588bf41628c",
                "timeoutBlockHeight": 2868778,
                "amount": 18360,
                "blindingKey": null,
                "refundAddress": null,
                "claimAddress": null,
                "bip21": "bitcoin:tb1p7cftn5u3ndt8ln0m6hruwyhsz8kc5sxt557ua03qcew0z29u5paqh8f7uu?amount=0.0001836&label=Send%20to%20L-BTC%20address"
              }
            }"#.to_string(),
            claim_private_key: "4b04c3b95570fc48c7f33bc900b801245c2be31b90d41616477574aedc5b9d28".to_string(),
            refund_private_key: "9e23d322577cfeb2b5490f3f86db58c806004afcb7c88995927bfdfc1c64cd8c".to_string(),
            payer_amount_sat: 18360,
            receiver_amount_sat: 17592,
            claim_fees_sat: 144,
            server_lockup_tx_id: None,
            user_lockup_tx_id,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: payment_state.unwrap_or(PaymentState::Created),
            accept_zero_conf,
        },
        Direction::Outgoing => ChainSwap {
            id: generate_random_string(4),
            direction,
            claim_address: "14DeLtifrayJXAWft3qhPbdY4HVJUgMyx1".to_string(),
            lockup_address: "tlq1pqg4e5r5a59gdl26ud6s7gna3mchqs20ycwl2lp67ejzy69fl7dwccwx9nqtr6ef848k7vpmvmdhsyeq2wp3vtn3gnlenhd0wrasv4qvr2dk0nz5tu0rw".to_string(),
            timeout_block_height: 1481523,
            preimage: "a95a028483df6112c15fdef513d9d8255ff0951d5c0856f85cf9c98352a0f71a".to_string(),
            description: Some("Bitcoin transfer".to_string()),
            create_response_json: r#"{
                "claim_details": {
                    "swapTree":{ 
                        "claimLeaf": {
                            "output": "82012088a9146a01e0a34b4e581da5133b5113b54b9033bb93dc8820dcfe4c6b840656e9e9cd53ba8b917d27a8091cba93b115b38e38d006d8a64e07ac",
                            "version": 192
                        },
                        "refundLeaf":{
                            "output": "20265c09aff38287656da668bf69f9a4372fe7f4c788afef3e481c3bf99d7da54cad034ac42bb1",
                            "version": 192
                        }
                    },
                    "lockupAddress": "tb1pujr9d8sqwvhjq8z9fdpp25jjm8t0934qyg22mj36g06cx8g5r8cst6eq8p",
                    "serverPublicKey": "03265c09aff38287656da668bf69f9a4372fe7f4c788afef3e481c3bf99d7da54c",
                    "timeoutBlockHeight": 2868298,
                    "amount": 0,
                    "blindingKey": null,
                    "refundAddress": null,
                    "claimAddress": null,
                    "bip21": null
                },
                "lockup_details": {
                    "swapTree": {
                        "claimLeaf": {
                            "output": "82012088a9146a01e0a34b4e581da5133b5113b54b9033bb93dc88202f40110df011392abfbf3efefe179dea85b9f4b499a2a808b68d03b61f1d9a62ac",
                            "version": 196
                            },
                            "refundLeaf": {
                                "output": "2051d819f25d113c42c047545facf55aa2b15af9056af020a3d7cd61ebc2adbecfad03339b16b1",
                                "version": 196
                            }
                        },
                        "lockupAddress": "tlq1pqg4e5r5a59gdl26ud6s7gna3mchqs20ycwl2lp67ejzy69fl7dwccwx9nqtr6ef848k7vpmvmdhsyeq2wp3vtn3gnlenhd0wrasv4qvr2dk0nz5tu0rw",
                        "serverPublicKey": "022f40110df011392abfbf3efefe179dea85b9f4b499a2a808b68d03b61f1d9a62",
                        "timeoutBlockHeight": 1481523,
                        "amount": 0,
                        "blindingKey": "f69c69bec80dc0161f6c03367a269ce9780f04a8702916d17a13009552251f44",
                        "refundAddress": null,
                        "claimAddress": null,
                        "bip21": "liquidtestnet:tlq1pqg4e5r5a59gdl26ud6s7gna3mchqs20ycwl2lp67ejzy69fl7dwccwx9nqtr6ef848k7vpmvmdhsyeq2wp3vtn3gnlenhd0wrasv4qvr2dk0nz5tu0rw?amount=0.00025247&label=Send%20to%20BTC%20address&assetid=144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49"
                    }
                }"#.to_string(),
            claim_private_key: "7d3cbecfb76cb8eccc2c2131f3e744311d3655377fe8723d23acb55b041b2b16".to_string(),
            refund_private_key: "2644c60cc6cd454ea809f0e32fc2871ab7c26603e3009e1fd313ae886c137eaa".to_string(),
            payer_amount_sat: 25490,
            receiver_amount_sat: 20000,
            claim_fees_sat: 2109,
            server_lockup_tx_id: None,
            user_lockup_tx_id,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: payment_state.unwrap_or(PaymentState::Created),
            accept_zero_conf,
        }
    }
}
