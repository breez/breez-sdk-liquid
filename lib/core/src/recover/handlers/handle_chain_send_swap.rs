use anyhow::Result;
use log::{debug, error, warn};
use lwk_wollet::elements::Txid;

use crate::prelude::*;
use crate::recover::model::*;

/// Handler for updating chain send swaps with recovered data
pub(crate) struct ChainSendSwapHandler;

impl ChainSendSwapHandler {
    /// Check if chain send swap recovery should be skipped
    pub fn should_skip_recovery(
        chain_swap: &ChainSwap,
        recovered_data: &RecoveredOnchainDataChainSend,
        is_within_grace_period: bool,
    ) -> bool {
        let swap_id = &chain_swap.id;

        let lockup_is_cleared = chain_swap.user_lockup_tx_id.is_some()
            && recovered_data.lbtc_user_lockup_tx_id.is_none();
        let refund_is_cleared =
            chain_swap.refund_tx_id.is_some() && recovered_data.lbtc_refund_tx_id.is_none();
        let claim_is_cleared =
            chain_swap.claim_tx_id.is_some() && recovered_data.btc_claim_tx_id.is_none();

        if is_within_grace_period && (lockup_is_cleared || refund_is_cleared || claim_is_cleared) {
            warn!(
                "Local outgoing chain swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us. Lockup clear: \
                {lockup_is_cleared} - Refund clear: {refund_is_cleared}"
            );
            return true;
        }

        false
    }

    /// Recover and update a chain send swap with data from the chain
    pub async fn recover_swap(
        chain_swap: &mut ChainSwap,
        context: &ChainSwapRecoveryContext,
        is_within_grace_period: bool,
    ) -> Result<()> {
        let swap_id = &chain_swap.id.clone();
        debug!("[Recover Chain Send] Recovering data for swap {swap_id}");

        // Extract claim script from swap
        let claim_script = chain_swap
            .get_claim_swap_script()
            .ok()
            .and_then(|script| script.as_bitcoin_script().ok())
            .and_then(|script| script.funding_addrs.map(|addr| addr.script_pubkey()))
            .ok_or_else(|| {
                anyhow::anyhow!("BTC claim script not found for Onchain Send Swap {swap_id}")
            })?;

        let lockup_script = chain_swap
            .get_lockup_swap_script()
            .ok()
            .and_then(|script| script.as_liquid_script().ok())
            .and_then(|script| script.funding_addrs.map(|addr| addr.script_pubkey()))
            .ok_or_else(|| {
                anyhow::anyhow!("LBTC lockup script not found for Onchain Send Swap {swap_id}")
            })?;

        let history = &SendChainSwapHistory {
            lbtc_lockup_script_history: context
                .lbtc_script_to_history_map
                .get(&lockup_script)
                .cloned()
                .unwrap_or_default(),
            btc_claim_script_history: context
                .btc_script_to_history_map
                .get(&claim_script)
                .cloned()
                .unwrap_or_default(),
            btc_claim_script_txs: context
                .btc_script_to_txs_map
                .get(&claim_script)
                .cloned()
                .unwrap_or_default(),
        };

        // First obtain transaction IDs from the history
        let recovered_data =
            Self::recover_onchain_data(&context.tx_map, swap_id, history, &claim_script)?;

        // Update the swap with recovered data
        Self::update_swap(
            chain_swap,
            &recovered_data,
            context.liquid_tip_height,
            context.bitcoin_tip_height,
            is_within_grace_period,
        )
    }

    /// Update a chain send swap with recovered data
    pub fn update_swap(
        chain_swap: &mut ChainSwap,
        recovered_data: &RecoveredOnchainDataChainSend,
        current_liquid_block_height: u32,
        current_bitcoin_block_height: u32,
        is_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(chain_swap, recovered_data, is_within_grace_period) {
            return Ok(());
        }

        // Update state based on chain tip
        let is_expired = current_liquid_block_height >= chain_swap.timeout_block_height
            || current_bitcoin_block_height >= chain_swap.claim_timeout_block_height;
        if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
            chain_swap.state = new_state;
        }

        // Update transaction IDs
        chain_swap.user_lockup_tx_id = recovered_data
            .lbtc_user_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.refund_tx_id = recovered_data
            .lbtc_refund_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.server_lockup_tx_id = recovered_data
            .btc_server_lockup_tx_id
            .clone()
            .map(|h| h.txid.to_string());
        chain_swap.claim_tx_id = recovered_data
            .btc_claim_tx_id
            .clone()
            .map(|h| h.txid.to_string());

