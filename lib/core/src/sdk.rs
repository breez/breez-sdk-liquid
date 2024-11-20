use std::collections::HashMap;
use std::time::Instant;
use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use boltz_client::{swaps::boltz::*, util::secrets::Preimage};
use buy::{BuyBitcoinApi, BuyBitcoinService};
use chain::bitcoin::HybridBitcoinChainService;
use chain::liquid::{HybridLiquidChainService, LiquidChainService};
use chain_swap::ESTIMATED_BTC_CLAIM_TX_VSIZE;
use futures_util::stream::select_all;
use futures_util::{StreamExt, TryFutureExt};
use lnurl::auth::SdkLnurlAuthSigner;
use log::{debug, error, info, warn};
use lwk_wollet::bitcoin::base64::Engine as _;
use lwk_wollet::elements::{AssetId, Txid};
use lwk_wollet::elements_miniscript::elements::bitcoin::bip32::Xpub;
use lwk_wollet::hashes::{sha256, Hash};
use lwk_wollet::secp256k1::ThirtyTwoByteHash;
use lwk_wollet::{ElementsNetwork, WalletTx};
use sdk_common::bitcoin::hashes::hex::ToHex;
use sdk_common::input_parser::InputType;
use sdk_common::liquid::LiquidAddressData;
use sdk_common::prelude::{FiatAPI, FiatCurrency, LnUrlPayError, LnUrlWithdrawError, Rate};
use signer::SdkSigner;
use tokio::sync::{watch, Mutex, RwLock};
use tokio::time::MissedTickBehavior;
use tokio_stream::wrappers::BroadcastStream;
use x509_parser::parse_x509_certificate;

use crate::chain::bitcoin::BitcoinChainService;
use crate::chain_swap::ChainSwapHandler;
use crate::ensure_sdk;
use crate::error::SdkError;
use crate::lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
use crate::model::PaymentState::*;
use crate::model::Signer;
use crate::receive_swap::ReceiveSwapHandler;
use crate::send_swap::SendSwapHandler;
use crate::swapper::{boltz::BoltzSwapper, Swapper, SwapperReconnectHandler, SwapperStatusStream};
use crate::wallet::{LiquidOnchainWallet, OnchainWallet};
use crate::{
    error::{PaymentError, SdkResult},
    event::EventManager,
    model::*,
    persist::Persister,
    utils, *,
};
use ::lightning::offers::invoice::Bolt12Invoice;
use ::lightning::offers::offer::Offer;

pub const DEFAULT_DATA_DIR: &str = ".data";
/// Number of blocks to monitor a swap after its timeout block height
pub const CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS: u32 = 4320;

pub struct LiquidSdk {
    pub(crate) config: Config,
    pub(crate) onchain_wallet: Arc<dyn OnchainWallet>,
    pub(crate) signer: Arc<Box<dyn Signer>>,
    pub(crate) persister: Arc<Persister>,
    pub(crate) event_manager: Arc<EventManager>,
    pub(crate) status_stream: Arc<dyn SwapperStatusStream>,
    pub(crate) swapper: Arc<dyn Swapper>,
    // TODO: Remove field if unnecessary
    #[allow(dead_code)]
    pub(crate) liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    pub(crate) bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    pub(crate) fiat_api: Arc<dyn FiatAPI>,
    pub(crate) is_started: RwLock<bool>,
    pub(crate) shutdown_sender: watch::Sender<()>,
    pub(crate) shutdown_receiver: watch::Receiver<()>,
    pub(crate) send_swap_handler: SendSwapHandler,
    pub(crate) receive_swap_handler: ReceiveSwapHandler,
    pub(crate) chain_swap_handler: Arc<ChainSwapHandler>,
    pub(crate) buy_bitcoin_service: Arc<dyn BuyBitcoinApi>,
}

impl LiquidSdk {
    /// Initializes the SDK services and starts the background tasks.
    /// This must be called to create the [LiquidSdk] instance.
    ///
    /// # Arguments
    ///
    /// * `req` - the [ConnectRequest] containing:
    ///     * `mnemonic` - the Liquid wallet mnemonic
    ///     * `config` - the SDK [Config]
    pub async fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let signer = Box::new(SdkSigner::new(
            req.mnemonic.as_ref(),
            req.config.network == LiquidNetwork::Mainnet,
        )?);

