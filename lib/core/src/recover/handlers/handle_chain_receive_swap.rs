use anyhow::Result;
use boltz_client::boltz::PairLimits;
use boltz_client::ElementsAddress;
use log::{debug, warn};
use lwk_wollet::elements::{secp256k1_zkp, AddressParams};
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;

use crate::prelude::*;
use crate::recover::model::*;

use super::determine_incoming_lockup_and_claim_txs;

/// Handler for updating chain receive swaps with recovered data
pub(crate) struct ChainReceiveSwapHandler;

impl ChainReceiveSwapHandler {
    /// Check if chain receive swap recovery should be skipped
    pub fn should_skip_recovery(
        chain_swap: &ChainSwap,
        recovered_data: &RecoveredOnchainDataChainReceive,
        is_within_grace_period: bool,
    ) -> bool {
        let swap_id = &chain_swap.id;

        let claim_is_cleared =
            chain_swap.claim_tx_id.is_some() && recovered_data.lbtc_claim_tx_id.is_none();
        let refund_is_cleared =
            chain_swap.refund_tx_id.is_some() && recovered_data.btc_refund_tx_id.is_none();

        if is_within_grace_period && (claim_is_cleared || refund_is_cleared) {
            warn!(
                "Local incoming chain swap {swap_id} was updated recently - skipping recovery \
                as it would clear a tx that may have been broadcasted by us. Claim clear: \
                {claim_is_cleared} - Refund clear: {refund_is_cleared}"
            );
            return true;
        }

        false
    }

    /// Recover and update a chain receive swap with data from the chain
    pub async fn recover_swap(
        chain_swap: &mut ChainSwap,
        context: &ChainSwapRecoveryContext,
        is_within_grace_period: bool,
    ) -> Result<()> {
        let swap_id = &chain_swap.id.clone();
        debug!("[Recover Chain Receive] Recovering data for swap {swap_id}");

        // Extract lockup script from swap
        let lockup_script = chain_swap
            .get_lockup_swap_script()
            .ok()
            .and_then(|script| script.as_bitcoin_script().ok())
            .and_then(|script| script.funding_addrs.map(|addr| addr.script_pubkey()))
            .ok_or_else(|| {
                anyhow::anyhow!("BTC lockup script not found for Onchain Receive Swap {swap_id}")
            })?;

        let claim_script = chain_swap
            .get_claim_swap_script()
            .ok()
            .and_then(|script| script.as_liquid_script().ok())
            .and_then(|script| script.funding_addrs.map(|addr| addr.script_pubkey()))
            .ok_or_else(|| {
                anyhow::anyhow!("BTC claim script not found for Onchain Send Swap {swap_id}")
            })?;

        let history = &ReceiveChainSwapHistory {
            lbtc_claim_script_history: context
                .lbtc_script_to_history_map
                .get(&claim_script)
                .cloned()
                .unwrap_or_default(),
            btc_lockup_script_history: context
                .btc_script_to_history_map
                .get(&lockup_script)
                .cloned()
                .unwrap_or(Vec::new()),
            btc_lockup_script_txs: context
                .btc_script_to_txs_map
                .get(&lockup_script)
                .cloned()
                .unwrap_or(Vec::new()),
            btc_lockup_script_balance: context
                .btc_script_to_balance_map
                .get(&lockup_script)
                .cloned(),
        };

        // First obtain transaction IDs from the history
        let recovered_data = Self::recover_onchain_data(
            &context.tx_map,
            history,
            &lockup_script,
            &context.master_blinding_key,
        )?;

        // Update the swap with recovered data
        Self::update_swap(
            chain_swap,
            &recovered_data,
            context.liquid_tip_height,
            context.bitcoin_tip_height,
            is_within_grace_period,
        )
    }

