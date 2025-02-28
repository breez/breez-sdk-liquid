use anyhow::Result;
use log::warn;

use crate::prelude::*;
use crate::recover::model::*;

/// Handler for updating chain receive swaps with recovered data
pub(crate) struct ChainReceiveSwapHandler;

impl ChainReceiveSwapHandler {
    /// Check if chain receive swap recovery should be skipped
    pub fn should_skip_recovery(
        chain_swap: &ChainSwap,
        recovered_data: &RecoveredOnchainDataChainReceive,
        is_local_within_grace_period: bool,
    ) -> bool {
        let swap_id = &chain_swap.id;

        let claim_is_cleared =
            chain_swap.claim_tx_id.is_some() && recovered_data.lbtc_claim_tx_id.is_none();
        let refund_is_cleared =
            chain_swap.refund_tx_id.is_some() && recovered_data.btc_refund_tx_id.is_none();

        if is_local_within_grace_period && (claim_is_cleared || refund_is_cleared) {
            warn!(
                "Local incoming chain swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us. Claim clear: \
                {claim_is_cleared} - Refund clear: {refund_is_cleared}"
            );
            return true;
        }

        false
    }

    /// Update a chain receive swap with recovered data
    pub fn update_swap(
        chain_swap: &mut ChainSwap,
        swap_id: &str,
        recovered_data: &RecoveredOnchainDataChainReceive,
        current_block_height: u32,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(chain_swap, recovered_data, is_local_within_grace_period) {
            return Ok(());
        }

        // Update amount if available
        if recovered_data.btc_user_lockup_amount_sat > 0 {
            chain_swap.actual_payer_amount_sat = Some(recovered_data.btc_user_lockup_amount_sat);
        }

        // Update state based on chain tip
        let is_expired = current_block_height >= chain_swap.timeout_block_height;
        let (expected_user_lockup_amount_sat, swap_limits) = match chain_swap.payer_amount_sat {
            0 => (None, Some(chain_swap.get_boltz_pair()?.limits)),
            expected => (Some(expected), None),
        };

        if let Some(new_state) = recovered_data.derive_partial_state(
            expected_user_lockup_amount_sat,
            swap_limits,
            is_expired,
            chain_swap.is_waiting_fee_acceptance(),
        ) {
            chain_swap.state = new_state;
        }

        // Update transaction IDs
        chain_swap.server_lockup_tx_id = recovered_data
            .lbtc_server_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap
            .claim_address
            .clone_from(&recovered_data.lbtc_claim_address);
        chain_swap.user_lockup_tx_id = recovered_data
            .btc_user_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.claim_tx_id = recovered_data
            .lbtc_claim_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.refund_tx_id = recovered_data
            .btc_refund_tx_id
            .clone()
            .map(|h| h.txid.to_string());

        Ok(())
    }
}