        Ok(())
    }

    /// Reconstruct Chain Send Swap tx IDs from the onchain data
    ///
    /// The implementation tolerates a `tx_map` that is older than the history in the sense that
    /// no incorrect data is recovered. Transactions that are missing from `tx_map` are simply not recovered.
    fn recover_onchain_data(
        tx_map: &TxMap,
        swap_id: &str,
        history: &SendChainSwapHistory,
        claim_script: &BtcScript,
    ) -> Result<RecoveredOnchainDataChainSend> {
        // If a history tx is one of our outgoing txs, it's a lockup tx
        let lbtc_user_lockup_tx_id = history
            .lbtc_lockup_script_history
            .iter()
            .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
            .cloned();

        if lbtc_user_lockup_tx_id.is_none() {
            error!("No lockup tx found when recovering data for Chain Send Swap {swap_id}");
        }

        // If a history tx is one of our incoming txs, it's a refund tx
        let lbtc_refund_tx_id = history
            .lbtc_lockup_script_history
            .iter()
            .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
            .cloned();

        let (btc_server_lockup_tx_id, btc_claim_tx_id) = match history
            .btc_claim_script_history
            .len()
        {
            // Only lockup tx available
            1 => (Some(history.btc_claim_script_history[0].clone()), None),

            2 => {
                let first_tx = history.btc_claim_script_txs[0].clone();
                let first_tx_id = history.btc_claim_script_history[0].clone();
                let second_tx_id = history.btc_claim_script_history[1].clone();

                // We check the full tx, to determine if this is the BTC lockup tx
                let is_first_tx_lockup_tx = first_tx
                    .output
                    .iter()
                    .any(|out| matches!(&out.script_pubkey, x if x == claim_script));

                match is_first_tx_lockup_tx {
                    true => (Some(first_tx_id), Some(second_tx_id)),
                    false => (Some(second_tx_id), Some(first_tx_id)),
                }
            }
            n => {
                warn!("BTC script history with length {n} found while recovering data for Chain Send Swap {swap_id}");
                (None, None)
            }
        };

        Ok(RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id,
            lbtc_refund_tx_id,
            btc_server_lockup_tx_id,
            btc_claim_tx_id,
        })
    }
}

pub(crate) struct RecoveredOnchainDataChainSend {
    /// LBTC tx initiated by the SDK (the "user" as per Boltz), sending funds to the swap funding address.
    pub(crate) lbtc_user_lockup_tx_id: Option<LBtcHistory>,
    /// LBTC tx initiated by the SDK to itself, in case the initial funds have to be refunded.
    pub(crate) lbtc_refund_tx_id: Option<LBtcHistory>,
    /// BTC tx locking up funds by the swapper
    pub(crate) btc_server_lockup_tx_id: Option<BtcHistory>,
    /// BTC tx that claims to the final BTC destination address. The final step in a successful swap.
    pub(crate) btc_claim_tx_id: Option<BtcHistory>,
}

// TODO: We have to be careful around overwriting the RefundPending state, as this swap monitored
// after the expiration of the swap and if new funds are detected on the lockup script they are refunded.
// Perhaps we should check in the recovery the lockup balance and set accordingly.
impl RecoveredOnchainDataChainSend {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lbtc_user_lockup_tx_id {
            Some(_) => match (&self.btc_claim_tx_id, &self.lbtc_refund_tx_id) {
                (Some(btc_claim_tx_id), None) => match btc_claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                (None, Some(lbtc_refund_tx_id)) => match lbtc_refund_tx_id.confirmed() {
                    true => Some(PaymentState::Failed),
                    false => Some(PaymentState::RefundPending),
                },
                (Some(btc_claim_tx_id), Some(lbtc_refund_tx_id)) => {
                    match btc_claim_tx_id.confirmed() {
                        true => match lbtc_refund_tx_id.confirmed() {
                            true => Some(PaymentState::Complete),
                            false => Some(PaymentState::RefundPending),
                        },
                        false => Some(PaymentState::Pending),
                    }
                }
                (None, None) => match is_expired {
                    true => Some(PaymentState::RefundPending),
                    false => Some(PaymentState::Pending),
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

#[derive(Clone)]
pub(crate) struct SendChainSwapHistory {
    pub(crate) lbtc_lockup_script_history: Vec<LBtcHistory>,
    pub(crate) btc_claim_script_history: Vec<BtcHistory>,
    pub(crate) btc_claim_script_txs: Vec<bitcoin::Transaction>,
}
