use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz::RevSwapStates;
use boltz_client::swaps::boltz::{self, SwapUpdateTxDetails};
use boltz_client::{Serialize, ToHex};
use log::{debug, error, info, warn};
use lwk_wollet::hashes::hex::DisplayHex;
use tokio::sync::{broadcast, Mutex};

use crate::chain::liquid::LiquidChainService;
use crate::model::PaymentState::{
    Complete, Created, Failed, Pending, RefundPending, Refundable, TimedOut,
};
use crate::model::{Config, PaymentTxData, PaymentType, ReceiveSwap};
use crate::prelude::Transaction;
use crate::{ensure_sdk, utils};
use crate::{
    error::PaymentError, model::PaymentState, persist::Persister, swapper::Swapper,
    wallet::OnchainWallet,
};

/// The minimum acceptable fee rate when claiming using zero-conf
pub const DEFAULT_ZERO_CONF_MIN_FEE_RATE_TESTNET: u32 = 100;
pub const DEFAULT_ZERO_CONF_MIN_FEE_RATE_MAINNET: u32 = 10;
/// The maximum acceptable amount in satoshi when claiming using zero-conf
pub const DEFAULT_ZERO_CONF_MAX_SAT: u64 = 100_000;

pub(crate) struct ReceiveSwapHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    subscription_notifier: broadcast::Sender<String>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
}

