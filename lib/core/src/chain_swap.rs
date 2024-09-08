use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::boltz::{
    BoltzApiClientV2, Cooperative, BOLTZ_MAINNET_URL_V2, BOLTZ_TESTNET_URL_V2,
};
use boltz_client::network::electrum::ElectrumConfig;
use boltz_client::swaps::boltz::{self, SwapUpdateTxDetails};
use boltz_client::swaps::{boltz::ChainSwapStates, boltz::CreateChainResponse};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Address, Amount, BtcSwapTx, Secp256k1, ToHex};
use futures_util::TryFutureExt;
use log::{debug, error, info, warn};
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::elements::{LockTime, Script, Transaction};
use lwk_wollet::History;
use tokio::sync::{broadcast, watch, Mutex};
use tokio::time::MissedTickBehavior;

use crate::chain::bitcoin::BitcoinChainService;
use crate::chain::liquid::LiquidChainService;
use crate::error::{SdkError, SdkResult};
use crate::model::PaymentState::{
    Complete, Created, Failed, Pending, RefundPending, Refundable, TimedOut,
};
use crate::model::{ChainSwap, Config, Direction, PaymentTxData, PaymentType};
use crate::prelude::{LiquidNetwork, SwapScriptV2};
use crate::sdk::CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS;
use crate::swapper::Swapper;
use crate::utils;
use crate::wallet::OnchainWallet;
use crate::{error::PaymentError, model::PaymentState, persist::Persister};

// Estimates based on https://github.com/BoltzExchange/boltz-backend/blob/ee4c77be1fcb9bb2b45703c542ad67f7efbf218d/lib/rates/FeeProvider.ts#L78
pub const ESTIMATED_BTC_CLAIM_TX_VSIZE: u64 = 111;

pub(crate) struct ChainSwapHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    subscription_notifier: broadcast::Sender<String>,
}

