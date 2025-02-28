use anyhow::Result;
use log::warn;

use crate::prelude::*;
use crate::recover::model::*;

/// Handler for updating receive swaps with recovered data
pub(crate) struct ReceiveSwapHandler;

impl ReceiveSwapHandler {
    /// Check if receive swap recovery should be skipped
    pub fn should_skip_recovery(
        receive_swap: &ReceiveSwap,
        recovered_data: &RecoveredOnchainDataReceive,
        is_local_within_grace_period: bool,
    ) -> bool {
        let swap_id = &receive_swap.id;
        let claim_is_cleared =
            receive_swap.claim_tx_id.is_some() && recovered_data.claim_tx_id.is_none();

        if is_local_within_grace_period && claim_is_cleared {
            warn!(
                "Local receive swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us (claim)"
            );
            return true;
        }

        false
    }

    /// Update a receive swap with recovered data
    pub fn update_swap(
        receive_swap: &mut ReceiveSwap,
        swap_id: &str,
        recovered_data: &RecoveredOnchainDataReceive,
        current_block_height: u32,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(receive_swap, recovered_data, is_local_within_grace_period) {
            return Ok(());
        }

        // Update state based on chain tip
        let timeout_block_height = receive_swap.timeout_block_height;
        let is_expired = current_block_height >= timeout_block_height;
        if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
            receive_swap.state = new_state;
        }

        // Update transaction IDs
        receive_swap.claim_tx_id = recovered_data
            .claim_tx_id
            .clone()
            .map(|history_tx_id| history_tx_id.txid.to_string());
        receive_swap.mrh_tx_id = recovered_data
            .mrh_tx_id
            .clone()
            .map(|history_tx_id| history_tx_id.txid.to_string());
        receive_swap.lockup_tx_id = recovered_data
            .lockup_tx_id
            .clone()
            .map(|history_tx_id| history_tx_id.txid.to_string());

        // Update amounts if we have MRH data
        if let Some(mrh_amount_sat) = recovered_data.mrh_amount_sat {
            receive_swap.payer_amount_sat = mrh_amount_sat;
            receive_swap.receiver_amount_sat = mrh_amount_sat;
        }

        Ok(())
    }
}
