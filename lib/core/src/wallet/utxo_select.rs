use std::collections::BTreeMap;

use anyhow::{anyhow, ensure, Result};
use lwk_wollet::{elements::AssetId, WalletTxOut};

use crate::wallet::network_fee::TxFee;

// TOTAL_TRIES in Core:
// https://github.com/bitcoin/bitcoin/blob/1d9da8da309d1dbf9aef15eb8dc43b4a2dc3d309/src/wallet/coinselection.cpp#L74
pub(crate) const UTXO_SELECTION_ITERATION_LIMIT: u32 = 100_000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct InOut {
    pub asset_id: AssetId,
    pub value: u64,
}

impl From<&WalletTxOut> for InOut {
    fn from(tx_out: &WalletTxOut) -> Self {
        Self {
            asset_id: tx_out.unblinded.asset,
            value: tx_out.unblinded.value,
        }
    }
}

pub(crate) struct WalletUtxoSelectRequest {
    pub policy_asset: AssetId,
    pub selection_asset: AssetId,
    pub wallet_utxos: Vec<InOut>,
    pub recipient_outputs: Vec<InOut>,
    pub fee_rate: Option<f64>,
}

pub(crate) fn utxo_select(
    WalletUtxoSelectRequest {
        policy_asset,
        selection_asset,
        wallet_utxos,
        recipient_outputs,
        fee_rate,
    }: WalletUtxoSelectRequest,
) -> Result<Vec<InOut>> {
    let mut recipient_output_amounts = BTreeMap::<AssetId, u64>::new();

    for input in wallet_utxos.iter() {
        ensure!(input.value > 0, anyhow!("Invalid amount {:?}", input));
    }

    for user_output in recipient_outputs.iter() {
        ensure!(
            user_output.value > 0,
            anyhow!("Invalid amount {:?}", user_output),
        );
        *recipient_output_amounts
            .entry(user_output.asset_id)
            .or_default() += user_output.value;
    }

    let mut selected_utxos = Vec::<InOut>::new();
    let mut change_outputs = Vec::<InOut>::new();

    for (&asset_id, &target_value) in recipient_output_amounts.iter() {
        if asset_id == selection_asset {
            let asset_utxos = wallet_utxos
                .iter()
                .filter(|utxo| utxo.asset_id == asset_id)
                .map(|utxo| utxo.value)
                .collect::<Vec<_>>();
            let available = asset_utxos.iter().sum::<u64>();

            ensure!(
                available >= target_value,
                anyhow!(
                    "Not enough UTXOs for asset {}, required: {}, available: {}",
                    asset_id,
                    target_value,
                    available
                )
            );

            let selected = if asset_id != policy_asset {
                utxo_select_best(target_value, &asset_utxos).ok_or(anyhow!("No utxos selected"))?
            } else {
                utxo_select_dynamic(target_value, &asset_utxos, |utxo_count, change_count| {
                    TxFee {
                        native_inputs: utxo_count,
                        nested_inputs: 0,
                        outputs: recipient_outputs.len() + change_count,
                    }
                    .fee(fee_rate)
                })
                .ok_or(anyhow!("No utxos selected"))?
            };

            let mut total_value = 0;
            for value in selected {
                selected_utxos.push(InOut { asset_id, value });
                total_value += value;
            }

            ensure!(
                total_value >= target_value,
                "Total value is less than target: {} < {}",
                total_value,
                target_value
            );
            let change_amount = total_value - target_value;
            if change_amount > 0 {
                change_outputs.push(InOut {
                    asset_id,
                    value: change_amount,
                });
            }
        }
    }

    Ok(selected_utxos)
}

pub(crate) fn utxo_select_fixed(
    target_value: u64,
    target_utxo_count: usize,
    utxos: &[u64],
) -> Option<Vec<u64>> {
    let selected_utxos = utxos
        .iter()
        .copied()
        .take(target_utxo_count)
        .collect::<Vec<_>>();
    let selected_value = selected_utxos.iter().sum::<u64>();
    if selected_value < target_value {
        None
    } else {
        Some(selected_utxos)
    }
}

pub(crate) fn utxo_select_basic(target_value: u64, utxos: &[u64]) -> Option<Vec<u64>> {
    let mut selected_utxos = Vec::new();
    let mut selected_value = 0;

    if target_value > 0 {
        for utxo in utxos {
            selected_utxos.push(*utxo);
            selected_value += utxo;
            if selected_value >= target_value {
                break;
            }
        }
    }

    if selected_value < target_value {
        None
    } else {
        Some(selected_utxos)
    }
}

pub(crate) fn utxo_select_best(target_value: u64, utxos: &[u64]) -> Option<Vec<u64>> {
    utxo_select_in_range(target_value, 0, 0, utxos)
        .or_else(|| utxo_select_basic(target_value, utxos))
}

