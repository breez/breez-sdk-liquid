use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltzv2;
use boltz_client::swaps::{boltz::ChainSwapStates, boltzv2::CreateChainResponse};
use log::{debug, error, info, warn};
use lwk_wollet::elements::Transaction;
use lwk_wollet::ElectrumUrl;
use tokio::sync::{broadcast, Mutex};

use crate::chain::bitcoin::{BitcoinChainService, ElectrumClient};
use crate::chain::ChainService;
use crate::model::PaymentState::{Complete, Created, Failed, Pending, TimedOut};
use crate::model::{ChainSwap, Config, Direction, PaymentTxData, PaymentType};
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{error::PaymentError, model::PaymentState, persist::Persister};

pub(crate) struct ChainSwapStateHandler {
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    liquid_chain_service: Arc<Mutex<dyn ChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    subscription_notifier: broadcast::Sender<String>,
}

impl ChainSwapStateHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        liquid_chain_service: Arc<Mutex<dyn ChainService>>,
    ) -> Result<Self> {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        let bitcoin_chain_service = Arc::new(Mutex::new(ElectrumClient::new(&ElectrumUrl::new(
            &config.bitcoin_electrum_url,
            true,
            true,
        ))?));
        Ok(Self {
            onchain_wallet,
            persister,
            swapper,
            liquid_chain_service,
            bitcoin_chain_service,
            subscription_notifier,
        })
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Chain swaps
    pub(crate) async fn on_new_status(&self, update: &boltzv2::Update) -> Result<()> {
        let id = &update.id;
        let swap = self
            .persister
            .fetch_chain_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Chain Swap found for ID {id}"))?;

        match swap.direction {
            Direction::Incoming => self.on_new_incoming_status(&swap, update).await,
            Direction::Outgoing => self.on_new_outgoing_status(&swap, update).await,
        }
    }

    async fn on_new_incoming_status(
        &self,
        swap: &ChainSwap,
        update: &boltzv2::Update,
    ) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = ChainSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

        info!("Handling incoming Chain Swap transition to {status:?} for swap {id}");
        // See https://docs.boltz.exchange/v/api/lifecycle#chain-swaps
        match swap_state {
            // Boltz announced the user lockup tx is in the mempool or has been confirmed.
            ChainSwapStates::TransactionMempool | ChainSwapStates::TransactionConfirmed => {
                if let Some(zero_conf_rejected) = update.zero_conf_rejected {
                    info!("Is zero conf rejected for Chain Swap {id}: {zero_conf_rejected}");
                    self.persister
                        .update_chain_swap_accept_zero_conf(id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(id, Pending, None, Some(&transaction.id), None, None)
                        .await?;
                }
                Ok(())
            }

            // Boltz announced the server lockup tx is in the mempool or has been confirmed.
            // If it's a zero conf swap or confirmed, proceed to cooperative claim
            ChainSwapStates::TransactionServerMempool
            | ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => match (swap.accept_zero_conf, swap_state) {
                        (true, _) | (_, ChainSwapStates::TransactionServerConfirmed) => {
                            if let Some(transaction) = update.transaction.clone() {
                                self.update_swap_info(
                                    id,
                                    Pending,
                                    Some(&transaction.id),
                                    None,
                                    None,
                                    None,
                                )
                                .await?;
                            }
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                        _ => info!("Waiting for server lockup confirmation for Chain Swap {id}"),
                    },
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. The transaction failed
            // 2. Lockup failed (too little funds were sent)
            // 3. The claim lockup was refunded
            // 4. The swap has expired (>24h)
            // We initiate a cooperative refund, and then fallback to a regular one
            ChainSwapStates::TransactionFailed
            | ChainSwapStates::TransactionLockupFailed
            | ChainSwapStates::TransactionRefunded
            | ChainSwapStates::SwapExpired => {
                match swap.refund_tx_id.clone() {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match swap.user_lockup_tx_id.clone() {
                            // If there is a lockup tx when receiving we need to refund to a sender address
                            // TODO: Set the chain swap to refundable
                            Some(_) => {}

                            // No user lockup tx was broadcast when sending or receiving
                            None => {
                                warn!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(id, Failed, None, None, None, None)
                                    .await?;
                            }
                        }
                    }
                    Some(refund_tx_id) => warn!(
                        "Refund tx for Chain Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                };
                Ok(())
            }

            _ => {
                debug!("Unhandled state for Chain Swap {id}: {swap_state:?}");
                Ok(())
            }
        }
    }

    async fn on_new_outgoing_status(
        &self,
        swap: &ChainSwap,
        update: &boltzv2::Update,
    ) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = ChainSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

        info!("Handling outgoing Chain Swap transition to {status:?} for swap {id}");
        // See https://docs.boltz.exchange/v/api/lifecycle#chain-swaps
        match swap_state {
            // The swap is created
            ChainSwapStates::Created => {
                match swap.user_lockup_tx_id.clone() {
                    // Create the user lockup tx when sending
                    None => {
                        let create_response = swap.get_boltz_create_response()?;
                        let user_lockup_tx = self.lockup_funds(id, &create_response).await?;
                        let lockup_tx_id = user_lockup_tx.txid().to_string();
                        let lockup_tx_fees_sat: u64 = user_lockup_tx.all_fees().values().sum();

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: None,
                            amount_sat: swap.receiver_amount_sat,
                            // This should be: boltz fee + lockup fee + claim fee
                            fees_sat: lockup_tx_fees_sat + swap.claim_fees_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        })?;

                        self.update_swap_info(id, Pending, None, Some(&lockup_tx_id), None, None)
                            .await?;
                    },

                    // Lockup tx already exists when sending
                    Some(lockup_tx_id) => warn!("User lockup tx for Chain Swap {id} was already broadcast: txid {lockup_tx_id}"),
                };
                Ok(())
            }

            // Boltz announced the user lockup tx is in the mempool or has been confirmed.
            ChainSwapStates::TransactionMempool | ChainSwapStates::TransactionConfirmed => {
                if let Some(zero_conf_rejected) = update.zero_conf_rejected {
                    info!("Is zero conf rejected for Chain Swap {id}: {zero_conf_rejected}");
                    self.persister
                        .update_chain_swap_accept_zero_conf(id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(id, Pending, None, Some(&transaction.id), None, None)
                        .await?;
                }
                Ok(())
            }

            // Boltz announced the server lockup tx is in the mempool or has been confirmed.
            // If it's a zero conf swap or confirmed, proceed to cooperative claim
            ChainSwapStates::TransactionServerMempool
            | ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => match (swap.accept_zero_conf, swap_state) {
                        (true, _) | (_, ChainSwapStates::TransactionServerConfirmed) => {
                            if let Some(transaction) = update.transaction.clone() {
                                self.update_swap_info(
                                    id,
                                    Pending,
                                    Some(&transaction.id),
                                    None,
                                    None,
                                    None,
                                )
                                .await?;
                            }
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                        _ => info!("Waiting for server lockup confirmation for Chain Swap {id}"),
                    },
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. The transaction failed
            // 2. Lockup failed (too little funds were sent)
            // 3. The claim lockup was refunded
            // 4. The swap has expired (>24h)
            // We initiate a cooperative refund, and then fallback to a regular one
            ChainSwapStates::TransactionFailed
            | ChainSwapStates::TransactionLockupFailed
            | ChainSwapStates::TransactionRefunded
            | ChainSwapStates::SwapExpired => {
                match swap.refund_tx_id.clone() {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match swap.user_lockup_tx_id.clone() {
                            Some(_) => {
                                warn!("Chain Swap {id} user lockup tx has been broadcast. Attempting refund.");
                                let refund_tx_id = self.refund(swap).await?;
                                info!("Broadcast refund tx for Chain Swap {id}. Tx id: {refund_tx_id}");
                                self.update_swap_info(
                                    id,
                                    Pending,
                                    None,
                                    None,
                                    None,
                                    Some(&refund_tx_id),
                                )
                                .await?;
                            }
                            None => {
                                warn!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(id, Failed, None, None, None, None)
                                    .await?;
                            }
                        }
                    }
                    Some(refund_tx_id) => warn!(
                        "Refund tx for Chain Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                };
                Ok(())
            }

            _ => {
                debug!("Unhandled state for Chain Swap {id}: {swap_state:?}");
                Ok(())
            }
        }
    }

    async fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateChainResponse,
    ) -> Result<Transaction, PaymentError> {
        let lockup_details = create_response.lockup_details.clone();

        debug!(
            "Initiated Chain Swap: send {} sats to liquid address {}",
            lockup_details.amount, lockup_details.lockup_address
        );

        let lockup_tx = self
            .onchain_wallet
            .build_tx(
                None,
                &lockup_details.lockup_address,
                lockup_details.amount as u64,
            )
            .await?;

        let lockup_tx_id = self
            .liquid_chain_service
            .lock()
            .await
            .broadcast(&lockup_tx)?
            .to_string();

        debug!(
          "Successfully broadcast lockup transaction for Chain Swap {swap_id}. Lockup tx id: {lockup_tx_id}"
        );
        Ok(lockup_tx)
    }

    /// Transitions a Chain swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        server_lockup_tx_id: Option<&str>,
        user_lockup_tx_id: Option<&str>,
        claim_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!("Transitioning Chain swap {swap_id} to {to_state:?} (server_lockup_tx_id = {:?}, user_lockup_tx_id = {:?}, claim_tx_id = {:?}), refund_tx_id = {:?})", server_lockup_tx_id, user_lockup_tx_id, claim_tx_id, refund_tx_id);

        let swap: ChainSwap = self
            .persister
            .fetch_chain_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Chain Swap not found {swap_id}"),
            })?;
        let payment_id = user_lockup_tx_id
            .map(|c| c.to_string())
            .or(swap.user_lockup_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_chain_swap_update(
            swap_id,
            to_state,
            server_lockup_tx_id,
            user_lockup_tx_id,
            claim_tx_id,
            refund_tx_id,
        )?;
        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn claim(&self, chain_swap: &ChainSwap) -> Result<(), PaymentError> {
        debug!("Initiating claim for Chain Swap {}", &chain_swap.id);
        let refund_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let claim_tx_id = self.swapper.claim_chain_swap(chain_swap, refund_address)?;

        if chain_swap.direction == Direction::Incoming {
            // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
            // This makes the tx known to the SDK (get_info, list_payments) instantly
            self.persister.insert_or_update_payment(PaymentTxData {
                tx_id: claim_tx_id.clone(),
                timestamp: None,
                amount_sat: chain_swap.receiver_amount_sat,
                fees_sat: 0,
                payment_type: PaymentType::Receive,
                is_confirmed: false,
            })?;
        }

        self.update_swap_info(
            &chain_swap.id,
            Complete,
            None,
            None,
            Some(&claim_tx_id),
            None,
        )
        .await?;
        Ok(())
    }

    async fn refund(&self, swap: &ChainSwap) -> Result<String, PaymentError> {
        let amount_sat = swap.receiver_amount_sat;
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();

        let fee = self
            .onchain_wallet
            .build_tx(None, &output_address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum();

        let refund_res = self
            .swapper
            .refund_chain_swap_cooperative(swap, &output_address, fee);
        match refund_res {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Cooperative refund failed: {:?}", e);
                self.refund_non_cooperative(swap, fee).await
            }
        }
    }

    async fn refund_non_cooperative(
        &self,
        swap: &ChainSwap,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Chain Swap {}",
            &swap.id
        );

        let current_height = match swap.direction {
            Direction::Incoming => self.bitcoin_chain_service.lock().await.tip()?.height as u32,
            Direction::Outgoing => self.liquid_chain_service.lock().await.tip()?.height,
        };

        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let refund_tx_id = self.swapper.refund_chain_swap_non_cooperative(
            swap,
            broadcast_fees_sat,
            &output_address,
            current_height,
        )?;

        info!(
            "Successfully broadcast non-cooperative refund for Chain Swap {}, tx: {}",
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

            (Created, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),
            (_, Failed) => Ok(()),
        }
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
        test_utils::{new_chain_swap, new_chain_swap_state_handler, new_persister},
    };

    #[tokio::test]
    async fn test_chain_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);

        let chain_swap_state_handler = new_chain_swap_state_handler(storage.clone())?;

        // Test valid combinations of states
        let all_states = HashSet::from([Created, Pending, Complete, TimedOut, Failed]);
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Failed]),
            ),
            (Pending, HashSet::from([Pending, Complete, Failed])),
            (TimedOut, HashSet::from([Failed])),
            (Complete, HashSet::from([])),
            (Failed, HashSet::from([Failed])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let chain_swap = new_chain_swap(Some(*first_state));
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_state_handler
                    .update_swap_info(&chain_swap.id, *allowed_state, None, None, None, None)
                    .await
                    .is_ok());
            }
        }

        // Test invalid combinations of states
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
                let chain_swap = new_chain_swap(Some(*first_state));
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_state_handler
                    .update_swap_info(&chain_swap.id, *disallowed_state, None, None, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
