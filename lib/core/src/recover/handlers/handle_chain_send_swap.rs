use anyhow::Result;
use log::warn;

use crate::prelude::*;
use crate::recover::model::*;

/// Handler for updating chain send swaps with recovered data
pub(crate) struct ChainSendSwapHandler;

impl ChainSendSwapHandler {
    /// Check if chain send swap recovery should be skipped
    pub fn should_skip_recovery(
        chain_swap: &ChainSwap,
        recovered_data: &RecoveredOnchainDataChainSend,
        is_local_within_grace_period: bool,
    ) -> bool {
        let swap_id = &chain_swap.id;

        let lockup_is_cleared = chain_swap.user_lockup_tx_id.is_some()
            && recovered_data.lbtc_user_lockup_tx_id.is_none();
        let refund_is_cleared =
            chain_swap.refund_tx_id.is_some() && recovered_data.lbtc_refund_tx_id.is_none();
        let claim_is_cleared =
            chain_swap.claim_tx_id.is_some() && recovered_data.btc_claim_tx_id.is_none();

        if is_local_within_grace_period
            && (lockup_is_cleared || refund_is_cleared || claim_is_cleared)
        {
            warn!(
                "Local outgoing chain swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us. Lockup clear: \
                {lockup_is_cleared} - Refund clear: {refund_is_cleared} - Claim clear: {claim_is_cleared}"
            );
            return true;
        }

        false
    }

    /// Update a chain send swap with recovered data
    pub fn update_swap(
        chain_swap: &mut ChainSwap,
        swap_id: &str,
        recovered_data: &RecoveredOnchainDataChainSend,
        current_block_height: u32,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(chain_swap, recovered_data, is_local_within_grace_period) {
            return Ok(());
        }

        // Update state based on chain tip
        let is_expired = current_block_height >= chain_swap.timeout_block_height;
        if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
            chain_swap.state = new_state;
        }

        // Update transaction IDs
        chain_swap.server_lockup_tx_id = recovered_data
            .btc_server_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.user_lockup_tx_id = recovered_data
            .lbtc_user_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.claim_tx_id = recovered_data
            .btc_claim_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.refund_tx_id = recovered_data
            .lbtc_refund_tx_id
            .clone()
            .map(|h| h.txid.to_string());

        Ok(())
    }
}