    /// Update a chain receive swap with recovered data
    pub fn update_swap(
        chain_swap: &mut ChainSwap,
        recovered_data: &RecoveredOnchainDataChainReceive,
        current_liquid_block_height: u32,
        current_bitcoin_block_height: u32,
        is_within_grace_period: bool,
    ) -> Result<()> {
        // Skip updating if within grace period and would clear transactions
        if Self::should_skip_recovery(chain_swap, recovered_data, is_within_grace_period) {
            return Ok(());
        }

        // Update amount if available
        if recovered_data.btc_user_lockup_amount_sat > 0 {
            chain_swap.actual_payer_amount_sat = Some(recovered_data.btc_user_lockup_amount_sat);
        }

        // Update state based on chain tip
        let is_expired = current_liquid_block_height >= chain_swap.claim_timeout_block_height
            || current_bitcoin_block_height >= chain_swap.timeout_block_height;
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
        chain_swap.claim_address = recovered_data.lbtc_claim_address.clone();
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

    /// Reconstruct Chain Receive Swap tx IDs from the onchain data
    ///
    /// The implementation tolerates a `tx_map` that is older than the history in the sense that
    /// no incorrect data is recovered. Transactions that are missing from `tx_map` are simply not recovered.
    fn recover_onchain_data(
        tx_map: &TxMap,
        history: &ReceiveChainSwapHistory,
        lockup_script: &BtcScript,
        master_blinding_key: &MasterBlindingKey,
    ) -> Result<RecoveredOnchainDataChainReceive> {
        let secp = secp256k1_zkp::Secp256k1::new();

        // Determine lockup and claim txs
        let (lbtc_server_lockup_tx_id, lbtc_claim_tx_id) =
            determine_incoming_lockup_and_claim_txs(&history.lbtc_claim_script_history, tx_map);

        // Get claim address from tx
        let lbtc_claim_address = if let Some(claim_tx_id) = &lbtc_claim_tx_id {
            tx_map
                .incoming_tx_map
                .get(&claim_tx_id.txid)
                .and_then(|tx| {
                    tx.outputs
                        .iter()
                        .find(|output| output.is_some())
                        .and_then(|output| output.clone().map(|o| o.script_pubkey))
                })
                .and_then(|script| {
                    ElementsAddress::from_script(
                        &script,
                        Some(master_blinding_key.blinding_key(&secp, &script)),
                        &AddressParams::LIQUID,
                    )
                    .map(|addr| addr.to_string())
                })
        } else {
            None
        };

        // Get current confirmed amount for lockup script
        let btc_user_lockup_address_balance_sat = history
            .btc_lockup_script_balance
            .as_ref()
            .map(|balance| balance.confirmed)
            .unwrap_or_default();

        // Process Bitcoin transactions
        let (btc_lockup_incoming_txs, btc_lockup_outgoing_txs): (Vec<_>, Vec<_>) =
            history.btc_lockup_script_txs.iter().partition(|tx| {
                tx.output
                    .iter()
                    .any(|out| matches!(&out.script_pubkey, x if x == lockup_script))
            });

        // Get user lockup tx from first incoming tx
        let btc_user_lockup_tx_id = btc_lockup_incoming_txs
            .first()
            .and_then(|tx| {
                history
                    .btc_lockup_script_history
                    .iter()
                    .find(|h| h.txid.as_raw_hash() == tx.compute_txid().as_raw_hash())
            })
            .cloned();

        // Get the lockup amount
        let btc_user_lockup_amount_sat = btc_lockup_incoming_txs
            .first()
            .and_then(|tx| {
                tx.output
                    .iter()
                    .find(|out| out.script_pubkey == *lockup_script)
                    .map(|out| out.value)
            })
            .unwrap_or_default()
            .to_sat();

        // Collect outgoing tx IDs
        let btc_outgoing_tx_ids: Vec<BtcHistory> = btc_lockup_outgoing_txs
            .iter()
            .filter_map(|tx| {
                history
                    .btc_lockup_script_history
                    .iter()
                    .find(|h| h.txid.as_raw_hash() == tx.compute_txid().as_raw_hash())
            })
            .cloned()
            .collect();

        // Get last unconfirmed tx or last tx
        let btc_last_outgoing_tx_id = btc_outgoing_tx_ids
            .iter()
            .rev()
            .find(|h| h.height == 0)
            .or(btc_outgoing_tx_ids.last())
            .cloned();

        // Determine the refund tx based on claim status
        let btc_refund_tx_id = match lbtc_claim_tx_id.is_some() {
            true => match btc_lockup_outgoing_txs.len() > 1 {
                true => btc_last_outgoing_tx_id,
                false => None,
            },
            false => btc_last_outgoing_tx_id,
        };

        Ok(RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id,
            lbtc_claim_tx_id,
            lbtc_claim_address,
            btc_user_lockup_tx_id,
            btc_user_lockup_address_balance_sat,
            btc_user_lockup_amount_sat,
            btc_refund_tx_id,
        })
    }
}

