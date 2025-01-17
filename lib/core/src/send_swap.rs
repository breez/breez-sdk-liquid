use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::swaps::boltz;
use boltz_client::swaps::{boltz::CreateSubmarineResponse, boltz::SubSwapStates};
use futures_util::TryFutureExt;
use log::{debug, error, info, warn};
use lwk_wollet::elements::{LockTime, Transaction};
use lwk_wollet::hashes::{sha256, Hash};
use sdk_common::prelude::{AesSuccessActionDataResult, SuccessAction, SuccessActionProcessed};
use tokio::sync::{broadcast, Mutex};

use crate::chain::liquid::LiquidChainService;
use crate::model::{
    BlockListener, Config, PaymentState::*, SendSwap, LIQUID_FEE_RATE_MSAT_PER_VBYTE,
};
use crate::persist::model::PaymentTxDetails;
use crate::prelude::{PaymentTxData, PaymentType, Swap};
use crate::recover::recoverer::Recoverer;
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

#[async_trait]
impl BlockListener for SendSwapHandler {
    async fn on_bitcoin_block(&self, _height: u32) {}

    async fn on_liquid_block(&self, _height: u32) {
        if let Err(err) = self.check_refunds().await {
            warn!("Could not refund expired swaps, error: {err:?}");
        }
    }
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
        let status = &update.status;
        let swap_state = SubSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid SubSwapState for Send Swap {id}: {status}"))?;
        let swap = self.fetch_send_swap_by_id(id)?;
        info!("Handling Send Swap transition to {swap_state:?} for swap {id}");

        // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
        match swap_state {
            // Boltz has locked the HTLC
            SubSwapStates::InvoiceSet => {
                warn!("Received `invoice.set` state for Send Swap {id}");
                Ok(())
            }

            // Boltz has detected the lockup in the mempool, we can speed up
            // the claim by doing so cooperatively
            SubSwapStates::TransactionClaimPending => {
                self.cooperate_claim(&swap).await.map_err(|e| {
                    error!("Could not cooperate Send Swap {id} claim: {e}");
                    anyhow!("Could not post claim details. Err: {e:?}")
                })?;

                Ok(())
            }

            // Boltz announced they successfully broadcast the (cooperative or non-cooperative) claim tx
            SubSwapStates::TransactionClaimed => {
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
                        self.update_swap_info(id, Complete, Some(&preimage), None, None)?;
                    }
                }

                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. Boltz failed to pay
            // 2. The swap has expired (>24h)
            // 3. Lockup failed (we sent too little funds)
            // We initiate a cooperative refund, and then fallback to a regular one
            SubSwapStates::TransactionLockupFailed
            | SubSwapStates::InvoiceFailedToPay
            | SubSwapStates::SwapExpired => {
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
                            )?;
                        }
                    },
                    // Do not attempt broadcasting a refund if lockup tx was never sent and swap is
                    // unrecoverable. We resolve the payment as failed.
                    None => {
                        warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has never been broadcast. Resolving payment as failed.");
                        self.update_swap_info(id, Failed, None, None, None)?;
                    }
                }

                Ok(())
            }

            _ => {
                debug!("Unhandled state for Send Swap {id}: {swap_state:?}");
                Ok(())
            }
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
            .build_tx_or_drain_tx(
                Some(LIQUID_FEE_RATE_MSAT_PER_VBYTE),
                &create_response.address,
                &self.config.lbtc_asset_id(),
                create_response.expected_amount,
            )
            .await?;
        let lockup_tx_id = lockup_tx.txid().to_string();

        self.persister
            .set_send_swap_lockup_tx_id(swap_id, &lockup_tx_id)?;

        info!("Broadcasting lockup tx {lockup_tx_id} for Send swap {swap_id}",);

        let broadcast_result = self.chain_service.lock().await.broadcast(&lockup_tx).await;

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
                asset_id: self.config.lbtc_asset_id(),
                amount: create_response.expected_amount,
                fees_sat: lockup_tx_fees_sat,
                payment_type: PaymentType::Send,
                is_confirmed: false,
                unblinding_data: None,
            },
            None,
            false,
        )?;

        self.update_swap_info(swap_id, Pending, None, Some(&lockup_tx_id), None)?;

        Ok(lockup_tx)
    }

    fn fetch_send_swap_by_id(&self, swap_id: &str) -> Result<SendSwap, PaymentError> {
        self.persister
            .fetch_send_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Send Swap not found {swap_id}"),
            })
    }

    // Updates the swap without state transition validation
    pub(crate) fn update_swap(&self, updated_swap: SendSwap) -> Result<(), PaymentError> {
        let swap = self.fetch_send_swap_by_id(&updated_swap.id)?;
        let lnurl_info_updated = self.update_swap_lnurl_info(&swap, &updated_swap)?;
        if updated_swap != swap || lnurl_info_updated {
            info!(
                "Updating Send swap {} to {:?} (lockup_tx_id = {:?}, refund_tx_id = {:?})",
                updated_swap.id,
                updated_swap.state,
                updated_swap.lockup_tx_id,
                updated_swap.refund_tx_id
            );
            self.persister.insert_or_update_send_swap(&updated_swap)?;
            let _ = self.subscription_notifier.send(updated_swap.id);
        }
        Ok(())
    }

    pub(crate) fn update_swap_lnurl_info(
        &self,
        swap: &SendSwap,
        updated_swap: &SendSwap,
    ) -> Result<bool> {
        if swap.preimage.is_none() {
            let Some(tx_id) = updated_swap.lockup_tx_id.clone() else {
                return Ok(false);
            };
            let Some(ref preimage_str) = updated_swap.preimage.clone() else {
                return Ok(false);
            };
            if let Some(PaymentTxDetails {
                destination,
                description,
                lnurl_info: Some(mut lnurl_info),
                ..
            }) = self.persister.get_payment_details(&tx_id)?
            {
                if let Some(SuccessAction::Aes { data }) =
                    lnurl_info.lnurl_pay_unprocessed_success_action.clone()
                {
                    debug!(
                        "Decrypting AES success action with preimage for Send Swap {}",
                        swap.id
                    );
                    let preimage = sha256::Hash::from_str(preimage_str)?;
                    let preimage_arr = preimage.to_byte_array();
                    let result = match (data, &preimage_arr).try_into() {
                        Ok(data) => AesSuccessActionDataResult::Decrypted { data },
                        Err(e) => AesSuccessActionDataResult::ErrorStatus {
                            reason: e.to_string(),
                        },
                    };
                    lnurl_info.lnurl_pay_success_action =
                        Some(SuccessActionProcessed::Aes { result });
                    self.persister
                        .insert_or_update_payment_details(PaymentTxDetails {
                            tx_id,
                            destination,
                            description,
                            lnurl_info: Some(lnurl_info),
                        })?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    // Updates the swap state with validation
    pub(crate) fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        preimage: Option<&str>,
        lockup_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Send swap {} to {:?} (lockup_tx_id = {:?}, refund_tx_id = {:?})",
            swap_id, to_state, lockup_tx_id, refund_tx_id
        );
        let swap = self.fetch_send_swap_by_id(swap_id)?;
        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_send_swap_update(
            swap_id,
            to_state,
            preimage,
            lockup_tx_id,
            refund_tx_id,
        )?;
        let updated_swap = self.fetch_send_swap_by_id(swap_id)?;
        let lnurl_info_updated = self.update_swap_lnurl_info(&swap, &updated_swap)?;
        if updated_swap != swap || lnurl_info_updated {
            let _ = self.subscription_notifier.send(updated_swap.id);
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
        )?;
        self.swapper
            .claim_send_swap_cooperative(send_swap, claim_tx_details, &output_address)?;
        Ok(())
    }

    pub(crate) async fn get_preimage_from_script_path_claim_spend(
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
                let claim_tx = self
                    .chain_service
                    .lock()
                    .await
                    .get_transactions(&[claim_tx_id])
                    .await
                    .map_err(|e| anyhow!("Failed to fetch claim txs {claim_tx_id:?}: {e}"))?
                    .first()
                    .cloned()
                    .ok_or(anyhow!("Claim tx not found for Send swap {id}"))?;

                Ok(Recoverer::get_send_swap_preimage_from_claim_tx(
                    id, &claim_tx,
                )?)
            }
        }
    }

    async fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        utils::verify_payment_hash(preimage, invoice)?;
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
            .broadcast(&refund_tx)
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
                let update_swap_info_result =
                    self.update_swap_info(&swap.id, RefundPending, None, None, Some(&refund_tx_id));
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
    pub(crate) async fn check_refunds(&self) -> Result<(), PaymentError> {
        let pending_swaps = self.persister.list_pending_send_swaps()?;
        self.try_refund_all(&pending_swaps).await;
        Ok(())
    }

    fn validate_state_transition(
        from_state: PaymentState,
        to_state: PaymentState,
    ) -> Result<(), PaymentError> {
        match (from_state, to_state) {
            (TimedOut, Created) => Ok(()),
            (_, Created) => Err(PaymentError::Generic {
                err: "Cannot transition from {from_state:?} to Created state".to_string(),
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

            (_, WaitingFeeAcceptance) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to WaitingFeeAcceptance state"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use anyhow::Result;

    use crate::{
        model::PaymentState::{self, *},
        test_utils::{
            persist::{create_persister, new_send_swap},
            send_swap::new_send_swap_handler,
        },
    };

    #[tokio::test]
    async fn test_send_swap_state_transitions() -> Result<()> {
        create_persister!(storage);
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
            (TimedOut, HashSet::from([TimedOut, Created, Failed])),
            (Complete, HashSet::from([])),
            (Refundable, HashSet::from([Failed])),
            (Failed, HashSet::from([Failed])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let send_swap = new_send_swap(Some(*first_state));
                storage.insert_or_update_send_swap(&send_swap)?;

                assert!(send_swap_handler
                    .update_swap_info(&send_swap.id, *allowed_state, None, None, None)
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
                storage.insert_or_update_send_swap(&send_swap)?;

                assert!(send_swap_handler
                    .update_swap_info(&send_swap.id, *disallowed_state, None, None, None)
                    .is_err());
            }
        }

        Ok(())
    }
}
