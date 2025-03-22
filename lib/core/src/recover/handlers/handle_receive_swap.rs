use anyhow::Result;
use boltz_client::ElementsAddress;
use log::{debug, warn};
use lwk_wollet::elements::Txid;
use lwk_wollet::WalletTx;
use std::collections::HashMap;
use std::str::FromStr;

use crate::prelude::*;
use crate::recover::model::*;

use super::determine_incoming_lockup_and_claim_txs;

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

    /// Recover and update a receive swap with data from the chain
    pub(crate) async fn recover_swap(
        receive_swap: &mut ReceiveSwap,
        context: &RecoveryContext,
        is_local_within_grace_period: bool,
    ) -> Result<()> {
        let swap_id = &receive_swap.id.clone();
        debug!("[Recover Receive] Recovering data for swap {swap_id}");

        let mrh_script = ElementsAddress::from_str(&receive_swap.mrh_address)
            .map_err(|_| anyhow::anyhow!("Invalid MRH address for swap {swap_id}"))?
            .script_pubkey();
        let claim_script = receive_swap.claim_script()?;
        let history = ReceiveSwapHistory {
            lbtc_mrh_script_history: context
                .lbtc_script_to_history_map
                .get(&mrh_script)
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter(|&tx_history| tx_history.height < receive_swap.timeout_block_height as i32)
                .cloned()
                .collect(),

            lbtc_claim_script_history: context
                .lbtc_script_to_history_map
                .get(&claim_script)
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter(|&tx_history| tx_history.height < receive_swap.timeout_block_height as i32)
                .cloned()
                .collect(),
        };

        // First obtain recovered data from the history
        let recovered_data =
            Self::recover_onchain_data(&context.tx_map, &history, receive_swap.created_at)?;

        // Update the swap with recovered data
        Self::update_swap(
            receive_swap,
            &recovered_data,
            context.liquid_tip_height,
            is_local_within_grace_period,
        )
    }

    /// Update a receive swap with recovered data
    pub fn update_swap(
        receive_swap: &mut ReceiveSwap,
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

    /// Reconstruct Receive Swap tx IDs from the onchain data
    ///
    /// The implementation tolerates a `tx_map` that is older than the history in the sense that
    /// no incorrect data is recovered. Transactions that are missing from `tx_map` are simply not recovered.
    fn recover_onchain_data(
        tx_map: &TxMap,
        history: &ReceiveSwapHistory,
        swap_timestamp: u32,
    ) -> Result<RecoveredOnchainDataReceive> {
        // The MRH script history txs filtered by the swap timestamp
        let mrh_txs: HashMap<Txid, WalletTx> = history
            .lbtc_mrh_script_history
            .iter()
            .filter_map(|h| tx_map.incoming_tx_map.get(&h.txid))
            .filter(|tx| tx.timestamp.map(|t| t > swap_timestamp).unwrap_or(true))
            .map(|tx| (tx.txid, tx.clone()))
            .collect();

        let mrh_tx_id = history
            .lbtc_mrh_script_history
            .iter()
            .find(|&tx| mrh_txs.contains_key::<Txid>(&tx.txid))
            .cloned();

        let mrh_amount_sat = mrh_tx_id
            .clone()
            .and_then(|h| mrh_txs.get(&h.txid))
            .map(|tx| tx.balance.values().sum::<i64>().unsigned_abs());

        let (lockup_tx_id, claim_tx_id) =
            determine_incoming_lockup_and_claim_txs(&history.lbtc_claim_script_history, tx_map);

        // Take only the lockup_tx_id and claim_tx_id if either are set,
        // otherwise take the mrh_tx_id and mrh_amount_sat
        let recovered_onchain_data = match (lockup_tx_id.as_ref(), claim_tx_id.as_ref()) {
            (Some(_), None) | (Some(_), Some(_)) => RecoveredOnchainDataReceive {
                lockup_tx_id,
                claim_tx_id,
                mrh_tx_id: None,
                mrh_amount_sat: None,
            },
            _ => RecoveredOnchainDataReceive {
                lockup_tx_id: None,
                claim_tx_id: None,
                mrh_tx_id,
                mrh_amount_sat,
            },
        };

        Ok(recovered_onchain_data)
    }
}

pub(crate) struct RecoveredOnchainDataReceive {
    pub(crate) lockup_tx_id: Option<History<elements::Txid>>,
    pub(crate) claim_tx_id: Option<History<elements::Txid>>,
    pub(crate) mrh_tx_id: Option<History<elements::Txid>>,
    pub(crate) mrh_amount_sat: Option<u64>,
}

impl RecoveredOnchainDataReceive {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(claim_tx_id) => match claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                None => match is_expired {
                    true => Some(PaymentState::Failed),
                    false => Some(PaymentState::Pending),
                },
            },
            None => match &self.mrh_tx_id {
                Some(mrh_tx_id) => match mrh_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created. In this case we return None.
                None => match is_expired {
                    true => Some(PaymentState::Failed),
                    false => None,
                },
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReceiveSwapHistory {
    pub(crate) lbtc_claim_script_history: LBtcHistory,
    pub(crate) lbtc_mrh_script_history: LBtcHistory,
}
