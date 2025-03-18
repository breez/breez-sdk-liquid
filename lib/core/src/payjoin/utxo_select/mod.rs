mod asset;
mod tests;

use std::collections::BTreeMap;

use anyhow::{anyhow, ensure, Result};
use asset::{asset_select, AssetSelectRequest, AssetSelectResult};
use lwk_wollet::elements::AssetId;

use crate::payjoin::{
    model::InOut,
    network_fee::{self, TxFee},
};

// TOTAL_TRIES in Core:
// https://github.com/bitcoin/bitcoin/blob/1d9da8da309d1dbf9aef15eb8dc43b4a2dc3d309/src/wallet/coinselection.cpp#L74
const UTXO_SELECTION_ITERATION_LIMIT: u32 = 100_000;

#[derive(Debug, Clone)]
pub(crate) struct UtxoSelectRequest {
    pub policy_asset: AssetId,
    pub fee_asset: AssetId,
    pub price: f64,
    pub fixed_fee: u64,
    pub wallet_utxos: Vec<InOut>,
    pub server_utxos: Vec<InOut>,
    pub user_outputs: Vec<InOut>,
}

#[derive(Debug)]
pub(crate) struct UtxoSelectResult {
    pub user_inputs: Vec<InOut>,
    pub client_inputs: Vec<InOut>,
    pub server_inputs: Vec<InOut>,

    pub user_outputs: Vec<InOut>,
    pub change_outputs: Vec<InOut>,
    pub server_fee: InOut,
    pub server_change: Option<InOut>,
    pub fee_change: Option<InOut>,
    pub network_fee: InOut,

    pub cost: u64,
}

