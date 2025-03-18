use std::sync::Arc;

use anyhow::Result;
use boltz_client::ToHex;
use log::{debug, error, warn};
use lwk_wollet::elements::Txid;

use crate::prelude::*;
use crate::recover::model::*;
use crate::swapper::Swapper;
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

    /// Recover and update a send swap with data from the chain
    pub async fn recover_swap(
        send_swap: &mut SendSwap,
        context: &RecoveryContext,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        let swap_id = send_swap.id.clone();
        debug!("[Recover Send] Recovering data for swap {swap_id}");
        let swap_script = send_swap.get_swap_script()?;
        let lockup_script = swap_script
            .funding_addrs
            .ok_or(anyhow::anyhow!("no funding address found"))?
            .script_pubkey();

        let empty_history = LBtcHistory::new();
        let history = context
            .lbtc_script_to_history_map
            .get(&lockup_script)
            .unwrap_or(&empty_history);

        // First obtain transaction IDs from the history
        let mut recovered_data = Self::recover_onchain_data(&context.tx_map, &swap_id, history)?;

        // Recover preimage if needed
        if let (Some(claim_tx_id), None) = (&recovered_data.claim_tx_id, &send_swap.preimage) {
            match Self::recover_preimage(
                context,
                claim_tx_id.txid,
                &swap_id,
                context.swapper.clone(),
            )
            .await
            {
                Ok(Some(preimage)) => {
                    recovered_data.preimage = Some(preimage);
                }
                Ok(None) => {
                    warn!("No preimage found for Send Swap {swap_id}");
                    recovered_data.claim_tx_id = None;
                }
                Err(e) => {
                    error!("Failed to recover preimage for swap {swap_id}: {e}");
                    recovered_data.claim_tx_id = None
                }
            }
        }

        // Update the swap with recovered data
        Self::update_swap(
            send_swap,
            &swap_id,
            &recovered_data,
            context.liquid_tip_height,
            is_local_within_grace_period,
        )
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
            match utils::verify_payment_hash(preimage, &send_swap.invoice) {
                Ok(_) => send_swap.preimage = Some(preimage.clone()),
                Err(e) => {
                    error!("Failed to verify recovered preimage for swap {swap_id}: {e}");
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

    /// Reconstruct Send Swap tx IDs from the onchain data
    ///
    /// The implementation tolerates a `tx_map` that is older than the history in the sense that
    /// no incorrect data is recovered. Transactions that are missing from `tx_map` are simply not recovered.
    fn recover_onchain_data(
        tx_map: &TxMap,
        swap_id: &str,
        history: &[History<elements::Txid>],
    ) -> Result<RecoveredOnchainDataSend> {
        // If a history tx is one of our outgoing txs, it's a lockup tx
        let lockup_tx_id = history
            .iter()
            .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
            .cloned();

        let claim_tx_id = if lockup_tx_id.is_some() {
            // A history tx that is neither a known incoming or outgoing tx is a claim tx.
            //
            // Only find the claim_tx from the history if we find a lockup_tx. Not doing so will select
            // the first tx as the claim, whereas we should check that the claim is not the lockup.
            history
                .iter()
                .filter(|&tx| !tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .find(|&tx| !tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned()
        } else {
            error!("No lockup tx found when recovering data for Send Swap {swap_id}");
            None
        };

        // If a history tx is one of our incoming txs, it's a refund tx
        let refund_tx_id = history
            .iter()
            .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
            .cloned();

        Ok(RecoveredOnchainDataSend {
            lockup_tx_id,
            claim_tx_id,
            refund_tx_id,
            preimage: None,
        })
    }

    /// Tries to recover the preimage for a send swap from its claim tx
    async fn recover_preimage(
        context: &RecoveryContext,
        claim_tx_id: Txid,
        swap_id: &str,
        swapper: Arc<dyn Swapper>,
    ) -> Result<Option<String>> {
        // Try cooperative first
        if let Ok(preimage) = swapper.get_submarine_preimage(swap_id).await {
            println!("Found Send Swap {swap_id} preimage cooperatively: {preimage}");
            return Ok(Some(preimage));
        }
        warn!("Could not recover Send swap {swap_id} preimage cooperatively");
        let claim_txs = context
            .liquid_chain_service
            .get_transactions(&[claim_tx_id])
            .await?;

        match claim_txs.is_empty() {
            false => Self::extract_preimage_from_claim_tx(&claim_txs[0], swap_id).map(Some),
            true => {
                warn!("Could not recover Send swap {swap_id} preimage non cooperatively");
                Ok(None)
            }
        }
    }

    /// Extracts the preimage from a claim tx
    pub fn extract_preimage_from_claim_tx(
        claim_tx: &lwk_wollet::elements::Transaction,
        swap_id: &str,
    ) -> Result<String> {
        use lwk_wollet::bitcoin::Witness;
        use lwk_wollet::hashes::{sha256, Hash as _};

        let input = claim_tx
            .input
            .first()
            .ok_or_else(|| anyhow::anyhow!("Found no input for claim tx"))?;

        let script_witness_bytes = input.clone().witness.script_witness;
        log::debug!("Found Send Swap {swap_id} claim tx witness: {script_witness_bytes:?}");
        let script_witness = Witness::from(script_witness_bytes);

        let preimage_bytes = script_witness
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Claim tx witness has no preimage"))?;
        let preimage = sha256::Hash::from_slice(preimage_bytes)
            .map_err(|e| anyhow::anyhow!("Claim tx witness has invalid preimage: {e}"))?;
        let preimage_hex = preimage.to_hex();
        log::debug!("Found Send Swap {swap_id} claim tx preimage: {preimage_hex}");

        Ok(preimage_hex)
    }
}

pub(crate) struct RecoveredOnchainDataSend {
    pub(crate) lockup_tx_id: Option<History<elements::Txid>>,
    pub(crate) claim_tx_id: Option<History<elements::Txid>>,
    pub(crate) refund_tx_id: Option<History<elements::Txid>>,
    pub(crate) preimage: Option<String>,
}

impl RecoveredOnchainDataSend {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(_) => Some(PaymentState::Complete),
                None => match &self.refund_tx_id {
                    Some(refund_tx_id) => match refund_tx_id.confirmed() {
                        true => Some(PaymentState::Failed),
                        false => Some(PaymentState::RefundPending),
                    },
                    None => match is_expired {
                        true => Some(PaymentState::RefundPending),
                        false => Some(PaymentState::Pending),
                    },
                },
            },
            None => match is_expired {
                true => Some(PaymentState::Failed),
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created or TimedOut. In this case we return None.
                false => None,
            },
        }
    }
}
