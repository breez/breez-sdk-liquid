use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltzv2;
use boltz_client::swaps::{boltz::SubSwapStates, boltzv2::CreateSubmarineResponse};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Amount, Bolt11Invoice, ToHex};
use log::{debug, error, info, warn};
use lwk_wollet::bitcoin::Witness;
use lwk_wollet::elements::Transaction;
use lwk_wollet::hashes::{sha256, Hash};
use tokio::sync::broadcast;

use crate::model::PaymentState::{Complete, Created, Failed, Pending, TimedOut};
use crate::model::{Config, SendSwap};
use crate::sdk::ChainService;
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{ensure_sdk, get_invoice_amount};
use crate::{
    error::PaymentError,
    model::{PaymentState, PaymentTxData, PaymentType},
    persist::Persister,
};
#[derive(Clone)]
pub(crate) struct SendSwapStateHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    chain_service: Arc<dyn ChainService>,
    subscription_notifier: broadcast::Sender<String>,
}

impl SendSwapStateHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        chain_service: Arc<dyn ChainService>,
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
    pub(crate) async fn on_new_status(&self, update: &boltzv2::Update) -> Result<()> {
        let id = update.id();
        let swap_state = update.status();
        let swap = self
            .persister
            .fetch_send_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Send Swap found for ID {id}"))?;

        info!("Handling Send Swap transition to {swap_state:?} for swap {id}");

        // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
        match SubSwapStates::from_str(swap_state) {
            // Boltz has locked the HTLC, we proceed with locking up the funds
            Ok(SubSwapStates::InvoiceSet) => {
                match (swap.state, swap.lockup_tx_id.clone()) {
                    (PaymentState::Created, None) | (PaymentState::TimedOut, None) => {
                        let create_response = swap.get_boltz_create_response()?;
                        let lockup_tx = self.lockup_funds(id, &create_response).await?;
                        let lockup_tx_id = lockup_tx.txid().to_string();
                        let lockup_tx_fees_sat: u64 = lockup_tx.all_fees().values().sum();

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: None,
                            amount_sat: swap.payer_amount_sat,
                            fees_sat: lockup_tx_fees_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        })?;

                        self.update_swap_info(id, Pending, None, Some(&lockup_tx_id), None)
                            .await?;
                    }
                    (_, Some(lockup_tx_id)) => {
                        warn!("Lockup tx for Send Swap {id} was already broadcast: txid {lockup_tx_id}")
                    }
                    (state, _) => {
                        debug!("Send Swap {id} is in an invalid state for {swap_state}: {state:?}")
                    }
                }
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
                        let preimage = self.get_preimage_from_script_path_claim_spend(&swap)?;
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
                            warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has been broadcast. Attempting refund.");

                            let refund_tx_id = self.refund(&swap).await?;
                            info!("Broadcast refund tx for Send Swap {id}. Tx id: {refund_tx_id}");
                            self.update_swap_info(id, Pending, None, None, Some(&refund_tx_id))
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

    async fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateSubmarineResponse,
    ) -> Result<Transaction, PaymentError> {
        debug!(
            "Initiated Send Swap: send {} sats to liquid address {}",
            create_response.expected_amount, create_response.address
        );

        let lockup_tx = self
            .onchain_wallet
            .build_tx(
                None,
                &create_response.address,
                create_response.expected_amount,
            )
            .await?;

        let lockup_tx_id = self.chain_service.broadcast(&lockup_tx)?.to_string();

        debug!("Successfully broadcast lockup tx for Send Swap {swap_id}. Lockup tx id: {lockup_tx_id}");
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
        let claim_tx_details = self.swapper.get_claim_tx_details(send_swap)?;
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

    fn get_preimage_from_script_path_claim_spend(
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
            .get_scripts_history(&[&swap_script_pk])?
            .into_iter()
            .flatten()
            .collect();

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
                    .get_transactions(&[claim_tx_id])
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

    async fn refund(&self, swap: &SendSwap) -> Result<String, PaymentError> {
        let amount_sat = get_invoice_amount!(swap.invoice);
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();

        let fee = self
            .onchain_wallet
            .build_tx(None, &output_address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum();
        let broadcast_fees_sat = Amount::from_sat(fee);

        let refund_res =
            self.swapper
                .refund_send_swap_cooperative(swap, &output_address, broadcast_fees_sat);
        match refund_res {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Cooperative refund failed: {:?}", e);
                self.refund_non_cooperative(swap, broadcast_fees_sat).await
            }
        }
    }

    async fn refund_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: Amount,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Send Swap {}",
            &swap.id
        );

        let current_height = self.onchain_wallet.tip().await.height();
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let refund_tx_id = self.swapper.refund_send_swap_non_cooperative(
            swap,
            broadcast_fees_sat,
            &output_address,
            current_height,
        )?;

        info!(
            "Successfully broadcast non-cooperative refund for Send Swap {}, tx: {}",
            swap.id, refund_tx_id
        );
        Ok(refund_tx_id)
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
            (Complete | Failed | TimedOut, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending, Complete) => Ok(()),
            (Complete | Failed | TimedOut, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created | TimedOut, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (_, Failed) => Ok(()),
        }
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice =
            Bolt11Invoice::from_str(invoice).map_err(|err| PaymentError::InvalidInvoice {
                err: err.to_string(),
            })?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
    }
}
