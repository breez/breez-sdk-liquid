#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use lwk_wollet::elements::AssetId;

    use crate::payjoin::{
        model::InOut,
        utxo_select::{
            utxo_select, utxo_select_basic, utxo_select_best, utxo_select_fixed,
            utxo_select_in_range, UtxoSelectRequest,
        },
    };

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_utxo_select_basic() {
        // Basic case - should select UTXOs in order until target is met
        let utxos = vec![100, 200, 300, 400];
        let selected = utxo_select_basic(300, &utxos);
        assert_eq!(selected, Some(vec![100, 200]));

        // Exact match with one UTXO
        let selected = utxo_select_basic(300, &[300, 400, 500]);
        assert_eq!(selected, Some(vec![300]));

        // First UTXO is enough
        let selected = utxo_select_basic(50, &[100, 200, 300]);
        assert_eq!(selected, Some(vec![100]));

        // Need all UTXOs
        let selected = utxo_select_basic(590, &[100, 200, 300]);
        assert_eq!(selected, Some(vec![100, 200, 300]));

        // Not enough UTXOs available
        let selected = utxo_select_basic(1000, &[100, 200, 300]);
        assert_eq!(selected, None);

        // Empty UTXO list
        let selected = utxo_select_basic(100, &[]);
        assert_eq!(selected, None);

        // Zero target amount
        let selected = utxo_select_basic(0, &[100, 200]);
        assert_eq!(selected, Some(vec![]));

        // Large values to check for overflow
        let large_value = u64::MAX / 3;
        let utxos = vec![large_value, large_value, large_value];
        let selected = utxo_select_basic(large_value * 2, &utxos);
        assert_eq!(selected, Some(vec![large_value, large_value]));

        // UTXO order matters - should take in original order
        let utxos = vec![400, 100, 300, 200];
        let selected = utxo_select_basic(450, &utxos);
        assert_eq!(selected, Some(vec![400, 100]));

        // With just-enough UTXOs
        let utxos = vec![100, 200, 300, 400];
        let selected = utxo_select_basic(1000, &utxos);
        assert_eq!(selected, Some(vec![100, 200, 300, 400]));
    }

    #[sdk_macros::test_all]
    fn test_utxo_select_fixed() {
        let utxos = vec![100, 200, 300, 400];

        // Should take first two UTXOs (100 + 200 = 300)
        let selected = utxo_select_fixed(300, 2, &utxos);
        assert_eq!(selected, Some(vec![100, 200]));

        // Not enough with just one UTXO
        let selected = utxo_select_fixed(150, 1, &utxos);
        assert_eq!(selected, None);

        // Target exceeds available in requested count
        let selected = utxo_select_fixed(350, 2, &utxos);
        assert_eq!(selected, None);

        // With exactly the required amount
        let selected = utxo_select_fixed(300, 1, &[300]);
        assert_eq!(selected, Some(vec![300]));

        // With empty utxos
        let selected = utxo_select_fixed(100, 1, &[]);
        assert_eq!(selected, None);

        // With zero target value
        let selected = utxo_select_fixed(0, 2, &utxos);
        assert_eq!(selected, Some(vec![100, 200]));

        // With zero target count
        let selected = utxo_select_fixed(100, 0, &utxos);
        assert_eq!(selected, None);

        // With more UTXOs than requested count but still not enough value
        let selected = utxo_select_fixed(1000, 3, &utxos);
        assert_eq!(selected, None);

        // With exactly enough UTXOs to meet the target
        let selected = utxo_select_fixed(600, 3, &utxos);
        assert_eq!(selected, Some(vec![100, 200, 300]));

        // With large values to test for potential overflow issues
        let large_value = u64::MAX / 2;
        let utxos = vec![large_value, large_value / 2];
        let selected = utxo_select_fixed(large_value, 1, &utxos);
        assert_eq!(selected, Some(vec![large_value]));
    }

    #[sdk_macros::test_all]
    fn test_utxo_select_best() {
        let utxos = vec![100, 200, 300, 400];

        // Should find optimal solution
        let selected = utxo_select_best(300, &utxos);
        assert_eq!(selected, Some(vec![300]));

        // Should fallback to basic selection as no exact utxo set can be found
        let selected: Option<Vec<u64>> = utxo_select_best(450, &utxos);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().iter().sum::<u64>(), 600);

        // Should use all UTXOs as fallback when needed
        let selected = utxo_select_best(950, &utxos);
        assert_eq!(selected, Some(vec![100, 200, 300, 400]));
    }

    #[sdk_macros::test_all]
    fn test_utxo_select_in_range() {
        let utxos = vec![50, 100, 200, 300, 400];

        // Exact match
        let selected = utxo_select_in_range(300, 0, 0, &utxos);
        assert_eq!(selected, Some(vec![300]));

        // Within range
        let selected = utxo_select_in_range(350, 50, 0, &utxos);
        assert_eq!(selected, Some(vec![400]));

        // Multiple UTXOs needed
        let selected = utxo_select_in_range(350, 0, 0, &utxos);
        assert_eq!(selected, Some(vec![300, 50]));

        // With target count
        let selected = utxo_select_in_range(250, 0, 2, &utxos);
        assert_eq!(selected, Some(vec![200, 50]));
    }

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
}