pub(crate) fn utxo_select(req: UtxoSelectRequest) -> Result<UtxoSelectResult> {
    let utxo_select_res = utxo_select_inner(req.clone());

    if let Ok(res) = &utxo_select_res {
        validate_selection(&req, res)?;
    }
    utxo_select_res
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

/// Try to select utxos so that their sum is in the range [target_value..target_value + upper_bound_delta].
/// Set `upper_bound_delta` to 0 if you want to find utxos without change.
/// All the values must be "sane" so their sum does not overflow.
fn utxo_select_in_range(
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

fn utxo_select_inner(
    UtxoSelectRequest {
        policy_asset,
        fee_asset,
        price,
        fixed_fee,
        wallet_utxos,
        server_utxos,
        user_outputs,
    }: UtxoSelectRequest,
) -> Result<UtxoSelectResult> {
    ensure!(fee_asset != policy_asset);
    ensure!(price > 0.0);
    ensure!(fixed_fee > 0);
    ensure!(wallet_utxos.iter().all(|utxo| utxo.value > 0));

    ensure!(server_utxos
        .iter()
        .all(|utxo| utxo.asset_id == policy_asset && utxo.value > 0));

    let fee_utoxs = wallet_utxos
        .iter()
        .filter(|utxo| utxo.asset_id == fee_asset)
        .map(|utxo| utxo.value)
        .collect::<Vec<_>>();
    let mut server_utxos = server_utxos
        .iter()
        .map(|utxo| utxo.value)
        .collect::<Vec<_>>();
    server_utxos.sort();
    server_utxos.reverse();

    let AssetSelectResult {
        asset_inputs,
        user_outputs,
        change_outputs,
        user_output_amounts,
    } = asset_select(AssetSelectRequest {
        fee_asset,
        wallet_utxos,
        user_outputs,
    })?;

    let mut best_selection: Option<UtxoSelectResult> = None;

    for with_fee_change in [false, true] {
        for with_server_change in [false, true] {
            for server_input_count in 1..=server_utxos.len() {
                for fee_input_count in 1..=fee_utoxs.len() {
                    if fee_input_count != fee_utoxs.len() {
                        continue;
                    }
                    let user_input_count = asset_inputs.len() + fee_input_count;

                    let output_count = user_outputs.len()
                        + change_outputs.len()
                        + usize::from(with_fee_change)
                        + usize::from(with_server_change)
                        + 1; // Server fee output

                    let min_network_fee = TxFee {
                        server_inputs: server_input_count,
                        user_inputs: user_input_count,
                        outputs: output_count,
                    }
                    .fee();

                    let server_inputs = if with_server_change {
                        utxo_select_fixed(min_network_fee + 1, server_input_count, &server_utxos)
                    } else {
                        let upper_bound_delta = network_fee::weight_to_fee(
                            network_fee::WEIGHT_VOUT_NESTED,
                            network_fee::MIN_FEE_RATE,
                        );
                        utxo_select_in_range(
                            min_network_fee,
                            upper_bound_delta,
                            server_input_count,
                            &server_utxos,
                        )
                    };

                    let server_inputs = match server_inputs {
                        Some(server_inputs) => server_inputs,
                        None => continue,
                    };

                    let server_input = server_inputs.iter().sum::<u64>();
                    let server_change = if with_server_change {
                        server_input - min_network_fee
                    } else {
                        0
                    };
                    let network_fee = server_input - server_change;
                    let min_asset_fee = (network_fee as f64 * price) as u64 + fixed_fee;

                    let user_asset_output = user_output_amounts
                        .get(&fee_asset)
                        .copied()
                        .unwrap_or_default();

                    let fee_asset_target = user_asset_output + min_asset_fee;
                    let fee_asset_inputs = if with_fee_change {
                        utxo_select_fixed(fee_asset_target + 1, fee_input_count, &fee_utoxs)
                    } else {
                        let upper_bound_delta = (network_fee::weight_to_fee(
                            network_fee::WEIGHT_VOUT_NESTED,
                            network_fee::MIN_FEE_RATE,
                        ) as f64
                            * price) as u64;
                        utxo_select_in_range(
                            fee_asset_target,
                            upper_bound_delta,
                            fee_input_count,
                            &fee_utoxs,
                        )
                    };

                    let fee_asset_inputs = match fee_asset_inputs {
                        Some(fee_asset_inputs) => fee_asset_inputs,
                        None => continue,
                    };

                    let fee_input = fee_asset_inputs.iter().sum::<u64>();
                    let fee_change = if with_fee_change {
                        fee_input - fee_asset_target
                    } else {
                        0
                    };
                    let server_fee = fee_input - fee_change - user_asset_output;
                    let new_cost = server_fee;

                    if best_selection
                        .as_ref()
                        .map(|best| best.cost > new_cost)
                        .unwrap_or(true)
                    {
                        let user_outputs = user_outputs.clone();

                        best_selection = Some(UtxoSelectResult {
                            user_inputs: asset_inputs.clone(),
                            client_inputs: fee_asset_inputs
                                .iter()
                                .map(|value| InOut {
                                    asset_id: fee_asset,
                                    value: *value,
                                })
                                .collect(),
                            server_inputs: server_inputs
                                .iter()
                                .map(|value| InOut {
                                    asset_id: policy_asset,
                                    value: *value,
                                })
                                .collect(),
                            user_outputs,
                            change_outputs: change_outputs.clone(),
                            server_fee: InOut {
                                asset_id: fee_asset,
                                value: server_fee,
                            },
                            server_change: with_server_change.then_some(InOut {
                                asset_id: policy_asset,
                                value: server_change,
                            }),
                            fee_change: with_fee_change.then_some(InOut {
                                asset_id: fee_asset,
                                value: fee_change,
                            }),
                            network_fee: InOut {
                                asset_id: policy_asset,
                                value: network_fee,
                            },
                            cost: new_cost,
                        })
                    };
                }
            }
        }
    }

    best_selection.ok_or_else(|| anyhow!("Utxo selection failed"))
}

fn validate_selection(
    req: &UtxoSelectRequest,
    UtxoSelectResult {
        user_inputs,
        client_inputs,
        server_inputs,
        user_outputs,
        change_outputs,
        server_fee,
        server_change,
        fee_change,
        network_fee,
        cost: _,
    }: &UtxoSelectResult,
) -> Result<()> {
    let mut inputs = BTreeMap::<AssetId, u64>::new();
    let mut outputs = BTreeMap::<AssetId, u64>::new();

    for input in user_inputs
        .iter()
        .chain(client_inputs.iter())
        .chain(server_inputs.iter())
    {
        *inputs.entry(input.asset_id).or_default() += input.value;
    }

    for output in user_outputs
        .iter()
        .chain(change_outputs.iter())
        .chain(std::iter::once(server_fee))
        .chain(server_change.iter())
        .chain(fee_change.iter())
        .chain(std::iter::once(network_fee))
    {
        *outputs.entry(output.asset_id).or_default() += output.value;
    }

    ensure!(inputs == outputs, "Check failed: {inputs:?} != {outputs:?}");

    let client_input_count = user_inputs.len() + client_inputs.len();
    let server_input_count = server_inputs.len();
    let output_count = user_outputs.len()
        + change_outputs.len()
        + 1
        + usize::from(server_change.is_some())
        + usize::from(fee_change.is_some());

    let min_network_fee = TxFee {
        server_inputs: server_input_count,
        user_inputs: client_input_count,
        outputs: output_count,
    }
    .fee();

    let actual_network_fee = network_fee.value;
    ensure!(actual_network_fee >= min_network_fee);
    ensure!(actual_network_fee <= 2 * min_network_fee);

    let min_server_fee = (actual_network_fee as f64 * req.price) as u64 + req.fixed_fee;
    let actual_server_fee = server_fee.value;
    ensure!(actual_server_fee >= min_server_fee);
    ensure!(actual_server_fee <= 2 * min_server_fee);

    Ok(())
}