pub(crate) fn utxo_select_dynamic<F>(
    target_value: u64,
    utxos: &[u64],
    get_fee: F,
) -> Option<Vec<u64>>
where
    F: Fn(/* utxo_count */ usize, /* change_count */ usize) -> u64,
{
    let mut best_fee = u64::MAX;
    let mut best_selection = None;

    // Find the best utxo selection with no change and the least utxo count
    for utxo_count in 1..=utxos.len() {
        let fee = get_fee(utxo_count, 0);
        if fee < best_fee {
            if let Some(selected_utxos) =
                utxo_select_in_range(target_value + fee, 0, utxo_count, utxos)
            {
                best_fee = fee;
                best_selection = Some(selected_utxos);
            }
        }
    }
    // Find the best utxo selection with the least utxo count and change output
    let available_value = utxos.iter().sum::<u64>();
    for utxo_count in 1..=utxos.len() {
        let fee = get_fee(utxo_count, 1);
        if fee < best_fee {
            let target_incl_fee = target_value + fee;
            let upper_bound_delta = available_value.saturating_sub(target_incl_fee);
            if let Some(selected_utxos) =
                utxo_select_in_range(target_incl_fee, upper_bound_delta, utxo_count, utxos)
            {
                best_fee = fee;
                best_selection = Some(selected_utxos);
            }
        }
    }
    best_selection
}

/// Try to select utxos so that their sum is in the range [target_value..target_value + upper_bound_delta].
/// Set `upper_bound_delta` to 0 if you want to find utxos without change.
/// All the values must be "sane" so their sum does not overflow.
pub(crate) fn utxo_select_in_range(
    target_value: u64,
    upper_bound_delta: u64,
    target_utxo_count: usize,
    utxos: &[u64],
) -> Option<Vec<u64>> {
    let mut utxos = utxos.to_vec();
    utxos.sort();
    utxos.reverse();

    let mut iteration = 0;
    let mut index = 0;
    let mut value = 0;

    let mut current_change = 0;
    let mut best_change = u64::MAX;

    let mut index_selection: Vec<usize> = vec![];
    let mut best_selection: Option<Vec<usize>> = None;

    let upper_bound = target_value + upper_bound_delta;
    let mut available_value = utxos.iter().sum::<u64>();

    if available_value < target_value {
        return None;
    }

    while iteration < UTXO_SELECTION_ITERATION_LIMIT {
        let mut step_back = false;

        if available_value + value < target_value
            // If any of the conditions are met, step back.
            //
            // Provides an upper bound on the change value that is allowed.
            // Since value is lost when we create a change output due to increasing the size of the
            // transaction by an output (the change output), we accept solutions that may be
            // larger than the target.  The change is added to the solutions change score.
            // However, values greater than value + upper_bound_delta are not considered.
            //
            // This creates a range of possible solutions where;
            // range = (target, target + upper_bound_delta]
            //
            // That is, the range includes solutions that exactly equal the target up to but not
            // including values greater than target + upper_bound_delta.
            || value > upper_bound
            || current_change > best_change
        {
            step_back = true;
        } else if value >= target_value {
            // Value meets or exceeds the target.
            // Record the solution and the change then continue.
            step_back = true;

            let change = value - target_value;
            current_change += change;

            // Check if index_selection is better than the previous known best, and
            // update best_selection accordingly.
            if current_change <= best_change
                && ((target_utxo_count == 0
                    && best_selection.clone().is_none_or(|best_selection| {
                        index_selection.len() <= best_selection.len()
                    }))
                    || index_selection.len() == target_utxo_count)
            {
                best_selection = Some(index_selection.clone());
                best_change = current_change;
            }

            current_change = current_change.saturating_sub(change);
        }

        if target_utxo_count != 0 && index_selection.len() >= target_utxo_count {
            step_back = true;
        }

        if step_back {
            // Step back
            if index_selection.is_empty() {
                break;
            }

            loop {
                index -= 1;

                if index_selection.last().is_none_or(|last| index <= *last) {
                    break;
                }

                let utxo_value = utxos[index];
                available_value += utxo_value;
            }

            if index_selection.last().is_some_and(|last| index == *last) {
                let utxo_value = utxos[index];
                value = value.saturating_sub(utxo_value);
                index_selection.pop();
            }
        } else {
            // Add the utxo to the current selection
            let utxo_value = utxos[index];

            index_selection.push(index);

            value += utxo_value;
            available_value = available_value.saturating_sub(utxo_value);
        }

        index += 1;
        iteration += 1;
    }

    best_selection.map(|best_selection| best_selection.iter().map(|index| utxos[*index]).collect())
}

