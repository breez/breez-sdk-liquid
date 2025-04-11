use std::collections::BTreeMap;

use anyhow::{anyhow, ensure, Result};
use lwk_wollet::elements::AssetId;

use crate::wallet::utxo_select::{utxo_select_best, InOut};

pub(crate) struct AssetSelectRequest {
    pub fee_asset: AssetId,
    pub wallet_utxos: Vec<InOut>,
    pub user_outputs: Vec<InOut>,
}

pub(crate) struct AssetSelectResult {
    pub asset_inputs: Vec<InOut>,
    pub user_outputs: Vec<InOut>,
    pub change_outputs: Vec<InOut>,
    pub user_output_amounts: BTreeMap<AssetId, u64>,
}

pub(crate) fn asset_select(
    AssetSelectRequest {
        fee_asset,
        wallet_utxos,
        user_outputs,
    }: AssetSelectRequest,
) -> Result<AssetSelectResult> {
    let mut user_output_amounts = BTreeMap::<AssetId, u64>::new();

    for input in wallet_utxos.iter() {
        ensure!(input.value > 0, anyhow!("Invalid amount {:?}", input));
    }

    for user_output in user_outputs.iter() {
        ensure!(
            user_output.value > 0,
            anyhow!("Invalid amount {:?}", user_output),
        );
        *user_output_amounts.entry(user_output.asset_id).or_default() += user_output.value;
    }

    let mut asset_inputs = Vec::<InOut>::new();
    let mut change_outputs = Vec::<InOut>::new();

    for (&asset_id, &target_value) in user_output_amounts.iter() {
        if asset_id != fee_asset {
            let wallet_utxo = wallet_utxos
                .iter()
                .filter(|utxo| utxo.asset_id == asset_id)
                .map(|utxo| utxo.value)
                .collect::<Vec<_>>();
            let available = wallet_utxo.iter().sum::<u64>();

            ensure!(
                available >= target_value,
                anyhow!(
                    "Not enough UTXOs for asset {}, required: {}, available: {}",
                    asset_id,
                    target_value,
                    available
                )
            );

            let selected =
                utxo_select_best(target_value, &wallet_utxo).ok_or(anyhow!("No utxos selected"))?;

            let mut total_value = 0;
            for value in selected {
                asset_inputs.push(InOut { asset_id, value });
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

    Ok(AssetSelectResult {
        asset_inputs,
        user_outputs,
        change_outputs,
        user_output_amounts,
    })
}