impl ChainSwapHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
        bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    ) -> Result<Self> {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Ok(Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            liquid_chain_service,
            bitcoin_chain_service,
            subscription_notifier,
        })
    }

    pub(crate) async fn start(self: Arc<Self>, mut shutdown: watch::Receiver<()>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut rescan_interval = tokio::time::interval(Duration::from_secs(60 * 10));
            rescan_interval.set_missed_tick_behavior(MissedTickBehavior::Burst);

            loop {
                tokio::select! {
                    _ = rescan_interval.tick() => {
                        if let Err(e) = cloned.rescan_incoming_chain_swaps().await {
                            error!("Error checking incoming chain swaps: {e:?}");
                        }
                        if let Err(e) = cloned.rescan_outgoing_chain_swaps().await {
                            error!("Error checking outgoing chain swaps: {e:?}");
                        }
                    },
                    _ = shutdown.changed() => {
                        info!("Received shutdown signal, exiting chain swap loop");
                        return;
                    }
                }
            }
        });
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Chain swaps
    pub(crate) async fn on_new_status(&self, update: &boltz::Update) -> Result<()> {
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

    pub(crate) async fn rescan_incoming_chain_swaps(&self) -> Result<()> {
        let current_height = self.bitcoin_chain_service.lock().await.tip()?.height as u32;
        let chain_swaps: Vec<ChainSwap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .filter(|s| s.direction == Direction::Incoming)
            .collect();
        info!(
            "Rescanning {} incoming Chain Swap(s) at height {}",
            chain_swaps.len(),
            current_height
        );
        for swap in chain_swaps {
            if let Err(e) = self.rescan_incoming_chain_swap(&swap, current_height).await {
                error!("Error rescanning incoming Chain Swap {}: {e:?}", swap.id);
            }
        }
        Ok(())
    }

    async fn rescan_incoming_chain_swap(
        &self,
        swap: &ChainSwap,
        current_height: u32,
    ) -> Result<()> {
        let monitoring_block_height =
            swap.timeout_block_height + CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS;
        let is_swap_expired = current_height > swap.timeout_block_height;
        let is_monitoring_expired = current_height > monitoring_block_height;

        if (is_swap_expired && !is_monitoring_expired) || swap.state == RefundPending {
            let swap_script = swap.get_lockup_swap_script()?.as_bitcoin_script()?;
            let script_pubkey = swap_script
                .to_address(self.config.network.as_bitcoin_chain())
                .map_err(|e| anyhow!("Error getting script address: {e:?}"))?
                .script_pubkey();
            let script_balance = self
                .bitcoin_chain_service
                .lock()
                .await
                .script_get_balance(script_pubkey.as_script())?;
            info!(
                "Incoming Chain Swap {} has {} confirmed and {} unconfirmed sats",
                swap.id, script_balance.confirmed, script_balance.unconfirmed
            );

            if script_balance.confirmed > 0
                && script_balance.unconfirmed == 0
                && swap.state != Refundable
            {
                // If there are unspent funds sent to the lockup script address then set
                // the state to Refundable.
                info!(
                    "Incoming Chain Swap {} has {} unspent sats. Setting the swap to refundable",
                    swap.id, script_balance.confirmed
                );
                self.update_swap_info(&swap.id, Refundable, None, None, None, None)
                    .await?;
            } else if script_balance.confirmed == 0 {
                // If the funds sent to the lockup script address are spent then set the
                // state back to Complete/Failed.
                let to_state = match swap.claim_tx_id {
                    Some(_) => Complete,
                    None => Failed,
                };

                if to_state != swap.state {
                    info!(
                        "Incoming Chain Swap {} has 0 unspent sats. Setting the swap to {:?}",
                        swap.id, to_state
                    );
                    self.update_swap_info(&swap.id, to_state, None, None, None, None)
                        .await?;
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn rescan_outgoing_chain_swaps(&self) -> Result<()> {
        let current_height = self.bitcoin_chain_service.lock().await.tip()?.height as u32;
        let chain_swaps: Vec<ChainSwap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .filter(|s| {
                s.direction == Direction::Outgoing
                    && s.state == PaymentState::Pending
                    && s.claim_tx_id.is_some()
            })
            .collect();
        info!(
            "Rescanning {} outgoing Chain Swap(s) at height {}",
            chain_swaps.len(),
            current_height
        );
        for swap in chain_swaps {
            if let Err(e) = self.rescan_outgoing_chain_swap(&swap).await {
                error!("Error rescanning outgoing Chain Swap {}: {e:?}", swap.id);
            }
        }
        Ok(())
    }

    async fn rescan_outgoing_chain_swap(&self, swap: &ChainSwap) -> Result<()> {
        let address = Address::from_str(&swap.claim_address)?;
        let claim_tx_id = swap.claim_tx_id.clone().ok_or(anyhow!("No claim tx id"))?;
        let script_pubkey = address.assume_checked().script_pubkey();
        let script_history = self
            .bitcoin_chain_service
            .lock()
            .await
            .get_script_history(script_pubkey.as_script())?;
        let claim_tx_history = script_history
            .iter()
            .find(|h| h.txid.to_hex().eq(&claim_tx_id) && h.height > 0);
        if claim_tx_history.is_some() {
            info!(
                "Outgoing Chain Swap {} claim tx is confirmed. Setting the swap to Complete",
                swap.id
            );
            self.update_swap_info(&swap.id, Complete, None, None, None, None)
                .await?;
        }
        Ok(())
    }

    async fn on_new_incoming_status(&self, swap: &ChainSwap, update: &boltz::Update) -> Result<()> {
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

            // Boltz announced the server lockup tx is in the mempool.
            // Verify the transaction and claim if zero-conf
            ChainSwapStates::TransactionServerMempool => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self
                            .verify_server_lockup_tx(swap, &transaction, false)
                            .await
                        {
                            warn!("Server lockup mempool transaction for incoming Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify server lockup transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!("Server lockup mempool transaction was verified for incoming Chain Swap {}", swap.id);
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;

                        if swap.accept_zero_conf {
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // Boltz announced the server lockup tx has been confirmed.
            // Verify the transaction and claim
            ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_user_lockup_tx(swap).await {
                            warn!("User lockup transaction for incoming Chain Swap {} could not be verified. err: {}", swap.id, e);
                            return Err(anyhow!("Could not verify user lockup transaction: {e}",));
                        }

                        if let Err(e) = self.verify_server_lockup_tx(swap, &transaction, true).await
                        {
                            warn!("Server lockup transaction for incoming Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify server lockup transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!(
                            "Server lockup transaction was verified for incoming Chain Swap {}",
                            swap.id
                        );
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;
                        self.claim(swap).await.map_err(|e| {
                            error!("Could not cooperate Chain Swap {id} claim: {e}");
                            anyhow!("Could not post claim details. Err: {e:?}")
                        })?;
                    }
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
                        match self.verify_user_lockup_tx(swap).await {
                            Ok(_) => {
                                info!("Chain Swap {id} user lockup tx was broadcast. Setting the swap to refundable.");
                                self.update_swap_info(id, Refundable, None, None, None, None)
                                    .await?;
                            }
                            Err(_) => {
                                info!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
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

    async fn on_new_outgoing_status(&self, swap: &ChainSwap, update: &boltz::Update) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = ChainSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

        info!("Handling outgoing Chain Swap transition to {status:?} for swap {id}");
        // See https://docs.boltz.exchange/v/api/lifecycle#chain-swaps
        match swap_state {
            // The swap is created
            ChainSwapStates::Created => {
                match (swap.state, swap.user_lockup_tx_id.clone()) {
                    // The swap timed out before receiving this status
                    (TimedOut, _) => warn!("Chain Swap {id} timed out, do not broadcast a lockup tx"),

                    // Create the user lockup tx
                    (_, None) => {
                        let create_response = swap.get_boltz_create_response()?;
                        let user_lockup_tx = self.lockup_funds(id, &create_response).await?;
                        let lockup_tx_id = user_lockup_tx.txid().to_string();
                        let lockup_tx_fees_sat: u64 = user_lockup_tx.all_fees().values().sum();

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: Some(utils::now()),
                            amount_sat: swap.receiver_amount_sat,
                            // This should be: boltz fee + lockup fee + claim fee
                            fees_sat: lockup_tx_fees_sat + swap.claim_fees_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        }, None)?;

                        self.update_swap_info(id, Pending, None, Some(&lockup_tx_id), None, None)
                            .await?;
                    },

                    // Lockup tx already exists
                    (_, Some(lockup_tx_id)) => warn!("User lockup tx for Chain Swap {id} was already broadcast: txid {lockup_tx_id}"),
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

            // Boltz announced the server lockup tx is in the mempool.
            // Verify the transaction and claim if zero-conf
            ChainSwapStates::TransactionServerMempool => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self
                            .verify_server_lockup_tx(swap, &transaction, false)
                            .await
                        {
                            warn!("Server lockup mempool transaction for outgoing Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify server lockup transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!("Server lockup mempool transaction was verified for outgoing Chain Swap {}", swap.id);
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;

                        if swap.accept_zero_conf {
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // Boltz announced the server lockup tx has been confirmed.
            // Verify the transaction and claim
            ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_user_lockup_tx(swap).await {
                            warn!("User lockup transaction for outgoing Chain Swap {} could not be verified. err: {}", swap.id, e);
                            return Err(anyhow!("Could not verify user lockup transaction: {e}",));
                        }

                        if let Err(e) = self.verify_server_lockup_tx(swap, &transaction, true).await
                        {
                            warn!("Server lockup transaction for outgoing Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify server lockup transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!(
                            "Server lockup transaction was verified for outgoing Chain Swap {}",
                            swap.id
                        );
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;
                        self.claim(swap).await.map_err(|e| {
                            error!("Could not cooperate Chain Swap {id} claim: {e}");
                            anyhow!("Could not post claim details. Err: {e:?}")
                        })?;
                    }
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
                        match swap.user_lockup_tx_id {
                            Some(_) => {
                                warn!("Chain Swap {id} user lockup tx has been broadcast.");
                                if let Err(err) = self.refund_outgoing_swap(swap, true).await {
                                    warn!("Could not refund outgoing Chain swap cooperatively, error: {err:?}");
                                    // Set the payment state to `RefundPending`. This ensures that the
                                    // background thread will pick it up and try to refund it
                                    // periodically
                                    self.update_swap_info(
                                        &swap.id,
                                        RefundPending,
                                        None,
                                        None,
                                        None,
                                        None,
                                    )
                                    .await?;
                                };
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
            .broadcast(&lockup_tx, Some(swap_id))
            .await?
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
        let payment_id = match swap.direction {
            Direction::Incoming => claim_tx_id.map(|c| c.to_string()).or(swap.claim_tx_id),
            Direction::Outgoing => user_lockup_tx_id
                .map(|c| c.to_string())
                .or(swap.user_lockup_tx_id),
        };

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
        let claim_tx_id = self.swapper.claim_chain_swap(chain_swap)?;

        if chain_swap.direction == Direction::Incoming {
            // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
            // This makes the tx known to the SDK (get_info, list_payments) instantly
            self.persister.insert_or_update_payment(
                PaymentTxData {
                    tx_id: claim_tx_id.clone(),
                    timestamp: Some(utils::now()),
                    amount_sat: chain_swap.receiver_amount_sat,
                    fees_sat: 0,
                    payment_type: PaymentType::Receive,
                    is_confirmed: false,
                },
                None,
            )?;
        }

        self.update_swap_info(
            &chain_swap.id,
            Pending,
            None,
            None,
            Some(&claim_tx_id),
            None,
        )
        .await?;
        Ok(())
    }

    fn bitcoin_electrum_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.config.network.into(),
            &self.config.bitcoin_electrum_url,
            true,
            true,
            100,
        )
    }

    fn boltz_client(&self) -> BoltzApiClientV2 {
        let boltz_url = match self.config.network {
            LiquidNetwork::Mainnet => BOLTZ_MAINNET_URL_V2,
            LiquidNetwork::Testnet => BOLTZ_TESTNET_URL_V2,
        };
        BoltzApiClientV2::new(boltz_url)
    }

    pub async fn prepare_refund(
        &self,
        lockup_address: &str,
        output_address: &str,
        sat_per_vbyte: u32,
    ) -> SdkResult<(u32, u64, Option<String>)> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(SdkError::Generic {
                err: format!("Swap {} not found", lockup_address),
            })?;
        if let Some(refund_tx_id) = swap.refund_tx_id.clone() {
            warn!(
                "A refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                swap.id
            );
        }

        let refund_keypair = swap.get_refund_keypair()?;
        let preimage = Preimage::from_str(&swap.preimage)?;
        let swap_script = swap.get_lockup_swap_script()?;

        let refund_tx_vsize = match swap_script {
            SwapScriptV2::Bitcoin(swap_script) => {
                let refund_tx = BtcSwapTx::new_refund(
                    swap_script,
                    &output_address.into(),
                    &self.bitcoin_electrum_config(),
                )?;
                refund_tx.size(&refund_keypair, &preimage)? as u32
            }
            SwapScriptV2::Liquid(swap_script) => {
                let refund_tx = utils::new_lbtc_refund_tx(
                    swap_script,
                    output_address,
                    &self.config,
                    self.liquid_chain_service.clone(),
                )
                .await?;
                refund_tx.size(&refund_keypair, &preimage)? as u32
            }
        };
        let refund_tx_fee_sat = (refund_tx_vsize as f32 * sat_per_vbyte as f32).ceil() as u64;

        Ok((refund_tx_vsize, refund_tx_fee_sat, swap.refund_tx_id))
    }

    pub(crate) async fn refund_incoming_swap(
        &self,
        lockup_address: &str,
        refund_address: &str,
        sat_per_vbyte: u32,
        is_cooperative: bool,
    ) -> Result<String, PaymentError> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(PaymentError::Generic {
                err: format!("Swap {} not found", lockup_address),
            })?;

        if let Some(refund_tx_id) = swap.refund_tx_id.clone() {
            warn!(
                "A refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                swap.id
            );
        }

        info!(
            "Initiating refund for incoming Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        let SwapScriptV2::Bitcoin(swap_script) = swap.get_lockup_swap_script()? else {
            return Err(PaymentError::Generic {
                err: "Unexpected swap script type found".to_string(),
            });
        };
        let refund_keypair = swap.get_refund_keypair()?;

        let boltz_client = self.boltz_client();
        let cooperative = match is_cooperative {
            true => Some(Cooperative {
                boltz_api: &boltz_client,
                swap_id: swap.id.clone(),
                pub_nonce: None,
                partial_sig: None,
            }),
            false => None,
        };

        let refund_tx = BtcSwapTx::new_refund(
            swap_script.clone(),
            &refund_address.into(),
            &self.bitcoin_electrum_config(),
        )?;
        let broadcast_fees_sat = utils::estimate_btc_refund_fees_sat(
            sat_per_vbyte,
            &refund_tx,
            &Preimage::from_str(&swap.preimage)?,
            &refund_keypair,
        )?;
        let signed_tx = refund_tx.sign_refund(&refund_keypair, broadcast_fees_sat, cooperative)?;
        let refund_tx_id = refund_tx
            .broadcast(&signed_tx, &self.bitcoin_electrum_config())?
            .to_string();

        info!(
            "Successfully broadcast refund for incoming Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        Ok(refund_tx_id)
    }

    fn liquid_electrum_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.config.network.into(),
            &self.config.liquid_electrum_url,
            true,
            true,
            100,
        )
    }

    pub(crate) async fn refund_outgoing_swap(
        &self,
        swap: &ChainSwap,
        is_cooperative: bool,
    ) -> Result<String, PaymentError> {
        if !is_cooperative && !self.check_swap_expiry(swap).await? {
            return Err(PaymentError::Generic {
                err: format!(
                    "Cannot refund non-cooperatively: Locktime for outgoing Chain swap {} has not elapsed yet.",
                    swap.id
                ),
            });
        }

        info!(
            "Initiating refund for outgoing Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let SwapScriptV2::Liquid(swap_script) = swap.get_lockup_swap_script()? else {
            return Err(PaymentError::Generic {
                err: "Unexpected swap script type found".to_string(),
            });
        };
        let refund_keypair = swap.get_refund_keypair()?;

        let boltz_client = self.boltz_client();
        let (cooperative, lowball) = match is_cooperative {
            true => {
                let cooperative = Some(Cooperative {
                    boltz_api: &boltz_client,
                    swap_id: swap.id.clone(),
                    pub_nonce: None,
                    partial_sig: None,
                });
                let lowball = Some((&boltz_client, self.config.network.into()));
                (cooperative, lowball)
            }
            false => (None, None),
        };

        let refund_tx = utils::new_lbtc_refund_tx(
            swap_script,
            &output_address,
            &self.config,
            self.liquid_chain_service.clone(),
        )
        .await?;
        let broadcast_fees_sat = utils::estimate_lbtc_refund_fees_sat(
            self.config.network,
            &refund_tx,
            &refund_keypair,
            cooperative.clone(),
        )?;
        let signed_tx = refund_tx.sign_refund(
            &refund_keypair,
            Amount::from_sat(broadcast_fees_sat),
            cooperative,
        )?;
        let refund_tx_id =
            refund_tx.broadcast(&signed_tx, &self.liquid_electrum_config(), lowball)?;

        info!(
            "Successfully broadcast refund for outgoing Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        Ok(refund_tx_id)
    }

    async fn check_swap_expiry(&self, swap: &ChainSwap) -> Result<bool> {
        let swap_creation_time = UNIX_EPOCH + Duration::from_secs(swap.created_at as u64);
        let duration_since_creation_time = SystemTime::now().duration_since(swap_creation_time)?;
        if duration_since_creation_time.as_secs() < 60 * 10 {
            return Ok(false);
        }

        match swap.direction {
            Direction::Incoming => {
                let swap_script = swap.get_lockup_swap_script()?.as_bitcoin_script()?;
                let current_height = self.bitcoin_chain_service.lock().await.tip()?.height as u32;
                let locktime_from_height = boltz_client::LockTime::from_height(current_height)
                    .map_err(|e| PaymentError::Generic {
                        err: format!("Error getting locktime from height {current_height:?}: {e}",),
                    })?;

                info!("Checking Chain Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", swap.id, swap_script.locktime);
                Ok(swap_script.locktime.is_implied_by(locktime_from_height))
            }
            Direction::Outgoing => {
                let swap_script = swap.get_lockup_swap_script()?.as_liquid_script()?;
                let current_height = self.liquid_chain_service.lock().await.tip().await?;
                let locktime_from_height = LockTime::from_height(current_height)?;

                info!("Checking Chain Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", swap.id, swap_script.locktime);
                Ok(utils::is_locktime_expired(
                    locktime_from_height,
                    swap_script.locktime,
                ))
            }
        }
    }

    pub(crate) async fn track_refunds_and_refundables(&self) -> Result<(), PaymentError> {
        let pending_swaps = self.persister.list_pending_chain_swaps()?;
        for swap in pending_swaps {
            if swap.refund_tx_id.is_some() {
                continue;
            }

            match swap.direction {
                // Track refunds
                Direction::Outgoing => {
                    let refund_tx_id_result: Result<String, PaymentError> = match swap.state {
                        Pending => self.refund_outgoing_swap(&swap, false).await,
                        RefundPending => {
                            self.refund_outgoing_swap(&swap, true)
                                .or_else(|_| self.refund_outgoing_swap(&swap, false))
                                .await
                        }
                        _ => {
                            warn!(
                                "Invalid outgoing Chain swap state for swap {} when attempting to refund, state: {:?}",
                                swap.id, swap.state
                            );
                            continue;
                        }
                    };

                    if let Ok(refund_tx_id) = refund_tx_id_result {
                        let update_swap_info_result = self
                            .update_swap_info(
                                &swap.id,
                                RefundPending,
                                None,
                                None,
                                None,
                                Some(&refund_tx_id),
                            )
                            .await;
                        if let Err(err) = update_swap_info_result {
                            warn!(
                                "Could not update outgoing Chain swap {} information, error: {err:?}",
                                swap.id
                            );
                        };
                    }
                }

                // Track refundables by verifying that the expiry has elapsed, and set the state of the incoming swap to `Refundable`
                Direction::Incoming => {
                    if swap.user_lockup_tx_id.is_some()
                        && self.check_swap_expiry(&swap).await.unwrap_or(false)
                    {
                        let update_swap_info_result = self
                            .update_swap_info(&swap.id, Refundable, None, None, None, None)
                            .await;

                        if let Err(err) = update_swap_info_result {
                            warn!(
                                "Could not update Chain swap {} information, error: {err:?}",
                                swap.id
                            );
                        }
                    }
                }
            }
        }

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

            (Created | Pending | RefundPending, Complete) => Ok(()),
            (_, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (Created | Pending | RefundPending | Failed | Complete, Refundable) => Ok(()),
            (_, Refundable) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Refundable state"),
            }),

            (Pending | Refundable, RefundPending) => Ok(()),
            (_, RefundPending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to RefundPending state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),

            (_, Failed) => Ok(()),
        }
    }

    async fn verify_server_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        match chain_swap.direction {
            Direction::Incoming => {
                self.verify_incoming_server_lockup_tx(
                    chain_swap,
                    swap_update_tx,
                    verify_confirmation,
                )
                .await
            }
            Direction::Outgoing => {
                self.verify_outgoing_server_lockup_tx(
                    chain_swap,
                    swap_update_tx,
                    verify_confirmation,
                )
                .await
            }
        }
    }

    async fn verify_incoming_server_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        let swap_script = chain_swap.get_claim_swap_script()?;
        let claim_details = chain_swap.get_boltz_create_response()?.claim_details;
        // Verify transaction
        let liquid_swap_script = swap_script.as_liquid_script()?;
        let address = liquid_swap_script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let tx = self
            .liquid_chain_service
            .lock()
            .await
            .verify_tx(
                &address,
                &swap_update_tx.id,
                &swap_update_tx.hex,
                verify_confirmation,
            )
            .await?;
        // Verify RBF
        let rbf_explicit = tx.input.iter().any(|tx_in| tx_in.sequence.is_rbf());
        if !verify_confirmation && rbf_explicit {
            return Err(anyhow!("Transaction signals RBF"));
        }
        // Verify amount
        let secp = Secp256k1::new();
        let to_address_output = tx
            .output
            .iter()
            .filter(|tx_out| tx_out.script_pubkey == address.script_pubkey());
        let mut value = 0;
        for tx_out in to_address_output {
            value += tx_out
                .unblind(&secp, liquid_swap_script.blinding_key.secret_key())?
                .value;
        }
        if value < claim_details.amount as u64 {
            return Err(anyhow!(
                "Transaction value {value} sats is less than {} sats",
                claim_details.amount
            ));
        }
        Ok(())
    }

    async fn verify_outgoing_server_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        let swap_script = chain_swap.get_claim_swap_script()?;
        let claim_details = chain_swap.get_boltz_create_response()?.claim_details;
        // Verify transaction
        let address = swap_script
            .as_bitcoin_script()?
            .to_address(self.config.network.as_bitcoin_chain())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let tx = self
            .bitcoin_chain_service
            .lock()
            .await
            .verify_tx(
                &address,
                &swap_update_tx.id,
                &swap_update_tx.hex,
                verify_confirmation,
            )
            .await?;
        // Verify RBF
        let rbf_explicit = tx.input.iter().any(|input| input.sequence.is_rbf());
        if !verify_confirmation && rbf_explicit {
            return Err(anyhow!("Transaction signals RBF"));
        }
        // Verify amount
        let value: u64 = tx
            .output
            .iter()
            .filter(|tx_out| tx_out.script_pubkey == address.script_pubkey())
            .map(|tx_out| tx_out.value.to_sat())
            .sum();
        if value < claim_details.amount as u64 {
            return Err(anyhow!(
                "Transaction value {value} sats is less than {} sats",
                claim_details.amount
            ));
        }
        Ok(())
    }

    async fn verify_user_lockup_tx(&self, chain_swap: &ChainSwap) -> Result<String> {
        let script_history = match chain_swap.direction {
            Direction::Incoming => self.fetch_incoming_user_script_history(chain_swap).await,
            Direction::Outgoing => self.fetch_outgoing_user_script_history(chain_swap).await,
        }?;

        match chain_swap.user_lockup_tx_id.clone() {
            Some(user_lockup_tx_id) => {
                script_history
                    .iter()
                    .find(|h| h.txid.to_hex() == user_lockup_tx_id)
                    .ok_or(anyhow!("Transaction was not found in script history"))?;
                Ok(user_lockup_tx_id)
            }
            None => {
                let txid = script_history
                    .first()
                    .ok_or(anyhow!("Script history has no transactions"))?
                    .txid
                    .to_hex();
                self.update_swap_info(&chain_swap.id, Pending, None, Some(&txid), None, None)
                    .await?;
                Ok(txid)
            }
        }
    }

    async fn fetch_incoming_user_script_history(
        &self,
        chain_swap: &ChainSwap,
    ) -> Result<Vec<History>> {
        let swap_script = chain_swap.get_lockup_swap_script()?;
        let address = swap_script
            .as_bitcoin_script()?
            .to_address(self.config.network.as_bitcoin_chain())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let script_pubkey = address.script_pubkey();
        let script = script_pubkey.as_script();
        self.bitcoin_chain_service
            .lock()
            .await
            .get_script_history_with_retry(script, 5)
            .await
    }

    async fn fetch_outgoing_user_script_history(
        &self,
        chain_swap: &ChainSwap,
    ) -> Result<Vec<History>> {
        let swap_script = chain_swap.get_lockup_swap_script()?;
        let address = swap_script
            .as_liquid_script()?
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?
            .to_unconfidential();
        let script = Script::from_hex(hex::encode(address.script_pubkey().as_bytes()).as_str())
            .map_err(|e| anyhow!("Failed to get script from address {e:?}"))?;
        self.liquid_chain_service
            .lock()
            .await
            .get_script_history_with_retry(&script, 5)
            .await
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
        model::{
            Direction,
            PaymentState::{self, *},
        },
        test_utils::{
            chain_swap::{new_chain_swap, new_chain_swap_handler},
            persist::new_persister,
        },
    };

    #[tokio::test]
    async fn test_chain_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);

        let chain_swap_handler = new_chain_swap_handler(storage.clone())?;

        // Test valid combinations of states
        let all_states = HashSet::from([Created, Pending, Complete, TimedOut, Failed]);
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Refundable, Failed]),
            ),
            (
                Pending,
                HashSet::from([Pending, Complete, Refundable, RefundPending, Failed]),
            ),
            (TimedOut, HashSet::from([Failed])),
            (Complete, HashSet::from([Refundable])),
            (Refundable, HashSet::from([RefundPending, Failed])),
            (RefundPending, HashSet::from([Refundable, Complete, Failed])),
            (Failed, HashSet::from([Failed, Refundable])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let chain_swap =
                    new_chain_swap(Direction::Incoming, Some(*first_state), false, None);
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_handler
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
                let chain_swap =
                    new_chain_swap(Direction::Incoming, Some(*first_state), false, None);
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_handler
                    .update_swap_info(&chain_swap.id, *disallowed_state, None, None, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
