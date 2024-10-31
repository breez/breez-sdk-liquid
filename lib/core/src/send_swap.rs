use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz;
use boltz_client::swaps::{boltz::CreateSubmarineResponse, boltz::SubSwapStates};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Bolt11Invoice, ToHex};
use futures_util::TryFutureExt;
use log::{debug, error, info, warn};
use lwk_wollet::bitcoin::Witness;
use lwk_wollet::elements::{LockTime, Transaction};
use lwk_wollet::hashes::{sha256, Hash};
use tokio::sync::{broadcast, Mutex};

use crate::chain::liquid::LiquidChainService;
use crate::model::{Config, PaymentState::*, SendSwap};
use crate::prelude::{PaymentTxData, PaymentType, Swap};
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{ensure_sdk, utils};
use crate::{
    error::PaymentError,
    model::{PaymentState, Transaction as SdkTransaction},
    persist::Persister,
};

#[derive(Clone)]
pub(crate) struct SendSwapHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    chain_service: Arc<Mutex<dyn LiquidChainService>>,
    subscription_notifier: broadcast::Sender<String>,
}

impl SendSwapHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        chain_service: Arc<Mutex<dyn LiquidChainService>>,
    ) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            chain_service,
            subscription_notifier,
        }
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Send swaps
    pub(crate) async fn on_new_status(&self, update: &boltz::Update) -> Result<()> {
        let id = &update.id;
        let swap_state = &update.status;
        let swap = self
            .persister
            .fetch_send_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Send Swap found for ID {id}"))?;

        info!("Handling Send Swap transition to {swap_state:?} for swap {id}");

        // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
        match SubSwapStates::from_str(swap_state) {
            // Boltz has locked the HTLC
            Ok(SubSwapStates::InvoiceSet) => {
                warn!("Received `invoice.set` state for Send Swap {id}");
                Ok(())
            }

            // Boltz has detected the lockup in the mempool, we can speed up
            // the claim by doing so cooperatively
            Ok(SubSwapStates::TransactionClaimPending) => {
                self.cooperate_claim(&swap).await.map_err(|e| {
                    error!("Could not cooperate Send Swap {id} claim: {e}");
                    anyhow!("Could not post claim details. Err: {e:?}")
                })?;

                Ok(())
            }

            // Boltz announced they successfully broadcast the (cooperative or non-cooperative) claim tx
            Ok(SubSwapStates::TransactionClaimed) => {
                debug!("Send Swap {id} has been claimed");

                match swap.preimage {
                    Some(_) => {
                        debug!("The claim tx was a key path spend (cooperative claim)");
                        // Preimage was already validated and stored, PaymentSucceeded event emitted,
                        // when the cooperative claim was handled.
                    }
                    None => {
                        debug!("The claim tx was a script path spend (non-cooperative claim)");
                        let preimage = self
                            .get_preimage_from_script_path_claim_spend(&swap)
                            .await?;
                        self.validate_send_swap_preimage(id, &swap.invoice, &preimage)
                            .await?;
                        self.update_swap_info(id, Complete, Some(&preimage), None, None)
                            .await?;
                    }
                }

                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. Boltz failed to pay
            // 2. The swap has expired (>24h)
            // 3. Lockup failed (we sent too little funds)
            // We initiate a cooperative refund, and then fallback to a regular one
            Ok(
                SubSwapStates::TransactionLockupFailed
                | SubSwapStates::InvoiceFailedToPay
                | SubSwapStates::SwapExpired,
            ) => {
                match swap.lockup_tx_id {
                    Some(_) => match swap.refund_tx_id {
                        Some(refund_tx_id) => warn!(
                        "Refund tx for Send Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                        None => {
                            warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has been broadcast.");
                            let refund_tx_id = match self.refund(&swap, true).await {
                                Ok(refund_tx_id) => Some(refund_tx_id),
                                Err(e) => {
                                    warn!("Could not refund Send swap {id} cooperatively: {e:?}");
                                    None
                                }
                            };
                            // Set the payment state to `RefundPending`. This ensures that the
                            // background thread will pick it up and try to refund it
                            // periodically
                            self.update_swap_info(
                                &swap.id,
                                RefundPending,
                                None,
                                None,
                                refund_tx_id.as_deref(),
                            )
                            .await?;
                        }
                    },
                    // Do not attempt broadcasting a refund if lockup tx was never sent and swap is
                    // unrecoverable. We resolve the payment as failed.
                    None => {
                        warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has never been broadcast. Resolving payment as failed.");
                        self.update_swap_info(id, Failed, None, None, None).await?;
                    }
                }

                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Send Swap {id}: {swap_state}");
                Ok(())
            }

            _ => Err(anyhow!(
                "Invalid SubSwapState for Send Swap {id}: {swap_state}"
            )),
        }
    }

    pub(crate) async fn try_lockup(
        &self,
        swap: &SendSwap,
        create_response: &CreateSubmarineResponse,
    ) -> Result<Transaction, PaymentError> {
        if swap.lockup_tx_id.is_some() {
            debug!("Lockup tx was already broadcast for Send Swap {}", swap.id);
            return Err(PaymentError::PaymentInProgress);
        }

        let swap_id = &swap.id;
        debug!(
            "Initiated Send Swap: send {} sats to liquid address {}",
            create_response.expected_amount, create_response.address
        );

        let lockup_tx = self
            .onchain_wallet
            .build_tx(
                self.config
                    .lowball_fee_rate_msat_per_vbyte()
                    .map(|v| v as f32),
                &create_response.address,
                create_response.expected_amount,
            )
            .await?;
        let lockup_tx_id = lockup_tx.txid().to_string();

        self.persister
            .set_send_swap_lockup_tx_id(swap_id, &lockup_tx_id)?;

        info!("Broadcasting lockup tx {lockup_tx_id} for Send swap {swap_id}",);

        let broadcast_result = self
            .chain_service
            .lock()
            .await
            .broadcast(&lockup_tx, Some(swap_id))
            .await;

        if let Err(err) = broadcast_result {
            debug!("Could not broadcast lockup tx for Send Swap {swap_id}: {err:?}");
            self.persister
                .unset_send_swap_lockup_tx_id(swap_id, &lockup_tx_id)?;
            return Err(err.into());
        }

        info!("Successfully broadcast lockup tx for Send Swap {swap_id}. Lockup tx id: {lockup_tx_id}");

        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        let lockup_tx_fees_sat: u64 = lockup_tx.all_fees().values().sum();
        self.persister.insert_or_update_payment(
            PaymentTxData {
                tx_id: lockup_tx_id.clone(),
                timestamp: Some(utils::now()),
                amount_sat: swap.payer_amount_sat,
                fees_sat: lockup_tx_fees_sat,
                payment_type: PaymentType::Send,
                is_confirmed: false,
            },
            None,
            None,
        )?;

        self.update_swap_info(swap_id, Pending, None, Some(&lockup_tx_id), None)
            .await?;

        Ok(lockup_tx)
    }

    /// Transitions a Send swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        preimage: Option<&str>,
        lockup_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!("Transitioning Send swap {swap_id} to {to_state:?} (lockup_tx_id = {lockup_tx_id:?}, refund_tx_id = {refund_tx_id:?})");

        let swap: SendSwap = self
            .persister
            .fetch_send_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Send Swap not found {swap_id}"),
            })?;
        let payment_id = lockup_tx_id.map(|c| c.to_string()).or(swap.lockup_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_send_swap_update(
            swap_id,
            to_state,
            preimage,
            lockup_tx_id,
            refund_tx_id,
        )?;
        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn cooperate_claim(&self, send_swap: &SendSwap) -> Result<(), PaymentError> {
        debug!(
            "Claim is pending for Send Swap {}. Initiating cooperative claim",
            &send_swap.id
        );
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let claim_tx_details = self.swapper.get_send_claim_tx_details(send_swap)?;
        self.update_swap_info(
            &send_swap.id,
            Complete,
            Some(&claim_tx_details.preimage),
            None,
            None,
        )
        .await?;
        self.swapper
            .claim_send_swap_cooperative(send_swap, claim_tx_details, &output_address)?;
        Ok(())
    }

    async fn get_preimage_from_script_path_claim_spend(
        &self,
        swap: &SendSwap,
    ) -> Result<String, PaymentError> {
        info!("Retrieving preimage from non-cooperative claim tx");

        let id = &swap.id;
        let swap_script = swap.get_swap_script()?;
        let swap_script_pk = swap_script
            .to_address(self.config.network.into())?
            .script_pubkey();
        debug!("Found Send Swap swap_script_pk: {swap_script_pk:?}");

        // Get tx history of the swap script (lockup address)
        let history: Vec<_> = self
            .chain_service
            .lock()
            .await
            .get_script_history(&swap_script_pk)
            .await?;

        // We expect at most 2 txs: lockup and maybe the claim
        ensure_sdk!(
            history.len() <= 2,
            PaymentError::Generic {
                err: "Lockup address history for Send Swap {id} has more than 2 txs".to_string()
            }
        );

        match history.get(1) {
            None => Err(PaymentError::Generic {
                err: format!("Send Swap {id} has no claim tx"),
            }),
            Some(claim_tx_entry) => {
                let claim_tx_id = claim_tx_entry.txid;
                debug!("Send Swap {id} has claim tx {claim_tx_id}");

                let claim_tx = self
                    .chain_service
                    .lock()
                    .await
                    .get_transactions(&[claim_tx_id])
                    .await
                    .map_err(|e| anyhow!("Failed to fetch claim tx {claim_tx_id}: {e}"))?
                    .first()
                    .cloned()
                    .ok_or_else(|| anyhow!("Fetching claim tx returned an empty list"))?;

                let input = claim_tx
                    .input
                    .first()
                    .ok_or_else(|| anyhow!("Found no input for claim tx"))?;

                let script_witness_bytes = input.clone().witness.script_witness;
                info!("Found Send Swap {id} claim tx witness: {script_witness_bytes:?}");
                let script_witness = Witness::from(script_witness_bytes);

                let preimage_bytes = script_witness
                    .nth(1)
                    .ok_or_else(|| anyhow!("Claim tx witness has no preimage"))?;
                let preimage = sha256::Hash::from_slice(preimage_bytes)
                    .map_err(|e| anyhow!("Claim tx witness has invalid preimage: {e}"))?;
                let preimage_hex = preimage.to_hex();
                debug!("Found Send Swap {id} claim tx preimage: {preimage_hex}");

                Ok(preimage_hex)
            }
        }
    }

    async fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    pub(crate) async fn refund(
        &self,
        swap: &SendSwap,
        is_cooperative: bool,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating refund for Send Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        let swap_script = swap.get_swap_script()?;
        let refund_address = self.onchain_wallet.next_unused_address().await?.to_string();

        let liquid_chain_service = self.chain_service.lock().await;
        let script_pk = swap_script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Could not retrieve address from swap script: {e:?}"))?
            .to_unconfidential()
            .script_pubkey();
        let utxos = liquid_chain_service.get_script_utxos(&script_pk).await?;
        let SdkTransaction::Liquid(refund_tx) = self.swapper.create_refund_tx(
            Swap::Send(swap.clone()),
            &refund_address,
            utxos,
            None,
            is_cooperative,
        )?
        else {
            return Err(PaymentError::Generic {
                err: format!(
                    "Unexpected refund tx type returned for Send swap {}",
                    swap.id
                ),
            });
        };
        let refund_tx_id = liquid_chain_service
            .broadcast(&refund_tx, Some(&swap.id))
            .await?
            .to_string();

        info!(
            "Successfully broadcast refund for Send Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        Ok(refund_tx_id)
    }

    async fn check_swap_expiry(&self, swap: &SendSwap) -> Result<bool> {
        let swap_creation_time = UNIX_EPOCH + Duration::from_secs(swap.created_at as u64);
        let duration_since_creation_time = SystemTime::now().duration_since(swap_creation_time)?;
        if duration_since_creation_time.as_secs() < 60 * 10 {
            return Ok(false);
        }

        let swap_script = swap.get_swap_script()?;
        let current_height = self.onchain_wallet.tip().await.height();
        let locktime_from_height = LockTime::from_height(current_height)?;

        info!("Checking Send Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", swap.id, swap_script.locktime);
        Ok(utils::is_locktime_expired(
            locktime_from_height,
            swap_script.locktime,
        ))
    }

    // Attempts both cooperative and non-cooperative refunds, and updates the swap info accordingly
    pub(crate) async fn try_refund_all(&self, swaps: &[SendSwap]) {
        for swap in swaps {
            if swap.refund_tx_id.is_some() {
                continue;
            }

            let has_swap_expired = self.check_swap_expiry(swap).await.unwrap_or(false);

            if !has_swap_expired && swap.state == Pending {
                continue;
            }

            let refund_tx_id_result = match swap.state {
                Pending => self.refund(swap, false).await,
                RefundPending => match has_swap_expired {
                    true => {
                        self.refund(swap, true)
                            .or_else(|e| {
                                warn!("Failed to initiate cooperative refund, switching to non-cooperative: {e:?}");
                                self.refund(swap, false)
                            })
                            .await
                    }
                    false => self.refund(swap, true).await,
                },
                _ => {
                    continue;
                }
            };

            if let Ok(refund_tx_id) = refund_tx_id_result {
                let update_swap_info_result = self
                    .update_swap_info(&swap.id, RefundPending, None, None, Some(&refund_tx_id))
                    .await;
                if let Err(err) = update_swap_info_result {
                    warn!(
                        "Could not update Send swap {} information, error: {err:?}",
                        swap.id
                    );
                };
            }
        }
    }

    // Attempts refunding all payments whose state is `RefundPending` and with no
    // refund_tx_id field present
    pub(crate) async fn track_refunds(&self) -> Result<(), PaymentError> {
        let pending_swaps = self.persister.list_pending_send_swaps()?;
        self.try_refund_all(&pending_swaps).await;
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

            (Pending, RefundPending) => Ok(()),
            (_, RefundPending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to RefundPending state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),
            (_, Failed) => Ok(()),
        }
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice = Bolt11Invoice::from_str(invoice)
            .map_err(|err| PaymentError::invalid_invoice(&err.to_string()))?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
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
            persist::{new_persister, new_send_swap},
            send_swap::new_send_swap_handler,
        },
    };

    #[tokio::test]
    async fn test_send_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);
        let send_swap_handler = new_send_swap_handler(storage.clone())?;

        // Test valid combinations of states
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Failed]),
            ),
            (
                Pending,
                HashSet::from([Pending, RefundPending, Complete, Failed]),
            ),
            (TimedOut, HashSet::from([TimedOut, Failed])),
            (Complete, HashSet::from([])),
            (Refundable, HashSet::from([Failed])),
            (Failed, HashSet::from([Failed])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let send_swap = new_send_swap(Some(*first_state));
                storage.insert_send_swap(&send_swap)?;

                assert!(send_swap_handler
                    .update_swap_info(&send_swap.id, *allowed_state, None, None, None)
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
                let send_swap = new_send_swap(Some(*first_state));
                storage.insert_send_swap(&send_swap)?;

                assert!(send_swap_handler
                    .update_swap_info(&send_swap.id, *disallowed_state, None, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
