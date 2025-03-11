use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, bail, Result};
use boltz_client::swaps::boltz::RevSwapStates;
use boltz_client::{boltz, Serialize, ToHex};
use log::{debug, error, info, warn};
use lwk_wollet::elements::secp256k1_zkp::Secp256k1;
use lwk_wollet::elements::{Transaction, Txid};
use lwk_wollet::hashes::hex::DisplayHex;
use lwk_wollet::secp256k1::SecretKey;
use tokio::sync::broadcast;

use crate::chain::liquid::LiquidChainService;
use crate::model::{BlockListener, PaymentState::*};
use crate::model::{Config, PaymentTxData, PaymentType, ReceiveSwap};
use crate::prelude::Swap;
use crate::{ensure_sdk, utils};
use crate::{
    error::PaymentError, model::PaymentState, persist::Persister, swapper::Swapper,
    wallet::OnchainWallet,
};

/// The maximum acceptable amount in satoshi when claiming using zero-conf
pub const DEFAULT_ZERO_CONF_MAX_SAT: u64 = 1_000_000;

pub(crate) struct ReceiveSwapHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    subscription_notifier: broadcast::Sender<String>,
    liquid_chain_service: Arc<dyn LiquidChainService>,
}

#[sdk_macros::async_trait]
impl BlockListener for ReceiveSwapHandler {
    async fn on_bitcoin_block(&self, _height: u32) {}

    async fn on_liquid_block(&self, height: u32) {
        if let Err(e) = self.claim_confirmed_lockups(height).await {
            error!("Error claiming confirmed lockups: {e:?}");
        }
    }
}

