use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::{
    boltz::{self},
    swaps::boltz::{ChainSwapStates, CreateChainResponse, SwapUpdateTxDetails},
    ElementsLockTime, Secp256k1, Serialize, ToHex,
};
use futures_util::TryFutureExt;
use log::{debug, error, info, warn};
use lwk_wollet::{
    elements::{hex::FromHex, Script, Transaction},
    hashes::hex::DisplayHex,
    History,
};
use tokio::sync::{broadcast, Mutex};

use crate::model::{BlockListener, ChainSwapUpdate};
use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    ensure_sdk,
    error::{PaymentError, SdkError, SdkResult},
    model::{
        ChainSwap, Config, Direction,
        PaymentState::{self, *},
        PaymentTxData, PaymentType, Swap, SwapScriptV2, Transaction as SdkTransaction,
    },
    persist::Persister,
    swapper::Swapper,
    utils,
    wallet::OnchainWallet,
};

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

#[async_trait]
impl BlockListener for ChainSwapHandler {
    async fn on_bitcoin_block(&self, height: u32) {
        if let Err(e) = self.claim_outgoing(height).await {
            error!("Error claiming outgoing: {e:?}");
        }
    }

    async fn on_liquid_block(&self, height: u32) {
        if let Err(e) = self.refund_outgoing(height).await {
            warn!("Error refunding outgoing: {e:?}");
        }
        if let Err(e) = self.claim_incoming(height).await {
            error!("Error claiming incoming: {e:?}");
        }
    }
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

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Chain swaps
    pub(crate) async fn on_new_status(&self, update: &boltz::Update) -> Result<()> {
        let id = &update.id;
        let swap = self.fetch_chain_swap_by_id(id)?;

        if let Some(sync_state) = self.persister.get_sync_state_by_data_id(&swap.id)? {
            if !sync_state.is_local {
                let status = &update.status;
                let swap_state = ChainSwapStates::from_str(status)
                    .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

                match swap_state {
                    // If the swap is not local (pulled from real-time sync) we do not claim twice
                    ChainSwapStates::TransactionServerMempool
                    | ChainSwapStates::TransactionServerConfirmed => {
                        log::debug!("Received {swap_state:?} for non-local Chain swap {id} from status stream, skipping update.");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        match swap.direction {
            Direction::Incoming => self.on_new_incoming_status(&swap, update).await,
            Direction::Outgoing => self.on_new_outgoing_status(&swap, update).await,
        }
    }

    async fn claim_incoming(&self, height: u32) -> Result<()> {
        let chain_swaps: Vec<ChainSwap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .filter(|s| {
                s.direction == Direction::Incoming && s.state == Pending && s.claim_tx_id.is_none()
            })
            .collect();
        info!(
            "Rescanning {} incoming Chain Swap(s) server lockup txs at height {}",
            chain_swaps.len(),
            height
        );
        for swap in chain_swaps {
            if let Err(e) = self.claim_confirmed_server_lockup(&swap).await {
                error!(
                    "Error rescanning server lockup of incoming Chain Swap {}: {e:?}",
                    swap.id,
                );
            }
        }
        Ok(())
    }

    async fn claim_outgoing(&self, height: u32) -> Result<()> {
        let chain_swaps: Vec<ChainSwap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .filter(|s| {
                s.direction == Direction::Outgoing && s.state == Pending && s.claim_tx_id.is_none()
            })
            .collect();
        info!(
            "Rescanning {} outgoing Chain Swap(s) server lockup txs at height {}",
            chain_swaps.len(),
            height
        );
        for swap in chain_swaps {
            if let Err(e) = self.claim_confirmed_server_lockup(&swap).await {
                error!(
                    "Error rescanning server lockup of outgoing Chain Swap {}: {e:?}",
                    swap.id
                );
            }
        }
        Ok(())
    }

    async fn claim_confirmed_server_lockup(&self, swap: &ChainSwap) -> Result<()> {
        let Some(tx_id) = swap.server_lockup_tx_id.clone() else {
            // Skip the rescan if there is no server_lockup_tx_id yet
            return Ok(());
        };
        let swap_id = &swap.id;
        let swap_script = swap.get_claim_swap_script()?;
        let script_history = match swap.direction {
            Direction::Incoming => self.fetch_liquid_script_history(&swap_script).await,
            Direction::Outgoing => self.fetch_bitcoin_script_history(&swap_script).await,
        }?;
        let tx_history = script_history
            .iter()
            .find(|h| h.txid.to_hex().eq(&tx_id))
            .ok_or(anyhow!(
                "Server lockup tx for Chain Swap {swap_id} was not found, txid={tx_id}"
            ))?;
        if tx_history.height > 0 {
            info!("Chain Swap {swap_id} server lockup tx is confirmed");
            self.claim(swap_id)
                .await
                .map_err(|e| anyhow!("Could not claim Chain Swap {swap_id}: {e:?}"))?;
        }
        Ok(())
    }

    async fn on_new_incoming_status(&self, swap: &ChainSwap, update: &boltz::Update) -> Result<()> {
        let id = update.id.clone();
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
                        .update_chain_swap_accept_zero_conf(&id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(&ChainSwapUpdate {
                        swap_id: id,
                        to_state: Pending,
                        user_lockup_tx_id: Some(transaction.id),
                        ..Default::default()
                    })?;
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
                        self.update_swap_info(&ChainSwapUpdate {
                            swap_id: id.clone(),
                            to_state: Pending,
                            server_lockup_tx_id: Some(transaction.id),
                            ..Default::default()
                        })?;

                        if swap.accept_zero_conf {
                            self.claim(&id).await.map_err(|e| {
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

                        let verify_res =
                            self.verify_server_lockup_tx(swap, &transaction, true).await;

                        // Set the server_lockup_tx_id if it is verified or not.
                        // If it is not yet confirmed, then it will be claimed after confirmation
                        // in rescan_incoming_chain_swap_server_lockup_tx()
                        self.update_swap_info(&ChainSwapUpdate {
                            swap_id: id.clone(),
                            to_state: Pending,
                            server_lockup_tx_id: Some(transaction.id.clone()),
                            ..Default::default()
                        })?;

                        match verify_res {
                            Ok(_) => {
                                info!("Server lockup transaction was verified for incoming Chain Swap {}", swap.id);
                                self.claim(&id).await.map_err(|e| {
                                    error!("Could not cooperate Chain Swap {id} claim: {e}");
                                    anyhow!("Could not post claim details. Err: {e:?}")
                                })?;
                            }
                            Err(e) => {
                                warn!("Server lockup transaction for incoming Chain Swap {} could not be verified. txid: {}, err: {}", swap.id, transaction.id, e);
                                return Err(anyhow!(
                                    "Could not verify server lockup transaction {}: {e}",
                                    transaction.id
                                ));
                            }
                        }
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
                // Zero-amount Receive Chain Swaps also get to TransactionLockupFailed when user locks up funds
                let is_zero_amount = swap.payer_amount_sat == 0;
                if matches!(swap_state, ChainSwapStates::TransactionLockupFailed) && is_zero_amount
                {
                    match self.handle_amountless_update(swap).await {
                        Ok(_) => {
                            // We successfully accepted the quote, the swap should continue as normal
                            return Ok(()); // Break from TxLockupFailed branch
                        }
                        // In case of error, we continue and mark it as refundable
                        Err(e) => error!("Failed to accept the quote for swap {}: {e:?}", &swap.id),
                    }
                }

                match swap.refund_tx_id.clone() {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match self.verify_user_lockup_tx(swap).await {
                            Ok(_) => {
                                info!("Chain Swap {id} user lockup tx was broadcast. Setting the swap to refundable.");
                                self.update_swap_info(&ChainSwapUpdate {
                                    swap_id: id,
                                    to_state: Refundable,
                                    ..Default::default()
                                })?;
                            }
                            Err(_) => {
                                info!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(&ChainSwapUpdate {
                                    swap_id: id,
                                    to_state: Failed,
                                    ..Default::default()
                                })?;
                            }
                        }
                    }
                    Some(refund_tx_id) => warn!(
                        "Refund for Chain Swap {id} was already broadcast: txid {refund_tx_id}"
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

    async fn handle_amountless_update(&self, swap: &ChainSwap) -> Result<(), PaymentError> {
        let quote = self
            .swapper
            .get_zero_amount_chain_swap_quote(&swap.id)
            .map(|quote| quote.to_sat())?;
        info!("Got quote of {quote} sat for swap {}", &swap.id);

        self.validate_and_update_amountless_swap(swap, quote)
            .await?;
        self.swapper
            .accept_zero_amount_chain_swap_quote(&swap.id, quote)
    }

    async fn validate_and_update_amountless_swap(
        &self,
        swap: &ChainSwap,
        quote_server_lockup_amount_sat: u64,
    ) -> Result<(), PaymentError> {
        debug!("Validating {swap:?}");

        ensure_sdk!(
            matches!(swap.direction, Direction::Incoming),
            PaymentError::generic(&format!(
                "Only an incoming chain swap can be a zero-amount swap. Swap ID: {}",
                &swap.id
            ))
        );

        let script_pubkey = swap.get_receive_lockup_swap_script_pubkey(self.config.network)?;
        let script_balance = self
            .bitcoin_chain_service
            .lock()
            .await
            .script_get_balance_with_retry(script_pubkey.as_script(), 10)
            .await?;
        debug!("Found lockup balance {script_balance:?}");
        let user_lockup_amount_sat = match script_balance.confirmed > 0 {
            true => script_balance.confirmed,
            false => match script_balance.unconfirmed > 0 {
                true => script_balance.unconfirmed.unsigned_abs(),
                false => 0,
            },
        };
        ensure_sdk!(
            user_lockup_amount_sat > 0,
            PaymentError::generic("Lockup address has no confirmed or unconfirmed balance")
        );

        let pair = swap.get_boltz_pair()?;
        let swapper_service_feerate = pair.fees.percentage;
        let swapper_server_fees_sat = pair.fees.server();
        let service_fees_sat =
            ((swapper_service_feerate / 100.0) * user_lockup_amount_sat as f64).ceil() as u64;
        let fees_sat = swapper_server_fees_sat + service_fees_sat;
        ensure_sdk!(
            user_lockup_amount_sat > fees_sat,
            PaymentError::generic(&format!("Invalid quote: fees ({fees_sat} sat) are higher than user lockup ({user_lockup_amount_sat} sat)"))
        );

        let expected_server_lockup_amount_sat = user_lockup_amount_sat - fees_sat;
        debug!("user_lockup_amount_sat = {}, service_fees_sat = {}, server_fees_sat = {}, expected_server_lockup_amount_sat = {}, quote_server_lockup_amount_sat = {}",
            user_lockup_amount_sat, service_fees_sat, swapper_server_fees_sat, expected_server_lockup_amount_sat, quote_server_lockup_amount_sat);
        ensure_sdk!(
            expected_server_lockup_amount_sat <= quote_server_lockup_amount_sat,
            PaymentError::generic(&format!("Invalid quote: expected at least {expected_server_lockup_amount_sat} sat, got {quote_server_lockup_amount_sat} sat"))
        );

        let receiver_amount_sat = quote_server_lockup_amount_sat - swap.claim_fees_sat;
        self.persister.update_zero_amount_swap_values(
            &swap.id,
            user_lockup_amount_sat,
            receiver_amount_sat,
        )?;

        Ok(())
    }

    async fn on_new_outgoing_status(&self, swap: &ChainSwap, update: &boltz::Update) -> Result<()> {
        let id = update.id.clone();
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
                        let user_lockup_tx = self.lockup_funds(&id, &create_response).await?;
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
                            unblinding_data: None,
                        }, None, false)?;

                        self.update_swap_info(&ChainSwapUpdate {
                            swap_id: id,
                            to_state: Pending,
                            user_lockup_tx_id: Some(lockup_tx_id),
                            ..Default::default()
                        })?;
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
                        .update_chain_swap_accept_zero_conf(&id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(&ChainSwapUpdate {
                        swap_id: id,
                        to_state: Pending,
                        user_lockup_tx_id: Some(transaction.id),
                        ..Default::default()
                    })?;
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
                        self.update_swap_info(&ChainSwapUpdate {
                            swap_id: id.clone(),
                            to_state: Pending,
                            server_lockup_tx_id: Some(transaction.id),
                            ..Default::default()
                        })?;

                        if swap.accept_zero_conf {
                            self.claim(&id).await.map_err(|e| {
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
                        self.update_swap_info(&ChainSwapUpdate {
                            swap_id: id.clone(),
                            to_state: Pending,
                            server_lockup_tx_id: Some(transaction.id),
                            ..Default::default()
                        })?;
                        self.claim(&id).await.map_err(|e| {
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
                match &swap.refund_tx_id {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match swap.user_lockup_tx_id {
                            Some(_) => {
                                warn!("Chain Swap {id} user lockup tx has been broadcast.");
                                let refund_tx_id = match self.refund_outgoing_swap(swap, true).await
                                {
                                    Ok(refund_tx_id) => Some(refund_tx_id),
                                    Err(e) => {
                                        warn!(
                                            "Could not refund Send swap {id} cooperatively: {e:?}"
                                        );
                                        None
                                    }
                                };
                                // Set the payment state to `RefundPending`. This ensures that the
                                // background thread will pick it up and try to refund it
                                // periodically
                                self.update_swap_info(&ChainSwapUpdate {
                                    swap_id: id,
                                    to_state: RefundPending,
                                    refund_tx_id,
                                    ..Default::default()
                                })?;
                            }
                            None => {
                                warn!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(&ChainSwapUpdate {
                                    swap_id: id,
                                    to_state: Failed,
                                    ..Default::default()
                                })?;
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
            .build_tx_or_drain_tx(
                self.config.lowball_fee_rate_msat_per_vbyte(),
                &lockup_details.lockup_address,
                lockup_details.amount,
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

    fn fetch_chain_swap_by_id(&self, swap_id: &str) -> Result<ChainSwap, PaymentError> {
        self.persister
            .fetch_chain_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Chain Swap not found {swap_id}"),
            })
    }

    // Updates the swap without state transition validation
    pub(crate) fn update_swap(&self, updated_swap: ChainSwap) -> Result<(), PaymentError> {
        let swap = self.fetch_chain_swap_by_id(&updated_swap.id)?;
        if updated_swap != swap {
            info!(
                "Updating Chain swap {} to {:?} (user_lockup_tx_id = {:?}, server_lockup_tx_id = {:?}, claim_tx_id = {:?}, refund_tx_id = {:?})",
                updated_swap.id,
                updated_swap.state,
                updated_swap.user_lockup_tx_id,
                updated_swap.server_lockup_tx_id,
                updated_swap.claim_tx_id,
                updated_swap.refund_tx_id
            );
            self.persister.insert_or_update_chain_swap(&updated_swap)?;
            let _ = self.subscription_notifier.send(updated_swap.id);
        }
        Ok(())
    }

    // Updates the swap state with validation
    pub(crate) fn update_swap_info(
        &self,
        swap_update: &ChainSwapUpdate,
    ) -> Result<(), PaymentError> {
        info!("Updating Chain swap {swap_update:?}");
        let swap = self.fetch_chain_swap_by_id(&swap_update.swap_id)?;
        Self::validate_state_transition(swap.state, swap_update.to_state)?;
        self.persister.try_handle_chain_swap_update(swap_update)?;
        let updated_swap = self.fetch_chain_swap_by_id(&swap_update.swap_id)?;
        if updated_swap != swap {
            let _ = self.subscription_notifier.send(updated_swap.id);
        }
        Ok(())
    }

    async fn claim(&self, swap_id: &str) -> Result<(), PaymentError> {
        let swap = self.fetch_chain_swap_by_id(swap_id)?;
        ensure_sdk!(swap.claim_tx_id.is_none(), PaymentError::AlreadyClaimed);

        debug!("Initiating claim for Chain Swap {swap_id}");
        // Derive a new Liquid address if one is not already set for an incoming swap,
        // or use the set Bitcoin address for an outgoing swap
        let claim_address = match (swap.direction, swap.claim_address.clone()) {
            (Direction::Incoming, None) => {
                Some(self.onchain_wallet.next_unused_address().await?.to_string())
            }
            _ => swap.claim_address.clone(),
        };
        let claim_tx = self
            .swapper
            .create_claim_tx(Swap::Chain(swap.clone()), claim_address.clone())?;

        // Set the swap claim_tx_id before broadcasting.
        // If another claim_tx_id has been set in the meantime, don't broadcast the claim tx
        let tx_id = claim_tx.txid();
        match self
            .persister
            .set_chain_swap_claim_tx_id(swap_id, claim_address, &tx_id)
        {
            Ok(_) => {
                let broadcast_res = match claim_tx {
                    // We attempt broadcasting via chain service, then fallback to Boltz
                    SdkTransaction::Liquid(tx) => {
                        let liquid_chain_service = self.liquid_chain_service.lock().await;
                        liquid_chain_service
                            .broadcast(&tx, Some(&swap.id))
                            .await
                            .map(|tx_id| tx_id.to_hex())
                            .or_else(|err| {
                                debug!(
                                    "Could not broadcast claim tx via chain service for Chain swap {swap_id}: {err:?}"
                                );
                                let claim_tx_hex = tx.serialize().to_lower_hex_string();
                                self.swapper.broadcast_tx(self.config.network.into(), &claim_tx_hex)
                            })
                    }
                    SdkTransaction::Bitcoin(tx) => {
                        let bitcoin_chain_service = self.bitcoin_chain_service.lock().await;
                        bitcoin_chain_service
                            .broadcast(&tx)
                            .map(|tx_id| tx_id.to_hex())
                            .map_err(|err| PaymentError::Generic {
                                err: err.to_string(),
                            })
                    }
                };

                match broadcast_res {
                    Ok(claim_tx_id) => {
                        let payment_id = match swap.direction {
                            Direction::Incoming => {
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
                                        unblinding_data: None,
                                    },
                                    None,
                                    false,
                                )?;
                                Some(claim_tx_id.clone())
                            }
                            Direction::Outgoing => swap.user_lockup_tx_id,
                        };

                        info!("Successfully broadcast claim tx {claim_tx_id} for Chain Swap {swap_id}");
                        // The claim_tx_id is already set by set_chain_swap_claim_tx_id. Manually trigger notifying
                        // subscribers as update_swap_info will not recognise a change to the swap
                        payment_id.and_then(|payment_id| {
                            self.subscription_notifier.send(payment_id).ok()
                        });
                        Ok(())
                    }
                    Err(err) => {
                        // Multiple attempts to broadcast have failed. Unset the swap claim_tx_id
                        debug!(
                            "Could not broadcast claim tx via swapper for Chain swap {swap_id}: {err:?}"
                        );
                        self.persister
                            .unset_chain_swap_claim_tx_id(swap_id, &tx_id)?;
                        Err(err)
                    }
                }
            }
            Err(err) => {
                debug!(
                    "Failed to set claim_tx_id after creating tx for Chain swap {swap_id}: txid {tx_id}"
                );
                Err(err)
            }
        }
    }

    pub(crate) async fn prepare_refund(
        &self,
        lockup_address: &str,
        refund_address: &str,
        fee_rate_sat_per_vb: u32,
    ) -> SdkResult<(u32, u64, Option<String>)> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(SdkError::generic(format!(
                "Chain Swap with lockup address {lockup_address} not found"
            )))?;

        let refund_tx_id = swap.refund_tx_id.clone();
        if let Some(refund_tx_id) = &refund_tx_id {
            warn!(
                "A refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                swap.id
            );
        }

        let (refund_tx_size, refund_tx_fees_sat) = self.swapper.estimate_refund_broadcast(
            Swap::Chain(swap),
            refund_address,
            Some(fee_rate_sat_per_vb as f64),
        )?;

        Ok((refund_tx_size, refund_tx_fees_sat, refund_tx_id))
    }

    pub(crate) async fn refund_incoming_swap(
        &self,
        lockup_address: &str,
        refund_address: &str,
        broadcast_fee_rate_sat_per_vb: u32,
        is_cooperative: bool,
    ) -> Result<String, PaymentError> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(PaymentError::Generic {
                err: format!("Swap for lockup address {} not found", lockup_address),
            })?;
        let id = &swap.id;

        ensure_sdk!(
            swap.state == Refundable,
            PaymentError::Generic {
                err: format!("Chain Swap {id} was not marked as `Refundable`")
            }
        );

        info!("Initiating refund for incoming Chain Swap {id}, is_cooperative: {is_cooperative}",);

        let SwapScriptV2::Bitcoin(swap_script) = swap.get_lockup_swap_script()? else {
            return Err(PaymentError::Generic {
                err: "Unexpected swap script type found".to_string(),
            });
        };

        let bitcoin_chain_service = self.bitcoin_chain_service.lock().await;
        let script_pk = swap_script
            .to_address(self.config.network.as_bitcoin_chain())
            .map_err(|e| anyhow!("Could not retrieve address from swap script: {e:?}"))?
            .script_pubkey();
        let utxos = bitcoin_chain_service.get_script_utxos(&script_pk).await?;

        let SdkTransaction::Bitcoin(refund_tx) = self.swapper.create_refund_tx(
            Swap::Chain(swap.clone()),
            refund_address,
            utxos,
            Some(broadcast_fee_rate_sat_per_vb as f64),
            is_cooperative,
        )?
        else {
            return Err(PaymentError::Generic {
                err: format!("Unexpected refund tx type returned for incoming Chain swap {id}",),
            });
        };
        let refund_tx_id = bitcoin_chain_service.broadcast(&refund_tx)?.to_string();

        info!("Successfully broadcast refund for incoming Chain Swap {id}, is_cooperative: {is_cooperative}");

        // After refund tx is broadcasted, set the payment state to `RefundPending`. This ensures:
        // - the swap is not shown in `list-refundables` anymore
        // - the background thread will move it to Failed once the refund tx confirms
        self.update_swap_info(&ChainSwapUpdate {
            swap_id: swap.id,
            to_state: RefundPending,
            refund_tx_id: Some(refund_tx_id.clone()),
            ..Default::default()
        })?;

        Ok(refund_tx_id)
    }

    pub(crate) async fn refund_outgoing_swap(
        &self,
        swap: &ChainSwap,
        is_cooperative: bool,
    ) -> Result<String, PaymentError> {
        ensure_sdk!(
            swap.refund_tx_id.is_none(),
            PaymentError::Generic {
                err: format!(
                    "A refund tx for outgoing Chain Swap {} was already broadcast",
                    swap.id
                )
            }
        );

        info!(
            "Initiating refund for outgoing Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        let SwapScriptV2::Liquid(swap_script) = swap.get_lockup_swap_script()? else {
            return Err(PaymentError::Generic {
                err: "Unexpected swap script type found".to_string(),
            });
        };

        let liquid_chain_service = self.liquid_chain_service.lock().await;
        let script_pk = swap_script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Could not retrieve address from swap script: {e:?}"))?
            .to_unconfidential()
            .script_pubkey();
        let utxos = liquid_chain_service.get_script_utxos(&script_pk).await?;

        let refund_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let SdkTransaction::Liquid(refund_tx) = self.swapper.create_refund_tx(
            Swap::Chain(swap.clone()),
            &refund_address,
            utxos,
            None,
            is_cooperative,
        )?
        else {
            return Err(PaymentError::Generic {
                err: format!(
                    "Unexpected refund tx type returned for outgoing Chain swap {}",
                    swap.id
                ),
            });
        };
        let refund_tx_id = liquid_chain_service
            .broadcast(&refund_tx, Some(&swap.id))
            .await?
            .to_string();

        info!(
            "Successfully broadcast refund for outgoing Chain Swap {}, is_cooperative: {is_cooperative}",
            swap.id
        );

        Ok(refund_tx_id)
    }

    async fn refund_outgoing(&self, height: u32) -> Result<(), PaymentError> {
        // Get all pending outgoing chain swaps with no refund tx
        let pending_swaps: Vec<ChainSwap> = self
            .persister
            .list_pending_chain_swaps()?
            .into_iter()
            .filter(|s| s.direction == Direction::Outgoing && s.refund_tx_id.is_none())
            .collect();
        for swap in pending_swaps {
            let swap_script = swap.get_lockup_swap_script()?.as_liquid_script()?;
            let locktime_from_height = ElementsLockTime::from_height(height)
                .map_err(|e| PaymentError::Generic { err: e.to_string() })?;
            info!("Checking Chain Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", swap.id, swap_script.locktime);
            let has_swap_expired =
                utils::is_locktime_expired(locktime_from_height, swap_script.locktime);
            if has_swap_expired || swap.state == RefundPending {
                let refund_tx_id_res = match swap.state {
                    Pending => self.refund_outgoing_swap(&swap, false).await,
                    RefundPending => match has_swap_expired {
                        true => {
                            self.refund_outgoing_swap(&swap, true)
                                .or_else(|e| {
                                    warn!("Failed to initiate cooperative refund, switching to non-cooperative: {e:?}");
                                    self.refund_outgoing_swap(&swap, false)
                                })
                                .await
                        }
                        false => self.refund_outgoing_swap(&swap, true).await,
                    },
                    _ => {
                        continue;
                    }
                };

                if let Ok(refund_tx_id) = refund_tx_id_res {
                    let update_swap_info_res = self.update_swap_info(&ChainSwapUpdate {
                        swap_id: swap.id.clone(),
                        to_state: RefundPending,
                        refund_tx_id: Some(refund_tx_id),
                        ..Default::default()
                    });
                    if let Err(err) = update_swap_info_res {
                        warn!(
                            "Could not update outgoing Chain swap {} information: {err:?}",
                            swap.id
                        );
                    };
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
        if value < claim_details.amount {
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
        if value < claim_details.amount {
            return Err(anyhow!(
                "Transaction value {value} sats is less than {} sats",
                claim_details.amount
            ));
        }
        Ok(())
    }

    async fn verify_user_lockup_tx(&self, chain_swap: &ChainSwap) -> Result<String> {
        let swap_script = chain_swap.get_lockup_swap_script()?;
        let script_history = match chain_swap.direction {
            Direction::Incoming => self.fetch_bitcoin_script_history(&swap_script).await,
            Direction::Outgoing => self.fetch_liquid_script_history(&swap_script).await,
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
                self.update_swap_info(&ChainSwapUpdate {
                    swap_id: chain_swap.id.clone(),
                    to_state: Pending,
                    user_lockup_tx_id: Some(txid.clone()),
                    ..Default::default()
                })?;
                Ok(txid)
            }
        }
    }

    async fn fetch_bitcoin_script_history(
        &self,
        swap_script: &SwapScriptV2,
    ) -> Result<Vec<History>> {
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

    async fn fetch_liquid_script_history(
        &self,
        swap_script: &SwapScriptV2,
    ) -> Result<Vec<History>> {
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
    use anyhow::Result;
    use std::collections::{HashMap, HashSet};

    use crate::{
        model::{
            ChainSwapUpdate, Direction,
            PaymentState::{self, *},
        },
        test_utils::{
            chain_swap::{new_chain_swap, new_chain_swap_handler},
            persist::create_persister,
        },
    };

    #[tokio::test]
    async fn test_chain_swap_state_transitions() -> Result<()> {
        create_persister!(persister);

        let chain_swap_handler = new_chain_swap_handler(persister.clone())?;

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
                persister.insert_or_update_chain_swap(&chain_swap)?;

                assert!(chain_swap_handler
                    .update_swap_info(&ChainSwapUpdate {
                        swap_id: chain_swap.id,
                        to_state: *allowed_state,
                        ..Default::default()
                    })
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
                persister.insert_or_update_chain_swap(&chain_swap)?;

                assert!(chain_swap_handler
                    .update_swap_info(&ChainSwapUpdate {
                        swap_id: chain_swap.id,
                        to_state: *disallowed_state,
                        ..Default::default()
                    })
                    .is_err());
            }
        }

        Ok(())
    }
}