impl ReceiveSwapHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
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
        let swap_state = &update.status;
        let receive_swap = self
            .persister
            .fetch_receive_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Receive Swap found for ID {id}"))?;

        info!("Handling Receive Swap transition to {swap_state:?} for swap {id}");

        match RevSwapStates::from_str(swap_state) {
            Ok(
                RevSwapStates::SwapExpired
                | RevSwapStates::InvoiceExpired
                | RevSwapStates::TransactionFailed
                | RevSwapStates::TransactionRefunded,
            ) => {
                error!("Swap {id} entered into an unrecoverable state: {swap_state:?}");
                self.update_swap_info(id, Failed, None, None).await?;
                Ok(())
            }
            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // Execute 0-conf preconditions check
            Ok(RevSwapStates::TransactionMempool) => {
                let Some(transaction) = update.transaction.clone() else {
                    return Err(anyhow!("Unexpected payload from Boltz status stream"));
                };

                if let Some(claim_tx_id) = receive_swap.claim_tx_id {
                    return Err(anyhow!(
                        "Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}"
                    ));
                }

                // looking for lockup script history to verify lockup was broadcasted
                if let Err(e) = self
                    .verify_lockup_tx(&receive_swap, &transaction, false)
                    .await
                {
                    return Err(anyhow!(
                        "swapper mempool reported lockup could not be verified. txid: {}, err: {}",
                        transaction.id,
                        e
                    ));
                }
                info!("swapper lockup was verified");

                let lockup_tx_id = &transaction.id;
                self.update_swap_info(id, Pending, None, Some(lockup_tx_id))
                    .await?;

                let lockup_tx = utils::deserialize_tx_hex(&transaction.hex)?;

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

                // If the fees are higher than our estimated value
                let tx_fees: u64 = lockup_tx.all_fees().values().sum();
                let min_fee_rate = self.config.zero_conf_min_fee_rate_msat as f32 / 1000.0;
                let lower_bound_estimated_fees = lockup_tx.vsize() as f32 * min_fee_rate * 0.8;

                if lower_bound_estimated_fees > tx_fees as f32 {
                    warn!("[Receive Swap {id}] Lockup tx fees are too low: Expected at least {lower_bound_estimated_fees} sat, got {tx_fees} sat. Waiting for confirmation...");
                    return Ok(());
                }

                debug!("[Receive Swap {id}] Lockup tx fees are within acceptable range ({tx_fees} > {lower_bound_estimated_fees} sat). Proceeding with claim.");

                match self.claim(&receive_swap).await {
                    Ok(_) => {}
                    Err(err) => match err {
                        PaymentError::AlreadyClaimed => {
                            warn!("Funds already claimed for Receive Swap {id}")
                        }
                        _ => error!("Claim for Receive Swap {id} failed: {err}"),
                    },
                }

                Ok(())
            }
            Ok(RevSwapStates::TransactionConfirmed) => {
                let Some(transaction) = update.transaction.clone() else {
                    return Err(anyhow!("Unexpected payload from Boltz status stream"));
                };

                // looking for lockup script history to verify lockup was broadcasted and confirmed
                if let Err(e) = self
                    .verify_lockup_tx(&receive_swap, &transaction, true)
                    .await
                {
                    return Err(anyhow!(
                        "swapper reported lockup could not be verified. txid: {}, err: {}",
                        transaction.id,
                        e
                    ));
                }
                info!("swapper lockup was verified, moving to claim");

                match receive_swap.claim_tx_id {
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                    None => {
                        self.update_swap_info(&receive_swap.id, Pending, None, None)
                            .await?;
                        match self.claim(&receive_swap).await {
                            Ok(_) => {}
                            Err(err) => match err {
                                PaymentError::AlreadyClaimed => {
                                    warn!("Funds already claimed for Receive Swap {id}")
                                }
                                _ => error!("Claim for Receive Swap {id} failed: {err}"),
                            },
                        }
                    }
                }
                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Receive Swap {id}: {swap_state}");
                Ok(())
            }

            _ => Err(anyhow!(
                "Invalid RevSwapState for Receive Swap {id}: {swap_state}"
            )),
        }
    }

    /// Transitions a Receive swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
        lockup_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Receive swap {swap_id} to {to_state:?} (claim_tx_id = {claim_tx_id:?}, lockup_tx_id = {lockup_tx_id:?})"
        );

        let swap = self
            .persister
            .fetch_receive_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Receive Swap not found {swap_id}"),
            })?;
        let payment_id = claim_tx_id
            .or(lockup_tx_id)
            .map(|id| id.to_string())
            .or(swap.claim_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_receive_swap_update(
            swap_id,
            to_state,
            claim_tx_id,
            lockup_tx_id,
        )?;

        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn claim(&self, swap: &ReceiveSwap) -> Result<(), PaymentError> {
        ensure_sdk!(swap.claim_tx_id.is_none(), PaymentError::AlreadyClaimed);

        let swap_id = &swap.id;

        info!("Initiating claim for Receive Swap {swap_id}");

        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let Transaction::Liquid(claim_tx) =
            self.swapper.new_receive_claim_tx(swap, claim_address)?
        else {
            return Err(PaymentError::Generic {
                err: format!("Constructed invalid transaction for Receive swap {swap_id}"),
            });
        };

        // We attempt broadcasting via chain service, then fallback to Boltz
        let liquid_chain_service = self.liquid_chain_service.lock().await;
        let broadcast_response = liquid_chain_service
            .broadcast(&claim_tx, Some(&swap.id))
            .await;
        let claim_tx_id = match broadcast_response {
            Ok(tx_id) => tx_id.to_hex(),
            Err(err) => {
                debug!(
                    "Could not broadcast claim tx via chain service for Receive swap {swap_id}: {err:?}"
                );
                let claim_tx_hex = claim_tx.serialize().to_lower_hex_string();
                self.swapper
                    .broadcast_tx(self.config.network.into(), &claim_tx_hex)?
            }
        };

        // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        self.persister.insert_or_update_payment(
            PaymentTxData {
                tx_id: claim_tx_id.clone(),
                timestamp: Some(utils::now()),
                amount_sat: swap.receiver_amount_sat,
                fees_sat: 0,
                payment_type: PaymentType::Receive,
                is_confirmed: false,
            },
            None,
            None,
        )?;

        info!("Successfully broadcast claim tx {claim_tx_id} for Receive Swap {swap_id}");

        self.update_swap_info(swap_id, Pending, Some(&claim_tx_id), None)
            .await?;

        Ok(())
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
        }
    }

    async fn verify_lockup_tx(
        &self,
        receive_swap: &ReceiveSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        // Looking for lockup script history to verify lockup was broadcasted
        let script = receive_swap.get_swap_script()?;
        let address =
            script
                .to_address(self.config.network.into())
                .map_err(|e| PaymentError::Generic {
                    err: format!("Failed to get swap script address {e:?}"),
                })?;
        self.liquid_chain_service
            .lock()
            .await
            .verify_tx(
                &address,
                &swap_update_tx.id,
                &swap_update_tx.hex,
                verify_confirmation,
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    };

    use anyhow::Result;

    use crate::{
        model::PaymentState::{self, *},
        test_utils::{
            persist::{new_persister, new_receive_swap},
            receive_swap::new_receive_swap_handler,
        },
    };

    #[tokio::test]
    async fn test_receive_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);

        let receive_swap_state_handler = new_receive_swap_handler(storage.clone())?;

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
                let receive_swap = new_receive_swap(Some(*first_state));
                storage.insert_receive_swap(&receive_swap)?;

                assert!(receive_swap_state_handler
                    .update_swap_info(&receive_swap.id, *allowed_state, None, None)
                    .await
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
                let receive_swap = new_receive_swap(Some(*first_state));
                storage.insert_receive_swap(&receive_swap)?;

                assert!(receive_swap_state_handler
                    .update_swap_info(&receive_swap.id, *disallowed_state, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