#[cfg(test)]
mod tests {
    use crate::wallet::utxo_select::{
        utxo_select_basic, utxo_select_best, utxo_select_dynamic, utxo_select_fixed,
        utxo_select_in_range,
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
    fn test_utxo_select_dynamic() {
        // Test with a simple fee function that charges 10 per input
        let fee_fn = |utxo_count, _change_count| 10 * utxo_count as u64;

        // Basic case - should select UTXOs considering fee
        let utxos = vec![100, 200, 300, 400];
        let selected = utxo_select_dynamic(300, &utxos, fee_fn);
        // Should select 300 + 10 (fee for 1 input) = 310 target
        assert_eq!(selected, Some(vec![400]));

        // Exact match with fee
        let utxos = vec![100, 200, 300];
        let selected = utxo_select_dynamic(290, &utxos, fee_fn);
        // Should select 290 + 10 (fee for 1 input) = 300 target
        assert_eq!(selected, Some(vec![300]));

        // Exact match with multiple UTXOs
        let utxos = vec![100, 200, 300];
        let selected = utxo_select_dynamic(380, &utxos, fee_fn);
        // Should select 380 + 10 + 10 (fee for 2 inputs) = 400 target
        assert_eq!(selected, Some(vec![300, 100]));

        // Test with a more complex fee function
        let complex_fee_fn = |utxo_count, _change_count| 5 + (utxo_count as u64 * 15);

        // Should minimize UTXOs to reduce fee
        let utxos = vec![50, 60, 70, 100, 200, 300];
        let selected = utxo_select_dynamic(250, &utxos, complex_fee_fn);
        assert!(selected.is_some());
        let selection = selected.unwrap();
        // Fee for this many inputs
        let fee = 5 + (selection.len() as u64 * 15);
        // Check that selected UTXOs cover target + fee
        assert!(selection.iter().sum::<u64>() >= 250 + fee);

        // Test with insufficient UTXOs
        let utxos = vec![50, 60, 70];
        let selected = utxo_select_dynamic(500, &utxos, fee_fn);
        assert_eq!(selected, None);

        // Test with empty UTXOs
        let utxos: Vec<u64> = vec![];
        let selected = utxo_select_dynamic(100, &utxos, fee_fn);
        assert_eq!(selected, None);

        // Test with exact match including fee
        let utxos = vec![100, 150, 210];
        // Target 200 + fee for 1 input (10) = 210
        let selected = utxo_select_dynamic(200, &utxos, fee_fn);
        assert_eq!(selected, Some(vec![210]));

        // Test with high fee function that makes selection impossible
        let high_fee_fn = |utxo_count, _change_count| 1000 * utxo_count as u64;
        let utxos = vec![100, 200, 300];
        let selected = utxo_select_dynamic(250, &utxos, high_fee_fn);
        // Can't satisfy because fee would be too high
        assert_eq!(selected, None);

        // Test with change count variation

        // Test with fee function that varies based on change count
        let change_sensitive_fee_fn =
            |utxo_count, change_count| 10 * utxo_count as u64 + 20 * change_count as u64;

        // Test case where no-change is better than with-change due to fee
        let utxos = vec![250, 300, 400];
        let selected = utxo_select_dynamic(200, &utxos, change_sensitive_fee_fn);
        // Should prefer 250 with no change (fee: 10) over e.g., 300 with change (fee: 10+20=30)
        assert_eq!(selected, Some(vec![250]));

        // Test case where with-change is better than no-change due to UTXO efficiency
        let utxos = vec![100, 150, 400];
        // Target 250 + basic fee
        let selected = utxo_select_dynamic(250, &utxos, change_sensitive_fee_fn);
        // Should select 400 with change - creates change but uses fewer inputs
        assert_eq!(selected, Some(vec![400]));

        // Test with near-exact match where change would be minimal
        let utxos = vec![100, 150, 203, 300];
        let fee_fn = |utxo_count, change_count| 5 * utxo_count as u64 + 10 * change_count as u64;
        // Target 195 + fee
        let selected = utxo_select_dynamic(195, &utxos, fee_fn);
        // Should prefer 300 (fee: 5) over 100+150 (fee: 10) even though there's change
        assert_eq!(selected, Some(vec![300]));

        // Test with large values to ensure no overflows
        let large_value = u64::MAX / 4;
        let utxos = vec![large_value, large_value * 2, large_value / 2];
        let selected = utxo_select_dynamic(large_value, &utxos, fee_fn);
        assert!(selected.is_some());
        let sum: u64 = selected.as_ref().unwrap().iter().sum();
        assert!(
            sum >= large_value
                + fee_fn(
                    selected.unwrap().len(),
                    if sum > large_value { 1 } else { 0 }
                )
        );

        // Test with many small UTXOs vs fewer large UTXOs
        let many_small = vec![10; 30]; // 30 UTXOs of value 10
        let few_large = vec![100, 200];
        let all_utxos = [many_small.clone(), few_large.clone()].concat();

        // Target that could be satisfied by many small or few large
        let target = 250;

        // Fee function with high per-input cost
        let high_input_fee =
            |utxo_count, change_count| 8 * utxo_count as u64 + 2 * change_count as u64;
        let selected = utxo_select_dynamic(target, &all_utxos, high_input_fee);

        // Should prefer the few large UTXOs due to high per-input fee
        assert!(selected.is_some());
        let selection = selected.unwrap();
        assert!(selection.len() <= 3); // Should use at most 3 UTXOs

        // Test with UTXOs requiring exact selection
        let utxos = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        // Target requiring specific combination (e.g., 10+40+50 = 100)
        let target = 100;
        let fee_fn = |utxo_count, _change_count| utxo_count as u64 * 5;
        let selected = utxo_select_dynamic(target, &utxos, fee_fn);

        assert!(selected.is_some());
        let selection = selected.unwrap();
        let fee = fee_fn(selection.len(), 0);
        assert_eq!(selection.iter().sum::<u64>(), target + fee);
    }
}