        Self::connect_with_signer(ConnectWithSignerRequest { config: req.config }, signer)
            .inspect_err(|e| error!("Failed to connect: {:?}", e))
            .await
    }

    pub async fn connect_with_signer(
        req: ConnectWithSignerRequest,
        signer: Box<dyn Signer>,
    ) -> Result<Arc<LiquidSdk>> {
        let maybe_swapper_proxy_url =
            match BreezServer::new("https://bs1.breez.technology:443".into(), None) {
                Ok(breez_server) => breez_server
                    .fetch_boltz_swapper_urls()
                    .await
                    .ok()
                    .and_then(|swapper_urls| swapper_urls.first().cloned()),
                Err(_) => None,
            };
        let sdk = LiquidSdk::new(req.config, maybe_swapper_proxy_url, Arc::new(signer))?;
        sdk.start()
            .inspect_err(|e| error!("Failed to start an SDK instance: {:?}", e))
            .await?;
        Ok(sdk)
    }

    fn validate_api_key(api_key: &str) -> Result<()> {
        let api_key_decoded = lwk_wollet::bitcoin::base64::engine::general_purpose::STANDARD
            .decode(api_key.as_bytes())
            .map_err(|err| anyhow!("Could not base64 decode the Breez API key: {err:?}"))?;
        let (_rem, cert) = parse_x509_certificate(&api_key_decoded)
            .map_err(|err| anyhow!("Invaid certificate for Breez API key: {err:?}"))?;

        let issuer = cert
            .issuer()
            .iter_common_name()
            .next()
            .and_then(|cn| cn.as_str().ok());
        match issuer {
            Some(common_name) => ensure_sdk!(
                common_name.starts_with("Breez"),
                anyhow!("Invalid certificate found for Breez API key: issuer mismatch. Please confirm that the certificate's origin is trusted")
            ),
            _ => {
                return Err(anyhow!("Could not parse Breez API key certificate: issuer is invalid or not found."))
            }
        }

        Ok(())
    }

    fn new(
        config: Config,
        swapper_proxy_url: Option<String>,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Arc<Self>> {
        match (config.network, &config.breez_api_key) {
            (_, Some(api_key)) => Self::validate_api_key(api_key)?,
            (LiquidNetwork::Mainnet, None) => {
                return Err(anyhow!("Breez API key must be provided on mainnet."));
            }
            (LiquidNetwork::Testnet, None) => {}
        };

        fs::create_dir_all(&config.working_dir)?;
        let fingerprint_hex: String =
            Xpub::decode(signer.xpub()?.as_slice())?.identifier()[0..4].to_hex();
        let working_dir = config.get_wallet_dir(&config.working_dir, &fingerprint_hex)?;
        let cache_dir = config.get_wallet_dir(
            config.cache_dir.as_ref().unwrap_or(&config.working_dir),
            &fingerprint_hex,
        )?;

        let persister = Arc::new(Persister::new(&working_dir, config.network)?);
        persister.init()?;

        let onchain_wallet = Arc::new(LiquidOnchainWallet::new(
            config.clone(),
            &cache_dir,
            persister.clone(),
            signer.clone(),
        )?);

        let event_manager = Arc::new(EventManager::new());
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        if let Some(swapper_proxy_url) = swapper_proxy_url {
            persister.set_swapper_proxy_url(swapper_proxy_url)?;
        }
        let cached_swapper_proxy_url = persister.get_swapper_proxy_url()?;
        let swapper = Arc::new(BoltzSwapper::new(config.clone(), cached_swapper_proxy_url));
        let status_stream = Arc::<dyn SwapperStatusStream>::from(swapper.create_status_stream());

        let liquid_chain_service =
            Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));
        let bitcoin_chain_service =
            Arc::new(Mutex::new(HybridBitcoinChainService::new(config.clone())?));

        let send_swap_handler = SendSwapHandler::new(
            config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
        );

        let receive_swap_handler = ReceiveSwapHandler::new(
            config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
        );

        let chain_swap_handler = Arc::new(ChainSwapHandler::new(
            config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
        )?);

        let breez_server = Arc::new(BreezServer::new(PRODUCTION_BREEZSERVER_URL.into(), None)?);

        let buy_bitcoin_service =
            Arc::new(BuyBitcoinService::new(config.clone(), breez_server.clone()));

        let sdk = Arc::new(LiquidSdk {
            config: config.clone(),
            onchain_wallet,
            signer: signer.clone(),
            persister: persister.clone(),
            event_manager,
            status_stream: status_stream.clone(),
            swapper,
            bitcoin_chain_service,
            liquid_chain_service,
            fiat_api: breez_server,
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
            send_swap_handler,
            receive_swap_handler,
            chain_swap_handler,
            buy_bitcoin_service,
        });
        Ok(sdk)
    }

    /// Starts an SDK instance.
    ///
    /// Internal method. Should only be called once per instance.
    /// Should only be called as part of [LiquidSdk::connect].
    async fn start(self: &Arc<LiquidSdk>) -> SdkResult<()> {
        let mut is_started = self.is_started.write().await;
        let start_ts = Instant::now();

        self.persister
            .update_send_swaps_by_state(Created, TimedOut)
            .inspect_err(|e| error!("Failed to update send swaps by state: {:?}", e))?;

        self.start_background_tasks()
            .inspect_err(|e| error!("Failed to start background tasks: {:?}", e))
            .await?;
        *is_started = true;

        let start_duration = start_ts.elapsed();
        info!("Liquid SDK initialized in: {start_duration:?}");
        Ok(())
    }

    /// Starts background tasks.
    ///
    /// Internal method. Should only be used as part of [LiquidSdk::start].
    async fn start_background_tasks(self: &Arc<LiquidSdk>) -> SdkResult<()> {
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

        let reconnect_handler = Box::new(SwapperReconnectHandler::new(
            self.persister.clone(),
            self.status_stream.clone(),
        ));
        self.status_stream
            .clone()
            .start(reconnect_handler, self.shutdown_receiver.clone())
            .await;
        self.chain_swap_handler
            .clone()
            .start(self.shutdown_receiver.clone())
            .await;
        self.track_swap_updates().await;
        self.track_pending_swaps().await;

        Ok(())
    }

    async fn ensure_is_started(&self) -> SdkResult<()> {
        let is_started = self.is_started.read().await;
        ensure_sdk!(*is_started, SdkError::NotStarted);
        Ok(())
    }

    /// Disconnects the [LiquidSdk] instance and stops the background tasks.
    pub async fn disconnect(&self) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let mut is_started = self.is_started.write().await;
        self.shutdown_sender
            .send(())
            .map_err(|e| SdkError::generic(format!("Shutdown failed: {e}")))?;
        *is_started = false;
        Ok(())
    }

    async fn track_swap_updates(self: &Arc<LiquidSdk>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut updates_stream = cloned.status_stream.subscribe_swap_updates();
            let swaps_streams = vec![
                cloned.send_swap_handler.subscribe_payment_updates(),
                cloned.receive_swap_handler.subscribe_payment_updates(),
                cloned.chain_swap_handler.subscribe_payment_updates(),
            ];
            let mut combined_swap_streams =
                select_all(swaps_streams.into_iter().map(BroadcastStream::new));
            loop {
                tokio::select! {
                    payment_id = combined_swap_streams.next() => {
                      if let Some(payment_id) = payment_id {
                        match payment_id {
                            Ok(payment_id) => {
                              if let Err(e) = cloned.emit_payment_updated(Some(payment_id)).await {
                                error!("Failed to emit payment update: {e:?}");
                              }
                            }
                            Err(e) => error!("Failed to receive swap state change: {e:?}")
                        }
                      }
                    }
                    update = updates_stream.recv() => match update {
                        Ok(update) => {
                            let id = &update.id;
                            match cloned.persister.fetch_swap_by_id(id) {
                                Ok(Swap::Send(_)) => match cloned.send_swap_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Send Swap {id} update"),
                                    Err(e) => error!("Failed to handle Send Swap {id} update: {e}")
                                },
                                Ok(Swap::Receive(_)) => match cloned.receive_swap_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Receive Swap {id} update"),
                                    Err(e) => error!("Failed to handle Receive Swap {id} update: {e}")
                                },
                                Ok(Swap::Chain(_)) => match cloned.chain_swap_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Chain Swap {id} update"),
                                    Err(e) => error!("Failed to handle Chain Swap {id} update: {e}")
                                },
                                _ => {
                                    error!("Could not find Swap {id}");
                                }
                            }
                        }
                        Err(e) => error!("Received stream error: {e:?}"),
                    },
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting swap updates loop");
                        return;
                    }
                }
            }
        });
    }

    async fn track_pending_swaps(self: &Arc<LiquidSdk>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(err) = cloned.send_swap_handler.track_refunds().await {
                            warn!("Could not refund expired swaps, error: {err:?}");
                        }
                        if let Err(err) = cloned.chain_swap_handler.track_refunds_and_refundables().await {
                            warn!("Could not refund expired swaps, error: {err:?}");
                        }
                    },
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting pending swaps loop");
                        return;
                    }
                }
            }
        });
    }

    async fn notify_event_listeners(&self, e: SdkEvent) -> Result<()> {
        self.event_manager.notify(e).await;
        Ok(())
    }

    /// Adds an event listener to the [LiquidSdk] instance, where all [SdkEvent]'s will be emitted to.
    /// The event listener can be removed be calling [LiquidSdk::remove_event_listener].
    ///
    /// # Arguments
    ///
    /// * `listener` - The listener which is an implementation of the [EventListener] trait
    pub async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        Ok(self.event_manager.add(listener).await?)
    }

    /// Removes an event listener from the [LiquidSdk] instance.
    ///
    /// # Arguments
    ///
    /// * `id` - the event listener id returned by [LiquidSdk::add_event_listener]
    pub async fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        self.event_manager.remove(id).await;
        Ok(())
    }

    async fn emit_payment_updated(&self, payment_id: Option<String>) -> Result<()> {
        if let Some(id) = payment_id {
            match self.persister.get_payment(&id)? {
                Some(payment) => {
                    match payment.status {
                        Complete => {
                            self.notify_event_listeners(SdkEvent::PaymentSucceeded {
                                details: payment,
                            })
                            .await?
                        }
                        Pending => {
                            match &payment.details.get_swap_id() {
                                Some(swap_id) => match self.persister.fetch_swap_by_id(swap_id)? {
                                    Swap::Chain(ChainSwap { claim_tx_id, .. })
                                    | Swap::Receive(ReceiveSwap { claim_tx_id, .. }) => {
                                        match claim_tx_id {
                                            Some(_) => {
                                                // The claim tx has now been broadcast
                                                self.notify_event_listeners(
                                                    SdkEvent::PaymentWaitingConfirmation {
                                                        details: payment,
                                                    },
                                                )
                                                .await?
                                            }
                                            None => {
                                                // The lockup tx is in the mempool/confirmed
                                                self.notify_event_listeners(
                                                    SdkEvent::PaymentPending { details: payment },
                                                )
                                                .await?
                                            }
                                        }
                                    }
                                    Swap::Send(_) => {
                                        // The lockup tx is in the mempool/confirmed
                                        self.notify_event_listeners(SdkEvent::PaymentPending {
                                            details: payment,
                                        })
                                        .await?
                                    }
                                },
                                // Here we probably have a liquid address payment so we emit PaymentWaitingConfirmation
                                None => {
                                    self.notify_event_listeners(
                                        SdkEvent::PaymentWaitingConfirmation { details: payment },
                                    )
                                    .await?
                                }
                            };
                        }
                        RefundPending => {
                            // The swap state has changed to RefundPending
                            self.notify_event_listeners(SdkEvent::PaymentRefundPending {
                                details: payment,
                            })
                            .await?
                        }
                        Failed => match payment.payment_type {
                            PaymentType::Receive => {
                                self.notify_event_listeners(SdkEvent::PaymentFailed {
                                    details: payment,
                                })
                                .await?
                            }
                            PaymentType::Send => {
                                // The refund tx is confirmed
                                self.notify_event_listeners(SdkEvent::PaymentRefunded {
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

    /// Get the wallet info, calculating the current pending and confirmed balances.
    pub async fn get_info(&self) -> Result<GetInfoResponse> {
        self.ensure_is_started().await?;
        let mut pending_send_sat = 0;
        let mut pending_receive_sat = 0;
        let mut confirmed_sent_sat = 0;
        let mut confirmed_received_sat = 0;

        for p in self
            .list_payments(&ListPaymentsRequest {
                ..Default::default()
            })
            .await?
        {
            match p.payment_type {
                PaymentType::Send => match p.status {
                    Complete => confirmed_sent_sat += p.amount_sat,
                    Failed => {
                        confirmed_sent_sat += p.amount_sat;
                        confirmed_received_sat +=
                            p.details.get_refund_tx_amount_sat().unwrap_or_default();
                    }
                    Pending => match p.details.get_refund_tx_amount_sat() {
                        Some(refund_tx_amount_sat) => {
                            confirmed_sent_sat += p.amount_sat;
                            pending_receive_sat += refund_tx_amount_sat;
                        }
                        None => pending_send_sat += p.amount_sat,
                    },
                    Created => pending_send_sat += p.amount_sat,
                    Refundable | RefundPending | TimedOut => {}
                },
                PaymentType::Receive => match p.status {
                    Complete => confirmed_received_sat += p.amount_sat,
                    Pending => pending_receive_sat += p.amount_sat,
                    Created | Refundable | RefundPending | Failed | TimedOut => {}
                },
            }
        }

        Ok(GetInfoResponse {
            balance_sat: confirmed_received_sat - confirmed_sent_sat - pending_send_sat,
            pending_send_sat,
            pending_receive_sat,
            fingerprint: self.onchain_wallet.fingerprint()?,
            pubkey: self.onchain_wallet.pubkey()?,
        })
    }

    /// Sign given message with the private key. Returns a zbase encoded signature.
    pub fn sign_message(&self, req: &SignMessageRequest) -> SdkResult<SignMessageResponse> {
        let signature = self.onchain_wallet.sign_message(&req.message)?;
        Ok(SignMessageResponse { signature })
    }

    /// Check whether given message was signed by the given
    /// pubkey and the signature (zbase encoded) is valid.
    pub fn check_message(&self, req: &CheckMessageRequest) -> SdkResult<CheckMessageResponse> {
        let is_valid =
            self.onchain_wallet
                .check_message(&req.message, &req.pubkey, &req.signature)?;
        Ok(CheckMessageResponse { is_valid })
    }

    async fn validate_bitcoin_address(&self, input: &str) -> Result<String, PaymentError> {
        match sdk::LiquidSdk::parse(input).await? {
            InputType::BitcoinAddress {
                address: bitcoin_address_data,
                ..
            } => match bitcoin_address_data.network == self.config.network.into() {
                true => Ok(bitcoin_address_data.address),
                false => Err(PaymentError::InvalidNetwork {
                    err: format!(
                        "Not a {} address",
                        Into::<Network>::into(self.config.network)
                    ),
                }),
            },
            _ => Err(PaymentError::Generic {
                err: "Invalid Bitcoin address".to_string(),
            }),
        }
    }

    fn validate_bolt11_invoice(&self, invoice: &str) -> Result<Bolt11Invoice, PaymentError> {
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|err| PaymentError::invalid_invoice(&err.to_string()))?;

        match (invoice.network().to_string().as_str(), self.config.network) {
            ("bitcoin", LiquidNetwork::Mainnet) => {}
            ("testnet", LiquidNetwork::Testnet) => {}
            _ => {
                return Err(PaymentError::InvalidNetwork {
                    err: "Invoice cannot be paid on the current network".to_string(),
                })
            }
        }

        ensure_sdk!(
            !invoice.is_expired(),
            PaymentError::invalid_invoice("Invoice has expired")
        );

        Ok(invoice)
    }

    fn validate_bolt12_invoice(
        &self,
        offer: &LNOffer,
        user_specified_receiver_amount_sat: u64,
        invoice: &str,
    ) -> Result<Bolt12Invoice, PaymentError> {
        let invoice_parsed = utils::parse_bolt12_invoice(invoice)?;
        let invoice_signing_pubkey = invoice_parsed.signing_pubkey().to_hex();

        // Check if the invoice is signed by same key as the offer
        match &offer.signing_pubkey {
            None => {
                ensure_sdk!(
                    &offer
                        .paths
                        .iter()
                        .filter_map(|path| path.blinded_hops.last())
                        .any(|last_hop| &invoice_signing_pubkey == last_hop),
                    PaymentError::invalid_invoice(
                        "Invalid Bolt12 invoice signing key when using blinded path"
                    )
                );
            }
            Some(offer_signing_pubkey) => {
                ensure_sdk!(
                    offer_signing_pubkey == &invoice_signing_pubkey,
                    PaymentError::invalid_invoice("Invalid Bolt12 invoice signing key")
                );
            }
        }

        let receiver_amount_sat = invoice_parsed.amount_msats() / 1_000;
        ensure_sdk!(
            receiver_amount_sat == user_specified_receiver_amount_sat,
            PaymentError::invalid_invoice("Invalid Bolt12 invoice amount")
        );

        Ok(invoice_parsed)
    }

    /// For submarine swaps (Liquid -> LN), the output amount (invoice amount) is checked if it fits
    /// the pair limits. This is unlike all the other swap types, where the input amount is checked.
    fn validate_submarine_pairs(
        &self,
        receiver_amount_sat: u64,
    ) -> Result<SubmarinePair, PaymentError> {
        let lbtc_pair = self
            .swapper
            .get_submarine_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(receiver_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(receiver_amount_sat);

        ensure_sdk!(
            receiver_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(lbtc_pair)
    }

    fn get_chain_pair(&self, direction: Direction) -> Result<ChainPair, PaymentError> {
        self.swapper
            .get_chain_pair(direction)?
            .ok_or(PaymentError::PairsNotFound)
    }

    /// Validates if the `user_lockup_amount_sat` fits within the limits of this pair
    fn validate_user_lockup_amount_for_chain_pair(
        &self,
        pair: &ChainPair,
        user_lockup_amount_sat: u64,
    ) -> Result<(), PaymentError> {
        pair.limits.within(user_lockup_amount_sat)?;

        let fees_sat = pair.fees.total(user_lockup_amount_sat);
        ensure_sdk!(
            user_lockup_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(())
    }

    fn get_and_validate_chain_pair(
        &self,
        direction: Direction,
        user_lockup_amount_sat: u64,
    ) -> Result<ChainPair, PaymentError> {
        let pair = self.get_chain_pair(direction)?;
        self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;
        Ok(pair)
    }

    /// Estimate the onchain fee for sending the given amount to the given destination address
    async fn estimate_onchain_tx_fee(
        &self,
        amount_sat: u64,
        address: &str,
    ) -> Result<u64, PaymentError> {
        let fee_rate_msat_per_vbyte = self.config.lowball_fee_rate_msat_per_vbyte();
        Ok(self
            .onchain_wallet
            .build_tx(fee_rate_msat_per_vbyte, address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }

    fn get_temp_p2tr_addr(&self) -> &str {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        match self.config.network {
            LiquidNetwork::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            LiquidNetwork::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        }
    }

    /// Estimate the lockup tx fee for Send and Chain Send swaps
    async fn estimate_lockup_tx_fee(
        &self,
        user_lockup_amount_sat: u64,
    ) -> Result<u64, PaymentError> {
        let temp_p2tr_addr = self.get_temp_p2tr_addr();
        self.estimate_onchain_tx_fee(user_lockup_amount_sat, temp_p2tr_addr)
            .await
    }

    async fn estimate_drain_tx_fee(
        &self,
        enforce_amount_sat: Option<u64>,
        address: Option<&str>,
    ) -> Result<u64, PaymentError> {
        let receipent_address = address.unwrap_or(self.get_temp_p2tr_addr());
        let fee_rate_msat_per_vbyte = self.config.lowball_fee_rate_msat_per_vbyte();
        let fee_sat = self
            .onchain_wallet
            .build_drain_tx(
                fee_rate_msat_per_vbyte,
                receipent_address,
                enforce_amount_sat,
            )
            .await?
            .all_fees()
            .values()
            .sum();
        info!("Estimated drain tx fee: {fee_sat} sat");

        Ok(fee_sat)
    }

    async fn estimate_onchain_tx_or_drain_tx_fee(
        &self,
        amount_sat: u64,
        address: &str,
    ) -> Result<u64, PaymentError> {
        match self.estimate_onchain_tx_fee(amount_sat, address).await {
            Ok(fees_sat) => Ok(fees_sat),
            Err(PaymentError::InsufficientFunds) => self
                .estimate_drain_tx_fee(Some(amount_sat), Some(address))
                .await
                .map_err(|_| PaymentError::InsufficientFunds),
            Err(e) => Err(e),
        }
    }

    async fn estimate_lockup_tx_or_drain_tx_fee(
        &self,
        amount_sat: u64,
    ) -> Result<u64, PaymentError> {
        let temp_p2tr_addr = self.get_temp_p2tr_addr();
        self.estimate_onchain_tx_or_drain_tx_fee(amount_sat, temp_p2tr_addr)
            .await
    }

    /// Prepares to pay a Lightning invoice via a submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareSendRequest] containing:
    ///     * `destination` - Either a Liquid BIP21 URI/address, a BOLT11 invoice or a BOLT12 offer
    ///     * `amount` - The optional amount of type [PayAmount]. Should only be specified
    ///        when paying directly onchain or via amount-less BIP21.
    ///        - [PayAmount::Drain] which uses all funds
    ///        - [PayAmount::Receiver] which sets the amount the receiver should receive
    ///
    /// # Returns
    /// Returns a [PrepareSendResponse] containing:
    ///     * `destination` - the parsed destination, of type [SendDestination]
    ///     * `fees_sat` - the additional fees which will be paid by the sender
    pub async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.ensure_is_started().await?;

        let get_info_res = self.get_info().await?;
        let fees_sat;
        let receiver_amount_sat;
        let payment_destination;

        match Self::parse(&req.destination).await {
            Ok(InputType::LiquidAddress {
                address: mut liquid_address_data,
            }) => {
                let amount = match (liquid_address_data.amount_sat, req.amount.clone()) {
                    (None, None) => {
                        return Err(PaymentError::AmountMissing {
                            err: "Amount must be set when paying to a Liquid address".to_string(),
                        });
                    }
                    (Some(bip21_amount_sat), None) => PayAmount::Receiver {
                        amount_sat: bip21_amount_sat,
                    },
                    (_, Some(amount)) => amount,
                };

                ensure_sdk!(
                    liquid_address_data.network == self.config.network.into(),
                    PaymentError::InvalidNetwork {
                        err: format!(
                            "Cannot send payment from {} to {}",
                            Into::<sdk_common::bitcoin::Network>::into(self.config.network),
                            liquid_address_data.network
                        )
                    }
                );

                (receiver_amount_sat, fees_sat) = match amount {
                    PayAmount::Drain => {
                        ensure_sdk!(
                            get_info_res.pending_receive_sat == 0
                                && get_info_res.pending_send_sat == 0,
                            PaymentError::Generic {
                                err: "Cannot drain while there are pending payments".to_string(),
                            }
                        );
                        let drain_fees_sat = self
                            .estimate_drain_tx_fee(None, Some(&liquid_address_data.address))
                            .await?;
                        let drain_amount_sat = get_info_res.balance_sat - drain_fees_sat;
                        info!("Drain amount: {drain_amount_sat} sat");
                        (drain_amount_sat, drain_fees_sat)
                    }
                    PayAmount::Receiver { amount_sat } => {
                        let fees_sat = self
                            .estimate_onchain_tx_or_drain_tx_fee(
                                amount_sat,
                                &liquid_address_data.address,
                            )
                            .await?;
                        (amount_sat, fees_sat)
                    }
                };

                liquid_address_data.amount_sat = Some(receiver_amount_sat);
                payment_destination = SendDestination::LiquidAddress {
                    address_data: liquid_address_data,
                };
            }
            Ok(InputType::Bolt11 { invoice }) => {
                self.ensure_send_is_not_self_transfer(&invoice.bolt11)?;
                self.validate_bolt11_invoice(&invoice.bolt11)?;

                receiver_amount_sat = invoice.amount_msat.ok_or(PaymentError::amount_missing(
                    "Expected invoice with an amount",
                ))? / 1000;

                if let Some(PayAmount::Receiver { amount_sat }) = req.amount {
                    ensure_sdk!(
                        receiver_amount_sat == amount_sat,
                        PaymentError::Generic {
                            err: "Receiver amount and invoice amount do not match".to_string()
                        }
                    );
                }

                let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;

                fees_sat = match self.swapper.check_for_mrh(&invoice.bolt11)? {
                    Some((lbtc_address, _)) => {
                        self.estimate_onchain_tx_or_drain_tx_fee(receiver_amount_sat, &lbtc_address)
                            .await?
                    }
                    None => {
                        let boltz_fees_total = lbtc_pair.fees.total(receiver_amount_sat);
                        let user_lockup_amount_sat = receiver_amount_sat + boltz_fees_total;
                        let lockup_fees_sat = self
                            .estimate_lockup_tx_or_drain_tx_fee(user_lockup_amount_sat)
                            .await?;
                        boltz_fees_total + lockup_fees_sat
                    }
                };
                payment_destination = SendDestination::Bolt11 { invoice };
            }
            Ok(InputType::Bolt12Offer { offer }) => {
                receiver_amount_sat = match req.amount {
                    Some(PayAmount::Receiver { amount_sat }) => Ok(amount_sat),
                    _ => Err(PaymentError::amount_missing(
                        "Expected PayAmount of type Receiver when processing a Bolt12 offer",
                    )),
                }?;
                if let Some(Amount::Bitcoin { amount_msat }) = &offer.min_amount {
                    ensure_sdk!(
                        receiver_amount_sat >= amount_msat / 1_000,
                        PaymentError::invalid_invoice(
                            "Invalid receiver amount: below offer minimum"
                        )
                    );
                }

                let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;

                let boltz_fees_total = lbtc_pair.fees.total(receiver_amount_sat);
                let lockup_fees_sat = self
                    .estimate_lockup_tx_or_drain_tx_fee(receiver_amount_sat + boltz_fees_total)
                    .await?;
                fees_sat = boltz_fees_total + lockup_fees_sat;

                payment_destination = SendDestination::Bolt12 {
                    offer,
                    receiver_amount_sat,
                };
            }
            _ => {
                return Err(PaymentError::generic("Destination is not valid"));
            }
        };

        let payer_amount_sat = receiver_amount_sat + fees_sat;
        ensure_sdk!(
            payer_amount_sat <= get_info_res.balance_sat,
            PaymentError::InsufficientFunds
        );

        Ok(PrepareSendResponse {
            destination: payment_destination,
            fees_sat,
        })
    }

    fn ensure_send_is_not_self_transfer(&self, invoice: &str) -> Result<(), PaymentError> {
        match self.persister.fetch_receive_swap_by_invoice(invoice)? {
            None => Ok(()),
            Some(_) => Err(PaymentError::SelfTransferNotSupported),
        }
    }

    /// Either pays a Lightning invoice via a submarine swap or sends funds directly to an address.
    ///
    /// Depending on [Config]'s `payment_timeout_sec`, this function will return:
    /// * [PaymentState::Pending] payment - if the payment could be initiated but didn't yet
    ///   complete in this time
    /// * [PaymentState::Complete] payment - if the payment was successfully completed in this time
    ///
    /// # Arguments
    ///
    /// * `req` - A [SendPaymentRequest], containing:
    ///     * `prepare_response` - the [PrepareSendResponse] returned by [LiquidSdk::prepare_send_payment]
    ///
    /// # Errors
    ///
    /// * [PaymentError::PaymentTimeout] - if the payment could not be initiated in this time
    pub async fn send_payment(
        &self,
        req: &SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let PrepareSendResponse {
            fees_sat,
            destination: payment_destination,
        } = &req.prepare_response;

        match payment_destination {
            SendDestination::LiquidAddress {
                address_data: liquid_address_data,
            } => {
                let Some(amount_sat) = liquid_address_data.amount_sat else {
                    return Err(PaymentError::AmountMissing { err: "`amount_sat` must be present when paying to a `SendDestination::LiquidAddress`".to_string() });
                };

                ensure_sdk!(
                    liquid_address_data.network == self.config.network.into(),
                    PaymentError::InvalidNetwork {
                        err: format!(
                            "Cannot send payment from {} to {}",
                            Into::<sdk_common::bitcoin::Network>::into(self.config.network),
                            liquid_address_data.network
                        )
                    }
                );

                let payer_amount_sat = amount_sat + fees_sat;
                ensure_sdk!(
                    payer_amount_sat <= self.get_info().await?.balance_sat,
                    PaymentError::InsufficientFunds
                );

                self.pay_liquid(liquid_address_data.clone(), amount_sat, *fees_sat)
                    .await
            }
            SendDestination::Bolt11 { invoice } => {
                self.pay_bolt11_invoice(&invoice.bolt11, *fees_sat).await
            }
            SendDestination::Bolt12 {
                offer,
                receiver_amount_sat,
            } => {
                let bolt12_invoice = self
                    .swapper
                    .get_bolt12_invoice(&offer.offer, *receiver_amount_sat)?;
                self.pay_bolt12_invoice(offer, *receiver_amount_sat, &bolt12_invoice, *fees_sat)
                    .await
            }
        }
    }

    async fn pay_bolt11_invoice(
        &self,
        invoice: &str,
        fees_sat: u64,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_send_is_not_self_transfer(invoice)?;
        let bolt11_invoice = self.validate_bolt11_invoice(invoice)?;

        let amount_sat = get_invoice_amount!(invoice);
        let payer_amount_sat = amount_sat + fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        let description = match bolt11_invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };

        match self.swapper.check_for_mrh(invoice)? {
            // If we find a valid MRH, extract the BIP21 address and pay to it via onchain tx
            Some((address, _)) => {
                info!("Found MRH for L-BTC address {address}, invoice amount_sat {amount_sat}");
                self.pay_liquid(
                    LiquidAddressData {
                        address,
                        network: self.config.network.into(),
                        asset_id: None,
                        amount_sat: None,
                        label: None,
                        message: None,
                    },
                    amount_sat,
                    fees_sat,
                )
                .await
            }

            // If no MRH found, perform usual swap
            None => {
                self.send_payment_via_swap(
                    invoice,
                    None,
                    &bolt11_invoice.payment_hash().to_string(),
                    description,
                    amount_sat,
                    fees_sat,
                )
                .await
            }
        }
    }

    async fn pay_bolt12_invoice(
        &self,
        offer: &LNOffer,
        user_specified_receiver_amount_sat: u64,
        invoice_str: &str,
        fees_sat: u64,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let invoice =
            self.validate_bolt12_invoice(offer, user_specified_receiver_amount_sat, invoice_str)?;

        let receiver_amount_sat = invoice.amount_msats() / 1_000;
        let payer_amount_sat = receiver_amount_sat + fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        self.send_payment_via_swap(
            invoice_str,
            Some(offer.offer.clone()),
            &invoice.payment_hash().to_string(),
            invoice.description().map(|desc| desc.to_string()),
            receiver_amount_sat,
            fees_sat,
        )
        .await
    }

    /// Performs a Send Payment by doing an onchain tx to a L-BTC address
    async fn pay_liquid(
        &self,
        address_data: LiquidAddressData,
        receiver_amount_sat: u64,
        fees_sat: u64,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let tx = self
            .onchain_wallet
            .build_tx_or_drain_tx(
                self.config.lowball_fee_rate_msat_per_vbyte(),
                &address_data.address,
                receiver_amount_sat,
            )
            .await?;
        let tx_fees_sat = tx.all_fees().values().sum::<u64>();
        ensure_sdk!(tx_fees_sat <= fees_sat, PaymentError::InvalidOrExpiredFees);

        let tx_id = tx.txid().to_string();
        let payer_amount_sat = receiver_amount_sat + tx_fees_sat;
        info!(
            "Built onchain L-BTC tx with receiver_amount_sat = {receiver_amount_sat}, fees_sat = {fees_sat} and txid = {tx_id}"
        );

        let liquid_chain_service = self.liquid_chain_service.lock().await;
        let tx_id = liquid_chain_service.broadcast(&tx, None).await?.to_string();

        // We insert a pseudo-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        let tx_data = PaymentTxData {
            tx_id: tx_id.clone(),
            timestamp: Some(utils::now()),
            amount_sat: payer_amount_sat,
            fees_sat,
            payment_type: PaymentType::Send,
            is_confirmed: false,
        };

        let destination = address_data.to_uri().unwrap_or(address_data.address);
        let description = address_data.message;

        self.persister.insert_or_update_payment(
            tx_data.clone(),
            Some(destination.clone()),
            description.clone(),
        )?;
        self.emit_payment_updated(Some(tx_id)).await?; // Emit Pending event

        let payment_details = PaymentDetails::Liquid {
            destination,
            description: description.unwrap_or("Liquid transfer".to_string()),
        };

        Ok(SendPaymentResponse {
            payment: Payment::from_tx_data(tx_data, None, payment_details),
        })
    }

    /// Performs a Send Payment by doing a swap (create it, fund it, track it, etc).
    ///
    /// If `bolt12_offer` is set, `invoice` refers to a Bolt12 invoice, otherwise it's a Bolt11 one.
    async fn send_payment_via_swap(
        &self,
        invoice: &str,
        bolt12_offer: Option<String>,
        payment_hash: &str,
        description: Option<String>,
        receiver_amount_sat: u64,
        fees_sat: u64,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;
        let boltz_fees_total = lbtc_pair.fees.total(receiver_amount_sat);
        let user_lockup_amount_sat = receiver_amount_sat + boltz_fees_total;
        let lockup_tx_fees_sat = self
            .estimate_lockup_tx_or_drain_tx_fee(user_lockup_amount_sat)
            .await?;
        ensure_sdk!(
            fees_sat == boltz_fees_total + lockup_tx_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let swap = match self.persister.fetch_send_swap_by_invoice(invoice)? {
            Some(swap) => match swap.state {
                Created => swap,
                TimedOut => {
                    self.send_swap_handler
                        .update_swap_info(&swap.id, PaymentState::Created, None, None, None)
                        .await?;
                    swap
                }
                Pending => return Err(PaymentError::PaymentInProgress),
                Complete => return Err(PaymentError::AlreadyPaid),
                RefundPending | Refundable | Failed => {
                    return Err(PaymentError::invalid_invoice(
                        "Payment has already failed. Please try with another invoice",
                    ))
                }
            },
            None => {
                let keypair = utils::generate_keypair();
                let refund_public_key = boltz_client::PublicKey {
                    compressed: true,
                    inner: keypair.public_key(),
                };
                let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
                    url,
                    hash_swap_id: Some(true),
                    status: Some(vec![
                        SubSwapStates::InvoiceFailedToPay,
                        SubSwapStates::SwapExpired,
                        SubSwapStates::TransactionClaimPending,
                        SubSwapStates::TransactionLockupFailed,
                    ]),
                });
                let create_response = self.swapper.create_send_swap(CreateSubmarineRequest {
                    from: "L-BTC".to_string(),
                    to: "BTC".to_string(),
                    invoice: invoice.to_string(),
                    refund_public_key,
                    pair_hash: Some(lbtc_pair.hash),
                    referral_id: None,
                    webhook,
                })?;

                let swap_id = &create_response.id;
                let create_response_json =
                    SendSwap::from_boltz_struct_to_json(&create_response, swap_id)?;

                let payer_amount_sat = fees_sat + receiver_amount_sat;
                let swap = SendSwap {
                    id: swap_id.clone(),
                    invoice: invoice.to_string(),
                    bolt12_offer,
                    payment_hash: Some(payment_hash.to_string()),
                    description,
                    preimage: None,
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
                swap
            }
        };
        self.status_stream.track_swap_id(&swap.id)?;

        let create_response = swap.get_boltz_create_response()?;
        self.send_swap_handler
            .try_lockup(&swap, &create_response)
            .await?;

        self.wait_for_payment(Swap::Send(swap), create_response.accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    /// Fetch the current payment limits for [LiquidSdk::send_payment] and [LiquidSdk::receive_payment].
    pub async fn fetch_lightning_limits(
        &self,
    ) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        self.ensure_is_started().await?;

        let submarine_pair = self
            .swapper
            .get_submarine_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let send_limits = submarine_pair.limits;

        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let receive_limits = reverse_pair.limits;

        Ok(LightningPaymentLimitsResponse {
            send: Limits {
                min_sat: send_limits.minimal,
                max_sat: send_limits.maximal,
                max_zero_conf_sat: send_limits.maximal_zero_conf,
            },
            receive: Limits {
                min_sat: receive_limits.minimal,
                max_sat: receive_limits.maximal,
                max_zero_conf_sat: self.config.zero_conf_max_amount_sat(),
            },
        })
    }

    /// Fetch the current payment limits for [LiquidSdk::pay_onchain] and [LiquidSdk::receive_onchain].
    pub async fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        self.ensure_is_started().await?;

        let (pair_outgoing, pair_incoming) = self.swapper.get_chain_pairs()?;
        let send_limits = pair_outgoing
            .ok_or(PaymentError::PairsNotFound)
            .map(|pair| pair.limits)?;
        let receive_limits = pair_incoming
            .ok_or(PaymentError::PairsNotFound)
            .map(|pair| pair.limits)?;

        Ok(OnchainPaymentLimitsResponse {
            send: Limits {
                min_sat: send_limits.minimal,
                max_sat: send_limits.maximal,
                max_zero_conf_sat: send_limits.maximal_zero_conf,
            },
            receive: Limits {
                min_sat: receive_limits.minimal,
                max_sat: receive_limits.maximal,
                max_zero_conf_sat: receive_limits.maximal_zero_conf,
            },
        })
    }

    /// Prepares to pay to a Bitcoin address via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PreparePayOnchainRequest] containing:
    ///     * `amount` - which can be of two types: [PayAmount::Drain], which uses all funds,
    ///        and [PayAmount::Receiver], which sets the amount the receiver should receive
    ///     * `fee_rate_sat_per_vbyte` - the optional fee rate of the Bitcoin claim transaction. Defaults to the swapper estimated claim fee
    pub async fn prepare_pay_onchain(
        &self,
        req: &PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let get_info_res = self.get_info().await?;
        let pair = self.get_chain_pair(Direction::Outgoing)?;
        let claim_fees_sat = match req.fee_rate_sat_per_vbyte {
            Some(sat_per_vbyte) => ESTIMATED_BTC_CLAIM_TX_VSIZE * sat_per_vbyte as u64,
            None => pair.clone().fees.claim_estimate(),
        };
        let server_fees_sat = pair.fees.server();

        info!("Preparing for onchain payment of kind: {:?}", req.amount);
        let (payer_amount_sat, receiver_amount_sat, total_fees_sat) = match req.amount {
            PayAmount::Receiver { amount_sat } => {
                let receiver_amount_sat = amount_sat;

                let user_lockup_amount_sat_without_service_fee =
                    receiver_amount_sat + claim_fees_sat + server_fees_sat;

                // The resulting invoice amount contains the service fee, which is rounded up with ceil()
                // Therefore, when calculating the user_lockup amount, we must also round it up with ceil()
                let user_lockup_amount_sat = (user_lockup_amount_sat_without_service_fee as f64
                    * 100.0
                    / (100.0 - pair.fees.percentage))
                    .ceil() as u64;
                self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;

                let lockup_fees_sat = self.estimate_lockup_tx_fee(user_lockup_amount_sat).await?;

                let boltz_fees_sat =
                    user_lockup_amount_sat - user_lockup_amount_sat_without_service_fee;
                let total_fees_sat =
                    boltz_fees_sat + lockup_fees_sat + claim_fees_sat + server_fees_sat;
                let payer_amount_sat = receiver_amount_sat + total_fees_sat;

                (payer_amount_sat, receiver_amount_sat, total_fees_sat)
            }
            PayAmount::Drain => {
                ensure_sdk!(
                    get_info_res.pending_receive_sat == 0 && get_info_res.pending_send_sat == 0,
                    PaymentError::Generic {
                        err: "Cannot drain while there are pending payments".to_string(),
                    }
                );
                let payer_amount_sat = get_info_res.balance_sat;
                let lockup_fees_sat = self.estimate_drain_tx_fee(None, None).await?;

                let user_lockup_amount_sat = payer_amount_sat - lockup_fees_sat;
                self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;

                let boltz_fees_sat = pair.fees.boltz(user_lockup_amount_sat);
                let total_fees_sat =
                    boltz_fees_sat + lockup_fees_sat + claim_fees_sat + server_fees_sat;
                let receiver_amount_sat = payer_amount_sat - total_fees_sat;

                (payer_amount_sat, receiver_amount_sat, total_fees_sat)
            }
        };

        let res = PreparePayOnchainResponse {
            receiver_amount_sat,
            claim_fees_sat,
            total_fees_sat,
        };

        ensure_sdk!(
            payer_amount_sat <= get_info_res.balance_sat,
            PaymentError::InsufficientFunds
        );

        info!("Prepared onchain payment: {res:?}");
        Ok(res)
    }

    /// Pays to a Bitcoin address via a chain swap.
    ///
    /// Depending on [Config]'s `payment_timeout_sec`, this function will return:
    /// * [PaymentState::Pending] payment - if the payment could be initiated but didn't yet
    ///   complete in this time
    /// * [PaymentState::Complete] payment - if the payment was successfully completed in this time
    ///
    /// # Arguments
    ///
    /// * `req` - the [PayOnchainRequest] containing:
    ///     * `address` - the Bitcoin address to pay to
    ///     * `prepare_response` - the [PreparePayOnchainResponse] from calling [LiquidSdk::prepare_pay_onchain]
    ///
    /// # Errors
    ///
    /// * [PaymentError::PaymentTimeout] - if the payment could not be initiated in this time
    pub async fn pay_onchain(
        &self,
        req: &PayOnchainRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;
        info!("Paying onchain, request = {req:?}");

        let claim_address = self.validate_bitcoin_address(&req.address).await?;
        let balance_sat = self.get_info().await?.balance_sat;
        let receiver_amount_sat = req.prepare_response.receiver_amount_sat;
        let pair = self.get_chain_pair(Direction::Outgoing)?;
        let claim_fees_sat = req.prepare_response.claim_fees_sat;
        let server_fees_sat = pair.fees.server();
        let server_lockup_amount_sat = receiver_amount_sat + claim_fees_sat;

        let user_lockup_amount_sat_without_service_fee =
            receiver_amount_sat + claim_fees_sat + server_fees_sat;

        // The resulting invoice amount contains the service fee, which is rounded up with ceil()
        // Therefore, when calculating the user_lockup amount, we must also round it up with ceil()
        let user_lockup_amount_sat = (user_lockup_amount_sat_without_service_fee as f64 * 100.0
            / (100.0 - pair.fees.percentage))
            .ceil() as u64;
        let boltz_fee_sat = user_lockup_amount_sat - user_lockup_amount_sat_without_service_fee;
        self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;

        let payer_amount_sat = req.prepare_response.total_fees_sat + receiver_amount_sat;

        let lockup_fees_sat = match payer_amount_sat == balance_sat {
            true => self.estimate_drain_tx_fee(None, None).await?,
            false => self.estimate_lockup_tx_fee(user_lockup_amount_sat).await?,
        };

        ensure_sdk!(
            req.prepare_response.total_fees_sat
                == boltz_fee_sat + lockup_fees_sat + claim_fees_sat + server_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        ensure_sdk!(
            payer_amount_sat <= balance_sat,
            PaymentError::InsufficientFunds
        );

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;

        let claim_keypair = utils::generate_keypair();
        let claim_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: claim_keypair.public_key(),
        };
        let refund_keypair = utils::generate_keypair();
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: refund_keypair.public_key(),
        };
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::TransactionLockupFailed,
                ChainSwapStates::TransactionServerConfirmed,
            ]),
        });
        let create_response = self.swapper.create_chain_swap(CreateChainRequest {
            from: "L-BTC".to_string(),
            to: "BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: Some(claim_public_key),
            refund_public_key: Some(refund_public_key),
            user_lock_amount: None,
            server_lock_amount: Some(server_lockup_amount_sat),
            pair_hash: Some(pair.hash),
            referral_id: None,
            webhook,
        })?;

        let create_response_json =
            ChainSwap::from_boltz_struct_to_json(&create_response, &create_response.id)?;
        let swap_id = create_response.id;

        let accept_zero_conf = server_lockup_amount_sat <= pair.limits.maximal_zero_conf;
        let payer_amount_sat = req.prepare_response.total_fees_sat + receiver_amount_sat;

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Outgoing,
            claim_address: Some(claim_address),
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            description: Some("Bitcoin transfer".to_string()),
            payer_amount_sat,
            receiver_amount_sat,
            claim_fees_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key: claim_keypair.display_secret().to_string(),
            refund_private_key: refund_keypair.display_secret().to_string(),
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: PaymentState::Created,
        };
        self.persister.insert_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap_id)?;

        self.wait_for_payment(Swap::Chain(swap), accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    async fn wait_for_payment(
        &self,
        swap: Swap,
        accept_zero_conf: bool,
    ) -> Result<Payment, PaymentError> {
        let timeout_fut = tokio::time::sleep(Duration::from_secs(self.config.payment_timeout_sec));
        tokio::pin!(timeout_fut);

        let expected_swap_id = swap.id();
        let mut events_stream = self.event_manager.subscribe();
        let mut maybe_payment: Option<Payment> = None;

        loop {
            tokio::select! {
                _ = &mut timeout_fut => match maybe_payment {
                    Some(payment) => return Ok(payment),
                    None => {
                        debug!("Timeout occurred without payment, set swap to timed out");
                        match swap {
                            Swap::Send(_) => self.send_swap_handler.update_swap_info(&expected_swap_id, TimedOut, None, None, None).await?,
                            Swap::Chain(_) => self.chain_swap_handler.update_swap_info(&expected_swap_id, TimedOut, None, None, None, None).await?,
                            _ => ()
                        }
                        return Err(PaymentError::PaymentTimeout)
                    },
                },
                event = events_stream.recv() => match event {
                    Ok(SdkEvent::PaymentPending { details: payment }) => {
                        let maybe_payment_swap_id = payment.details.get_swap_id();
                        if matches!(maybe_payment_swap_id, Some(swap_id) if swap_id == expected_swap_id) {
                            match accept_zero_conf {
                                true => {
                                    debug!("Received Send Payment pending event with zero-conf accepted");
                                    return Ok(payment)
                                }
                                false => {
                                    debug!("Received Send Payment pending event, waiting for confirmation");
                                    maybe_payment = Some(payment);
                                }
                            }
                        };
                    },
                    Ok(SdkEvent::PaymentSucceeded { details: payment }) => {
                        let maybe_payment_swap_id = payment.details.get_swap_id();
                        if matches!(maybe_payment_swap_id, Some(swap_id) if swap_id == expected_swap_id) {
                            debug!("Received Send Payment succeed event");
                            return Ok(payment);
                        }
                    },
                    Ok(event) => debug!("Unhandled event: {event:?}"),
                    Err(e) => debug!("Received error waiting for event: {e:?}"),
                }
            }
        }
    }

    /// Prepares to receive a Lightning payment via a reverse submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareReceiveRequest] containing:
    ///     * `payer_amount_sat` - the amount in satoshis to be paid by the payer
    ///     * `payment_method` - the supported payment methods; either an invoice, a Liquid address or a Bitcoin address
    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.ensure_is_started().await?;

        let fees_sat;
        match req.payment_method {
            PaymentMethod::Lightning => {
                let Some(payer_amount_sat) = req.payer_amount_sat else {
                    return Err(PaymentError::AmountMissing { err: "`payer_amount_sat` must be specified when `PaymentMethod::Lightning` is used.".to_string() });
                };
                let reverse_pair = self
                    .swapper
                    .get_reverse_swap_pairs()?
                    .ok_or(PaymentError::PairsNotFound)?;

                fees_sat = reverse_pair.fees.total(payer_amount_sat);

                ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

                reverse_pair
                    .limits
                    .within(payer_amount_sat)
                    .map_err(|_| PaymentError::AmountOutOfRange)?;

                debug!(
                    "Preparing Lightning Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat"
                );
            }
            PaymentMethod::BitcoinAddress => {
                let Some(payer_amount_sat) = req.payer_amount_sat else {
                    return Err(PaymentError::AmountMissing { err: "`payer_amount_sat` must be specified when `PaymentMethod::BitcoinAddress` is used.".to_string() });
                };
                let pair =
                    self.get_and_validate_chain_pair(Direction::Incoming, payer_amount_sat)?;
                let claim_fees_sat = pair.fees.claim_estimate();
                let server_fees_sat = pair.fees.server();
                fees_sat = pair.fees.boltz(payer_amount_sat) + claim_fees_sat + server_fees_sat;
                debug!(
                    "Preparing Chain Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat"
                );
            }
            PaymentMethod::LiquidAddress => {
                fees_sat = 0;
                debug!(
                    "Preparing Liquid Receive Swap with: amount_sat {:?} sat, fees_sat {fees_sat} sat",
                    req.payer_amount_sat
                );
            }
        };

        Ok(PrepareReceiveResponse {
            payer_amount_sat: req.payer_amount_sat,
            fees_sat,
            payment_method: req.payment_method.clone(),
        })
    }

    /// Receive a Lightning payment via a reverse submarine swap, a chain swap or via direct Liquid
    /// payment.
    ///
    /// # Arguments
    ///
    /// * `req` - the [ReceivePaymentRequest] containing:
    ///     * `prepare_response` - the [PrepareReceiveResponse] from calling [LiquidSdk::prepare_receive_payment]
    ///     * `description` - the optional payment description
    ///     * `use_description_hash` - optional if true uses the hash of the description
    ///
    /// # Returns
    ///
    /// * A [ReceivePaymentResponse] containing:
    ///     * `destination` - the final destination to be paid by the payer, either a BIP21 URI (Liquid or Bitcoin), a Liquid address or an invoice
    pub async fn receive_payment(
        &self,
        req: &ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let PrepareReceiveResponse {
            payment_method,
            payer_amount_sat: amount_sat,
            fees_sat,
        } = &req.prepare_response;

        match payment_method {
            PaymentMethod::Lightning => {
                let Some(amount_sat) = amount_sat else {
                    return Err(PaymentError::AmountMissing { err: "`amount_sat` must be specified when `PaymentMethod::Lightning` is used.".to_string() });
                };
                let (description, description_hash) = match (
                    req.description.clone(),
                    req.use_description_hash.unwrap_or_default(),
                ) {
                    (Some(description), true) => (
                        None,
                        Some(sha256::Hash::hash(description.as_bytes()).to_hex()),
                    ),
                    (_, false) => (req.description.clone(), None),
                    _ => {
                        return Err(PaymentError::InvalidDescription {
                            err: "Missing payment description to hash".to_string(),
                        })
                    }
                };
                self.create_receive_swap(*amount_sat, *fees_sat, description, description_hash)
                    .await
            }
            PaymentMethod::BitcoinAddress => {
                let Some(amount_sat) = amount_sat else {
                    return Err(PaymentError::AmountMissing { err: "`amount_sat` must be specified when `PaymentMethod::BitcoinAddress` is used.".to_string() });
                };
                self.receive_onchain(*amount_sat, *fees_sat).await
            }
            PaymentMethod::LiquidAddress => {
                let address = self.onchain_wallet.next_unused_address().await?.to_string();

                let receive_destination = match amount_sat {
                    Some(amount_sat) => LiquidAddressData {
                        address: address.to_string(),
                        network: self.config.network.into(),
                        amount_sat: Some(*amount_sat),
                        asset_id: Some(AssetId::LIQUID_BTC.to_hex()),
                        label: None,
                        message: req.description.clone(),
                    }
                    .to_uri()
                    .map_err(|e| PaymentError::Generic {
                        err: format!("Could not build BIP21 URI: {e:?}"),
                    })?,
                    None => address,
                };

                Ok(ReceivePaymentResponse {
                    destination: receive_destination,
                })
            }
        }
    }

    async fn create_receive_swap(
        &self,
        payer_amount_sat: u64,
        fees_sat: u64,
        description: Option<String>,
        description_hash: Option<String>,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let new_fees_sat = reverse_pair.fees.total(payer_amount_sat);
        ensure_sdk!(fees_sat == new_fees_sat, PaymentError::InvalidOrExpiredFees);

        debug!("Creating Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        let keypair = utils::generate_keypair();

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        // Address to be used for a BIP-21 direct payment
        let mrh_addr = self.onchain_wallet.next_unused_address().await?;

        // Signature of the claim public key of the SHA256 hash of the address for the direct payment
        let mrh_addr_str = mrh_addr.to_string();
        let mrh_addr_hash = sha256::Hash::hash(mrh_addr_str.as_bytes());
        let mrh_addr_hash_sig = keypair.sign_schnorr(mrh_addr_hash.into());

        let receiver_amount_sat = payer_amount_sat - fees_sat;
        let webhook_claim_status =
            match receiver_amount_sat > self.config.zero_conf_max_amount_sat() {
                true => RevSwapStates::TransactionConfirmed,
                false => RevSwapStates::TransactionMempool,
            };
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![webhook_claim_status]),
        });

        let v2_req = CreateReverseRequest {
            invoice_amount: payer_amount_sat,
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: keypair.public_key().into(),
            description,
            description_hash,
            address: Some(mrh_addr_str.clone()),
            address_signature: Some(mrh_addr_hash_sig.to_hex()),
            referral_id: None,
            webhook,
        };
        let create_response = self.swapper.create_receive_swap(v2_req)?;

        // Reserve this address until the timeout block height
        self.persister.insert_or_update_reserved_address(
            &mrh_addr_str,
            create_response.timeout_block_height,
        )?;

        // Check if correct MRH was added to the invoice by Boltz
        let (bip21_lbtc_address, _bip21_amount_btc) = self
            .swapper
            .check_for_mrh(&create_response.invoice)?
            .ok_or(PaymentError::receive_error("Invoice has no MRH"))?;
        ensure_sdk!(
            bip21_lbtc_address == mrh_addr_str,
            PaymentError::receive_error("Invoice has incorrect address in MRH")
        );

        let swap_id = create_response.id.clone();
        let invoice = Bolt11Invoice::from_str(&create_response.invoice)
            .map_err(|err| PaymentError::invalid_invoice(&err.to_string()))?;
        let payer_amount_sat =
            invoice
                .amount_milli_satoshis()
                .ok_or(PaymentError::invalid_invoice(
                    "Invoice does not contain an amount",
                ))?
                / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        ensure_sdk!(
            invoice.payment_hash().to_string() == preimage_hash,
            PaymentError::invalid_invoice("Invalid preimage returned by swapper")
        );

        let create_response_json = ReceiveSwap::from_boltz_struct_to_json(
            &create_response,
            &swap_id,
            &invoice.to_string(),
        )?;
        let invoice_description = match invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };
        self.persister
            .insert_receive_swap(&ReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                create_response_json,
                claim_private_key: keypair.display_secret().to_string(),
                invoice: invoice.to_string(),
                payment_hash: Some(preimage_hash),
                description: invoice_description,
                payer_amount_sat,
                receiver_amount_sat,
                claim_fees_sat: reverse_pair.fees.claim_estimate(),
                claim_tx_id: None,
                lockup_tx_id: None,
                mrh_address: mrh_addr_str,
                mrh_script_pubkey: mrh_addr.to_unconfidential().script_pubkey().to_hex(),
                mrh_tx_id: None,
                created_at: utils::now(),
                state: PaymentState::Created,
            })
            .map_err(|_| PaymentError::PersistError)?;
        self.status_stream.track_swap_id(&swap_id)?;

        Ok(ReceivePaymentResponse {
            destination: invoice.to_string(),
        })
    }

    async fn create_receive_chain_swap(
        &self,
        user_lockup_amount_sat: u64,
        fees_sat: u64,
    ) -> Result<ChainSwap, PaymentError> {
        let pair = self.get_and_validate_chain_pair(Direction::Incoming, user_lockup_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();

        ensure_sdk!(
            fees_sat == pair.fees.boltz(user_lockup_amount_sat) + claim_fees_sat + server_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;

        let claim_keypair = utils::generate_keypair();
        let claim_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: claim_keypair.public_key(),
        };
        let refund_keypair = utils::generate_keypair();
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: refund_keypair.public_key(),
        };
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::TransactionLockupFailed,
                ChainSwapStates::TransactionServerConfirmed,
            ]),
        });
        let create_response = self.swapper.create_chain_swap(CreateChainRequest {
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: Some(claim_public_key),
            refund_public_key: Some(refund_public_key),
            user_lock_amount: Some(user_lockup_amount_sat),
            server_lock_amount: None,
            pair_hash: Some(pair.hash),
            referral_id: None,
            webhook,
        })?;

        let swap_id = create_response.id.clone();
        let create_response_json =
            ChainSwap::from_boltz_struct_to_json(&create_response, &swap_id)?;

        let accept_zero_conf = user_lockup_amount_sat <= pair.limits.maximal_zero_conf;
        let receiver_amount_sat = user_lockup_amount_sat - fees_sat;

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Incoming,
            claim_address: None,
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            description: Some("Bitcoin transfer".to_string()),
            payer_amount_sat: user_lockup_amount_sat,
            receiver_amount_sat,
            claim_fees_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key: claim_keypair.display_secret().to_string(),
            refund_private_key: refund_keypair.display_secret().to_string(),
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: PaymentState::Created,
        };
        self.persister.insert_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap.id)?;
        Ok(swap)
    }

    /// Receive from a Bitcoin transaction via a chain swap.
    async fn receive_onchain(
        &self,
        payer_amount_sat: u64,
        fees_sat: u64,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let swap = self
            .create_receive_chain_swap(payer_amount_sat, fees_sat)
            .await?;
        let create_response = swap.get_boltz_create_response()?;
        let address = create_response.lockup_details.lockup_address;

        let amount = create_response.lockup_details.amount as f64 / 100_000_000.0;
        let bip21 = create_response.lockup_details.bip21.unwrap_or(format!(
            "bitcoin:{address}?amount={amount}&label=Send%20to%20L-BTC%20address"
        ));

        Ok(ReceivePaymentResponse { destination: bip21 })
    }

    /// List all failed chain swaps that need to be refunded.
    /// They can be refunded by calling [LiquidSdk::prepare_refund] then [LiquidSdk::refund].
    pub async fn list_refundables(&self) -> SdkResult<Vec<RefundableSwap>> {
        let chain_swaps = self.persister.list_refundable_chain_swaps()?;

        let mut lockup_script_pubkeys = vec![];
        for swap in &chain_swaps {
            let script_pubkey = swap.get_receive_lockup_swap_script_pubkey(self.config.network)?;
            lockup_script_pubkeys.push(script_pubkey);
        }
        let lockup_scripts: Vec<&boltz_client::bitcoin::Script> = lockup_script_pubkeys
            .iter()
            .map(|s| s.as_script())
            .collect();
        let scripts_balance = self
            .bitcoin_chain_service
            .lock()
            .await
            .scripts_get_balance(&lockup_scripts)?;

        let mut refundables = vec![];
        for (chain_swap, script_balance) in chain_swaps.into_iter().zip(scripts_balance) {
            let swap_id = &chain_swap.id;
            let refundable_confirmed_sat = script_balance.confirmed;
            info!("Incoming Chain Swap {swap_id} is refundable with {refundable_confirmed_sat} confirmed sats");

            let refundable: RefundableSwap = chain_swap.to_refundable(refundable_confirmed_sat);
            refundables.push(refundable);
        }

        Ok(refundables)
    }

    /// Prepares to refund a failed chain swap by calculating the refund transaction size and absolute fee.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareRefundRequest] containing:
    ///     * `swap_address` - the swap address to refund from [RefundableSwap::swap_address]
    ///     * `refund_address` - the Bitcoin address to refund to
    ///     * `fee_rate_sat_per_vbyte` - the fee rate at which to broadcast the refund transaction
    pub async fn prepare_refund(
        &self,
        req: &PrepareRefundRequest,
    ) -> SdkResult<PrepareRefundResponse> {
        let (tx_vsize, tx_fee_sat, refund_tx_id) = self
            .chain_swap_handler
            .prepare_refund(
                &req.swap_address,
                &req.refund_address,
                req.fee_rate_sat_per_vbyte,
            )
            .await?;
        Ok(PrepareRefundResponse {
            tx_vsize,
            tx_fee_sat,
            refund_tx_id,
        })
    }

    /// Refund a failed chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [RefundRequest] containing:
    ///     * `swap_address` - the swap address to refund from [RefundableSwap::swap_address]
    ///     * `refund_address` - the Bitcoin address to refund to
    ///     * `fee_rate_sat_per_vbyte` - the fee rate at which to broadcast the refund transaction
    pub async fn refund(&self, req: &RefundRequest) -> Result<RefundResponse, PaymentError> {
        let refund_tx_id = self
            .chain_swap_handler
            .refund_incoming_swap(
                &req.swap_address,
                &req.refund_address,
                req.fee_rate_sat_per_vbyte,
                true,
            )
            .or_else(|e| {
                warn!("Failed to initiate cooperative refund, switching to non-cooperative: {e:?}");
                self.chain_swap_handler.refund_incoming_swap(
                    &req.swap_address,
                    &req.refund_address,
                    req.fee_rate_sat_per_vbyte,
                    false,
                )
            })
            .await?;

        Ok(RefundResponse { refund_tx_id })
    }

    /// Rescans all expired chain swaps created from calling [LiquidSdk::receive_onchain] to check
    /// if there are any confirmed funds available to refund.
    ///
    /// Since it bypasses the monitoring period, this should be called rarely or when the caller
    /// expects there is a very old refundable chain swap. Otherwise, for relatively recent swaps
    /// (within last [CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS] blocks = ~30 days), calling this
    /// is not necessary as it happens automatically in the background.
    pub async fn rescan_onchain_swaps(&self) -> SdkResult<()> {
        self.chain_swap_handler
            .rescan_incoming_user_lockup_txs(true)
            .await?;
        Ok(())
    }

    fn validate_buy_bitcoin(&self, amount_sat: u64) -> Result<(), PaymentError> {
        ensure_sdk!(
            self.config.network == LiquidNetwork::Mainnet,
            PaymentError::invalid_network("Can only buy bitcoin on Mainnet")
        );
        // The Moonpay API defines BTC amounts as having precision = 5, so only 5 decimals are considered
        ensure_sdk!(
            amount_sat % 1_000 == 0,
            PaymentError::generic("Can only buy sat amounts that are multiples of 1000")
        );
        Ok(())
    }

    /// Prepares to buy Bitcoin via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareBuyBitcoinRequest] containing:
    ///     * `provider` - the [BuyBitcoinProvider] to use
    ///     * `amount_sat` - the amount in satoshis to buy from the provider
    pub async fn prepare_buy_bitcoin(
        &self,
        req: &PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        self.validate_buy_bitcoin(req.amount_sat)?;

        let res = self
            .prepare_receive_payment(&PrepareReceiveRequest {
                payment_method: PaymentMethod::BitcoinAddress,
                payer_amount_sat: Some(req.amount_sat),
            })
            .await?;

        let Some(amount_sat) = res.payer_amount_sat else {
            return Err(PaymentError::Generic {
                err: format!(
                    "Expected field `amount_sat` from response, got {:?}",
                    res.payer_amount_sat
                ),
            });
        };

        Ok(PrepareBuyBitcoinResponse {
            provider: req.provider,
            amount_sat,
            fees_sat: res.fees_sat,
        })
    }

    /// Generate a URL to a third party provider used to buy Bitcoin via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [BuyBitcoinRequest] containing:
    ///     * `prepare_response` - the [PrepareBuyBitcoinResponse] from calling [LiquidSdk::prepare_buy_bitcoin]
    ///     * `redirect_url` - the optional redirect URL the provider should redirect to after purchase
    pub async fn buy_bitcoin(&self, req: &BuyBitcoinRequest) -> Result<String, PaymentError> {
        self.validate_buy_bitcoin(req.prepare_response.amount_sat)?;

        let swap = self
            .create_receive_chain_swap(
                req.prepare_response.amount_sat,
                req.prepare_response.fees_sat,
            )
            .await?;

        Ok(self
            .buy_bitcoin_service
            .buy_bitcoin(
                req.prepare_response.provider,
                &swap,
                req.redirect_url.clone(),
            )
            .await?)
    }

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        let payments_before_sync: HashMap<String, Payment> = self
            .list_payments(&ListPaymentsRequest::default())
            .await?
            .into_iter()
            .flat_map(|payment| {
                // Index payments by both tx_id (lockup/claim) and refund_tx_id
                let mut res = vec![];
                if let Some(tx_id) = payment.tx_id.clone() {
                    res.push((tx_id, payment.clone()));
                }
                if let Some(refund_tx_id) = payment.get_refund_tx_id() {
                    res.push((refund_tx_id, payment));
                }
                res
            })
            .collect();
        if with_scan {
            self.onchain_wallet.full_scan().await?;
        }

        let pending_receive_swaps_by_claim_tx_id =
            self.persister.list_pending_receive_swaps_by_claim_tx_id()?;
        let ongoing_receive_swaps_by_mrh_script_pubkey = self
            .persister
            .list_ongoing_receive_swaps_by_mrh_script_pubkey()?;
        let pending_send_swaps_by_refund_tx_id =
            self.persister.list_pending_send_swaps_by_refund_tx_id()?;
        let pending_chain_swaps_by_claim_tx_id =
            self.persister.list_pending_chain_swaps_by_claim_tx_id()?;
        let pending_chain_swaps_by_refund_tx_id =
            self.persister.list_pending_chain_swaps_by_refund_tx_id()?;

        let tx_map: HashMap<Txid, WalletTx> = self
            .onchain_wallet
            .transactions()
            .await?
            .iter()
            .map(|tx| (tx.txid, tx.clone()))
            .collect();

        for tx in tx_map.values() {
            let tx_id = tx.txid.to_string();
            let is_tx_confirmed = tx.height.is_some();
            let amount_sat = tx.balance.values().sum::<i64>();
            let maybe_script_pubkey = tx
                .outputs
                .iter()
                .find(|output| output.is_some())
                .and_then(|output| output.clone().map(|o| o.script_pubkey.to_hex()));
            let mrh_script_pubkey = maybe_script_pubkey.clone().unwrap_or_default();

            self.persister.insert_or_update_payment(
                PaymentTxData {
                    tx_id: tx_id.clone(),
                    timestamp: tx.timestamp,
                    amount_sat: amount_sat.unsigned_abs(),
                    fees_sat: tx.fee,
                    payment_type: match amount_sat >= 0 {
                        true => PaymentType::Receive,
                        false => PaymentType::Send,
                    },
                    is_confirmed: is_tx_confirmed,
                },
                maybe_script_pubkey,
                None,
            )?;

            if let Some(swap) = pending_receive_swaps_by_claim_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.receive_swap_handler
                        .update_swap_info(&swap.id, Complete, None, None, None, None)
                        .await?;
                }
            } else if let Some(swap) =
                ongoing_receive_swaps_by_mrh_script_pubkey.get(&mrh_script_pubkey)
            {
                // Update the swap status according to the MRH tx confirmation state
                let to_state = match is_tx_confirmed {
                    true => Complete,
                    false => Pending,
                };
                self.receive_swap_handler
                    .update_swap_info(
                        &swap.id,
                        to_state,
                        None,
                        None,
                        Some(&tx_id),
                        Some(amount_sat.unsigned_abs()),
                    )
                    .await?;
                // Remove the used MRH address from the reserved addresses
                self.persister.delete_reserved_address(&swap.mrh_address)?;
            } else if let Some(swap) = pending_send_swaps_by_refund_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.send_swap_handler
                        .update_swap_info(&swap.id, Failed, None, None, None)
                        .await?;
                }
            } else if let Some(swap) = pending_chain_swaps_by_claim_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.chain_swap_handler
                        .update_swap_info(&swap.id, Complete, None, None, None, None)
                        .await?;
                }
            } else if let Some(swap) = pending_chain_swaps_by_refund_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.chain_swap_handler
                        .update_swap_info(&swap.id, Failed, None, None, None, None)
                        .await?;
                }
            } else {
                // Payments that are not directly associated with a swap
                match payments_before_sync.get(&tx_id) {
                    None => {
                        // A completely new payment brought in by this sync, in mempool or confirmed
                        // Covers events:
                        // - onchain Receive Pending and Complete
                        // - onchain Send Complete
                        self.emit_payment_updated(Some(tx_id)).await?;
                    }
                    Some(payment_before_sync) => {
                        if payment_before_sync.status == Pending && is_tx_confirmed {
                            // A know payment that was in the mempool, but is now confirmed
                            // Covers events: Send and Receive direct onchain payments transitioning to Complete
                            self.emit_payment_updated(Some(tx_id)).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Lists the SDK payments in reverse chronological order, from newest to oldest.
    /// The payments are determined based on onchain transactions and swaps.
    pub async fn list_payments(
        &self,
        req: &ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        Ok(self.persister.get_payments(req)?)
    }

    /// Retrieves a payment.
    ///
    /// # Arguments
    ///
    /// * `req` - the [GetPaymentRequest] containing:
    ///     * [GetPaymentRequest::Lightning] - the `payment_hash` of the lightning invoice
    ///
    /// # Returns
    ///
    /// Returns an `Option<Payment>` if found, or `None` if no payment matches the given request.
    pub async fn get_payment(
        &self,
        req: &GetPaymentRequest,
    ) -> Result<Option<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        Ok(self.persister.get_payment_by_request(req)?)
    }

    /// Empties the Liquid Wallet cache for the [Config::network].
    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.config.working_dir.clone());
        path.push(Into::<ElementsNetwork>::into(self.config.network).as_str());
        path.push("enc_cache");

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Synchronizes the local state with the mempool and onchain data.
    pub async fn sync(&self) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();
        let is_first_sync = !self
            .persister
            .get_is_first_sync_complete()?
            .unwrap_or(false);
        match is_first_sync {
            true => {
                self.event_manager.pause_notifications();
                self.sync_payments_with_chain_data(true).await?;
                self.event_manager.resume_notifications();
                self.persister.set_is_first_sync_complete(true)?;
            }
            false => {
                self.sync_payments_with_chain_data(true).await?;
            }
        }
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized with mempool and onchain data (t = {duration_ms} ms)");

        self.notify_event_listeners(SdkEvent::Synced).await?;
        Ok(())
    }

    /// Backup the local state to the provided backup path.
    ///
    /// # Arguments
    ///
    /// * `req` - the [BackupRequest] containing:
    ///     * `backup_path` - the optional backup path. Defaults to [Config::working_dir]
    pub fn backup(&self, req: BackupRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.backup(backup_path)
    }

    /// Restores the local state from the provided backup path.
    ///
    /// # Arguments
    ///
    /// * `req` - the [RestoreRequest] containing:
    ///     * `backup_path` - the optional backup path. Defaults to [Config::working_dir]
    pub fn restore(&self, req: RestoreRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        ensure_sdk!(
            backup_path.exists(),
            SdkError::generic("Backup file does not exist").into()
        );
        self.persister.restore_from_backup(backup_path)
    }

    /// Prepares to pay to an LNURL encoded pay request or lightning address.
    ///
    /// This is the second step of LNURL-pay flow. The first step is [parse], which also validates the LNURL
    /// destination and generates the [LnUrlPayRequest] payload needed here.
    ///
    /// This call will validate the `amount_msat` and `comment` parameters of `req` against the parameters
    /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, a [PrepareSendResponse] is
    /// prepared for the invoice. If the receiver has encoded a Magic Routing Hint in the invoice, the
    /// [PrepareSendResponse]'s `fees_sat` will reflect this.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareLnUrlPayRequest] containing:
    ///     * `data` - the [LnUrlPayRequestData] returned by [parse]
    ///     * `amount_msat` - the amount in millisatoshis for this payment
    ///     * `comment` - an optional comment for this payment
    ///     * `validate_success_action_url` - validates that, if there is a URL success action, the URL domain matches
    ///       the LNURL callback domain. Defaults to 'true'
    ///
    /// # Returns
    /// Returns a [PrepareLnUrlPayResponse] containing:
    ///     * `prepare_send_response` - the prepared [PrepareSendResponse] for the retreived invoice
    ///     * `success_action` - the optional unprocessed LUD-09 success action
    pub async fn prepare_lnurl_pay(
        &self,
        req: PrepareLnUrlPayRequest,
    ) -> Result<PrepareLnUrlPayResponse, LnUrlPayError> {
        match validate_lnurl_pay(
            req.amount_msat,
            &req.comment,
            &req.data,
            self.config.network.into(),
            req.validate_success_action_url,
        )
        .await?
        {
            ValidatedCallbackResponse::EndpointError { data } => {
                Err(LnUrlPayError::Generic { err: data.reason })
            }
            ValidatedCallbackResponse::EndpointSuccess { data } => {
                let prepare_response = self
                    .prepare_send_payment(&PrepareSendRequest {
                        destination: data.pr.clone(),
                        amount: None,
                    })
                    .await
                    .map_err(|e| LnUrlPayError::Generic { err: e.to_string() })?;

                Ok(PrepareLnUrlPayResponse {
                    destination: prepare_response.destination,
                    fees_sat: prepare_response.fees_sat,
                    success_action: data.success_action,
                })
            }
        }
    }

    /// Pay to an LNURL encoded pay request or lightning address.
    ///
    /// The final step of LNURL-pay flow, called after preparing the payment with [LiquidSdk::prepare_lnurl_pay].
    /// This call sends the payment using the [PrepareLnUrlPayResponse]'s `prepare_send_response` either via
    /// Lightning or directly to a Liquid address if a Magic Routing Hint is included in the invoice.
    /// Once the payment is made, the [PrepareLnUrlPayResponse]'s `success_action` is processed decrypting
    /// the AES data if needed.
    ///
    /// # Arguments
    ///
    /// * `req` - the [LnUrlPayRequest] containing:
    ///     * `prepare_response` - the [PrepareLnUrlPayResponse] returned by [LiquidSdk::prepare_lnurl_pay]
    pub async fn lnurl_pay(
        &self,
        req: model::LnUrlPayRequest,
    ) -> Result<LnUrlPayResult, LnUrlPayError> {
        let prepare_response = req.prepare_response;
        let payment = self
            .send_payment(&SendPaymentRequest {
                prepare_response: PrepareSendResponse {
                    destination: prepare_response.destination,
                    fees_sat: prepare_response.fees_sat,
                },
            })
            .await
            .map_err(|e| LnUrlPayError::Generic { err: e.to_string() })?
            .payment;

        let maybe_sa_processed: Option<SuccessActionProcessed> = match prepare_response
            .success_action
        {
            Some(sa) => {
                let processed_sa = match sa {
                    // For AES, we decrypt the contents on the fly
                    SuccessAction::Aes { data } => {
                        let PaymentDetails::Lightning { preimage, .. } = &payment.details else {
                            return Err(LnUrlPayError::Generic {
                                        err: format!("Invalid payment type: expected type `PaymentDetails::Lightning`, got payment details {:?}.", payment.details),
                                    });
                        };

                        let preimage_str = preimage.clone().ok_or(LnUrlPayError::Generic {
                            err: "Payment successful but no preimage found".to_string(),
                        })?;
                        let preimage = sha256::Hash::from_str(&preimage_str).map_err(|_| {
                            LnUrlPayError::Generic {
                                err: "Invalid preimage".to_string(),
                            }
                        })?;
                        let preimage_arr: [u8; 32] = preimage.into_32();
                        let result = match (data, &preimage_arr).try_into() {
                            Ok(data) => AesSuccessActionDataResult::Decrypted { data },
                            Err(e) => AesSuccessActionDataResult::ErrorStatus {
                                reason: e.to_string(),
                            },
                        };
                        SuccessActionProcessed::Aes { result }
                    }
                    SuccessAction::Message { data } => SuccessActionProcessed::Message { data },
                    SuccessAction::Url { data } => SuccessActionProcessed::Url { data },
                };
                Some(processed_sa)
            }
            None => None,
        };

        Ok(LnUrlPayResult::EndpointSuccess {
            data: model::LnUrlPaySuccessData {
                payment,
                success_action: maybe_sa_processed,
            },
        })
    }

    /// Second step of LNURL-withdraw. The first step is [parse], which also validates the LNURL destination
    /// and generates the [LnUrlWithdrawRequest] payload needed here.
    ///
    /// This call will validate the given `amount_msat` against the parameters
    /// of the LNURL endpoint (`data`). If they match the endpoint requirements, the LNURL withdraw
    /// request is made. A successful result here means the endpoint started the payment.
    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        let prepare_response = self
            .prepare_receive_payment(&{
                PrepareReceiveRequest {
                    payment_method: PaymentMethod::Lightning,
                    payer_amount_sat: Some(req.amount_msat / 1_000),
                }
            })
            .await?;
        let receive_res = self
            .receive_payment(&ReceivePaymentRequest {
                prepare_response,
                description: None,
                use_description_hash: Some(false),
            })
            .await?;

        if let Ok(invoice) = parse_invoice(&receive_res.destination) {
            let res = validate_lnurl_withdraw(req.data, invoice).await?;
            Ok(res)
        } else {
            Err(LnUrlWithdrawError::Generic {
                err: "Received unexpected output from receive request".to_string(),
            })
        }
    }

    /// Third and last step of LNURL-auth. The first step is [parse], which also validates the LNURL destination
    /// and generates the [LnUrlAuthRequestData] payload needed here. The second step is user approval of auth action.
    ///
    /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
    /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        Ok(perform_lnurl_auth(&req_data, &SdkLnurlAuthSigner::new(self.signer.clone())).await?)
    }

    /// Register for webhook callbacks at the given `webhook_url`. Each created swap after registering the
    /// webhook will include the `webhook_url`.
    ///
    /// This method should be called every time the application is started and when the `webhook_url` changes.
    /// For example, if the `webhook_url` contains a push notification token and the token changes after
    /// the application was started, then this method should be called to register for callbacks at
    /// the new correct `webhook_url`. To unregister a webhook call [LiquidSdk::unregister_webhook].
    pub async fn register_webhook(&self, webhook_url: String) -> SdkResult<()> {
        info!("Registering for webhook notifications");
        self.persister.set_webhook_url(webhook_url)?;
        Ok(())
    }

    /// Unregister webhook callbacks. Each swap already created will continue to use the registered
    /// `webhook_url` until complete.
    ///
    /// This can be called when callbacks are no longer needed or the `webhook_url`
    /// has changed such that it needs unregistering. For example, the token is valid but the locale changes.
    /// To register a webhook call [LiquidSdk::register_webhook].
    pub async fn unregister_webhook(&self) -> SdkResult<()> {
        info!("Unregistering for webhook notifications");
        self.persister.remove_webhook_url()?;
        Ok(())
    }

    /// Fetch live rates of fiat currencies, sorted by name.
    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        self.fiat_api.fetch_fiat_rates().await.map_err(Into::into)
    }

    /// List all supported fiat currencies for which there is a known exchange rate.
    /// List is sorted by the canonical name of the currency.
    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        self.fiat_api
            .list_fiat_currencies()
            .await
            .map_err(Into::into)
    }

    /// Get the recommended BTC fees based on the configured mempool.space instance.
    pub async fn recommended_fees(&self) -> Result<RecommendedFees, SdkError> {
        Ok(self
            .bitcoin_chain_service
            .lock()
            .await
            .recommended_fees()
            .await?)
    }

    /// Get the full default [Config] for specific [LiquidNetwork].
    pub fn default_config(
        network: LiquidNetwork,
        breez_api_key: Option<String>,
    ) -> Result<Config, SdkError> {
        let config = match network {
            LiquidNetwork::Mainnet => {
                let Some(breez_api_key) = breez_api_key else {
                    return Err(SdkError::Generic {
                        err: "Breez API key must be provided on mainnet.".to_string(),
                    });
                };
                Config::mainnet(breez_api_key)
            }
            LiquidNetwork::Testnet => Config::testnet(breez_api_key),
        };
        Ok(config)
    }

    /// Parses a string into an [InputType]. See [input_parser::parse].
    pub async fn parse(input: &str) -> Result<InputType, PaymentError> {
        if let Ok(offer) = input.parse::<Offer>() {
            // TODO This conversion (between lightning-v0.0.125 to -v0.0.118 Amount types)
            //      won't be needed when Liquid SDK uses the same lightning crate version as sdk-common
            let min_amount = offer
                .amount()
                .map(|amount| match amount {
                    ::lightning::offers::offer::Amount::Bitcoin { amount_msats } => {
                        Ok(Amount::Bitcoin {
                            amount_msat: amount_msats,
                        })
                    }
                    ::lightning::offers::offer::Amount::Currency {
                        iso4217_code,
                        amount,
                    } => Ok(Amount::Currency {
                        iso4217_code: String::from_utf8(iso4217_code.to_vec()).map_err(|_| {
                            anyhow!("Expecting a valid ISO 4217 character sequence")
                        })?,
                        fractional_amount: amount,
                    }),
                })
                .transpose()
                .map_err(|e: anyhow::Error| {
                    PaymentError::generic(&format!("Failed to reconstruct amount: {e:?}"))
                })?;

            return Ok(InputType::Bolt12Offer {
                offer: LNOffer {
                    offer: input.to_string(),
                    chains: offer
                        .chains()
                        .iter()
                        .map(|chain| chain.to_string())
                        .collect(),
                    min_amount,
                    description: offer.description().map(|d| d.to_string()),
                    absolute_expiry: offer.absolute_expiry().map(|expiry| expiry.as_secs()),
                    issuer: offer.issuer().map(|s| s.to_string()),
                    signing_pubkey: offer.signing_pubkey().map(|pk| pk.to_string()),
                    paths: offer
                        .paths()
                        .iter()
                        .map(|path| LnOfferBlindedPath {
                            blinded_hops: path
                                .blinded_hops()
                                .iter()
                                .map(|hop| hop.blinded_node_id.to_hex())
                                .collect(),
                        })
                        .collect::<Vec<LnOfferBlindedPath>>(),
                },
            });
        }

        parse(input)
            .await
            .map_err(|e| PaymentError::generic(&e.to_string()))
    }

    /// Parses a string into an [LNInvoice]. See [invoice::parse_invoice].
    pub fn parse_invoice(input: &str) -> Result<LNInvoice, PaymentError> {
        parse_invoice(input).map_err(|e| PaymentError::invalid_invoice(&e.to_string()))
    }

    /// Configures a global SDK logger that will log to file and will forward log events to
    /// an optional application-specific logger.
    ///
    /// If called, it should be called before any SDK methods (for example, before `connect`).
    ///
    /// It must be called only once in the application lifecycle. Alternatively, If the application
    /// already uses a globally-registered logger, this method shouldn't be called at all.
    ///
    /// ### Arguments
    ///
    /// - `log_dir`: Location where the the SDK log file will be created. The directory must already exist.
    ///
    /// - `app_logger`: Optional application logger.
    ///
    /// If the application is to use it's own logger, but would also like the SDK to log SDK-specific
    /// log output to a file in the configured `log_dir`, then do not register the
    /// app-specific logger as a global logger and instead call this method with the app logger as an arg.
    ///
    /// ### Errors
    ///
    /// An error is thrown if the log file cannot be created in the working directory.
    ///
    /// An error is thrown if a global logger is already configured.
    pub fn init_logging(log_dir: &str, app_logger: Option<Box<dyn log::Log>>) -> Result<()> {
        crate::logger::init_logging(log_dir, app_logger)
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use anyhow::{anyhow, Result};
    use boltz_client::{
        boltz::{self, SwapUpdateTxDetails},
        swaps::boltz::{ChainSwapStates, RevSwapStates, SubSwapStates},
    };
    use lwk_wollet::{elements::Txid, hashes::hex::DisplayHex};
    use tokio::sync::Mutex;

    use crate::{
        model::{Direction, PaymentState, Swap},
        sdk::LiquidSdk,
        test_utils::{
            chain::{MockBitcoinChainService, MockHistory, MockLiquidChainService},
            chain_swap::{new_chain_swap, TEST_BITCOIN_TX},
            persist::{new_persister, new_receive_swap, new_send_swap},
            sdk::{new_liquid_sdk, new_liquid_sdk_with_chain_services},
            status_stream::MockStatusStream,
            swapper::MockSwapper,
            wallet::TEST_LIQUID_TX,
        },
    };
    use paste::paste;

    struct NewSwapArgs {
        direction: Direction,
        accepts_zero_conf: bool,
        initial_payment_state: Option<PaymentState>,
        user_lockup_tx_id: Option<String>,
    }

    impl Default for NewSwapArgs {
        fn default() -> Self {
            Self {
                accepts_zero_conf: false,
                initial_payment_state: None,
                direction: Direction::Outgoing,
                user_lockup_tx_id: None,
            }
        }
    }

    impl NewSwapArgs {
        pub fn set_direction(mut self, direction: Direction) -> Self {
            self.direction = direction;
            self
        }

        pub fn set_accepts_zero_conf(mut self, accepts_zero_conf: bool) -> Self {
            self.accepts_zero_conf = accepts_zero_conf;
            self
        }

        pub fn set_user_lockup_tx_id(mut self, user_lockup_tx_id: Option<String>) -> Self {
            self.user_lockup_tx_id = user_lockup_tx_id;
            self
        }

        pub fn set_initial_payment_state(mut self, payment_state: PaymentState) -> Self {
            self.initial_payment_state = Some(payment_state);
            self
        }
    }

    macro_rules! trigger_swap_update {
        (
            $type:literal,
            $args:expr,
            $persister:expr,
            $status_stream:expr,
            $status:expr,
            $transaction:expr,
            $zero_conf_rejected:expr
        ) => {{
            let swap = match $type {
                "chain" => {
                    let swap = new_chain_swap(
                        $args.direction,
                        $args.initial_payment_state,
                        $args.accepts_zero_conf,
                        $args.user_lockup_tx_id,
                    );
                    $persister.insert_chain_swap(&swap).unwrap();
                    Swap::Chain(swap)
                }
                "send" => {
                    let swap = new_send_swap($args.initial_payment_state);
                    $persister.insert_send_swap(&swap).unwrap();
                    Swap::Send(swap)
                }
                "receive" => {
                    let swap = new_receive_swap($args.initial_payment_state);
                    $persister.insert_receive_swap(&swap).unwrap();
                    Swap::Receive(swap)
                }
                _ => panic!(),
            };

            $status_stream
                .clone()
                .send_mock_update(boltz::Update {
                    id: swap.id(),
                    status: $status.to_string(),
                    transaction: $transaction,
                    zero_conf_rejected: $zero_conf_rejected,
                })
                .await
                .unwrap();

            paste! {
                $persister.[<fetch _ $type _swap_by_id>](&swap.id())
                    .unwrap()
                    .ok_or(anyhow!("Could not retrieve {} swap", $type))
                    .unwrap()
            }
        }};
    }

    #[tokio::test]
    async fn test_receive_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());

        let sdk = Arc::new(new_liquid_sdk(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify the swap becomes invalid after final states are received
            let unrecoverable_states: [RevSwapStates; 4] = [
                RevSwapStates::SwapExpired,
                RevSwapStates::InvoiceExpired,
                RevSwapStates::TransactionFailed,
                RevSwapStates::TransactionRefunded,
            ];

            for status in unrecoverable_states {
                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    None,
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Failed);
            }

            // Check that `TransactionMempool` and `TransactionConfirmed` correctly trigger the claim,
            // which in turn sets the `claim_tx_id`
            for status in [
                RevSwapStates::TransactionMempool,
                RevSwapStates::TransactionConfirmed,
            ] {
                let mock_tx = TEST_LIQUID_TX.clone();
                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    Some(SwapUpdateTxDetails {
                        id: mock_tx.txid().to_string(),
                        hex: lwk_wollet::elements::encode::serialize(&mock_tx)
                            .to_lower_hex_string(),
                    }),
                    None
                );
                assert!(persisted_swap.claim_tx_id.is_some());
            }
        })
        .await
        .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_send_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());

        let sdk = Arc::new(new_liquid_sdk(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify the swap becomes invalid after final states are received
            let unrecoverable_states: [SubSwapStates; 3] = [
                SubSwapStates::TransactionLockupFailed,
                SubSwapStates::InvoiceFailedToPay,
                SubSwapStates::SwapExpired,
            ];

            for status in unrecoverable_states {
                let persisted_swap = trigger_swap_update!(
                    "send",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    None,
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Failed);
            }

            // Verify that `TransactionClaimPending` correctly sets the state to `Complete`
            // and stores the preimage
            let persisted_swap = trigger_swap_update!(
                "send",
                NewSwapArgs::default(),
                persister,
                status_stream,
                SubSwapStates::TransactionClaimPending,
                None,
                None
            );
            assert_eq!(persisted_swap.state, PaymentState::Complete);
            assert!(persisted_swap.preimage.is_some());
        })
        .await
        .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_chain_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
        let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

        let sdk = Arc::new(new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            let trigger_failed: [ChainSwapStates; 3] = [
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::SwapExpired,
                ChainSwapStates::TransactionRefunded,
            ];

            // Checks that work for both incoming and outgoing chain swaps
            for direction in [Direction::Incoming, Direction::Outgoing] {
                // Verify the swap becomes invalid after final states are received
                for status in &trigger_failed {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default().set_direction(direction),
                        persister,
                        status_stream,
                        status,
                        None,
                        None
                    );
                    assert_eq!(persisted_swap.state, PaymentState::Failed);
                }

                let (mock_tx_hex, mock_tx_id) = match direction {
                    Direction::Incoming => {
                        let tx = TEST_LIQUID_TX.clone();
                        (
                            lwk_wollet::elements::encode::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                    Direction::Outgoing => {
                        let tx = TEST_BITCOIN_TX.clone();
                        (
                            sdk_common::bitcoin::consensus::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                };

                // Verify that `TransactionLockupFailed` correctly sets the state as
                // `RefundPending`/`Refundable` or as `Failed` depending on whether or not
                // `user_lockup_tx_id` is present
                for user_lockup_tx_id in &[None, Some(mock_tx_id.clone())] {
                    if let Some(user_lockup_tx_id) = user_lockup_tx_id {
                        match direction {
                            Direction::Incoming => {
                                bitcoin_chain_service
                                    .lock()
                                    .await
                                    .set_history(vec![MockHistory {
                                        txid: Txid::from_str(user_lockup_tx_id).unwrap(),
                                        height: 0,
                                        block_hash: None,
                                        block_timestamp: None,
                                    }]);
                            }
                            Direction::Outgoing => {
                                liquid_chain_service
                                    .lock()
                                    .await
                                    .set_history(vec![MockHistory {
                                        txid: Txid::from_str(user_lockup_tx_id).unwrap(),
                                        height: 0,
                                        block_hash: None,
                                        block_timestamp: None,
                                    }]);
                            }
                        }
                    }
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default()
                            .set_direction(direction)
                            .set_initial_payment_state(PaymentState::Pending)
                            .set_user_lockup_tx_id(user_lockup_tx_id.clone()),
                        persister,
                        status_stream,
                        ChainSwapStates::TransactionLockupFailed,
                        None,
                        None
                    );
                    let expected_state = if user_lockup_tx_id.is_some() {
                        match direction {
                            Direction::Incoming => PaymentState::Refundable,
                            Direction::Outgoing => PaymentState::RefundPending,
                        }
                    } else {
                        PaymentState::Failed
                    };
                    assert_eq!(persisted_swap.state, expected_state);
                }

                // Verify that `TransactionMempool` and `TransactionConfirmed` correctly set
                // `user_lockup_tx_id` and `accept_zero_conf`
                for status in [
                    ChainSwapStates::TransactionMempool,
                    ChainSwapStates::TransactionConfirmed,
                ] {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default().set_direction(direction),
                        persister,
                        status_stream,
                        status,
                        Some(SwapUpdateTxDetails {
                            id: mock_tx_id.clone(),
                            hex: mock_tx_hex.clone(),
                        }), // sets `update.transaction`
                        Some(true) // sets `update.zero_conf_rejected`
                    );
                    assert_eq!(persisted_swap.user_lockup_tx_id, Some(mock_tx_id.clone()));
                    assert!(!persisted_swap.accept_zero_conf);
                }

                // Verify that `TransactionServerMempool` correctly:
                // 1. Sets the payment as `Pending` and creates `server_lockup_tx_id` when
                //    `accepts_zero_conf` is false
                // 2. Sets the payment as `Pending` and creates `claim_tx_id` when `accepts_zero_conf`
                //    is true
                for accepts_zero_conf in [false, true] {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default()
                            .set_direction(direction)
                            .set_accepts_zero_conf(accepts_zero_conf),
                        persister,
                        status_stream,
                        ChainSwapStates::TransactionServerMempool,
                        Some(SwapUpdateTxDetails {
                            id: mock_tx_id.clone(),
                            hex: mock_tx_hex.clone(),
                        }),
                        None
                    );
                    match accepts_zero_conf {
                        false => {
                            assert_eq!(persisted_swap.state, PaymentState::Pending);
                            assert!(persisted_swap.server_lockup_tx_id.is_some());
                        }
                        true => {
                            assert_eq!(persisted_swap.state, PaymentState::Pending);
                            assert!(persisted_swap.claim_tx_id.is_some());
                        }
                    };
                }

                // Verify that `TransactionServerConfirmed` correctly
                // sets the payment as `Pending` and creates `claim_tx_id`
                let persisted_swap = trigger_swap_update!(
                    "chain",
                    NewSwapArgs::default().set_direction(direction),
                    persister,
                    status_stream,
                    ChainSwapStates::TransactionServerConfirmed,
                    Some(SwapUpdateTxDetails {
                        id: mock_tx_id,
                        hex: mock_tx_hex,
                    }),
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Pending);
                assert!(persisted_swap.claim_tx_id.is_some());
            }

            // For outgoing payments, verify that `Created` correctly sets the payment as `Pending` and creates
            // the `user_lockup_tx_id`
            let persisted_swap = trigger_swap_update!(
                "chain",
                NewSwapArgs::default().set_direction(Direction::Outgoing),
                persister,
                status_stream,
                ChainSwapStates::Created,
                None,
                None
            );
            assert_eq!(persisted_swap.state, PaymentState::Pending);
            assert!(persisted_swap.user_lockup_tx_id.is_some());
        })
        .await
        .unwrap();

        Ok(())
    }
}
