use std::time::Instant;
use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use boltz_client::lightning_invoice::Bolt11InvoiceDescription;
use boltz_client::network::Chain;
use boltz_client::swaps::boltzv2;
use boltz_client::ToHex;
use boltz_client::{
    network::electrum::ElectrumConfig,
    swaps::{
        boltz::{RevSwapStates, SubSwapStates},
        boltzv2::*,
        liquidv2::LBtcSwapTxV2,
    },
    util::secrets::Preimage,
    Amount, Bolt11Invoice, ElementsAddress, Keypair, LBtcSwapScriptV2,
};
use log::{debug, error, info, warn};
use lwk_common::{singlesig_desc, Signer, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::bitcoin::Witness;
use lwk_wollet::elements::{LockTime, LockTime::*};
use lwk_wollet::hashes::{sha256, Hash};
use lwk_wollet::{
    elements::{Address, Transaction},
    BlockchainBackend, ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister,
    Wollet as LwkWollet, WolletDescriptor,
};
use tokio::sync::{watch, Mutex, RwLock};

use crate::error::LiquidSdkError;
use crate::model::PaymentState::*;
use crate::{
    boltz_status_stream::BoltzStatusStream,
    ensure_sdk,
    error::{LiquidSdkResult, PaymentError},
    event::EventManager,
    get_invoice_amount,
    model::*,
    persist::Persister,
    utils,
};

/// Claim tx feerate, in sats per vbyte.
/// Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
pub const LIQUID_CLAIM_TX_FEERATE_MSAT: f32 = 100.0;

pub const DEFAULT_DATA_DIR: &str = ".data";

pub struct LiquidSdk {
    electrum_url: ElectrumUrl,
    network: Network,
    /// LWK Wollet, a watch-only Liquid wallet for this instance
    lwk_wollet: Arc<Mutex<LwkWollet>>,
    /// LWK Signer, for signing Liquid transactions
    lwk_signer: SwSigner,
    persister: Arc<Persister>,
    data_dir_path: String,
    event_manager: Arc<EventManager>,
    status_stream: Arc<BoltzStatusStream>,
    is_started: RwLock<bool>,
    shutdown_sender: watch::Sender<()>,
    shutdown_receiver: watch::Receiver<()>,
}

impl LiquidSdk {
    pub async fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let is_mainnet = req.network == Network::Mainnet;
        let signer = SwSigner::new(&req.mnemonic, is_mainnet)?;
        let descriptor = LiquidSdk::get_descriptor(&signer, req.network)?;

        let sdk = LiquidSdk::new(LiquidSdkOptions {
            signer,
            descriptor,
            electrum_url: None,
            data_dir_path: req.data_dir,
            network: req.network,
        })?;
        sdk.start().await?;

        Ok(sdk)
    }

    fn new(opts: LiquidSdkOptions) -> Result<Arc<Self>> {
        let network = opts.network;
        let elements_network: ElementsNetwork = opts.network.into();
        let electrum_url = opts.get_electrum_url();
        let data_dir_path = opts.data_dir_path.unwrap_or(DEFAULT_DATA_DIR.to_string());

        let lwk_persister = FsPersister::new(&data_dir_path, network.into(), &opts.descriptor)?;
        let lwk_wollet = LwkWollet::new(elements_network, lwk_persister, opts.descriptor)?;

        fs::create_dir_all(&data_dir_path)?;

        let persister = Arc::new(Persister::new(&data_dir_path, network)?);
        persister.init()?;

        let event_manager = Arc::new(EventManager::new());
        let status_stream = Arc::new(BoltzStatusStream::new(
            Self::boltz_url_v2(network),
            persister.clone(),
        ));
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        let sdk = Arc::new(LiquidSdk {
            lwk_wollet: Arc::new(Mutex::new(lwk_wollet)),
            network,
            electrum_url,
            lwk_signer: opts.signer,
            persister,
            data_dir_path,
            event_manager,
            status_stream,
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
        });

        Ok(sdk)
    }

    /// Starts an SDK instance.
    ///
    /// Internal method. Should only be called once per instance.
    /// Should only be called as part of [LiquidSdk::connect].
    async fn start(self: &Arc<LiquidSdk>) -> LiquidSdkResult<()> {
        let mut is_started = self.is_started.write().await;
        let start_ts = Instant::now();
        self.start_background_tasks().await?;
        *is_started = true;

        let start_duration = start_ts.elapsed();
        info!("Liquid SDK initialized in: {start_duration:?}");
        Ok(())
    }

    /// Starts background tasks.
    ///
    /// Internal method. Should only be used as part of [LiquidSdk::start].
    async fn start_background_tasks(self: &Arc<LiquidSdk>) -> LiquidSdkResult<()> {
        // Periodically run sync() in the background
        let sdk_clone = self.clone();
        let mut shutdown_rx_sync_loop = self.shutdown_receiver.clone();
        tokio::spawn(async move {
            loop {
                _ = sdk_clone.sync().await;

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(30)) => {}
                    _ = shutdown_rx_sync_loop.changed() => {
                        info!("Received shutdown signal, exiting periodic sync loop");
                        return;
                    }
                }
            }
        });

        self.status_stream
            .clone()
            .track_pending_swaps(self.shutdown_receiver.clone())
            .await;

        self.track_swap_updates(self.shutdown_receiver.clone())
            .await;

        Ok(())
    }

    async fn ensure_is_started(&self) -> LiquidSdkResult<()> {
        let is_started = self.is_started.read().await;
        ensure_sdk!(*is_started, LiquidSdkError::NotStarted);
        Ok(())
    }

    /// Trigger the stopping of background threads for this SDK instance.
    pub async fn disconnect(&self) -> LiquidSdkResult<()> {
        self.ensure_is_started().await?;

        let mut is_started = self.is_started.write().await;
        self.shutdown_sender
            .send(())
            .map_err(|e| LiquidSdkError::Generic {
                err: format!("Shutdown failed: {e}"),
            })?;
        *is_started = false;
        Ok(())
    }

    async fn track_swap_updates(self: &Arc<LiquidSdk>, mut shutdown: watch::Receiver<()>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut updates_stream = cloned.status_stream.subscribe_swap_updates();
            loop {
                tokio::select! {
                    _ = shutdown.changed() => {
                        info!("Received shutdown signal, exiting swap updates loop");
                        return;
                    },
                    update = updates_stream.recv() => match update {
                        Ok(boltzv2::Update { id, status }) => {
                            let _ = cloned.sync().await;

                            if let Ok(_) = cloned.try_handle_send_swap_boltz_status(&status, &id).await
                            {
                                info!("Handled send swap update");
                            } else if let Ok(_) = cloned
                                .try_handle_receive_swap_boltz_status(&status, &id)
                                .await
                            {
                                info!("Handled receive swap update");
                            } else {
                                warn!("Unhandled swap {id}: {status}")
                            }
                        }
                        Err(e) => error!("Received stream error: {e:?}"),
                    }
                }
            }
        });
    }

    async fn notify_event_listeners(&self, e: LiquidSdkEvent) -> Result<()> {
        self.event_manager.notify(e).await;
        Ok(())
    }

    pub async fn add_event_listener(
        &self,
        listener: Box<dyn EventListener>,
    ) -> LiquidSdkResult<String> {
        Ok(self.event_manager.add(listener).await?)
    }

    pub async fn remove_event_listener(&self, id: String) -> LiquidSdkResult<()> {
        self.event_manager.remove(id).await;
        Ok(())
    }

    fn get_descriptor(signer: &SwSigner, network: Network) -> Result<WolletDescriptor> {
        let is_mainnet = network == Network::Mainnet;
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
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
            (Complete | Failed, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending, Complete) => Ok(()),
            (Complete | Failed, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (_, Failed) => Ok(()),
        }
    }

    /// Transitions a Receive swap to a new state
    pub(crate) async fn try_handle_receive_swap_update(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Receive swap {swap_id} to {to_state:?} (claim_tx_id = {claim_tx_id:?})"
        );

        let swap = self
            .persister
            .fetch_receive_swap(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Receive Swap not found {swap_id}"),
            })?;
        let payment_id = claim_tx_id.map(|c| c.to_string()).or(swap.claim_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister
            .try_handle_receive_swap_update(swap_id, to_state, claim_tx_id)?;

        Ok(self.emit_payment_updated(payment_id).await?)
    }

    /// Transitions a Send swap to a new state
    pub(crate) async fn try_handle_send_swap_update(
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
            .fetch_send_swap(swap_id)
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
        Ok(self.emit_payment_updated(payment_id).await?)
    }

    async fn emit_payment_updated(&self, payment_id: Option<String>) -> Result<()> {
        if let Some(id) = payment_id {
            match self.persister.get_payment(id.clone())? {
                Some(payment) => {
                    match payment.status {
                        Complete => {
                            self.notify_event_listeners(LiquidSdkEvent::PaymentSucceed {
                                details: payment,
                            })
                            .await?
                        }
                        Pending => {
                            // The swap state has changed to Pending
                            match payment.swap_id.clone() {
                                Some(swap_id) => match payment.payment_type {
                                    PaymentType::Receive => {
                                        match self.persister.fetch_receive_swap(&swap_id)? {
                                            Some(swap) => match swap.claim_tx_id {
                                                Some(_) => {
                                                    // The claim tx has now been broadcast
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentWaitingConfirmation {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                                None => {
                                                    // The lockup tx is in the mempool/confirmed
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                            },
                                            None => debug!("Swap not found: {swap_id}"),
                                        }
                                    }
                                    PaymentType::Send => {
                                        match self.persister.fetch_send_swap(&swap_id)? {
                                            Some(swap) => match swap.refund_tx_id {
                                                Some(_) => {
                                                    // The refund tx has now been broadcast
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentRefundPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                                None => {
                                                    // The lockup tx is in the mempool/confirmed
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                            },
                                            None => debug!("Swap not found: {swap_id}"),
                                        }
                                    }
                                },
                                None => debug!("Payment has no swap id"),
                            }
                        }
                        Failed => match payment.payment_type {
                            PaymentType::Receive => {
                                self.notify_event_listeners(LiquidSdkEvent::PaymentFailed {
                                    details: payment,
                                })
                                .await?
                            }
                            PaymentType::Send => {
                                // The refund tx is confirmed
                                self.notify_event_listeners(LiquidSdkEvent::PaymentRefunded {
                                    details: payment,
                                })
                                .await?
                            }
                        },
                        _ => (),
                    };
                }
                None => debug!("Payment not found: {id}"),
            }
        }
        Ok(())
    }

    /// Handles status updates from Boltz for Receive swaps
    pub(crate) async fn try_handle_receive_swap_boltz_status(
        &self,
        swap_state: &str,
        id: &str,
    ) -> Result<()> {
        let receive_swap = self
            .persister
            .fetch_receive_swap(id)?
            .ok_or(anyhow!("No ongoing Receive Swap found for ID {id}"))?;

        info!("Handling Receive Swap transition to {swap_state:?} for swap {id}");

        match RevSwapStates::from_str(swap_state) {
            Ok(RevSwapStates::SwapExpired
            | RevSwapStates::InvoiceExpired
            | RevSwapStates::TransactionFailed
            | RevSwapStates::TransactionRefunded) => {
                error!("Swap {id} entered into an unrecoverable state: {swap_state:?}");
                self.try_handle_receive_swap_update(id, Failed, None).await?;
                Ok(())
            }

            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // TODO Add 0-conf preconditions check: https://github.com/breez/breez-liquid-sdk/issues/187
            Ok(RevSwapStates::TransactionMempool
            // The lockup tx is confirmed => try to claim
            | RevSwapStates::TransactionConfirmed) => {
                match receive_swap.claim_tx_id {
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                    None => match self.try_claim(&receive_swap).await {
                        Ok(()) => {}
                        Err(err) => match err {
                            PaymentError::AlreadyClaimed => warn!("Funds already claimed for Receive Swap {id}"),
                            _ => error!("Claim for Receive Swap {id} failed: {err}")
                        }
                    },
                }
                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Receive Swap {id}: {swap_state}");
                Ok(())
            },

            _ => Err(anyhow!("Invalid RevSwapState for Receive Swap {id}: {swap_state}")),
        }
    }

    /// Handles status updates from Boltz for Send swaps
    pub(crate) async fn try_handle_send_swap_boltz_status(
        &self,
        swap_state: &str,
        id: &str,
    ) -> Result<()> {
        let ongoing_send_swap = self
            .persister
            .fetch_send_swap(id)?
            .ok_or(anyhow!("No ongoing Send Swap found for ID {id}"))?;

        info!("Handling Send Swap transition to {swap_state:?} for swap {id}");

        // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
        match SubSwapStates::from_str(swap_state) {
            // Boltz has locked the HTLC, we proceed with locking up the funds
            Ok(SubSwapStates::InvoiceSet) => {
                match (
                    ongoing_send_swap.state,
                    ongoing_send_swap.lockup_tx_id.clone(),
                ) {
                    (PaymentState::Created, None) => {
                        let create_response = ongoing_send_swap.get_boltz_create_response()?;
                        let lockup_tx_id = self.lockup_funds(id, &create_response).await?;

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: None,
                            amount_sat: ongoing_send_swap.payer_amount_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        })?;

                        self.try_handle_send_swap_update(
                            id,
                            Pending,
                            None,
                            Some(&lockup_tx_id),
                            None,
                        )
                        .await?;
                    }
                    (_, Some(lockup_tx_id)) => warn!(
                        "Lockup tx for Send Swap {id} was already broadcast: txid {lockup_tx_id}"
                    ),
                    (state, _) => {
                        debug!("Send Swap {id} is in an invalid state for {swap_state}: {state:?}")
                    }
                }
                Ok(())
            }

            // Boltz has detected the lockup in the mempool, we can speed up
            // the claim by doing so cooperatively
            Ok(SubSwapStates::TransactionClaimPending) => {
                let keypair = ongoing_send_swap.get_refund_keypair()?;
                let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
                    &ongoing_send_swap.get_boltz_create_response()?,
                    keypair.public_key().into(),
                )
                .map_err(|e| {
                    anyhow!("Could not rebuild refund details for Send Swap {id}: {e:?}")
                })?;

                self.cooperate_send_swap_claim(
                    id,
                    &swap_script,
                    &ongoing_send_swap.invoice,
                    &keypair,
                )
                .await
                .map_err(|e| anyhow!("Could not post claim details. Err: {e:?}"))?;

                Ok(())
            }

            Ok(SubSwapStates::TransactionClaimed) => {
                debug!("Send Swap {id} has been claimed");
                let preimage =
                    self.get_preimage_from_script_path_claim_spend(&ongoing_send_swap)?;
                self.validate_send_swap_preimage(id, &ongoing_send_swap.invoice, &preimage)
                    .await?;
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
                match ongoing_send_swap.refund_tx_id {
                    Some(refund_tx_id) => warn!(
                        "Refund tx for Send Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                    None => {
                        warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}");

                        let refund_tx_id = self.try_refund(&ongoing_send_swap).await?;
                        info!("Broadcast refund tx for Send Swap {id}. Tx id: {refund_tx_id}");
                        self.try_handle_send_swap_update(
                            id,
                            Pending,
                            None,
                            None,
                            Some(&refund_tx_id),
                        )
                        .await?;
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

    /// Gets the next unused onchain Liquid address
    async fn next_unused_address(&self) -> Result<Address, lwk_wollet::Error> {
        let lwk_wollet = self.lwk_wollet.lock().await;
        Ok(lwk_wollet.address(None)?.address().clone())
    }

    pub async fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse> {
        self.ensure_is_started().await?;

        debug!("next_unused_address: {}", self.next_unused_address().await?);
        if req.with_scan {
            self.sync().await?;
        }

        let mut pending_send_sat = 0;
        let mut pending_receive_sat = 0;
        let mut confirmed_sent_sat = 0;
        let mut confirmed_received_sat = 0;

        for p in self.list_payments().await? {
            match p.payment_type {
                PaymentType::Send => match p.status {
                    Complete => confirmed_sent_sat += p.amount_sat,
                    Failed => {
                        confirmed_sent_sat += p.amount_sat;
                        confirmed_received_sat += p.refund_tx_amount_sat.unwrap_or_default();
                    }
                    Pending => match p.refund_tx_amount_sat {
                        Some(refund_tx_amount_sat) => {
                            confirmed_sent_sat += p.amount_sat;
                            pending_receive_sat += refund_tx_amount_sat;
                        }
                        None => pending_send_sat += p.amount_sat,
                    },
                    Created => pending_send_sat += p.amount_sat,
                },
                PaymentType::Receive => match p.status {
                    Complete => confirmed_received_sat += p.amount_sat,
                    Failed => {}
                    _ => pending_receive_sat += p.amount_sat,
                },
            }
        }

        Ok(GetInfoResponse {
            balance_sat: confirmed_received_sat - confirmed_sent_sat - pending_send_sat,
            pending_send_sat,
            pending_receive_sat,
            pubkey: self.lwk_signer.xpub().public_key.to_string(),
        })
    }

    pub(crate) fn boltz_client_v2(&self) -> BoltzApiClientV2 {
        BoltzApiClientV2::new(Self::boltz_url_v2(self.network))
    }

    pub(crate) fn boltz_url_v2(network: Network) -> &'static str {
        match network {
            Network::Testnet => BOLTZ_TESTNET_URL_V2,
            Network::Mainnet => BOLTZ_MAINNET_URL_V2,
        }
    }

    fn network_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.network.into(),
            &self.electrum_url.to_string(),
            true,
            true,
            100,
        )
    }

    async fn build_tx(
        &self,
        fee_rate: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.lwk_wollet.lock().await;
        let mut pset = lwk_wollet::TxBuilder::new(self.network.into())
            .add_lbtc_recipient(
                &ElementsAddress::from_str(recipient_address).map_err(|e| {
                    PaymentError::Generic {
                        err: format!(
                            "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                        ),
                    }
                })?,
                amount_sat,
            )?
            .fee_rate(fee_rate)
            .finish(&lwk_wollet)?;
        let signer = AnySigner::Software(self.lwk_signer.clone());
        signer.sign(&mut pset)?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    fn validate_invoice(&self, invoice: &str) -> Result<Bolt11Invoice, PaymentError> {
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|_| PaymentError::InvalidInvoice)?;

        match (invoice.network().to_string().as_str(), self.network) {
            ("bitcoin", Network::Mainnet) => {}
            ("testnet", Network::Testnet) => {}
            _ => return Err(PaymentError::InvalidInvoice),
        }

        ensure_sdk!(!invoice.is_expired(), PaymentError::InvalidInvoice);

        Ok(invoice)
    }

    fn validate_submarine_pairs(
        client: &BoltzApiClientV2,
        receiver_amount_sat: u64,
    ) -> Result<SubmarinePair, PaymentError> {
        let lbtc_pair = client
            .get_submarine_pairs()?
            .get_lbtc_to_btc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(receiver_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(receiver_amount_sat);

        ensure_sdk!(
            receiver_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(lbtc_pair)
    }

    async fn get_broadcast_fee_estimation(&self, amount_sat: u64) -> Result<u64> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.network {
            Network::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            Network::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        };

        // Create a throw-away tx similar to the lockup tx, in order to estimate fees
        Ok(self
            .build_tx(None, temp_p2tr_addr, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }

    pub async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.ensure_is_started().await?;

        let invoice = self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;

        let client = self.boltz_client_v2();
        let lbtc_pair = Self::validate_submarine_pairs(&client, receiver_amount_sat)?;

        let broadcast_fees_sat = self
            .get_broadcast_fee_estimation(receiver_amount_sat)
            .await?;

        Ok(PrepareSendResponse {
            invoice: req.invoice.clone(),
            fees_sat: lbtc_pair.fees.total(receiver_amount_sat) + broadcast_fees_sat,
        })
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice = Bolt11Invoice::from_str(invoice).map_err(|_| PaymentError::InvalidInvoice)?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
    }

    async fn new_refund_tx(
        &self,
        swap_id: &str,
        swap_script: &LBtcSwapScriptV2,
    ) -> Result<LBtcSwapTxV2, PaymentError> {
        let output_address = self.next_unused_address().await?.to_string();
        let network_config = self.network_config();
        Ok(LBtcSwapTxV2::new_refund(
            swap_script.clone(),
            &output_address,
            &network_config,
            Self::boltz_url_v2(self.network).to_string(),
            swap_id.to_string(),
        )?)
    }

    async fn try_refund_cooperative(
        &self,
        swap: &SendSwap,
        refund_tx: &LBtcSwapTxV2,
        broadcast_fees_sat: Amount,
        is_lowball: Option<(&BoltzApiClientV2, Chain)>,
    ) -> Result<String, PaymentError> {
        info!("Initiating cooperative refund for Send Swap {}", &swap.id);
        let tx = refund_tx.sign_refund(
            &swap.get_refund_keypair()?,
            broadcast_fees_sat,
            Some((&self.boltz_client_v2(), &swap.id)),
        )?;

        let refund_tx_id = refund_tx.broadcast(&tx, &self.network_config(), is_lowball)?;
        info!(
            "Successfully broadcast cooperative refund for Send Swap {}",
            &swap.id
        );
        Ok(refund_tx_id.clone())
    }

    async fn try_refund_non_cooperative(
        &self,
        swap: &SendSwap,
        swap_script: &LBtcSwapScriptV2,
        refund_tx: LBtcSwapTxV2,
        broadcast_fees_sat: Amount,
        is_lowball: Option<(&BoltzApiClientV2, Chain)>,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Send Swap {}",
            &swap.id
        );

        let current_height = self.lwk_wollet.lock().await.tip().height();
        let locktime_from_height =
            LockTime::from_height(current_height).map_err(|e| PaymentError::Generic {
                err: format!("Cannot convert current block height to lock time: {e:?}"),
            })?;

        info!("locktime info: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}",  swap_script.locktime);
        let is_locktime_satisfied = match (locktime_from_height, swap_script.locktime) {
            (Blocks(n), Blocks(lock_time)) => n >= lock_time,
            (Seconds(n), Seconds(lock_time)) => n >= lock_time,
            _ => false, // Not using the same units
        };
        if !is_locktime_satisfied {
            return Err(PaymentError::Generic {
                err: format!(
                    "Cannot refund non-cooperatively. Lock time not elapsed yet. Current tip: {:?}. Script lock time: {:?}",
                    locktime_from_height, swap_script.locktime
                )
            });
        }

        let tx = refund_tx.sign_refund(&swap.get_refund_keypair()?, broadcast_fees_sat, None)?;
        let refund_tx_id = refund_tx.broadcast(&tx, &self.network_config(), is_lowball)?;
        info!(
            "Successfully broadcast non-cooperative refund for Send Swap {}",
            swap.id
        );
        Ok(refund_tx_id)
    }

    async fn try_refund(&self, swap: &SendSwap) -> Result<String, PaymentError> {
        let id = &swap.id;
        let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
            &swap.get_boltz_create_response()?,
            swap.get_refund_keypair()?.public_key().into(),
        )
        .map_err(|e| anyhow!("Could not rebuild refund details for Send Swap {id}: {e:?}"))?;

        let refund_tx = self.new_refund_tx(&swap.id, &swap_script).await?;
        let amount_sat = get_invoice_amount!(swap.invoice);
        let broadcast_fees_sat =
            Amount::from_sat(self.get_broadcast_fee_estimation(amount_sat).await?);
        let client = self.boltz_client_v2();
        let is_lowball = match self.network {
            Network::Mainnet => None,
            Network::Testnet => Some((&client, boltz_client::network::Chain::LiquidTestnet)),
        };

        match self
            .try_refund_cooperative(swap, &refund_tx, broadcast_fees_sat, is_lowball)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Cooperative refund failed: {:?}", e);
                self.try_refund_non_cooperative(
                    swap,
                    &swap_script,
                    refund_tx,
                    broadcast_fees_sat,
                    is_lowball,
                )
                .await
            }
        }
    }

    /// Check if the provided preimage matches our invoice. If so, mark the Send payment as [Complete].
    async fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        self.try_handle_send_swap_update(swap_id, Complete, Some(preimage), None, None)
            .await
    }

    /// Interact with Boltz to assist in them doing a cooperative claim
    async fn cooperate_send_swap_claim(
        &self,
        swap_id: &str,
        swap_script: &LBtcSwapScriptV2,
        invoice: &str,
        keypair: &Keypair,
    ) -> Result<(), PaymentError> {
        debug!("Claim is pending for Send Swap {swap_id}. Initiating cooperative claim");
        let client = self.boltz_client_v2();
        let refund_tx = self.new_refund_tx(swap_id, swap_script).await?;

        let claim_tx_response = client.get_claim_tx_details(&swap_id.to_string())?;
        debug!("Received claim tx details: {:?}", &claim_tx_response);

        self.validate_send_swap_preimage(swap_id, invoice, &claim_tx_response.preimage)
            .await?;

        let (partial_sig, pub_nonce) =
            refund_tx.submarine_partial_sig(keypair, &claim_tx_response)?;

        client.post_claim_tx_details(&swap_id.to_string(), pub_nonce, partial_sig)?;
        debug!("Successfully sent claim details for Send Swap {swap_id}");
        Ok(())
    }

    async fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateSubmarineResponse,
    ) -> Result<String, PaymentError> {
        debug!(
            "Initiated Send Swap: send {} sats to liquid address {}",
            create_response.expected_amount, create_response.address
        );

        let lockup_tx = self
            .build_tx(
                None,
                &create_response.address,
                create_response.expected_amount,
            )
            .await?;

        let electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let lockup_tx_id = electrum_client.broadcast(&lockup_tx)?.to_string();

        debug!(
            "Successfully broadcast lockup transaction for Send Swap {swap_id}. Lockup tx id: {lockup_tx_id}"
        );
        Ok(lockup_tx_id)
    }

    pub async fn send_payment(
        &self,
        req: &PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = get_invoice_amount!(req.invoice);

        let client = self.boltz_client_v2();
        let lbtc_pair = Self::validate_submarine_pairs(&client, receiver_amount_sat)?;
        let broadcast_fees_sat = self
            .get_broadcast_fee_estimation(receiver_amount_sat)
            .await?;
        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.total(receiver_amount_sat) + broadcast_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let keypair = utils::generate_keypair();
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: keypair.public_key(),
        };

        let create_response = client.post_swap_req(&CreateSubmarineRequest {
            from: "L-BTC".to_string(),
            to: "BTC".to_string(),
            invoice: req.invoice.to_string(),
            refund_public_key,
            pair_hash: Some(lbtc_pair.hash),
            referral_id: None,
        })?;

        let swap_id = &create_response.id;
        let accept_zero_conf = create_response.accept_zero_conf;
        let create_response_json = SendSwap::from_boltz_struct_to_json(&create_response, swap_id)?;

        let payer_amount_sat = req.fees_sat + receiver_amount_sat;
        let swap = SendSwap {
            id: swap_id.clone(),
            invoice: req.invoice.clone(),
            payer_amount_sat,
            receiver_amount_sat,
            create_response_json,
            lockup_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: PaymentState::Created,
            refund_private_key: keypair.display_secret().to_string(),
        };
        self.persister.insert_send_swap(&swap)?;
        self.status_stream.track_swap_id(swap_id)?;

        self.wait_for_payment(swap.id, accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    async fn wait_for_payment(
        &self,
        swap_id: String,
        accept_zero_conf: bool,
    ) -> Result<Payment, PaymentError> {
        let timeout_fut = tokio::time::sleep(Duration::from_secs(15));
        tokio::pin!(timeout_fut);

        let mut events_stream = self.event_manager.subscribe();
        let mut maybe_payment: Option<Payment> = None;

        loop {
            tokio::select! {
                _ = &mut timeout_fut => match maybe_payment {
                    Some(payment) => return Ok(payment),
                    None => return Err(PaymentError::PaymentTimeout),
                },
                event = events_stream.recv() => match event {
                    Ok(LiquidSdkEvent::PaymentPending { details }) => match details.swap_id.clone() {
                        Some(id) if id == swap_id => match accept_zero_conf {
                            true => {
                                debug!("Received Send Payment pending event with zero-conf accepted");
                                return Ok(details)
                            }
                            false => {
                                debug!("Received Send Payment pending event, waiting for confirmation");
                                maybe_payment = Some(details);
                            }
                        },
                        _ => error!("Received Send Payment pending event for payment without swap ID"),
                    },
                    Ok(LiquidSdkEvent::PaymentSucceed { details }) => match details.swap_id.clone()
                    {
                        Some(id) if id == swap_id => {
                            debug!("Received Send Payment succeed event");
                            return Ok(details);
                        }
                        _ => error!("Received Send Payment succeed event for payment without swap ID"),
                    },
                    Ok(event) => debug!("Unhandled event: {event:?}"),
                    Err(e) => debug!("Received error waiting for event: {e:?}"),
                }
            }
        }
    }

    async fn try_claim(&self, ongoing_receive_swap: &ReceiveSwap) -> Result<(), PaymentError> {
        ensure_sdk!(
            ongoing_receive_swap.claim_tx_id.is_none(),
            PaymentError::AlreadyClaimed
        );

        let swap_id = &ongoing_receive_swap.id;
        debug!("Trying to claim Receive Swap {swap_id}",);

        self.try_handle_receive_swap_update(swap_id, Pending, None)
            .await?;

        let keypair = ongoing_receive_swap.get_claim_keypair()?;
        let create_response = ongoing_receive_swap.get_boltz_create_response()?;
        let swap_script = LBtcSwapScriptV2::reverse_from_swap_resp(
            &create_response,
            keypair.public_key().into(),
        )?;

        let claim_address = self.next_unused_address().await?.to_string();
        let claim_tx_wrapper = LBtcSwapTxV2::new_claim(
            swap_script,
            claim_address,
            &self.network_config(),
            Self::boltz_url_v2(self.network).into(),
            ongoing_receive_swap.id.clone(),
        )?;

        let claim_tx = claim_tx_wrapper.sign_claim(
            &keypair,
            &Preimage::from_str(&ongoing_receive_swap.preimage)?,
            Amount::from_sat(ongoing_receive_swap.claim_fees_sat),
            // Enable cooperative claim (Some) or not (None)
            Some((&self.boltz_client_v2(), swap_id.clone())),
            // None
        )?;

        let claim_tx_id = claim_tx_wrapper.broadcast(
            &claim_tx,
            &self.network_config(),
            Some((&self.boltz_client_v2(), self.network.into())),
        )?;
        info!("Successfully broadcast claim tx {claim_tx_id} for Receive Swap {swap_id}");
        debug!("Claim Tx {:?}", claim_tx);

        // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        self.persister.insert_or_update_payment(PaymentTxData {
            tx_id: claim_tx_id.clone(),
            timestamp: None,
            amount_sat: ongoing_receive_swap.receiver_amount_sat,
            payment_type: PaymentType::Receive,
            is_confirmed: false,
        })?;

        self.try_handle_receive_swap_update(swap_id, Pending, Some(&claim_tx_id))
            .await?;

        Ok(())
    }

    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.ensure_is_started().await?;

        let reverse_pair = self
            .boltz_client_v2()
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = reverse_pair.fees.total(req.payer_amount_sat);

        ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

        reverse_pair
            .limits
            .within(payer_amount_sat)
            .map_err(|_| PaymentError::AmountOutOfRange)?;

        debug!("Preparing Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        Ok(PrepareReceiveResponse {
            payer_amount_sat,
            fees_sat,
        })
    }

    pub async fn receive_payment(
        &self,
        req: &PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = req.fees_sat;

        let reverse_pair = self
            .boltz_client_v2()
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;
        let new_fees_sat = reverse_pair.fees.total(req.payer_amount_sat);
        ensure_sdk!(fees_sat == new_fees_sat, PaymentError::InvalidOrExpiredFees);

        debug!("Creating Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        let keypair = utils::generate_keypair();

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        let v2_req = CreateReverseRequest {
            invoice_amount: req.payer_amount_sat as u32, // TODO update our model
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: keypair.public_key().into(),
            address: None,
            address_signature: None,
            referral_id: None,
        };
        let create_response = self.boltz_client_v2().post_reverse_req(v2_req)?;

        let swap_id = create_response.id.clone();
        let invoice = Bolt11Invoice::from_str(&create_response.invoice)
            .map_err(|_| PaymentError::InvalidInvoice)?;
        let payer_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::InvalidInvoice)?
            / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(PaymentError::InvalidInvoice);
        };

        let create_response_json = ReceiveSwap::from_boltz_struct_to_json(
            &create_response,
            &swap_id,
            &invoice.to_string(),
        )?;
        self.persister
            .insert_receive_swap(&ReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                create_response_json,
                claim_private_key: keypair.display_secret().to_string(),
                invoice: invoice.to_string(),
                payer_amount_sat,
                receiver_amount_sat: payer_amount_sat - req.fees_sat,
                claim_fees_sat: reverse_pair.fees.claim_estimate(),
                claim_tx_id: None,
                created_at: utils::now(),
                state: PaymentState::Created,
            })
            .map_err(|_| PaymentError::PersistError)?;
        self.status_stream.track_swap_id(&swap_id)?;

        Ok(ReceivePaymentResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        if with_scan {
            let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
            let mut lwk_wollet = self.lwk_wollet.lock().await;
            lwk_wollet::full_scan_with_electrum_client(&mut lwk_wollet, &mut electrum_client)?;
        }

        let pending_receive_swaps_by_claim_tx_id =
            self.persister.list_pending_receive_swaps_by_claim_tx_id()?;
        let pending_send_swaps_by_refund_tx_id =
            self.persister.list_pending_send_swaps_by_refund_tx_id()?;

        for tx in self.lwk_wollet.lock().await.transactions()? {
            let tx_id = tx.txid.to_string();
            let is_tx_confirmed = tx.height.is_some();
            let amount_sat = tx.balance.values().sum::<i64>();

            // Transition the swaps whose state depends on this tx being confirmed
            if is_tx_confirmed {
                if let Some(swap) = pending_receive_swaps_by_claim_tx_id.get(&tx_id) {
                    self.try_handle_receive_swap_update(&swap.id, Complete, None)
                        .await?;
                }
                if let Some(swap) = pending_send_swaps_by_refund_tx_id.get(&tx_id) {
                    self.try_handle_send_swap_update(&swap.id, Failed, None, None, None)
                        .await?;
                }
            }

            self.persister.insert_or_update_payment(PaymentTxData {
                tx_id,
                timestamp: tx.timestamp,
                amount_sat: amount_sat.unsigned_abs(),
                payment_type: match amount_sat >= 0 {
                    true => PaymentType::Receive,
                    false => PaymentType::Send,
                },
                is_confirmed: is_tx_confirmed,
            })?;
        }

        Ok(())
    }

    fn get_preimage_from_script_path_claim_spend(
        &self,
        swap: &SendSwap,
    ) -> Result<String, PaymentError> {
        info!("Retrieving preimage from non-cooperative claim tx");

        let id = &swap.id;
        let keypair = swap.get_refund_keypair()?;
        let create_response = swap.get_boltz_create_response()?;
        let electrum_client = ElectrumClient::new(&self.electrum_url)?;

        let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
            &create_response,
            keypair.public_key().into(),
        )?;
        let swap_script_pk = swap_script.to_address(self.network.into())?.script_pubkey();
        debug!("Found Send Swap swap_script_pk: {swap_script_pk:?}");

        // Get tx history of the swap script (lockup address)
        let history: Vec<_> = electrum_client
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

                let claim_tx = electrum_client
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

    /// Lists the SDK payments. The payments are determined based on onchain transactions and swaps.
    pub async fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        let mut payments: Vec<Payment> = self.persister.get_payments()?.values().cloned().collect();
        payments.sort_by_key(|p| p.timestamp);
        Ok(payments)
    }

    /// Empties all Liquid Wallet caches for this network type.
    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.data_dir_path.clone());
        path.push(Into::<ElementsNetwork>::into(self.network).as_str());
        path.push("enc_cache");

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Synchronize the DB with mempool and onchain data
    pub async fn sync(&self) -> LiquidSdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();
        self.sync_payments_with_chain_data(true).await?;
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized with mempool and onchain data (t = {duration_ms} ms)");

        self.notify_event_listeners(LiquidSdkEvent::Synced).await?;
        Ok(())
    }

    pub fn backup(&self, req: BackupRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.backup(backup_path)
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.restore_from_backup(backup_path)
    }

    pub fn parse_invoice(input: &str) -> Result<LNInvoice, PaymentError> {
        let input = input
            .strip_prefix("lightning:")
            .or(input.strip_prefix("LIGHTNING:"))
            .unwrap_or(input);
        let invoice = Bolt11Invoice::from_str(input).map_err(|_| PaymentError::InvalidInvoice)?;

        // Try to take payee pubkey from the tagged fields, if doesn't exist recover it from the signature
        let payee_pubkey: String = match invoice.payee_pub_key() {
            Some(key) => key.serialize().to_hex(),
            None => invoice.recover_payee_pub_key().serialize().to_hex(),
        };
        let description = match invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };
        let description_hash = match invoice.description() {
            Bolt11InvoiceDescription::Direct(_) => None,
            Bolt11InvoiceDescription::Hash(h) => Some(h.0.to_string()),
        };
        let timestamp = invoice
            .timestamp()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| PaymentError::InvalidInvoice)?
            .as_secs();
        let routing_hints = invoice
            .route_hints()
            .iter()
            .map(RouteHint::from_ldk_hint)
            .collect();

        let res = LNInvoice {
            bolt11: input.to_string(),
            network: invoice.currency().try_into()?,
            payee_pubkey,
            payment_hash: invoice.payment_hash().to_hex(),
            description,
            description_hash,
            amount_msat: invoice.amount_milli_satoshis(),
            timestamp,
            expiry: invoice.expiry_time().as_secs(),
            routing_hints,
            payment_secret: invoice.payment_secret().0.to_vec(),
            min_final_cltv_expiry_delta: invoice.min_final_cltv_expiry_delta(),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::model::*;
    use crate::sdk::{LiquidSdk, Network};

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn create_temp_dir() -> Result<(TempDir, String)> {
        let data_dir = TempDir::new(&uuid::Uuid::new_v4().to_string())?;
        let data_dir_str = data_dir
            .as_ref()
            .to_path_buf()
            .to_str()
            .expect("Expecting valid temporary path")
            .to_owned();
        Ok((data_dir, data_dir_str))
    }

    async fn list_pending(sdk: &LiquidSdk) -> Result<Vec<Payment>> {
        let payments = sdk.list_payments().await?;

        Ok(payments
            .iter()
            .filter(|p| matches!(&p.status, PaymentState::Pending))
            .cloned()
            .collect())
    }

    #[tokio::test]
    async fn normal_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            data_dir: Some(data_dir_str),
            network: Network::Testnet,
        })
        .await?;

        let invoice = "lntb10u1pnqwkjrpp5j8ucv9mgww0ajk95yfpvuq0gg5825s207clrzl5thvtuzfn68h0sdqqcqzzsxqr23srzjqv8clnrfs9keq3zlg589jvzpw87cqh6rjks0f9g2t9tvuvcqgcl45f6pqqqqqfcqqyqqqqlgqqqqqqgq2qsp5jnuprlxrargr6hgnnahl28nvutj3gkmxmmssu8ztfhmmey3gq2ss9qyyssq9ejvcp6frwklf73xvskzdcuhnnw8dmxag6v44pffwqrxznsly4nqedem3p3zhn6u4ln7k79vk6zv55jjljhnac4gnvr677fyhfgn07qp4x6wrq".to_string();
        sdk.prepare_send_payment(&PrepareSendRequest { invoice })
            .await?;
        assert!(!list_pending(&sdk).await?.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn reverse_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            data_dir: Some(data_dir_str),
            network: Network::Testnet,
        })
        .await?;

        let prepare_response = sdk
            .prepare_receive_payment(&PrepareReceiveRequest {
                payer_amount_sat: 1_000,
            })
            .await?;
        sdk.receive_payment(&prepare_response).await?;
        assert!(!list_pending(&sdk).await?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
