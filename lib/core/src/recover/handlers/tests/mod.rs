#![cfg(test)]
// Module declaration for test files
pub mod handle_chain_receive_swap_tests;
pub mod handle_chain_receive_swap_tests_integration;
pub mod handle_chain_send_swap_tests;
pub mod handle_chain_send_swap_tests_integration;
pub mod handle_receive_swap_tests;
pub mod handle_receive_swap_tests_integration;
pub mod handle_send_swap_tests;
pub mod handle_send_swap_tests_integration;

// Helper function to create a History txid for testing
use std::{collections::BTreeMap, str::FromStr};

use crate::model::{BtcHistory, LBtcHistory};
use crate::{bitcoin, elements};
use elements::{AssetId, Transaction, TxIn, TxInWitness, Txid};
use lwk_wollet::{hashes::Hash, WalletTx};

pub(crate) fn create_lbtc_history_txid(hex_id: &str, height: i32) -> LBtcHistory {
    let txid_bytes = hex::decode(format!("{hex_id:0>64}")).unwrap();
    let mut txid_array = [0u8; 32];
    txid_array.copy_from_slice(&txid_bytes);

    LBtcHistory {
        txid: elements::Txid::from_slice(&txid_array).unwrap(),
        height,
    }
}

pub(crate) fn create_btc_history_txid(hex_id: &str, height: i32) -> BtcHistory {
    let txid_bytes = hex::decode(format!("{hex_id:0>64}")).unwrap();
    let mut txid_array = [0u8; 32];
    txid_array.copy_from_slice(&txid_bytes);

    BtcHistory {
        txid: bitcoin::Txid::from_slice(&txid_array).unwrap(),
        height,
    }
}

// Create an empty LBTC transaction
pub(crate) fn create_empty_lbtc_transaction() -> Transaction {
    Transaction {
        version: 2,
        lock_time: elements::LockTime::from_height(0).unwrap(),
        input: vec![TxIn {
            previous_output: Default::default(),
            is_pegin: false,
            script_sig: elements::Script::new(),
            sequence: elements::Sequence::default(),
            asset_issuance: Default::default(),
            witness: TxInWitness::empty(),
        }],
        output: vec![],
    }
}

// Create a mock LBTC wallet transaction
pub(crate) fn create_mock_lbtc_wallet_tx(
    tx_id_hex: &str,
    height: u32,
    amount: i64,
    asset_id: AssetId,
) -> WalletTx {
    let tx_id = Txid::from_str(tx_id_hex).unwrap();

    WalletTx {
        txid: tx_id,
        tx: create_empty_lbtc_transaction(),
        height: Some(height),
        fee: 1000,
        timestamp: Some(1001), // Just after swap creation time
        balance: {
            let mut map = BTreeMap::new();
            map.insert(asset_id, amount);
            map
        },
        outputs: vec![],
        inputs: Vec::new(),
        type_: "".to_string(),
    }
}