pub(crate) struct RecoveredOnchainDataChainReceive {
    /// LBTC tx locking up funds by the swapper
    pub(crate) lbtc_server_lockup_tx_id: Option<LBtcHistory>,
    /// LBTC tx that claims to our wallet. The final step in a successful swap.
    pub(crate) lbtc_claim_tx_id: Option<LBtcHistory>,
    /// LBTC tx out address for the claim tx.
    pub(crate) lbtc_claim_address: Option<String>,
    /// BTC tx initiated by the payer (the "user" as per Boltz), sending funds to the swap funding address.
    pub(crate) btc_user_lockup_tx_id: Option<BtcHistory>,
    /// BTC total funds currently available at the swap funding address.
    pub(crate) btc_user_lockup_address_balance_sat: u64,
    /// BTC sent to lockup address as part of lockup tx.
    pub(crate) btc_user_lockup_amount_sat: u64,
    /// BTC tx initiated by the SDK to a user-chosen address, in case the initial funds have to be refunded.
    pub(crate) btc_refund_tx_id: Option<BtcHistory>,
}

impl RecoveredOnchainDataChainReceive {
    pub(crate) fn derive_partial_state(
        &self,
        expected_user_lockup_amount_sat: Option<u64>,
        swap_limits: Option<PairLimits>,
        is_expired: bool,
        is_waiting_fee_acceptance: bool,
    ) -> Option<PaymentState> {
        let unexpected_amount =
            expected_user_lockup_amount_sat.is_some_and(|expected_lockup_amount_sat| {
                expected_lockup_amount_sat != self.btc_user_lockup_amount_sat
            });
        let amount_out_of_bounds = swap_limits.is_some_and(|limits| {
            self.btc_user_lockup_amount_sat < limits.minimal
                || self.btc_user_lockup_amount_sat > limits.maximal
        });
        let is_expired_refundable = is_expired && self.btc_user_lockup_address_balance_sat > 0;
        let is_refundable = is_expired_refundable || unexpected_amount || amount_out_of_bounds;
        match &self.btc_user_lockup_tx_id {
            Some(_) => match (&self.lbtc_claim_tx_id, &self.btc_refund_tx_id) {
                (Some(lbtc_claim_tx_id), None) => match lbtc_claim_tx_id.confirmed() {
                    true => match is_expired_refundable {
                        true => Some(PaymentState::Refundable),
                        false => Some(PaymentState::Complete),
                    },
                    false => Some(PaymentState::Pending),
                },
                (None, Some(btc_refund_tx_id)) => match btc_refund_tx_id.confirmed() {
                    true => match is_expired_refundable {
                        true => Some(PaymentState::Refundable),
                        false => Some(PaymentState::Failed),
                    },
                    false => Some(PaymentState::RefundPending),
                },
                (Some(lbtc_claim_tx_id), Some(btc_refund_tx_id)) => {
                    match lbtc_claim_tx_id.confirmed() {
                        true => match btc_refund_tx_id.confirmed() {
                            true => match is_expired_refundable {
                                true => Some(PaymentState::Refundable),
                                false => Some(PaymentState::Complete),
                            },
                            false => Some(PaymentState::RefundPending),
                        },
                        false => Some(PaymentState::Pending),
                    }
                }
                (None, None) => match is_refundable {
                    true => Some(PaymentState::Refundable),
                    false => match is_waiting_fee_acceptance {
                        true => Some(PaymentState::WaitingFeeAcceptance),
                        false => Some(PaymentState::Pending),
                    },
                },
            },
            None => match is_expired {
                true => Some(PaymentState::Failed),
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created. In this case we return None.
                false => None,
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReceiveChainSwapHistory {
    pub(crate) lbtc_claim_script_history: Vec<LBtcHistory>,
    pub(crate) btc_lockup_script_history: Vec<BtcHistory>,
    pub(crate) btc_lockup_script_txs: Vec<bitcoin::Transaction>,
    pub(crate) btc_lockup_script_balance: Option<BtcScriptBalance>,
}
