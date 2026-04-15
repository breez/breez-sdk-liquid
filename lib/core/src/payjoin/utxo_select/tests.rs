#![cfg(test)]
use std::collections::BTreeMap;

use lwk_wollet::elements::AssetId;

use crate::{
    payjoin::utxo_select::{utxo_select, UtxoSelectRequest},
    wallet::utxo_select::InOut,
};

#[cfg(feature = "browser-tests")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::test_all]
fn test_utxo_select_success() {
    let policy_asset = AssetId::from_slice(&[1; 32]).unwrap();
    let fee_asset = AssetId::from_slice(&[2; 32]).unwrap();

    // Create wallet UTXOs with both policy and fee assets
    let wallet_utxos = vec![
        InOut {
            asset_id: policy_asset,
            value: 100000000,
        },
        InOut {
            asset_id: policy_asset,
            value: 200000000,
        },
        InOut {
            asset_id: fee_asset,
            value: 50000000,
        },
        InOut {
            asset_id: fee_asset,
            value: 80000000,
        },
    ];

    // Create server UTXOs (only policy asset)
    let server_utxos = vec![
        InOut {
            asset_id: policy_asset,
            value: 150000000,
        },
        InOut {
            asset_id: policy_asset,
            value: 250000000,
        },
    ];

    // User outputs (both assets)
    let user_outputs = vec![
        InOut {
            asset_id: policy_asset,
            value: 150000000,
        },
        InOut {
            asset_id: fee_asset,
            value: 20000000,
        },
    ];

    let req = UtxoSelectRequest {
        policy_asset,
        fee_asset,
        price: 84896.5,
        fixed_fee: 4000000,
        wallet_utxos,
        server_utxos,
        user_outputs,
    };

    let result = utxo_select(req);
    assert!(result.is_ok());

    let selection = result.unwrap();

    // Verify network fee is covered by server inputs
    assert!(selection.network_fee.value > 0);
    assert_eq!(selection.network_fee.asset_id, policy_asset);

    // Verify server fee is in fee_asset and reasonable
    assert!(selection.server_fee.value >= 100); // at least fixed fee
    assert_eq!(selection.server_fee.asset_id, fee_asset);

    // Verify all user outputs are present
    assert_eq!(selection.user_outputs.len(), 2);

    // Check input/output balance
    let mut input_sum_by_asset = BTreeMap::<AssetId, u64>::new();
    let mut output_sum_by_asset = BTreeMap::<AssetId, u64>::new();

    // Sum all inputs
    for input in selection
        .user_inputs
        .iter()
        .chain(selection.client_inputs.iter())
        .chain(selection.server_inputs.iter())
    {
        *input_sum_by_asset.entry(input.asset_id).or_default() += input.value;
    }

    // Sum all outputs
    for output in selection
        .user_outputs
        .iter()
        .chain(selection.change_outputs.iter())
        .chain(std::iter::once(&selection.server_fee))
        .chain(selection.server_change.iter())
        .chain(selection.fee_change.iter())
        .chain(std::iter::once(&selection.network_fee))
    {
        *output_sum_by_asset.entry(output.asset_id).or_default() += output.value;
    }

    // Input and output sums should match for each asset
    assert_eq!(input_sum_by_asset, output_sum_by_asset);
}

#[sdk_macros::test_all]
fn test_utxo_select_error_cases() {
    let policy_asset = AssetId::from_slice(&[1; 32]).unwrap();
    let fee_asset = AssetId::from_slice(&[2; 32]).unwrap();

    // Base valid request
    let valid_req = UtxoSelectRequest {
        policy_asset,
        fee_asset,
        price: 84896.5,
        fixed_fee: 4000000,
        wallet_utxos: vec![
            InOut {
                asset_id: policy_asset,
                value: 1000,
            },
            InOut {
                asset_id: fee_asset,
                value: 500,
            },
        ],
        server_utxos: vec![InOut {
            asset_id: policy_asset,
            value: 1500,
        }],
        user_outputs: vec![InOut {
            asset_id: policy_asset,
            value: 500,
        }],
    };

    // Same asset for policy and fee - should error
    let mut bad_req = valid_req.clone();
    bad_req.fee_asset = policy_asset;
    assert!(utxo_select(bad_req).is_err());

    // Zero price - should error
    let mut bad_req = valid_req.clone();
    bad_req.price = 0.0;
    assert!(utxo_select(bad_req).is_err());

    // Zero fixed fee - should error
    let mut bad_req = valid_req.clone();
    bad_req.fixed_fee = 0;
    assert!(utxo_select(bad_req).is_err());

    // Invalid server UTXO asset - should error
    let mut bad_req = valid_req.clone();
    bad_req.server_utxos = vec![InOut {
        asset_id: fee_asset,
        value: 1500,
    }];
    assert!(utxo_select(bad_req).is_err());

    // Insufficient fee assets - should error
    let mut bad_req = valid_req.clone();
    bad_req.wallet_utxos = vec![
        InOut {
            asset_id: policy_asset,
            value: 1000,
        },
        InOut {
            asset_id: fee_asset,
            value: 10,
        }, // Too small
    ];
    let result = utxo_select(bad_req);
    assert!(result.is_err());
}

#[sdk_macros::test_all]
fn test_utxo_select_with_change() {
    let policy_asset = AssetId::from_slice(&[1; 32]).unwrap();
    let fee_asset = AssetId::from_slice(&[2; 32]).unwrap();

    // Create a scenario where change is needed
    let wallet_utxos = vec![
        InOut {
            asset_id: policy_asset,
            value: 50000000,
        },
        InOut {
            asset_id: fee_asset,
            value: 20000000,
        },
    ];

    let server_utxos = vec![
        InOut {
            asset_id: policy_asset,
            value: 10000000,
        },
        InOut {
            asset_id: policy_asset,
            value: 20000000,
        },
    ];

    let user_outputs = vec![InOut {
        asset_id: policy_asset,
        value: 30000000,
    }];

    let req = UtxoSelectRequest {
        policy_asset,
        fee_asset,
        price: 84896.5,
        fixed_fee: 4000000,
        wallet_utxos,
        server_utxos,
        user_outputs,
    };

    let result = utxo_select(req);
    assert!(result.is_ok());

    let selection = result.unwrap();

    // Either policy asset change or fee asset change should exist
    assert!(selection.fee_change.is_some() || !selection.change_outputs.is_empty());

    // Verify change amounts are reasonable
    if let Some(fee_change) = &selection.fee_change {
        assert_eq!(fee_change.asset_id, fee_asset);
        assert!(fee_change.value > 0);
    }

    // Check that we're not wasting fees unnecessarily
    assert!(selection.cost <= selection.server_fee.value);
}
