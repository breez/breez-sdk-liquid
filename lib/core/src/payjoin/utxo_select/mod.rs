mod asset;
mod tests;

use std::collections::BTreeMap;

use anyhow::{anyhow, ensure, Result};
use asset::{asset_select, AssetSelectRequest, AssetSelectResult};
use lwk_wollet::elements::AssetId;

use crate::wallet::{
    network_fee::{self, TxFee},
    utxo_select::{utxo_select_fixed, utxo_select_in_range, InOut},
};

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
                        native_inputs: server_input_count,
                        nested_inputs: user_input_count,
                        outputs: output_count,
                    }
                    .fee(None);

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
        native_inputs: server_input_count,
        nested_inputs: client_input_count,
        outputs: output_count,
    }
    .fee(None);

    let actual_network_fee = network_fee.value;
    ensure!(actual_network_fee >= min_network_fee);
    ensure!(actual_network_fee <= 2 * min_network_fee);

    let min_server_fee = (actual_network_fee as f64 * req.price) as u64 + req.fixed_fee;
    let actual_server_fee = server_fee.value;
    ensure!(actual_server_fee >= min_server_fee);
    ensure!(actual_server_fee <= 2 * min_server_fee);

    Ok(())
}