impl ReceiveSwapHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        liquid_chain_service: Arc<dyn LiquidChainService>,
    ) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            subscription_notifier,
            liquid_chain_service,
        }
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Receive swaps
    pub(crate) async fn on_new_status(&self, update: &boltz::Update) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = RevSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid RevSwapState for Receive Swap {id}: {status}"))?;
        let receive_swap = self.fetch_receive_swap_by_id(id)?;

        info!("Handling Receive Swap transition to {swap_state:?} for swap {id}");

        match swap_state {
            RevSwapStates::SwapExpired
            | RevSwapStates::InvoiceExpired
            | RevSwapStates::TransactionFailed
            | RevSwapStates::TransactionRefunded => {
                match receive_swap.mrh_tx_id {
                    Some(mrh_tx_id) => {
                        warn!("Swap {id} is expired but MRH payment was received: txid {mrh_tx_id}")
                    }
                    None => {
                        error!("Swap {id} entered into an unrecoverable state: {swap_state:?}");
                        self.update_swap_info(id, Failed, None, None, None, None)?;
                    }
                }
                Ok(())
            }
            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // Execute 0-conf preconditions check
            RevSwapStates::TransactionMempool => {
                let Some(transaction) = update.transaction.clone() else {
                    return Err(anyhow!("Unexpected payload from Boltz status stream"));
                };

                if let Some(claim_tx_id) = receive_swap.claim_tx_id {
                    return Err(anyhow!(
                        "Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}"
                    ));
                }

                // Do not continue or claim the swap if it was already paid via MRH
                if let Some(mrh_tx_id) = receive_swap.mrh_tx_id {
                    return Err(anyhow!(
                        "MRH tx for Receive Swap {id} was already broadcast, ignoring swap: txid {mrh_tx_id}"
                    ));
                }

                // Looking for lockup script history to verify lockup was broadcasted
                let lockup_tx = match self
                    .verify_lockup_tx(&receive_swap, &transaction.id, &transaction.hex, false)
                    .await
                {
                    Ok(lockup_tx) => lockup_tx,
                    Err(e) => {
                        return Err(anyhow!(
                        "Swapper mempool reported lockup could not be verified. txid: {}, err: {}",
                        transaction.id,
                        e
                    ));
                    }
                };

                if let Err(e) = self
                    .verify_lockup_tx_amount(&receive_swap, &lockup_tx)
                    .await
                {
                    // The lockup amount in the tx is underpaid compared to the expected amount
                    self.update_swap_info(id, Failed, None, None, None, None)?;
                    return Err(anyhow!(
                        "Swapper underpaid lockup amount. txid: {}, err: {}",
                        transaction.id,
                        e
                    ));
                }
                info!("Swapper lockup was verified");

                let lockup_tx_id = &transaction.id;
                self.update_swap_info(id, Pending, None, Some(lockup_tx_id), None, None)?;

                // If the amount is greater than the zero-conf limit
                let max_amount_sat = self.config.zero_conf_max_amount_sat();
                let receiver_amount_sat = receive_swap.receiver_amount_sat;
                if receiver_amount_sat > max_amount_sat {
                    warn!("[Receive Swap {id}] Amount is too high to claim with zero-conf ({receiver_amount_sat} sat > {max_amount_sat} sat). Waiting for confirmation...");
                    return Ok(());
                }

                debug!("[Receive Swap {id}] Amount is within valid range for zero-conf ({receiver_amount_sat} < {max_amount_sat} sat)");

                // If the transaction has RBF, see https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki
                // TODO: Check for inherent RBF by ensuring all tx ancestors are confirmed
                let rbf_explicit = lockup_tx.input.iter().any(|input| input.sequence.is_rbf());
                // let rbf_inherent = lockup_tx_history.height < 0;

                if rbf_explicit {
                    warn!("[Receive Swap {id}] Lockup transaction signals RBF. Waiting for confirmation...");
                    return Ok(());
                }
                debug!("[Receive Swap {id}] Lockup tx does not signal RBF. Proceeding...");

                if receive_swap.metadata.is_local {
                    // Only claim a local swap
                    if let Err(err) = self.claim(id).await {
                        match err {
                            PaymentError::AlreadyClaimed => {
                                warn!("Funds already claimed for Receive Swap {id}")
                            }
                            _ => error!("Claim for Receive Swap {id} failed: {err}"),
                        }
                    }
                }

                Ok(())
            }
            RevSwapStates::TransactionConfirmed => {
                let Some(transaction) = update.transaction.clone() else {
                    return Err(anyhow!("Unexpected payload from Boltz status stream"));
                };

                // Do not continue or claim the swap if it was already paid via MRH
                if let Some(mrh_tx_id) = receive_swap.mrh_tx_id {
                    return Err(anyhow!(
                        "MRH tx for Receive Swap {id} was already broadcast, ignoring swap: txid {mrh_tx_id}"
                    ));
                }

                // Looking for lockup script history to verify lockup was broadcasted and confirmed
                let lockup_tx = match self
                    .verify_lockup_tx(&receive_swap, &transaction.id, &transaction.hex, true)
                    .await
                {
                    Ok(lockup_tx) => lockup_tx,
                    Err(e) => {
                        return Err(anyhow!(
                            "Swapper reported lockup could not be verified. txid: {}, err: {}",
                            transaction.id,
                            e
                        ));
                    }
                };

                if let Err(e) = self
                    .verify_lockup_tx_amount(&receive_swap, &lockup_tx)
                    .await
                {
                    // The lockup amount in the tx is underpaid compared to the expected amount
                    self.update_swap_info(id, Failed, None, None, None, None)?;
                    return Err(anyhow!(
                        "Swapper underpaid lockup amount. txid: {}, err: {}",
                        transaction.id,
                        e
                    ));
                }
                info!("Swapper lockup was verified, moving to claim");

                match receive_swap.claim_tx_id {
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                    None => {
                        self.update_swap_info(&receive_swap.id, Pending, None, None, None, None)?;

                        if receive_swap.metadata.is_local {
                            // Only claim a local swap
                            if let Err(err) = self.claim(id).await {
                                match err {
                                    PaymentError::AlreadyClaimed => {
                                        warn!("Funds already claimed for Receive Swap {id}")
                                    }
                                    _ => error!("Claim for Receive Swap {id} failed: {err}"),
                                }
                            }
                        }
                    }
                }
                Ok(())
            }

            _ => {
                debug!("Unhandled state for Receive Swap {id}: {swap_state:?}");
                Ok(())
            }
        }
    }

    fn fetch_receive_swap_by_id(&self, swap_id: &str) -> Result<ReceiveSwap, PaymentError> {
        self.persister
            .fetch_receive_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Receive Swap not found {swap_id}"),
            })
    }

    // Updates the swap without state transition validation
    pub(crate) fn update_swap(&self, updated_swap: ReceiveSwap) -> Result<(), PaymentError> {
        let swap = self.fetch_receive_swap_by_id(&updated_swap.id)?;
        if updated_swap != swap {
            info!(
                "Updating Receive swap {} to {:?} (claim_tx_id = {:?}, lockup_tx_id = {:?}, mrh_tx_id = {:?})",
                updated_swap.id, updated_swap.state, updated_swap.claim_tx_id, updated_swap.lockup_tx_id, updated_swap.mrh_tx_id
            );
            self.persister
                .insert_or_update_receive_swap(&updated_swap)?;
            let _ = self.subscription_notifier.send(updated_swap.id);
        }
        Ok(())
    }

    // Updates the swap state with validation
    pub(crate) fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
        lockup_tx_id: Option<&str>,
        mrh_tx_id: Option<&str>,
        mrh_amount_sat: Option<u64>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Receive swap {} to {:?} (claim_tx_id = {:?}, lockup_tx_id = {:?}, mrh_tx_id = {:?})",
            swap_id, to_state, claim_tx_id, lockup_tx_id, mrh_tx_id
        );
        let swap = self.fetch_receive_swap_by_id(swap_id)?;
        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_receive_swap_update(
            swap_id,
            to_state,
            claim_tx_id,
            lockup_tx_id,
            mrh_tx_id,
            mrh_amount_sat,
        )?;
        let updated_swap = self.fetch_receive_swap_by_id(swap_id)?;

        if mrh_tx_id.is_some() {
            self.persister.delete_reserved_address(&swap.mrh_address)?;
        }

        if updated_swap != swap {
            let _ = self.subscription_notifier.send(updated_swap.id);
        }
        Ok(())
    }

    async fn claim(&self, swap_id: &str) -> Result<(), PaymentError> {
        let swap = self.fetch_receive_swap_by_id(swap_id)?;
        ensure_sdk!(swap.claim_tx_id.is_none(), PaymentError::AlreadyClaimed);

        info!("Initiating claim for Receive Swap {swap_id}");
        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let crate::prelude::Transaction::Liquid(claim_tx) = self
            .swapper
            .create_claim_tx(Swap::Receive(swap.clone()), Some(claim_address))
            .await?
        else {
            return Err(PaymentError::Generic {
                err: format!("Constructed invalid transaction for Receive swap {swap_id}"),
            });
        };

        // Set the swap claim_tx_id before broadcasting.
        // If another claim_tx_id has been set in the meantime, don't broadcast the claim tx
        let tx_id = claim_tx.txid().to_hex();
        match self.persister.set_receive_swap_claim_tx_id(swap_id, &tx_id) {
            Ok(_) => {
                // We attempt broadcasting via chain service, then fallback to Boltz
                let broadcast_res = match self.liquid_chain_service.broadcast(&claim_tx).await {
                    Ok(tx_id) => Ok(tx_id.to_hex()),
                    Err(err) => {
                        debug!(
                            "Could not broadcast claim tx via chain service for Receive swap {swap_id}: {err:?}"
                        );
                        let claim_tx_hex = claim_tx.serialize().to_lower_hex_string();
                        self.swapper
                            .broadcast_tx(self.config.network.into(), &claim_tx_hex)
                            .await
                    }
                };
                match broadcast_res {
                    Ok(claim_tx_id) => {
                        // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(
                            PaymentTxData {
                                tx_id: claim_tx_id.clone(),
                                timestamp: Some(utils::now()),
                                asset_id: self.config.lbtc_asset_id(),
                                amount: swap.receiver_amount_sat,
                                fees_sat: 0,
                                payment_type: PaymentType::Receive,
                                is_confirmed: false,
                                unblinding_data: None,
                            },
                            None,
                            false,
                        )?;

                        info!("Successfully broadcast claim tx {claim_tx_id} for Receive Swap {swap_id}");
                        // The claim_tx_id is already set by set_receive_swap_claim_tx_id. Manually trigger notifying
                        // subscribers as update_swap_info will not recognise a change to the swap
                        _ = self.subscription_notifier.send(claim_tx_id);
                        Ok(())
                    }
                    Err(err) => {
                        // Multiple attempts to broadcast have failed. Unset the swap claim_tx_id
                        debug!(
                            "Could not broadcast claim tx via swapper for Receive swap {swap_id}: {err:?}"
                        );
                        self.persister
                            .unset_receive_swap_claim_tx_id(swap_id, &tx_id)?;
                        Err(err)
                    }
                }
            }
            Err(err) => {
                debug!(
                    "Failed to set claim_tx_id after creating tx for Receive swap {swap_id}: txid {tx_id}"
                );
                Err(err)
            }
        }
    }

    async fn claim_confirmed_lockups(&self, height: u32) -> Result<()> {
        let receive_swaps: Vec<ReceiveSwap> = self
            .persister
            .list_ongoing_receive_swaps(Some(true))?
            .into_iter()
            .filter(|s| s.lockup_tx_id.is_some() && s.claim_tx_id.is_none())
            .collect();
        info!(
            "Rescanning {} Receive Swap(s) lockup txs at height {}",
            receive_swaps.len(),
            height
        );
        for swap in receive_swaps {
            if let Err(e) = self.claim_confirmed_lockup(&swap).await {
                error!("Error rescanning Receive Swap {}: {e:?}", swap.id,);
            }
        }
        Ok(())
    }

    async fn claim_confirmed_lockup(&self, receive_swap: &ReceiveSwap) -> Result<()> {
        let Some(tx_id) = receive_swap.lockup_tx_id.clone() else {
            // Skip the rescan if there is no lockup_tx_id yet
            return Ok(());
        };
        let swap_id = &receive_swap.id;
        let tx_hex = self
            .liquid_chain_service
            .get_transaction_hex(&Txid::from_str(&tx_id)?)
            .await?
            .ok_or(anyhow!("Lockup tx not found for Receive swap {swap_id}"))?
            .serialize()
            .to_lower_hex_string();
        let lockup_tx = self
            .verify_lockup_tx(receive_swap, &tx_id, &tx_hex, true)
            .await?;
        if let Err(e) = self.verify_lockup_tx_amount(receive_swap, &lockup_tx).await {
            self.update_swap_info(swap_id, Failed, None, None, None, None)?;
            return Err(e);
        }
        info!("Receive Swap {swap_id} lockup tx is confirmed");
        self.claim(swap_id)
            .await
            .map_err(|e| anyhow!("Could not claim Receive Swap {swap_id}: {e:?}"))
    }

    fn validate_state_transition(
        from_state: PaymentState,
        to_state: PaymentState,
    ) -> Result<(), PaymentError> {
        match (from_state, to_state) {
            (_, Created) => Err(PaymentError::Generic {
                err: "Cannot transition to Created state".to_string(),
            }),

            (Created | Pending, Pending) => Ok(()),
            (_, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending, Complete) => Ok(()),
            (_, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created | TimedOut, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (_, Refundable) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Refundable state"),
            }),

            (_, RefundPending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to RefundPending state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),
            (_, Failed) => Ok(()),

            (_, WaitingFeeAcceptance) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to WaitingFeeAcceptance state"),
            }),
        }
    }

    async fn verify_lockup_tx(
        &self,
        receive_swap: &ReceiveSwap,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction> {
        // Looking for lockup script history to verify lockup was broadcasted
        let script = receive_swap.get_swap_script()?;
        let address = script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        self.liquid_chain_service
            .verify_tx(&address, tx_id, tx_hex, verify_confirmation)
            .await
    }

    async fn verify_lockup_tx_amount(
        &self,
        receive_swap: &ReceiveSwap,
        lockup_tx: &Transaction,
    ) -> Result<()> {
        let secp = Secp256k1::new();
        let script = receive_swap.get_swap_script()?;
        let address = script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let blinding_key = receive_swap
            .get_boltz_create_response()?
            .blinding_key
            .ok_or(anyhow!("Missing blinding key"))?;
        let tx_out = lockup_tx
            .output
            .iter()
            .find(|tx_out| tx_out.script_pubkey == address.script_pubkey())
            .ok_or(anyhow!("Failed to get tx output"))?;
        let lockup_amount_sat = tx_out
            .unblind(&secp, SecretKey::from_str(&blinding_key)?)
            .map(|o| o.value)?;
        let expected_lockup_amount_sat =
            receive_swap.receiver_amount_sat + receive_swap.claim_fees_sat;
        if lockup_amount_sat < expected_lockup_amount_sat {
            bail!(
                "Failed to verify lockup amount for Receive Swap {}: {} sat vs {} sat",
                receive_swap.id,
                expected_lockup_amount_sat,
                lockup_amount_sat
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use anyhow::Result;

    use crate::{
        model::PaymentState::{self, *},
        test_utils::{
            persist::{create_persister, new_receive_swap},
            receive_swap::new_receive_swap_handler,
        },
    };

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_receive_swap_state_transitions() -> Result<()> {
        create_persister!(persister);

        let receive_swap_state_handler = new_receive_swap_handler(persister.clone())?;

        // Test valid combinations of states
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Failed]),
            ),
            (Pending, HashSet::from([Pending, Complete, Failed])),
            (TimedOut, HashSet::from([TimedOut, Failed])),
            (Complete, HashSet::from([])),
            (Refundable, HashSet::from([Failed])),
            (RefundPending, HashSet::from([Failed])),
            (Failed, HashSet::from([Failed])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let receive_swap = new_receive_swap(Some(*first_state), None);
                persister.insert_or_update_receive_swap(&receive_swap)?;

                assert!(receive_swap_state_handler
                    .update_swap_info(&receive_swap.id, *allowed_state, None, None, None, None)
                    .is_ok());
            }
        }

        // Test invalid combinations of states
        let all_states = HashSet::from([Created, Pending, Complete, TimedOut, Failed]);
        let invalid_combinations: HashMap<PaymentState, HashSet<PaymentState>> = valid_combinations
            .iter()
            .map(|(first_state, allowed_states)| {
                (
                    *first_state,
                    all_states.difference(allowed_states).cloned().collect(),
                )
            })
            .collect();

        for (first_state, disallowed_states) in invalid_combinations.iter() {
            for disallowed_state in disallowed_states {
                let receive_swap = new_receive_swap(Some(*first_state), None);
                persister.insert_or_update_receive_swap(&receive_swap)?;

                assert!(receive_swap_state_handler
                    .update_swap_info(&receive_swap.id, *disallowed_state, None, None, None, None)
                    .is_err());
            }
        }

        Ok(())
    }
}
