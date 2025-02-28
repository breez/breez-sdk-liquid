use anyhow::Result;
use log::{error, warn};

use crate::prelude::*;
use crate::recover::model::*;
use crate::utils;

/// Handler for updating send swaps with recovered data
pub(crate) struct SendSwapHandler;

impl SendSwapHandler {
    /// Check if send swap recovery should be skipped
    pub fn should_skip_recovery(
        send_swap: &SendSwap,
        recovered_data: &RecoveredOnchainDataSend,
        is_local_within_grace_period: bool,
    ) -> bool {
        let swap_id = &send_swap.id;
        let lockup_is_cleared =
            send_swap.lockup_tx_id.is_some() && recovered_data.lockup_tx_id.is_none();
        let refund_is_cleared =
            send_swap.refund_tx_id.is_some() && recovered_data.refund_tx_id.is_none();

        if is_local_within_grace_period && (lockup_is_cleared || refund_is_cleared) {
            warn!(
                "Local send swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us. Lockup clear: \
                {lockup_is_cleared} - Refund clear: {refund_is_cleared}"
            );
            return true;
        }

        false
    }

    /// Update a send swap with recovered data
    pub fn update_swap(
        send_swap: &mut SendSwap,
        swap_id: &str,
        recovered_data: &RecoveredOnchainDataSend,
        current_block_height: u32,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(send_swap, recovered_data, is_local_within_grace_period) {
            return Ok(());
        }

        // Update transaction IDs
        send_swap.lockup_tx_id = recovered_data
            .lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        send_swap.refund_tx_id = recovered_data
            .refund_tx_id
            .clone()
            .map(|h| h.txid.to_string());

        // Update preimage if valid
        if let Some(preimage) = &recovered_data.preimage {
            if send_swap.preimage.is_none() {
                match utils::verify_payment_hash(preimage, &send_swap.invoice) {
                    Ok(_) => send_swap.preimage = Some(preimage.clone()),
                    Err(e) => {
                        error!("Failed to verify recovered preimage for swap {swap_id}: {e}");
                    }
                }
            }
        }

        // Update state based on recovered data
        let timeout_block_height = send_swap.timeout_block_height as u32;
        let is_expired = current_block_height >= timeout_block_height;
        if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
            send_swap.state = new_state;
        }

        Ok(())
    }
}
