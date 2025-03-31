use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Not as _;
use std::time::Instant;
use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, ensure, Result};
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
use lwk_wollet::elements::AssetId;
use lwk_wollet::elements_miniscript::elements::bitcoin::bip32::Xpub;
use lwk_wollet::hashes::{sha256, Hash};
use lwk_wollet::secp256k1::Message;
use lwk_wollet::ElementsNetwork;
use persist::model::PaymentTxDetails;
use recover::recoverer::Recoverer;
use sdk_common::bitcoin::hashes::hex::ToHex;
use sdk_common::input_parser::InputType;
use sdk_common::liquid::LiquidAddressData;
use sdk_common::prelude::{FiatAPI, FiatCurrency, LnUrlPayError, LnUrlWithdrawError, Rate};
use signer::SdkSigner;
use swapper::boltz::proxy::BoltzProxyFetcher;
use tokio::sync::{watch, RwLock};
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
use crate::swapper::SubscriptionHandler;
use crate::swapper::{
    boltz::BoltzSwapper, Swapper, SwapperStatusStream, SwapperSubscriptionHandler,
};
use crate::wallet::{LiquidOnchainWallet, OnchainWallet};
use crate::{
    error::{PaymentError, SdkResult},
    event::EventManager,
    model::*,
    persist::Persister,
    utils, *,
};
use sdk_common::lightning_125::offers::invoice::Bolt12Invoice;

use self::sync::client::BreezSyncerClient;
use self::sync::SyncService;

pub const DEFAULT_DATA_DIR: &str = ".data";
/// Number of blocks to monitor a swap after its timeout block height
pub const CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS: u32 = 4320;

/// A list of external input parsers that are used by default.
/// To opt-out, set `use_default_external_input_parsers` in [Config] to false.
pub const DEFAULT_EXTERNAL_INPUT_PARSERS: &[(&str, &str, &str)] = &[(
    "picknpay",
    "(.*)(za.co.electrum.picknpay)(.*)",
    "https://cryptoqr.net/.well-known/lnurlp/<input>",
)];

pub(crate) const NETWORK_PROPAGATION_GRACE_PERIOD: Duration = Duration::from_secs(30);

pub(crate) struct LiquidSdkBuilder {
    config: Config,
    signer: Arc<Box<dyn Signer>>,
    breez_server: Arc<BreezServer>,
    bitcoin_chain_service: Option<Arc<dyn BitcoinChainService>>,
    liquid_chain_service: Option<Arc<dyn LiquidChainService>>,
    onchain_wallet: Option<Arc<dyn OnchainWallet>>,
    persister: Option<Arc<Persister>>,
    recoverer: Option<Arc<Recoverer>>,
    rest_client: Option<Arc<dyn RestClient>>,
    status_stream: Option<Arc<dyn SwapperStatusStream>>,
    swapper: Option<Arc<dyn Swapper>>,
    sync_service: Option<Arc<SyncService>>,
}

#[allow(dead_code)]
impl LiquidSdkBuilder {
    pub fn new(
        config: Config,
        server_url: String,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<LiquidSdkBuilder> {
        let breez_server = Arc::new(BreezServer::new(server_url, None)?);
        Ok(LiquidSdkBuilder {
            config,
            signer,
            breez_server,
            bitcoin_chain_service: None,
            liquid_chain_service: None,
            onchain_wallet: None,
            persister: None,
            recoverer: None,
            rest_client: None,
            status_stream: None,
            swapper: None,
            sync_service: None,
        })
    }

    pub fn bitcoin_chain_service(
        &mut self,
        bitcoin_chain_service: Arc<dyn BitcoinChainService>,
    ) -> &mut Self {
        self.bitcoin_chain_service = Some(bitcoin_chain_service.clone());
        self
    }

    pub fn liquid_chain_service(
        &mut self,
        liquid_chain_service: Arc<dyn LiquidChainService>,
    ) -> &mut Self {
        self.liquid_chain_service = Some(liquid_chain_service.clone());
        self
    }

    pub fn recoverer(&mut self, recoverer: Arc<Recoverer>) -> &mut Self {
        self.recoverer = Some(recoverer.clone());
        self
    }

    pub fn onchain_wallet(&mut self, onchain_wallet: Arc<dyn OnchainWallet>) -> &mut Self {
        self.onchain_wallet = Some(onchain_wallet.clone());
        self
    }

    pub fn persister(&mut self, persister: Arc<Persister>) -> &mut Self {
        self.persister = Some(persister.clone());
        self
    }

    pub fn rest_client(&mut self, rest_client: Arc<dyn RestClient>) -> &mut Self {
        self.rest_client = Some(rest_client.clone());
        self
    }

    pub fn status_stream(&mut self, status_stream: Arc<dyn SwapperStatusStream>) -> &mut Self {
        self.status_stream = Some(status_stream.clone());
        self
    }

    pub fn swapper(&mut self, swapper: Arc<dyn Swapper>) -> &mut Self {
        self.swapper = Some(swapper.clone());
        self
    }

    pub fn sync_service(&mut self, sync_service: Arc<SyncService>) -> &mut Self {
        self.sync_service = Some(sync_service.clone());
        self
    }

    pub fn build(&self) -> Result<Arc<LiquidSdk>> {
        if let Some(breez_api_key) = &self.config.breez_api_key {
            LiquidSdk::validate_breez_api_key(breez_api_key)?
        }

        fs::create_dir_all(&self.config.working_dir)?;
        let fingerprint_hex: String =
            Xpub::decode(self.signer.xpub()?.as_slice())?.identifier()[0..4].to_hex();
        let working_dir = self
            .config
            .get_wallet_dir(&self.config.working_dir, &fingerprint_hex)?;
        let cache_dir = self.config.get_wallet_dir(
            self.config
                .cache_dir
                .as_ref()
                .unwrap_or(&self.config.working_dir),
            &fingerprint_hex,
        )?;

        let sync_enabled = self
            .config
            .sync_service_url
            .clone()
            .map(|_| true)
            .unwrap_or(false);

        let persister = match self.persister.clone() {
            Some(persister) => persister,
            None => {
                let persister = Arc::new(Persister::new(
                    &working_dir,
                    self.config.network,
                    sync_enabled,
                )?);
                persister.init()?;
                persister.replace_asset_metadata(self.config.asset_metadata.clone())?;
                persister
            }
        };

        let rest_client: Arc<dyn RestClient> = match self.rest_client.clone() {
            Some(rest_client) => rest_client,
            None => Arc::new(ReqwestRestClient::new()?),
        };

        let bitcoin_chain_service: Arc<dyn BitcoinChainService> =
            match self.bitcoin_chain_service.clone() {
                Some(bitcoin_chain_service) => bitcoin_chain_service,
                None => Arc::new(HybridBitcoinChainService::new(
                    self.config.clone(),
                    rest_client.clone(),
                )?),
            };

        let liquid_chain_service: Arc<dyn LiquidChainService> =
            match self.liquid_chain_service.clone() {
                Some(liquid_chain_service) => liquid_chain_service,
                None => Arc::new(HybridLiquidChainService::new(self.config.clone())?),
            };

        let onchain_wallet: Arc<dyn OnchainWallet> = match self.onchain_wallet.clone() {
            Some(onchain_wallet) => onchain_wallet,
            None => Arc::new(LiquidOnchainWallet::new(
                self.config.clone(),
                &cache_dir,
                persister.clone(),
                self.signer.clone(),
            )?),
        };

        let event_manager = Arc::new(EventManager::new());
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        let (swapper, status_stream): (Arc<dyn Swapper>, Arc<dyn SwapperStatusStream>) =
            match (self.swapper.clone(), self.status_stream.clone()) {
                (Some(swapper), Some(status_stream)) => (swapper, status_stream),
                (maybe_swapper, maybe_status_stream) => {
                    let proxy_url_fetcher = Arc::new(BoltzProxyFetcher::new(persister.clone()));
                    let boltz_swapper =
                        Arc::new(BoltzSwapper::new(self.config.clone(), proxy_url_fetcher)?);
                    (
                        maybe_swapper.unwrap_or(boltz_swapper.clone()),
                        maybe_status_stream.unwrap_or(boltz_swapper),
                    )
                }
            };

        let recoverer = match self.recoverer.clone() {
            Some(recoverer) => recoverer,
            None => Arc::new(Recoverer::new(
                self.signer.slip77_master_blinding_key()?,
                swapper.clone(),
                onchain_wallet.clone(),
                liquid_chain_service.clone(),
                bitcoin_chain_service.clone(),
                persister.clone(),
            )?),
        };

        let sync_service = match self.sync_service.clone() {
            Some(sync_service) => Some(sync_service),
            None => match self.config.sync_service_url.clone() {
                Some(sync_service_url) => {
                    if BREEZ_SYNC_SERVICE_URL == sync_service_url
                        && self.config.breez_api_key.is_none()
                    {
                        anyhow::bail!(
                            "Cannot start the Breez real-time sync service without providing a valid API key. See https://sdk-doc-liquid.breez.technology/guide/getting_started.html#api-key",
                        );
                    }

                    let syncer_client =
                        Box::new(BreezSyncerClient::new(self.config.breez_api_key.clone()));
                    Some(Arc::new(SyncService::new(
                        sync_service_url,
                        persister.clone(),
                        recoverer.clone(),
                        self.signer.clone(),
                        syncer_client,
                    )))
                }
                None => None,
            },
        };

        let send_swap_handler = SendSwapHandler::new(
            self.config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
            recoverer.clone(),
        );

        let receive_swap_handler = ReceiveSwapHandler::new(
            self.config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
        );

        let chain_swap_handler = Arc::new(ChainSwapHandler::new(
            self.config.clone(),
            onchain_wallet.clone(),
            persister.clone(),
            swapper.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
        )?);

        let buy_bitcoin_service = Arc::new(BuyBitcoinService::new(
            self.config.clone(),
            self.breez_server.clone(),
        ));

        let external_input_parsers = self.config.get_all_external_input_parsers();

        let sdk = Arc::new(LiquidSdk {
            config: self.config.clone(),
            onchain_wallet,
            signer: self.signer.clone(),
            persister: persister.clone(),
            rest_client,
            event_manager,
            status_stream: status_stream.clone(),
            swapper,
            recoverer,
            bitcoin_chain_service,
            liquid_chain_service,
            fiat_api: self.breez_server.clone(),
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
            send_swap_handler,
            receive_swap_handler,
            sync_service,
            chain_swap_handler,
            buy_bitcoin_service,
            external_input_parsers,
        });
        Ok(sdk)
    }
}

pub struct LiquidSdk {
    pub(crate) config: Config,
    pub(crate) onchain_wallet: Arc<dyn OnchainWallet>,
    pub(crate) signer: Arc<Box<dyn Signer>>,
    pub(crate) persister: Arc<Persister>,
    pub(crate) rest_client: Arc<dyn RestClient>,
    pub(crate) event_manager: Arc<EventManager>,
    pub(crate) status_stream: Arc<dyn SwapperStatusStream>,
    pub(crate) swapper: Arc<dyn Swapper>,
    pub(crate) recoverer: Arc<Recoverer>,
    pub(crate) liquid_chain_service: Arc<dyn LiquidChainService>,
    pub(crate) bitcoin_chain_service: Arc<dyn BitcoinChainService>,
    pub(crate) fiat_api: Arc<dyn FiatAPI>,
    pub(crate) is_started: RwLock<bool>,
    pub(crate) shutdown_sender: watch::Sender<()>,
    pub(crate) shutdown_receiver: watch::Receiver<()>,
    pub(crate) send_swap_handler: SendSwapHandler,
    pub(crate) sync_service: Option<Arc<SyncService>>,
    pub(crate) receive_swap_handler: ReceiveSwapHandler,
    pub(crate) chain_swap_handler: Arc<ChainSwapHandler>,
    pub(crate) buy_bitcoin_service: Arc<dyn BuyBitcoinApi>,
    pub(crate) external_input_parsers: Vec<ExternalInputParser>,
}

impl LiquidSdk {
    /// Initializes the SDK services and starts the background tasks.
    /// This must be called to create the [LiquidSdk] instance.
    ///
    /// # Arguments
    ///
    /// * `req` - the [ConnectRequest] containing:
    ///     * `config` - the SDK [Config]
    ///     * `mnemonic` - the optional Liquid wallet mnemonic
    ///     * `passphrase` - the optional passphrase for the mnemonic
    ///     * `seed` - the optional Liquid wallet seed
    pub async fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let start_ts = Instant::now();
        let is_mainnet = req.config.network == LiquidNetwork::Mainnet;

        let signer = match (req.mnemonic, req.seed) {
            (None, Some(seed)) => Box::new(SdkSigner::new_with_seed(seed, is_mainnet)?),
            (Some(mnemonic), None) => Box::new(SdkSigner::new(
                &mnemonic,
                req.passphrase.unwrap_or("".to_string()).as_ref(),
                is_mainnet,
            )?),
            _ => return Err(anyhow!("Either `mnemonic` or `seed` must be set")),
        };

        let sdk =
            Self::connect_with_signer(ConnectWithSignerRequest { config: req.config }, signer)
                .inspect_err(|e| error!("Failed to connect: {:?}", e))
                .await;

        let init_time = Instant::now().duration_since(start_ts);
        utils::log_print_header(init_time);

        sdk
    }

    pub async fn connect_with_signer(
        req: ConnectWithSignerRequest,
        signer: Box<dyn Signer>,
    ) -> Result<Arc<LiquidSdk>> {
        let sdk = LiquidSdkBuilder::new(
            req.config,
            PRODUCTION_BREEZSERVER_URL.into(),
            Arc::new(signer),
        )?
        .build()?;
        sdk.start()
            .inspect_err(|e| error!("Failed to start an SDK instance: {:?}", e))
            .await?;
        Ok(sdk)
    }

    fn validate_breez_api_key(api_key: &str) -> Result<()> {
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

    /// Starts an SDK instance.
    ///
    /// Internal method. Should only be called once per instance.
    /// Should only be called as part of [LiquidSdk::connect].
    async fn start(self: &Arc<LiquidSdk>) -> SdkResult<()> {
        let mut is_started = self.is_started.write().await;
        self.persister
            .update_send_swaps_by_state(Created, TimedOut, Some(true))
            .inspect_err(|e| error!("Failed to update send swaps by state: {:?}", e))?;

        self.start_background_tasks()
            .inspect_err(|e| error!("Failed to start background tasks: {:?}", e))
            .await?;
        *is_started = true;
        Ok(())
    }

    /// Starts background tasks.
    ///
    /// Internal method. Should only be used as part of [LiquidSdk::start].
    async fn start_background_tasks(self: &Arc<LiquidSdk>) -> SdkResult<()> {
        let subscription_handler = Box::new(SwapperSubscriptionHandler::new(
            self.persister.clone(),
            self.status_stream.clone(),
        ));
        self.status_stream
            .clone()
            .start(subscription_handler.clone(), self.shutdown_receiver.clone());
        if let Some(sync_service) = self.sync_service.clone() {
            sync_service.start(self.shutdown_receiver.clone());
        }
        self.track_new_blocks();
        self.track_swap_updates();
        self.track_realtime_sync_events(subscription_handler);

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

    fn track_realtime_sync_events(
        self: &Arc<LiquidSdk>,
        subscription_handler: Box<dyn SubscriptionHandler>,
    ) {
        let cloned = self.clone();
        let Some(sync_service) = cloned.sync_service.clone() else {
            return;
        };
        let mut shutdown_receiver = cloned.shutdown_receiver.clone();

        tokio::spawn(async move {
            let mut sync_events_receiver = sync_service.subscribe_events();
            loop {
                tokio::select! {
                    event = sync_events_receiver.recv() => {
                      if let Ok(e) = event {
                        match e {
                          sync::Event::SyncedCompleted{data} => {
                            info!(
                              "Received sync event: pulled {} records, pushed {} records",
                              data.pulled_records_count, data.pushed_records_count
                            );
                            if data.pulled_records_count > 0 {
                              subscription_handler.subscribe_swaps().await;
                            }
                          }
                        }
                      }
                    }
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting real-time sync loop");
                        return;
                    }
                }
            }
        });
    }

    fn track_new_blocks(self: &Arc<LiquidSdk>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut current_liquid_block: u32 = 0;
            let mut current_bitcoin_block: u32 = 0;
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        info!("Track blocks loop ticked");
                        // Get the Liquid tip and process a new block
                        let t0 = Instant::now();
                        let liquid_tip_res = cloned.liquid_chain_service.tip().await;
                        let duration_ms = Instant::now().duration_since(t0).as_millis();
                        info!("Fetched liquid tip at ({duration_ms} ms)");

                        let is_new_liquid_block = match &liquid_tip_res {
                            Ok(height) => {
                                debug!("Got Liquid tip: {height}");
                                let is_new_liquid_block = *height > current_liquid_block;
                                current_liquid_block = *height;
                                is_new_liquid_block
                            },
                            Err(e) => {
                                error!("Failed to fetch Liquid tip {e}");
                                false
                            }
                        };
                        // Get the Bitcoin tip and process a new block
                        let t0 = Instant::now();
                        let bitcoin_tip_res = cloned.bitcoin_chain_service.tip().map(|tip| tip.height as u32);
                        let duration_ms = Instant::now().duration_since(t0).as_millis();
                        info!("Fetched bitcoin tip at ({duration_ms} ms)");
                        let is_new_bitcoin_block = match &bitcoin_tip_res {
                            Ok(height) => {
                                debug!("Got Bitcoin tip: {height}");
                                let is_new_bitcoin_block = *height > current_bitcoin_block;
                                current_bitcoin_block = *height;
                                is_new_bitcoin_block
                            },
                            Err(e) => {
                                error!("Failed to fetch Bitcoin tip {e}");
                                false
                            }
                        };

                        if let (Ok(liquid_tip), Ok(bitcoin_tip)) = (liquid_tip_res, bitcoin_tip_res) {
                            cloned.persister.set_blockchain_info(&BlockchainInfo {
                                liquid_tip,
                                bitcoin_tip
                            })
                            .unwrap_or_else(|err| warn!("Could not update local tips: {err:?}"));
                        };

                        // Only partial sync when there are no new Liquid or Bitcoin blocks
                        let partial_sync = (is_new_liquid_block || is_new_bitcoin_block).not();
                        _ = cloned.sync(partial_sync).await;

                        // Update swap handlers
                        if is_new_liquid_block {
                            cloned.chain_swap_handler.on_liquid_block(current_liquid_block).await;
                            cloned.receive_swap_handler.on_liquid_block(current_liquid_block).await;
                            cloned.send_swap_handler.on_liquid_block(current_liquid_block).await;
                        }
                        if is_new_bitcoin_block {
                            cloned.chain_swap_handler.on_bitcoin_block(current_bitcoin_block).await;
                            cloned.receive_swap_handler.on_bitcoin_block(current_liquid_block).await;
                            cloned.send_swap_handler.on_bitcoin_block(current_bitcoin_block).await;
                        }
                    }

                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting track blocks loop");
                        return;
                    }
                }
            }
        });
    }

    fn track_swap_updates(self: &Arc<LiquidSdk>) {
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
                            // Ensure balance is fully synced before emitting PaymentSucceeded
                            self.sync_payments_with_chain_data(false).await?;
                            self.update_wallet_info().await?;
                            self.notify_event_listeners(SdkEvent::PaymentSucceeded {
                                details: payment,
                            })
                            .await?
                        }
                        Pending => {
                            match &payment.details.get_swap_id() {
                                Some(swap_id) => match self.persister.fetch_swap_by_id(swap_id)? {
                                    Swap::Chain(ChainSwap { claim_tx_id, .. }) => {
                                        if claim_tx_id.is_some() {
                                            // The claim tx has now been broadcast
                                            self.notify_event_listeners(
                                                SdkEvent::PaymentWaitingConfirmation {
                                                    details: payment,
                                                },
                                            )
                                            .await?
                                        } else {
                                            // The lockup tx is in the mempool/confirmed
                                            self.notify_event_listeners(SdkEvent::PaymentPending {
                                                details: payment,
                                            })
                                            .await?
                                        }
                                    }
                                    Swap::Receive(ReceiveSwap {
                                        claim_tx_id,
                                        mrh_tx_id,
                                        ..
                                    }) => {
                                        if claim_tx_id.is_some() || mrh_tx_id.is_some() {
                                            // The a claim or mrh tx has now been broadcast
                                            self.notify_event_listeners(
                                                SdkEvent::PaymentWaitingConfirmation {
                                                    details: payment,
                                                },
                                            )
                                            .await?
                                        } else {
                                            // The lockup tx is in the mempool/confirmed
                                            self.notify_event_listeners(SdkEvent::PaymentPending {
                                                details: payment,
                                            })
                                            .await?
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
                        WaitingFeeAcceptance => {
                            let swap_id = &payment
                                .details
                                .get_swap_id()
                                .ok_or(anyhow!("Payment WaitingFeeAcceptance must have a swap"))?;

                            ensure!(
                                matches!(
                                    self.persister.fetch_swap_by_id(swap_id)?,
                                    Swap::Chain(ChainSwap { .. })
                                ),
                                "Swap in WaitingFeeAcceptance payment must be chain swap"
                            );

                            self.notify_event_listeners(SdkEvent::PaymentWaitingFeeAcceptance {
                                details: payment,
                            })
                            .await?;
                        }
                        Refundable => {
                            self.notify_event_listeners(SdkEvent::PaymentRefundable {
                                details: payment,
                            })
                            .await?
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

    /// Get the wallet and blockchain info from local storage
    pub async fn get_info(&self) -> SdkResult<GetInfoResponse> {
        self.ensure_is_started().await?;
        let maybe_info = self.persister.get_info()?;
        match maybe_info {
            Some(info) => Ok(info),
            None => {
                self.update_wallet_info().await?;
                self.persister.get_info()?.ok_or(SdkError::Generic {
                    err: "Info not found".into(),
                })
            }
        }
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
        match self.parse(input).await? {
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
            ("regtest", LiquidNetwork::Regtest) => {}
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
    async fn validate_submarine_pairs(
        &self,
        receiver_amount_sat: u64,
    ) -> Result<SubmarinePair, PaymentError> {
        let lbtc_pair = self
            .swapper
            .get_submarine_pairs()
            .await?
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(receiver_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(receiver_amount_sat);

        ensure_sdk!(
            receiver_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(lbtc_pair)
    }

    async fn get_chain_pair(&self, direction: Direction) -> Result<ChainPair, PaymentError> {
        self.swapper
            .get_chain_pair(direction)
            .await?
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

    async fn get_and_validate_chain_pair(
        &self,
        direction: Direction,
        user_lockup_amount_sat: Option<u64>,
    ) -> Result<ChainPair, PaymentError> {
        let pair = self.get_chain_pair(direction).await?;
        if let Some(user_lockup_amount_sat) = user_lockup_amount_sat {
            self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;
        }
        Ok(pair)
    }

    /// Estimate the onchain fee for sending the given amount to the given destination address
    async fn estimate_onchain_tx_fee(
        &self,
        amount_sat: u64,
        address: &str,
        asset_id: &str,
    ) -> Result<u64, PaymentError> {
        let fee_sat = self
            .onchain_wallet
            .build_tx(
                Some(LIQUID_FEE_RATE_MSAT_PER_VBYTE),
                address,
                asset_id,
                amount_sat,
            )
            .await?
            .all_fees()
            .values()
            .sum::<u64>();
        info!("Estimated tx fee: {fee_sat} sat");
        Ok(fee_sat)
    }

    fn get_temp_p2tr_addr(&self) -> &str {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        match self.config.network {
            LiquidNetwork::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            LiquidNetwork::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5",
            LiquidNetwork::Regtest => "el1pqtjufhhy2se6lj2t7wufvpqqhnw66v57x2s0uu5dxs4fqlzlvh3hqe87vn83z3qreh8kxn49xe0h0fpe4kjkhl4gv99tdppupk0tdd485q8zegdag97r",
        }
    }

    /// Estimate the lockup tx fee for Send and Chain Send swaps
    async fn estimate_lockup_tx_fee(
        &self,
        user_lockup_amount_sat: u64,
    ) -> Result<u64, PaymentError> {
        let temp_p2tr_addr = self.get_temp_p2tr_addr();
        self.estimate_onchain_tx_fee(
            user_lockup_amount_sat,
            temp_p2tr_addr,
            self.config.lbtc_asset_id().as_str(),
        )
        .await
    }

    async fn estimate_drain_tx_fee(
        &self,
        enforce_amount_sat: Option<u64>,
        address: Option<&str>,
    ) -> Result<u64, PaymentError> {
        let receipent_address = address.unwrap_or(self.get_temp_p2tr_addr());
        let fee_sat = self
            .onchain_wallet
            .build_drain_tx(
                Some(LIQUID_FEE_RATE_MSAT_PER_VBYTE),
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
        asset_id: &str,
    ) -> Result<u64, PaymentError> {
        match self
            .estimate_onchain_tx_fee(amount_sat, address, asset_id)
            .await
        {
            Ok(fees_sat) => Ok(fees_sat),
            Err(PaymentError::InsufficientFunds) if asset_id.eq(&self.config.lbtc_asset_id()) => {
                self.estimate_drain_tx_fee(Some(amount_sat), Some(address))
                    .await
                    .map_err(|_| PaymentError::InsufficientFunds)
            }
            Err(e) => Err(e),
        }
    }

    async fn estimate_lockup_tx_or_drain_tx_fee(
        &self,
        amount_sat: u64,
    ) -> Result<u64, PaymentError> {
        let temp_p2tr_addr = self.get_temp_p2tr_addr();
        self.estimate_onchain_tx_or_drain_tx_fee(
            amount_sat,
            temp_p2tr_addr,
            &self.config.lbtc_asset_id(),
        )
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
    ///        - [PayAmount::Drain] which uses all Bitcoin funds
    ///        - [PayAmount::Bitcoin] which sets the amount in satoshi that will be received
    ///        - [PayAmount::Asset] which sets the amount of an asset that will be received
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
        let asset_id;
        let payment_destination;

        match self.parse(&req.destination).await {
            Ok(InputType::LiquidAddress {
                address: mut liquid_address_data,
            }) => {
                let amount = match (
                    liquid_address_data.amount,
                    liquid_address_data.amount_sat,
                    liquid_address_data.asset_id,
                    req.amount.clone(),
                ) {
                    (Some(amount), Some(amount_sat), Some(asset_id), None) => {
                        if asset_id.eq(&self.config.lbtc_asset_id()) {
                            PayAmount::Bitcoin {
                                receiver_amount_sat: amount_sat,
                            }
                        } else {
                            PayAmount::Asset {
                                asset_id,
                                receiver_amount: amount,
                            }
                        }
                    }
                    (_, Some(amount_sat), None, None) => PayAmount::Bitcoin {
                        receiver_amount_sat: amount_sat,
                    },
                    (_, _, _, Some(amount)) => amount,
                    _ => {
                        return Err(PaymentError::AmountMissing {
                            err: "Amount must be set when paying to a Liquid address".to_string(),
                        });
                    }
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

                (asset_id, receiver_amount_sat, fees_sat) = match amount {
                    PayAmount::Drain => {
                        ensure_sdk!(
                            get_info_res.wallet_info.pending_receive_sat == 0
                                && get_info_res.wallet_info.pending_send_sat == 0,
                            PaymentError::Generic {
                                err: "Cannot drain while there are pending payments".to_string(),
                            }
                        );
                        let drain_fees_sat = self
                            .estimate_drain_tx_fee(None, Some(&liquid_address_data.address))
                            .await?;
                        let drain_amount_sat =
                            get_info_res.wallet_info.balance_sat - drain_fees_sat;
                        info!("Drain amount: {drain_amount_sat} sat");
                        (
                            self.config.lbtc_asset_id(),
                            drain_amount_sat,
                            drain_fees_sat,
                        )
                    }
                    PayAmount::Bitcoin {
                        receiver_amount_sat,
                    } => {
                        let asset_id = self.config.lbtc_asset_id();
                        let fees_sat = self
                            .estimate_onchain_tx_or_drain_tx_fee(
                                receiver_amount_sat,
                                &liquid_address_data.address,
                                &asset_id,
                            )
                            .await?;
                        (asset_id, receiver_amount_sat, fees_sat)
                    }
                    PayAmount::Asset {
                        asset_id,
                        receiver_amount,
                    } => {
                        let asset_metadata = self.persister.get_asset_metadata(&asset_id)?.ok_or(
                            PaymentError::AssetError {
                                err: format!("Asset {asset_id} is not supported"),
                            },
                        )?;
                        let receiver_amount_sat = asset_metadata.amount_to_sat(receiver_amount);
                        let fees_sat = self
                            .estimate_onchain_tx_or_drain_tx_fee(
                                receiver_amount_sat,
                                &liquid_address_data.address,
                                &asset_id,
                            )
                            .await?;
                        (asset_id, receiver_amount_sat, fees_sat)
                    }
                };

                liquid_address_data.amount_sat = Some(receiver_amount_sat);
                liquid_address_data.asset_id = Some(asset_id.clone());
                payment_destination = SendDestination::LiquidAddress {
                    address_data: liquid_address_data,
                    bip353_address: None,
                };
            }
            Ok(InputType::Bolt11 { invoice }) => {
                self.ensure_send_is_not_self_transfer(&invoice.bolt11)?;
                self.validate_bolt11_invoice(&invoice.bolt11)?;

                let invoice_amount_sat = invoice.amount_msat.ok_or(
                    PaymentError::amount_missing("Expected invoice with an amount"),
                )? / 1000;

                if let Some(PayAmount::Bitcoin {
                    receiver_amount_sat: amount_sat,
                }) = req.amount
                {
                    ensure_sdk!(
                        invoice_amount_sat == amount_sat,
                        PaymentError::Generic {
                            err: "Receiver amount and invoice amount do not match".to_string()
                        }
                    );
                }

                let lbtc_pair = self.validate_submarine_pairs(invoice_amount_sat).await?;
                let mrh_address = self
                    .swapper
                    .check_for_mrh(&invoice.bolt11)
                    .await?
                    .map(|(address, _)| address);
                asset_id = self.config.lbtc_asset_id();
                (receiver_amount_sat, fees_sat, payment_destination) =
                    match (mrh_address.clone(), req.amount.clone()) {
                        (Some(lbtc_address), Some(PayAmount::Drain)) => {
                            // The BOLT11 invoice has an MRH and it is requested that the wallet balance is to be drained,
                            // therefore we use the MRH address and drain the balance (overpaying the invoice if neccessary)
                            let drain_fees_sat = self
                                .estimate_drain_tx_fee(None, Some(&lbtc_address))
                                .await?;
                            let drain_amount_sat =
                                get_info_res.wallet_info.balance_sat - drain_fees_sat;
                            let payment_destination = SendDestination::LiquidAddress {
                                address_data: LiquidAddressData {
                                    address: lbtc_address,
                                    asset_id: Some(asset_id.clone()),
                                    amount: None,
                                    amount_sat: Some(drain_amount_sat),
                                    network: self.config.network.into(),
                                    label: None,
                                    message: None,
                                },
                                bip353_address: None,
                            };
                            (drain_amount_sat, drain_fees_sat, payment_destination)
                        }
                        (Some(lbtc_address), _) => {
                            // The BOLT11 invoice has an MRH but no drain is requested,
                            // so we calculate the fees of a direct Liquid transaction
                            let fees_sat = self
                                .estimate_onchain_tx_or_drain_tx_fee(
                                    invoice_amount_sat,
                                    &lbtc_address,
                                    &asset_id,
                                )
                                .await?;
                            (
                                invoice_amount_sat,
                                fees_sat,
                                SendDestination::Bolt11 {
                                    invoice,
                                    bip353_address: None,
                                },
                            )
                        }
                        (None, _) => {
                            // The BOLT11 invoice has no MRH, so we calculate the fees using a swap
                            let boltz_fees_total = lbtc_pair.fees.total(invoice_amount_sat);
                            let user_lockup_amount_sat = invoice_amount_sat + boltz_fees_total;
                            let lockup_fees_sat = self
                                .estimate_lockup_tx_or_drain_tx_fee(user_lockup_amount_sat)
                                .await?;
                            let fees_sat = boltz_fees_total + lockup_fees_sat;
                            (
                                invoice_amount_sat,
                                fees_sat,
                                SendDestination::Bolt11 {
                                    invoice,
                                    bip353_address: None,
                                },
                            )
                        }
                    };
            }
            Ok(InputType::Bolt12Offer {
                offer,
                bip353_address,
            }) => {
                receiver_amount_sat = match req.amount {
                    Some(PayAmount::Bitcoin {
                        receiver_amount_sat: amount_sat,
                    }) => Ok(amount_sat),
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

                let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat).await?;

                let boltz_fees_total = lbtc_pair.fees.total(receiver_amount_sat);
                let lockup_fees_sat = self
                    .estimate_lockup_tx_or_drain_tx_fee(receiver_amount_sat + boltz_fees_total)
                    .await?;
                asset_id = self.config.lbtc_asset_id();
                fees_sat = boltz_fees_total + lockup_fees_sat;

                payment_destination = SendDestination::Bolt12 {
                    offer,
                    receiver_amount_sat,
                    bip353_address,
                };
            }
            _ => {
                return Err(PaymentError::generic("Destination is not valid"));
            }
        };

        get_info_res.wallet_info.validate_sufficient_funds(
            self.config.network,
            receiver_amount_sat,
            fees_sat,
            &asset_id,
        )?;

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
                bip353_address,
            } => {
                let Some(amount_sat) = liquid_address_data.amount_sat else {
                    return Err(PaymentError::AmountMissing {
                        err: "Amount must be set when paying to a Liquid address".to_string(),
                    });
                };
                let Some(ref asset_id) = liquid_address_data.asset_id else {
                    return Err(PaymentError::asset_error(
                        "Asset must be set when paying to a Liquid address",
                    ));
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

                self.get_info()
                    .await?
                    .wallet_info
                    .validate_sufficient_funds(
                        self.config.network,
                        amount_sat,
                        *fees_sat,
                        asset_id,
                    )?;
                let mut response = self
                    .pay_liquid(liquid_address_data.clone(), amount_sat, *fees_sat, true)
                    .await?;
                self.insert_bip353_payment_details(bip353_address, &mut response)?;
                Ok(response)
            }
            SendDestination::Bolt11 {
                invoice,
                bip353_address,
            } => {
                let mut response = self.pay_bolt11_invoice(&invoice.bolt11, *fees_sat).await?;
                self.insert_bip353_payment_details(bip353_address, &mut response)?;
                Ok(response)
            }
            SendDestination::Bolt12 {
                offer,
                receiver_amount_sat,
                bip353_address,
            } => {
                let bolt12_invoice = self
                    .swapper
                    .get_bolt12_invoice(&offer.offer, *receiver_amount_sat)
                    .await?;
                let mut response = self
                    .pay_bolt12_invoice(offer, *receiver_amount_sat, &bolt12_invoice, *fees_sat)
                    .await?;
                self.insert_bip353_payment_details(bip353_address, &mut response)?;
                Ok(response)
            }
        }
    }

    fn insert_bip353_payment_details(
        &self,
        bip353_address: &Option<String>,
        response: &mut SendPaymentResponse,
    ) -> Result<()> {
        if bip353_address.is_some() {
            if let (Some(tx_id), Some(destination)) =
                (&response.payment.tx_id, &response.payment.destination)
            {
                self.persister
                    .insert_or_update_payment_details(PaymentTxDetails {
                        tx_id: tx_id.clone(),
                        destination: destination.clone(),
                        description: None,
                        lnurl_info: None,
                        bip353_address: bip353_address.clone(),
                    })?;
                // Get the payment with the bip353_address details
                if let Some(payment) = self.persister.get_payment(tx_id)? {
                    response.payment = payment;
                }
            }
        }
        Ok(())
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
            payer_amount_sat <= self.get_info().await?.wallet_info.balance_sat,
            PaymentError::InsufficientFunds
        );

        let description = match bolt11_invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };

        match self.swapper.check_for_mrh(invoice).await? {
            // If we find a valid MRH, extract the BIP21 address and pay to it via onchain tx
            Some((address, _)) => {
                info!("Found MRH for L-BTC address {address}, invoice amount_sat {amount_sat}");
                self.pay_liquid(
                    LiquidAddressData {
                        address,
                        network: self.config.network.into(),
                        asset_id: None,
                        amount: None,
                        amount_sat: None,
                        label: None,
                        message: None,
                    },
                    amount_sat,
                    fees_sat,
                    false,
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
            payer_amount_sat <= self.get_info().await?.wallet_info.balance_sat,
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
        skip_already_paid_check: bool,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let destination = address_data
            .to_uri()
            .unwrap_or(address_data.address.clone());
        let asset_id = address_data.asset_id.unwrap_or(self.config.lbtc_asset_id());
        let payments = self.persister.get_payments(&ListPaymentsRequest {
            details: Some(ListPaymentDetails::Liquid {
                asset_id: Some(asset_id.clone()),
                destination: Some(destination.clone()),
            }),
            ..Default::default()
        })?;
        ensure_sdk!(
            skip_already_paid_check || payments.is_empty(),
            PaymentError::AlreadyPaid
        );

        let tx = self
            .onchain_wallet
            .build_tx_or_drain_tx(
                Some(LIQUID_FEE_RATE_MSAT_PER_VBYTE),
                &address_data.address,
                &asset_id,
                receiver_amount_sat,
            )
            .await?;
        let tx_id = tx.txid().to_string();
        let tx_fees_sat = tx.all_fees().values().sum::<u64>();
        ensure_sdk!(tx_fees_sat <= fees_sat, PaymentError::InvalidOrExpiredFees);

        info!(
            "Built onchain L-BTC tx with receiver_amount_sat = {receiver_amount_sat}, fees_sat = {fees_sat} and txid = {tx_id}"
        );

        let tx_id = self.liquid_chain_service.broadcast(&tx).await?.to_string();

        // We insert a pseudo-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        let tx_data = PaymentTxData {
            tx_id: tx_id.clone(),
            timestamp: Some(utils::now()),
            amount: receiver_amount_sat,
            fees_sat,
            payment_type: PaymentType::Send,
            is_confirmed: false,
            unblinding_data: None,
            asset_id: asset_id.clone(),
        };

        let description = address_data.message;

        self.persister.insert_or_update_payment(
            tx_data.clone(),
            Some(PaymentTxDetails {
                tx_id: tx_id.clone(),
                destination: destination.clone(),
                description: description.clone(),
                ..Default::default()
            }),
            false,
        )?;
        self.emit_payment_updated(Some(tx_id)).await?; // Emit Pending event

        let asset_info = self
            .persister
            .get_asset_metadata(&asset_id)?
            .map(|ref am| AssetInfo {
                name: am.name.clone(),
                ticker: am.ticker.clone(),
                amount: am.amount_from_sat(receiver_amount_sat),
            });
        let payment_details = PaymentDetails::Liquid {
            asset_id,
            destination,
            description: description.unwrap_or("Liquid transfer".to_string()),
            asset_info,
            lnurl_info: None,
            bip353_address: None,
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
        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat).await?;
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
                    self.send_swap_handler.update_swap_info(
                        &swap.id,
                        PaymentState::Created,
                        None,
                        None,
                        None,
                    )?;
                    swap
                }
                Pending => return Err(PaymentError::PaymentInProgress),
                Complete => return Err(PaymentError::AlreadyPaid),
                RefundPending | Refundable | Failed => {
                    return Err(PaymentError::invalid_invoice(
                        "Payment has already failed. Please try with another invoice",
                    ))
                }
                WaitingFeeAcceptance => {
                    return Err(PaymentError::Generic {
                        err: "Send swap payment cannot be in state WaitingFeeAcceptance"
                            .to_string(),
                    })
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
                let create_response = self
                    .swapper
                    .create_send_swap(CreateSubmarineRequest {
                        from: "L-BTC".to_string(),
                        to: "BTC".to_string(),
                        invoice: invoice.to_string(),
                        refund_public_key,
                        pair_hash: Some(lbtc_pair.hash.clone()),
                        referral_id: None,
                        webhook,
                    })
                    .await?;

                let swap_id = &create_response.id;
                let create_response_json =
                    SendSwap::from_boltz_struct_to_json(&create_response, swap_id)?;
                let destination_pubkey =
                    utils::get_invoice_destination_pubkey(invoice, bolt12_offer.is_some())?;

                let payer_amount_sat = fees_sat + receiver_amount_sat;
                let swap = SendSwap {
                    id: swap_id.to_string(),
                    invoice: invoice.to_string(),
                    bolt12_offer,
                    payment_hash: Some(payment_hash.to_string()),
                    destination_pubkey: Some(destination_pubkey),
                    timeout_block_height: create_response.timeout_block_height,
                    description,
                    preimage: None,
                    payer_amount_sat,
                    receiver_amount_sat,
                    pair_fees_json: serde_json::to_string(&lbtc_pair).map_err(|e| {
                        PaymentError::generic(&format!("Failed to serialize SubmarinePair: {e:?}"))
                    })?,
                    create_response_json,
                    lockup_tx_id: None,
                    refund_tx_id: None,
                    created_at: utils::now(),
                    state: PaymentState::Created,
                    refund_private_key: keypair.display_secret().to_string(),
                    metadata: Default::default(),
                };
                self.persister.insert_or_update_send_swap(&swap)?;
                swap
            }
        };
        self.status_stream.track_swap_id(&swap.id)?;

        let create_response = swap.get_boltz_create_response()?;
        self.send_swap_handler
            .try_lockup(&swap, &create_response)
            .await?;

        self.wait_for_payment_with_timeout(Swap::Send(swap), create_response.accept_zero_conf)
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
            .get_submarine_pairs()
            .await?
            .ok_or(PaymentError::PairsNotFound)?;
        let send_limits = submarine_pair.limits;

        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()
            .await?
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

        let (pair_outgoing, pair_incoming) = self.swapper.get_chain_pairs().await?;
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
    ///        and [PayAmount::Bitcoin], which sets the amount the receiver should receive
    ///     * `fee_rate_sat_per_vbyte` - the optional fee rate of the Bitcoin claim transaction. Defaults to the swapper estimated claim fee
    pub async fn prepare_pay_onchain(
        &self,
        req: &PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let get_info_res = self.get_info().await?;
        let pair = self.get_chain_pair(Direction::Outgoing).await?;
        let claim_fees_sat = match req.fee_rate_sat_per_vbyte {
            Some(sat_per_vbyte) => ESTIMATED_BTC_CLAIM_TX_VSIZE * sat_per_vbyte as u64,
            None => pair.clone().fees.claim_estimate(),
        };
        let server_fees_sat = pair.fees.server();

        info!("Preparing for onchain payment of kind: {:?}", req.amount);
        let (payer_amount_sat, receiver_amount_sat, total_fees_sat) = match req.amount {
            PayAmount::Bitcoin {
                receiver_amount_sat: amount_sat,
            } => {
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
                    get_info_res.wallet_info.pending_receive_sat == 0
                        && get_info_res.wallet_info.pending_send_sat == 0,
                    PaymentError::Generic {
                        err: "Cannot drain while there are pending payments".to_string(),
                    }
                );
                let payer_amount_sat = get_info_res.wallet_info.balance_sat;
                let lockup_fees_sat = self.estimate_drain_tx_fee(None, None).await?;

                let user_lockup_amount_sat = payer_amount_sat - lockup_fees_sat;
                self.validate_user_lockup_amount_for_chain_pair(&pair, user_lockup_amount_sat)?;

                let boltz_fees_sat = pair.fees.boltz(user_lockup_amount_sat);
                let total_fees_sat =
                    boltz_fees_sat + lockup_fees_sat + claim_fees_sat + server_fees_sat;
                let receiver_amount_sat = payer_amount_sat - total_fees_sat;

                (payer_amount_sat, receiver_amount_sat, total_fees_sat)
            }
            PayAmount::Asset { .. } => {
                return Err(PaymentError::asset_error(
                    "Cannot send an asset to a Bitcoin address",
                ))
            }
        };

        let res = PreparePayOnchainResponse {
            receiver_amount_sat,
            claim_fees_sat,
            total_fees_sat,
        };

        ensure_sdk!(
            payer_amount_sat <= get_info_res.wallet_info.balance_sat,
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
        let balance_sat = self.get_info().await?.wallet_info.balance_sat;
        let receiver_amount_sat = req.prepare_response.receiver_amount_sat;
        let pair = self.get_chain_pair(Direction::Outgoing).await?;
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
        let create_response = self
            .swapper
            .create_chain_swap(CreateChainRequest {
                from: "L-BTC".to_string(),
                to: "BTC".to_string(),
                preimage_hash: preimage.sha256,
                claim_public_key: Some(claim_public_key),
                refund_public_key: Some(refund_public_key),
                user_lock_amount: None,
                server_lock_amount: Some(server_lockup_amount_sat),
                pair_hash: Some(pair.hash.clone()),
                referral_id: None,
                webhook,
            })
            .await?;

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
            actual_payer_amount_sat: None,
            receiver_amount_sat,
            accepted_receiver_amount_sat: None,
            claim_fees_sat,
            pair_fees_json: serde_json::to_string(&pair).map_err(|e| {
                PaymentError::generic(&format!("Failed to serialize outgoing ChainPair: {e:?}"))
            })?,
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
            auto_accepted_fees: false,
            metadata: Default::default(),
        };
        self.persister.insert_or_update_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap_id)?;

        self.wait_for_payment_with_timeout(Swap::Chain(swap), accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    async fn wait_for_payment_with_timeout(
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
                        let update_res = match swap {
                            Swap::Send(_) => self.send_swap_handler.update_swap_info(&expected_swap_id, TimedOut, None, None, None),
                            Swap::Chain(_) => self.chain_swap_handler.update_swap_info(&ChainSwapUpdate {
                                    swap_id: expected_swap_id.clone(),
                                    to_state: TimedOut,
                                    ..Default::default()
                                }),
                            _ => Ok(())
                        };
                        return match update_res {
                            Ok(_) => Err(PaymentError::PaymentTimeout),
                            Err(_) => {
                                // Not able to transition the payment state to TimedOut, which means the payment
                                // state progressed but we didn't see the event before the timeout
                                self.persister.get_payment(&expected_swap_id).ok().flatten().ok_or(PaymentError::generic("Payment not found"))
                            }
                        }
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
                    Ok(event) => debug!("Unhandled event waiting for payment: {event:?}"),
                    Err(e) => debug!("Received error waiting for payment: {e:?}"),
                }
            }
        }
    }

    /// Prepares to receive a Lightning payment via a reverse submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareReceiveRequest] containing:
    ///     * `payment_method` - the supported payment methods; either an invoice, a Liquid address or a Bitcoin address
    ///     * `amount` - The optional amount of type [ReceiveAmount] to be paid.
    ///        - [ReceiveAmount::Bitcoin] which sets the amount in satoshi that should be paid
    ///        - [ReceiveAmount::Asset] which sets the amount of an asset that should be paid
    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.ensure_is_started().await?;

        let mut min_payer_amount_sat = None;
        let mut max_payer_amount_sat = None;
        let mut swapper_feerate = None;
        let fees_sat;
        match req.payment_method {
            PaymentMethod::Lightning => {
                let payer_amount_sat = match req.amount {
                    Some(ReceiveAmount::Asset { .. }) => {
                        return Err(PaymentError::asset_error(
                            "Cannot receive an asset when the payment method is Lightning",
                        ));
                    }
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => payer_amount_sat,
                    None => {
                        return Err(PaymentError::generic(
                            "Bitcoin payer amount must be set when the payment method is Lightning",
                        ));
                    }
                };
                let reverse_pair = self
                    .swapper
                    .get_reverse_swap_pairs()
                    .await?
                    .ok_or(PaymentError::PairsNotFound)?;

                fees_sat = reverse_pair.fees.total(payer_amount_sat);

                ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

                reverse_pair
                    .limits
                    .within(payer_amount_sat)
                    .map_err(|_| PaymentError::AmountOutOfRange)?;

                min_payer_amount_sat = Some(reverse_pair.limits.minimal);
                max_payer_amount_sat = Some(reverse_pair.limits.maximal);
                swapper_feerate = Some(reverse_pair.fees.percentage);

                debug!(
                    "Preparing Lightning Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat"
                );
            }
            PaymentMethod::BitcoinAddress => {
                let payer_amount_sat = match req.amount {
                    Some(ReceiveAmount::Asset { .. }) => {
                        return Err(PaymentError::asset_error(
                            "Cannot receive an asset when the payment method is Bitcoin",
                        ));
                    }
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => Some(payer_amount_sat),
                    None => None,
                };
                let pair = self
                    .get_and_validate_chain_pair(Direction::Incoming, payer_amount_sat)
                    .await?;
                let claim_fees_sat = pair.fees.claim_estimate();
                let server_fees_sat = pair.fees.server();
                let service_fees_sat = payer_amount_sat
                    .map(|user_lockup_amount_sat| pair.fees.boltz(user_lockup_amount_sat))
                    .unwrap_or_default();

                min_payer_amount_sat = Some(pair.limits.minimal);
                max_payer_amount_sat = Some(pair.limits.maximal);
                swapper_feerate = Some(pair.fees.percentage);

                fees_sat = service_fees_sat + claim_fees_sat + server_fees_sat;
                debug!("Preparing Chain Receive Swap with: payer_amount_sat {payer_amount_sat:?}, fees_sat {fees_sat}");
            }
            PaymentMethod::LiquidAddress => {
                let (asset_id, payer_amount, payer_amount_sat) = match req.amount.clone() {
                    Some(ReceiveAmount::Asset {
                        payer_amount,
                        asset_id,
                    }) => (asset_id, payer_amount, None),
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => {
                        (self.config.lbtc_asset_id(), None, Some(payer_amount_sat))
                    }
                    None => (self.config.lbtc_asset_id(), None, None),
                };
                fees_sat = 0;
                debug!("Preparing Liquid Receive with: asset_id {asset_id}, amount {payer_amount:?}, amount_sat {payer_amount_sat:?}, fees_sat {fees_sat}");
            }
        };

        Ok(PrepareReceiveResponse {
            amount: req.amount.clone(),
            fees_sat,
            payment_method: req.payment_method.clone(),
            min_payer_amount_sat,
            max_payer_amount_sat,
            swapper_feerate,
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
            amount,
            fees_sat,
            ..
        } = &req.prepare_response;

        match payment_method {
            PaymentMethod::Lightning => {
                let amount_sat = match amount.clone() {
                    Some(ReceiveAmount::Asset { .. }) => {
                        return Err(PaymentError::asset_error(
                            "Cannot receive an asset when the payment method is Lightning",
                        ));
                    }
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => payer_amount_sat,
                    None => {
                        return Err(PaymentError::generic(
                            "Bitcoin payer amount must be set when the payment method is Lightning",
                        ));
                    }
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
                self.create_receive_swap(amount_sat, *fees_sat, description, description_hash)
                    .await
            }
            PaymentMethod::BitcoinAddress => {
                let amount_sat = match amount.clone() {
                    Some(ReceiveAmount::Asset { .. }) => {
                        return Err(PaymentError::asset_error(
                            "Cannot receive an asset when the payment method is Bitcoin",
                        ));
                    }
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => Some(payer_amount_sat),
                    None => None,
                };
                self.receive_onchain(amount_sat, *fees_sat).await
            }
            PaymentMethod::LiquidAddress => {
                let lbtc_asset_id = self.config.lbtc_asset_id();
                let (asset_id, amount, amount_sat) = match amount.clone() {
                    Some(ReceiveAmount::Asset {
                        asset_id,
                        payer_amount,
                    }) => (asset_id, payer_amount, None),
                    Some(ReceiveAmount::Bitcoin { payer_amount_sat }) => {
                        (lbtc_asset_id.clone(), None, Some(payer_amount_sat))
                    }
                    None => (lbtc_asset_id.clone(), None, None),
                };

                let address = self.onchain_wallet.next_unused_address().await?.to_string();
                let receive_destination =
                    if asset_id.ne(&lbtc_asset_id) || amount.is_some() || amount_sat.is_some() {
                        LiquidAddressData {
                            address: address.to_string(),
                            network: self.config.network.into(),
                            amount,
                            amount_sat,
                            asset_id: Some(asset_id),
                            label: None,
                            message: req.description.clone(),
                        }
                        .to_uri()
                        .map_err(|e| PaymentError::Generic {
                            err: format!("Could not build BIP21 URI: {e:?}"),
                        })?
                    } else {
                        address
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
            .get_reverse_swap_pairs()
            .await?
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
        let mrh_addr_hash_sig =
            keypair.sign_schnorr(Message::from_digest_slice(mrh_addr_hash.as_byte_array())?);

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
        let create_response = self.swapper.create_receive_swap(v2_req).await?;

        // Reserve this address until the timeout block height
        self.persister.insert_or_update_reserved_address(
            &mrh_addr_str,
            create_response.timeout_block_height,
        )?;

        // Check if correct MRH was added to the invoice by Boltz
        let (bip21_lbtc_address, _bip21_amount_btc) = self
            .swapper
            .check_for_mrh(&create_response.invoice)
            .await?
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
        let destination_pubkey = invoice_pubkey(&invoice);

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
            .insert_or_update_receive_swap(&ReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                create_response_json,
                claim_private_key: keypair.display_secret().to_string(),
                invoice: invoice.to_string(),
                payment_hash: Some(preimage_hash),
                destination_pubkey: Some(destination_pubkey),
                timeout_block_height: create_response.timeout_block_height,
                description: invoice_description,
                payer_amount_sat,
                receiver_amount_sat,
                pair_fees_json: serde_json::to_string(&reverse_pair).map_err(|e| {
                    PaymentError::generic(&format!("Failed to serialize ReversePair: {e:?}"))
                })?,
                claim_fees_sat: reverse_pair.fees.claim_estimate(),
                lockup_tx_id: None,
                claim_tx_id: None,
                mrh_address: mrh_addr_str,
                mrh_tx_id: None,
                created_at: utils::now(),
                state: PaymentState::Created,
                metadata: Default::default(),
            })
            .map_err(|_| PaymentError::PersistError)?;
        self.status_stream.track_swap_id(&swap_id)?;

        Ok(ReceivePaymentResponse {
            destination: invoice.to_string(),
        })
    }

    async fn create_receive_chain_swap(
        &self,
        user_lockup_amount_sat: Option<u64>,
        fees_sat: u64,
    ) -> Result<ChainSwap, PaymentError> {
        let pair = self
            .get_and_validate_chain_pair(Direction::Incoming, user_lockup_amount_sat)
            .await?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();
        // Service fees are 0 if this is a zero-amount swap
        let service_fees_sat = user_lockup_amount_sat
            .map(|user_lockup_amount_sat| pair.fees.boltz(user_lockup_amount_sat))
            .unwrap_or_default();

        ensure_sdk!(
            fees_sat == service_fees_sat + claim_fees_sat + server_fees_sat,
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
        let create_response = self
            .swapper
            .create_chain_swap(CreateChainRequest {
                from: "BTC".to_string(),
                to: "L-BTC".to_string(),
                preimage_hash: preimage.sha256,
                claim_public_key: Some(claim_public_key),
                refund_public_key: Some(refund_public_key),
                user_lock_amount: user_lockup_amount_sat,
                server_lock_amount: None,
                pair_hash: Some(pair.hash.clone()),
                referral_id: None,
                webhook,
            })
            .await?;

        let swap_id = create_response.id.clone();
        let create_response_json =
            ChainSwap::from_boltz_struct_to_json(&create_response, &swap_id)?;

        let accept_zero_conf = user_lockup_amount_sat
            .map(|user_lockup_amount_sat| user_lockup_amount_sat <= pair.limits.maximal_zero_conf)
            .unwrap_or(false);
        let receiver_amount_sat = user_lockup_amount_sat
            .map(|user_lockup_amount_sat| user_lockup_amount_sat - fees_sat)
            .unwrap_or(0);

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Incoming,
            claim_address: None,
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            description: Some("Bitcoin transfer".to_string()),
            payer_amount_sat: user_lockup_amount_sat.unwrap_or(0),
            actual_payer_amount_sat: None,
            receiver_amount_sat,
            accepted_receiver_amount_sat: None,
            claim_fees_sat,
            pair_fees_json: serde_json::to_string(&pair).map_err(|e| {
                PaymentError::generic(&format!("Failed to serialize incoming ChainPair: {e:?}"))
            })?,
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
            auto_accepted_fees: false,
            metadata: Default::default(),
        };
        self.persister.insert_or_update_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap.id)?;
        Ok(swap)
    }

    /// Receive from a Bitcoin transaction via a chain swap.
    ///
    /// If no `user_lockup_amount_sat` is specified, this is an amountless swap and `fees_sat` exclude
    /// the service fees.
    async fn receive_onchain(
        &self,
        user_lockup_amount_sat: Option<u64>,
        fees_sat: u64,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let swap = self
            .create_receive_chain_swap(user_lockup_amount_sat, fees_sat)
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
        let scripts_utxos = self
            .bitcoin_chain_service
            .get_scripts_utxos(&lockup_scripts)?;

        let mut refundables = vec![];
        for (chain_swap, script_utxos) in chain_swaps.into_iter().zip(scripts_utxos) {
            let swap_id = &chain_swap.id;
            let amount_sat = script_utxos
                .iter()
                .filter_map(|utxo| utxo.as_bitcoin().cloned())
                .map(|(_, txo)| txo.value.to_sat())
                .sum();
            info!("Incoming Chain Swap {swap_id} is refundable with {amount_sat} sats");

            let refundable: RefundableSwap = chain_swap.to_refundable(amount_sat);
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
        let refund_address = self
            .validate_bitcoin_address(&req.refund_address)
            .await
            .map_err(|e| SdkError::Generic {
                err: format!("Failed to validate refund address: {e}"),
            })?;

        let (tx_vsize, tx_fee_sat, refund_tx_id) = self
            .chain_swap_handler
            .prepare_refund(
                &req.swap_address,
                &refund_address,
                req.fee_rate_sat_per_vbyte,
            )
            .await?;
        Ok(PrepareRefundResponse {
            tx_vsize,
            tx_fee_sat,
            last_refund_tx_id: refund_tx_id,
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
        let refund_address = self
            .validate_bitcoin_address(&req.refund_address)
            .await
            .map_err(|e| SdkError::Generic {
                err: format!("Failed to validate refund address: {e}"),
            })?;

        let refund_tx_id = self
            .chain_swap_handler
            .refund_incoming_swap(
                &req.swap_address,
                &refund_address,
                req.fee_rate_sat_per_vbyte,
                true,
            )
            .or_else(|e| {
                warn!("Failed to initiate cooperative refund, switching to non-cooperative: {e:?}");
                self.chain_swap_handler.refund_incoming_swap(
                    &req.swap_address,
                    &refund_address,
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
        let t0 = Instant::now();
        let mut rescannable_swaps: Vec<Swap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .map(Into::into)
            .collect();
        self.recoverer
            .recover_from_onchain(&mut rescannable_swaps)
            .await?;
        let scanned_len = rescannable_swaps.len();
        for swap in rescannable_swaps {
            let swap_id = &swap.id();
            if let Swap::Chain(chain_swap) = swap {
                if let Err(e) = self.chain_swap_handler.update_swap(chain_swap) {
                    error!("Error persisting rescanned Chain Swap {swap_id}: {e}");
                }
            }
        }
        info!(
            "Rescanned {} chain swaps in {} seconds",
            scanned_len,
            t0.elapsed().as_millis()
        );
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
                amount: Some(ReceiveAmount::Bitcoin {
                    payer_amount_sat: req.amount_sat,
                }),
            })
            .await?;

        let Some(ReceiveAmount::Bitcoin {
            payer_amount_sat: amount_sat,
        }) = res.amount
        else {
            return Err(PaymentError::Generic {
                err: format!(
                    "Error preparing receive payment, got amount: {:?}",
                    res.amount
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
                Some(req.prepare_response.amount_sat),
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

    pub(crate) async fn get_monitored_swaps_list(&self, partial_sync: bool) -> Result<Vec<Swap>> {
        let receive_swaps = self
            .persister
            .list_recoverable_receive_swaps()?
            .into_iter()
            .map(Into::into)
            .collect();
        match partial_sync {
            false => {
                let bitcoin_height = self.bitcoin_chain_service.tip()?.height as u32;
                let liquid_height = self.liquid_chain_service.tip().await?;
                let final_swap_states = [PaymentState::Complete, PaymentState::Failed];

                let send_swaps = self
                    .persister
                    .list_recoverable_send_swaps()?
                    .into_iter()
                    .map(Into::into)
                    .collect();
                let chain_swaps: Vec<Swap> = self
                    .persister
                    .list_chain_swaps()?
                    .into_iter()
                    .filter(|swap| match swap.direction {
                        Direction::Incoming => {
                            bitcoin_height
                                <= swap.timeout_block_height
                                    + CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS
                        }
                        Direction::Outgoing => {
                            !final_swap_states.contains(&swap.state)
                                && liquid_height <= swap.timeout_block_height
                        }
                    })
                    .map(Into::into)
                    .collect();
                Ok([receive_swaps, send_swaps, chain_swaps].concat())
            }
            true => Ok(receive_swaps),
        }
    }

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, partial_sync: bool) -> Result<()> {
        let mut recoverable_swaps = self.get_monitored_swaps_list(partial_sync).await?;
        let mut wallet_tx_map = self
            .recoverer
            .recover_from_onchain(&mut recoverable_swaps)
            .await?;

        let all_wallet_tx_ids: HashSet<String> =
            wallet_tx_map.keys().map(|txid| txid.to_string()).collect();

        for swap in recoverable_swaps {
            let swap_id = &swap.id();

            // Update the payment wallet txs before updating the swap so the tx data is pulled into the payment
            match swap {
                Swap::Receive(receive_swap) => {
                    let history_updates = vec![&receive_swap.claim_tx_id, &receive_swap.mrh_tx_id];
                    for tx_id in history_updates
                        .into_iter()
                        .flatten()
                        .collect::<Vec<&String>>()
                    {
                        if let Some(tx) =
                            wallet_tx_map.remove(&lwk_wollet::elements::Txid::from_str(tx_id)?)
                        {
                            self.persister
                                .insert_or_update_payment_with_wallet_tx(&tx)?;
                        }
                    }
                    if let Err(e) = self.receive_swap_handler.update_swap(receive_swap) {
                        error!("Error persisting recovered receive swap {swap_id}: {e}");
                    }
                }
                Swap::Send(send_swap) => {
                    let history_updates = vec![&send_swap.lockup_tx_id, &send_swap.refund_tx_id];
                    for tx_id in history_updates
                        .into_iter()
                        .flatten()
                        .collect::<Vec<&String>>()
                    {
                        if let Some(tx) =
                            wallet_tx_map.remove(&lwk_wollet::elements::Txid::from_str(tx_id)?)
                        {
                            self.persister
                                .insert_or_update_payment_with_wallet_tx(&tx)?;
                        }
                    }
                    if let Err(e) = self.send_swap_handler.update_swap(send_swap) {
                        error!("Error persisting recovered send swap {swap_id}: {e}");
                    }
                }
                Swap::Chain(chain_swap) => {
                    let history_updates = match chain_swap.direction {
                        Direction::Incoming => vec![&chain_swap.claim_tx_id],
                        Direction::Outgoing => {
                            vec![&chain_swap.user_lockup_tx_id, &chain_swap.refund_tx_id]
                        }
                    };
                    for tx_id in history_updates
                        .into_iter()
                        .flatten()
                        .collect::<Vec<&String>>()
                    {
                        if let Some(tx) =
                            wallet_tx_map.remove(&lwk_wollet::elements::Txid::from_str(tx_id)?)
                        {
                            self.persister
                                .insert_or_update_payment_with_wallet_tx(&tx)?;
                        }
                    }
                    if let Err(e) = self.chain_swap_handler.update_swap(chain_swap) {
                        error!("Error persisting recovered Chain Swap {swap_id}: {e}");
                    }
                }
            };
        }

        let non_swap_wallet_tx_map = wallet_tx_map;

        let payments = self
            .persister
            .get_payments_by_tx_id(&ListPaymentsRequest::default())?;

        // We query only these that may need update, should be a fast query.
        let unconfirmed_payment_txs_data = self.persister.list_unconfirmed_payment_txs_data()?;
        let unconfirmed_txs_by_id: HashMap<String, PaymentTxData> = unconfirmed_payment_txs_data
            .into_iter()
            .map(|tx| (tx.tx_id.clone(), tx))
            .collect::<HashMap<String, PaymentTxData>>();

        for tx in non_swap_wallet_tx_map.values() {
            let tx_id = tx.txid.to_string();
            let maybe_payment = payments.get(&tx_id);
            let mut updated = false;
            match maybe_payment {
                // When no payment is found or its a Liquid payment
                None
                | Some(Payment {
                    details: PaymentDetails::Liquid { .. },
                    ..
                }) => {
                    let updated_needed = maybe_payment
                        .is_none_or(|payment| payment.status == Pending && tx.height.is_some());
                    if updated_needed {
                        // An unknown tx which needs inserting or a known Liquid payment tx
                        // that was in the mempool, but is now confirmed
                        self.persister.insert_or_update_payment_with_wallet_tx(tx)?;
                        self.emit_payment_updated(Some(tx_id.clone())).await?;
                        updated = true
                    }
                }

                _ => {}
            }
            if !updated && unconfirmed_txs_by_id.contains_key(&tx_id) && tx.height.is_some() {
                // An unconfirmed tx that was not found in the payments table
                self.persister.insert_or_update_payment_with_wallet_tx(tx)?;
            }
        }

        let unknown_unconfirmed_txs: Vec<_> = unconfirmed_txs_by_id
            .iter()
            .filter(|(txid, _)| !all_wallet_tx_ids.contains(*txid))
            .map(|(_, tx)| tx)
            .collect();

        for unknown_unconfirmed_tx in unknown_unconfirmed_txs {
            if unknown_unconfirmed_tx.timestamp.is_some_and(|t| {
                (utils::now().saturating_sub(t)) > NETWORK_PROPAGATION_GRACE_PERIOD.as_secs() as u32
            }) {
                self.persister
                    .delete_payment_tx_data(&unknown_unconfirmed_tx.tx_id)?;
                info!(
                    "Found an unknown unconfirmed tx and deleted it. Txid: {}",
                    unknown_unconfirmed_tx.tx_id
                );
            } else {
                debug!(
                    "Found an unknown unconfirmed tx that was inserted at {:?}. \
                Keeping it to allow propagation through the network. Txid: {}",
                    unknown_unconfirmed_tx.timestamp, unknown_unconfirmed_tx.tx_id
                )
            }
        }

        self.update_wallet_info().await?;
        Ok(())
    }

    async fn update_wallet_info(&self) -> Result<()> {
        let asset_metadata: HashMap<String, AssetMetadata> = self
            .persister
            .list_asset_metadata()?
            .into_iter()
            .map(|am| (am.asset_id.clone(), am))
            .collect();
        let transactions = self.onchain_wallet.transactions().await?;
        let tx_ids = transactions
            .iter()
            .map(|tx| tx.txid.to_string())
            .collect::<Vec<_>>();
        let asset_balances = transactions
            .into_iter()
            .fold(BTreeMap::<AssetId, i64>::new(), |mut acc, tx| {
                tx.balance.into_iter().for_each(|(asset_id, balance)| {
                    // Consider only confirmed unspent outputs (confirmed transactions output reduced by unconfirmed spent outputs)
                    if tx.height.is_some() || balance < 0 {
                        *acc.entry(asset_id).or_default() += balance;
                    }
                });
                acc
            })
            .into_iter()
            .map(|(asset_id, balance)| {
                let asset_id = asset_id.to_hex();
                let balance_sat = balance.unsigned_abs();
                let maybe_asset_metadata = asset_metadata.get(&asset_id);
                AssetBalance {
                    asset_id,
                    balance_sat,
                    name: maybe_asset_metadata.map(|am| am.name.clone()),
                    ticker: maybe_asset_metadata.map(|am| am.ticker.clone()),
                    balance: maybe_asset_metadata.map(|am| am.amount_from_sat(balance_sat)),
                }
            })
            .collect::<Vec<AssetBalance>>();
        let mut balance_sat = asset_balances
            .clone()
            .into_iter()
            .find(|ab| ab.asset_id.eq(&self.config.lbtc_asset_id()))
            .map_or(0, |ab| ab.balance_sat);

        let mut pending_send_sat = 0;
        let mut pending_receive_sat = 0;
        let payments = self.persister.get_payments(&ListPaymentsRequest {
            states: Some(vec![
                PaymentState::Pending,
                PaymentState::RefundPending,
                PaymentState::WaitingFeeAcceptance,
            ]),
            ..Default::default()
        })?;

        for payment in payments {
            let is_lbtc_asset_id = payment.details.is_lbtc_asset_id(self.config.network);
            match payment.payment_type {
                PaymentType::Send => match payment.details.get_refund_tx_amount_sat() {
                    Some(refund_tx_amount_sat) => pending_receive_sat += refund_tx_amount_sat,
                    None => {
                        let total_sat = if is_lbtc_asset_id {
                            payment.amount_sat + payment.fees_sat
                        } else {
                            payment.fees_sat
                        };
                        if let Some(tx_id) = payment.tx_id {
                            if !tx_ids.contains(&tx_id) {
                                debug!("Deducting {total_sat} sats from balance");
                                balance_sat = balance_sat.saturating_sub(total_sat);
                            }
                        }
                        pending_send_sat += total_sat
                    }
                },
                PaymentType::Receive => {
                    if is_lbtc_asset_id {
                        pending_receive_sat += payment.amount_sat;
                    }
                }
            }
        }

        debug!("Onchain wallet balance: {balance_sat} sats");
        let info_response = WalletInfo {
            balance_sat,
            pending_send_sat,
            pending_receive_sat,
            fingerprint: self.onchain_wallet.fingerprint()?,
            pubkey: self.onchain_wallet.pubkey()?,
            asset_balances,
        };
        self.persister.set_wallet_info(&info_response)
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

    /// Fetches an up-to-date fees proposal for a [Payment] that is [WaitingFeeAcceptance].
    ///
    /// Use [LiquidSdk::accept_payment_proposed_fees] to accept the proposed fees and proceed
    /// with the payment.
    pub async fn fetch_payment_proposed_fees(
        &self,
        req: &FetchPaymentProposedFeesRequest,
    ) -> SdkResult<FetchPaymentProposedFeesResponse> {
        let chain_swap =
            self.persister
                .fetch_chain_swap_by_id(&req.swap_id)?
                .ok_or(SdkError::Generic {
                    err: format!("Could not find Swap {}", req.swap_id),
                })?;

        ensure_sdk!(
            chain_swap.state == WaitingFeeAcceptance,
            SdkError::Generic {
                err: "Payment is not WaitingFeeAcceptance".to_string()
            }
        );

        let server_lockup_quote = self
            .swapper
            .get_zero_amount_chain_swap_quote(&req.swap_id)
            .await?;

        let actual_payer_amount_sat =
            chain_swap
                .actual_payer_amount_sat
                .ok_or(SdkError::Generic {
                    err: "No actual payer amount found when state is WaitingFeeAcceptance"
                        .to_string(),
                })?;
        let fees_sat =
            actual_payer_amount_sat - server_lockup_quote.to_sat() + chain_swap.claim_fees_sat;

        Ok(FetchPaymentProposedFeesResponse {
            swap_id: req.swap_id.clone(),
            fees_sat,
            payer_amount_sat: actual_payer_amount_sat,
            receiver_amount_sat: actual_payer_amount_sat - fees_sat,
        })
    }

    /// Accepts proposed fees for a [Payment] that is [WaitingFeeAcceptance].
    ///
    /// Use [LiquidSdk::fetch_payment_proposed_fees] to get an up-to-date fees proposal.
    pub async fn accept_payment_proposed_fees(
        &self,
        req: &AcceptPaymentProposedFeesRequest,
    ) -> Result<(), PaymentError> {
        let FetchPaymentProposedFeesResponse {
            swap_id,
            fees_sat,
            payer_amount_sat,
            ..
        } = req.clone().response;

        let chain_swap =
            self.persister
                .fetch_chain_swap_by_id(&swap_id)?
                .ok_or(SdkError::Generic {
                    err: format!("Could not find Swap {}", swap_id),
                })?;

        ensure_sdk!(
            chain_swap.state == WaitingFeeAcceptance,
            PaymentError::Generic {
                err: "Payment is not WaitingFeeAcceptance".to_string()
            }
        );

        let server_lockup_quote = self
            .swapper
            .get_zero_amount_chain_swap_quote(&swap_id)
            .await?;

        ensure_sdk!(
            fees_sat == payer_amount_sat - server_lockup_quote.to_sat() + chain_swap.claim_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        self.persister
            .update_accepted_receiver_amount(&swap_id, Some(payer_amount_sat - fees_sat))?;
        self.swapper
            .accept_zero_amount_chain_swap_quote(&swap_id, server_lockup_quote.to_sat())
            .inspect_err(|e| {
                error!("Failed to accept zero-amount swap {swap_id} quote: {e} - trying to erase the accepted receiver amount...");
                let _ = self
                    .persister
                    .update_accepted_receiver_amount(&swap_id, None);
            }).await?;
        self.chain_swap_handler.update_swap_info(&ChainSwapUpdate {
            swap_id,
            to_state: Pending,
            ..Default::default()
        })
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
    pub async fn sync(&self, partial_sync: bool) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();

        if let Err(err) = self.onchain_wallet.full_scan().await {
            error!("Failed to scan wallet: {err:?}");
        }

        let is_first_sync = !self
            .persister
            .get_is_first_sync_complete()?
            .unwrap_or(false);
        match is_first_sync {
            true => {
                self.event_manager.pause_notifications();
                self.sync_payments_with_chain_data(partial_sync).await?;
                self.event_manager.resume_notifications();
                self.persister.set_is_first_sync_complete(true)?;
            }
            false => {
                self.sync_payments_with_chain_data(partial_sync).await?;
            }
        }
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized (partial: {partial_sync}) with mempool and onchain data ({duration_ms} ms)");

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
    /// This is the second step of LNURL-pay flow. The first step is [LiquidSdk::parse], which also validates the LNURL
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
    ///     * `data` - the [LnUrlPayRequestData] returned by [LiquidSdk::parse]
    ///     * `amount` - The optional amount of type [PayAmount].
    ///        - [PayAmount::Drain] which uses all funds
    ///        - [PayAmount::Receiver] which sets the amount the receiver should receive
    ///     * `bip353_address` - A BIP353 address, in case one was used in order to fetch the LNURL
    ///       Pay request data. Returned by [parse].
    ///     * `comment` - an optional comment for this payment
    ///     * `validate_success_action_url` - validates that, if there is a URL success action, the URL domain matches
    ///       the LNURL callback domain. Defaults to 'true'
    ///
    /// # Returns
    /// Returns a [PrepareLnUrlPayResponse] containing:
    ///     * `destination` - the destination of the payment
    ///     * `fees_sat` - The fees in satoshis to send the payment
    ///     * `data` - The [LnUrlPayRequestData] returned by [parse]
    ///     * `comment` - An optional comment for this payment
    ///     * `success_action` - the optional unprocessed LUD-09 success action
    pub async fn prepare_lnurl_pay(
        &self,
        req: PrepareLnUrlPayRequest,
    ) -> Result<PrepareLnUrlPayResponse, LnUrlPayError> {
        let amount_msat = match req.amount {
            PayAmount::Drain => {
                let get_info_res = self
                    .get_info()
                    .await
                    .map_err(|e| LnUrlPayError::Generic { err: e.to_string() })?;
                ensure_sdk!(
                    get_info_res.wallet_info.pending_receive_sat == 0
                        && get_info_res.wallet_info.pending_send_sat == 0,
                    LnUrlPayError::Generic {
                        err: "Cannot drain while there are pending payments".to_string(),
                    }
                );
                let lbtc_pair = self
                    .swapper
                    .get_submarine_pairs()
                    .await?
                    .ok_or(PaymentError::PairsNotFound)?;
                let drain_fees_sat = self.estimate_drain_tx_fee(None, None).await?;
                let drain_amount_sat = get_info_res.wallet_info.balance_sat - drain_fees_sat;
                // Get the inverse invoice amount by calculating a dummy amount then increment up to the drain amount
                let dummy_fees_sat = lbtc_pair.fees.total(drain_amount_sat);
                let dummy_amount_sat = drain_amount_sat - dummy_fees_sat;
                let invoice_amount_sat = utils::increment_invoice_amount_up_to_drain_amount(
                    dummy_amount_sat,
                    &lbtc_pair,
                    drain_amount_sat,
                );
                lbtc_pair
                    .limits
                    .within(invoice_amount_sat)
                    .map_err(|e| LnUrlPayError::Generic { err: e.message() })?;
                // Validate if we can actually drain the wallet with a swap
                let pair_fees_sat = lbtc_pair.fees.total(invoice_amount_sat);
                ensure_sdk!(
                    invoice_amount_sat + pair_fees_sat == drain_amount_sat,
                    LnUrlPayError::Generic {
                        err: "Cannot drain without leaving a remainder".to_string(),
                    }
                );

                invoice_amount_sat * 1000
            }
            PayAmount::Bitcoin {
                receiver_amount_sat,
            } => receiver_amount_sat * 1000,
            PayAmount::Asset { .. } => {
                return Err(LnUrlPayError::Generic {
                    err: "Cannot send an asset to a Bitcoin address".to_string(),
                })
            }
        };

        match validate_lnurl_pay(
            self.rest_client.as_ref(),
            amount_msat,
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
                        amount: Some(req.amount),
                    })
                    .await
                    .map_err(|e| LnUrlPayError::Generic { err: e.to_string() })?;

                let destination = match prepare_response.destination {
                    SendDestination::Bolt11 { invoice, .. } => SendDestination::Bolt11 {
                        invoice,
                        bip353_address: req.bip353_address,
                    },
                    SendDestination::LiquidAddress { address_data, .. } => {
                        SendDestination::LiquidAddress {
                            address_data,
                            bip353_address: req.bip353_address,
                        }
                    }
                    destination => destination,
                };

                Ok(PrepareLnUrlPayResponse {
                    destination,
                    fees_sat: prepare_response.fees_sat,
                    data: req.data,
                    comment: req.comment,
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
        let mut payment = self
            .send_payment(&SendPaymentRequest {
                prepare_response: PrepareSendResponse {
                    destination: prepare_response.destination.clone(),
                    fees_sat: prepare_response.fees_sat,
                },
            })
            .await
            .map_err(|e| LnUrlPayError::Generic { err: e.to_string() })?
            .payment;

        let maybe_sa_processed: Option<SuccessActionProcessed> = match prepare_response
            .success_action
            .clone()
        {
            Some(sa) => {
                match sa {
                    // For AES, we decrypt the contents if the preimage is available
                    SuccessAction::Aes { data } => {
                        let PaymentDetails::Lightning {
                            swap_id, preimage, ..
                        } = &payment.details
                        else {
                            return Err(LnUrlPayError::Generic {
                                err: format!("Invalid payment type: expected type `PaymentDetails::Lightning`, got payment details {:?}.", payment.details),
                            });
                        };

                        match preimage {
                            Some(preimage_str) => {
                                debug!(
                                    "Decrypting AES success action with preimage for Send Swap {}",
                                    swap_id
                                );
                                let preimage =
                                    sha256::Hash::from_str(preimage_str).map_err(|_| {
                                        LnUrlPayError::Generic {
                                            err: "Invalid preimage".to_string(),
                                        }
                                    })?;
                                let preimage_arr = preimage.to_byte_array();
                                let result = match (data, &preimage_arr).try_into() {
                                    Ok(data) => AesSuccessActionDataResult::Decrypted { data },
                                    Err(e) => AesSuccessActionDataResult::ErrorStatus {
                                        reason: e.to_string(),
                                    },
                                };
                                Some(SuccessActionProcessed::Aes { result })
                            }
                            None => {
                                debug!("Preimage not yet available to decrypt AES success action for Send Swap {}", swap_id);
                                None
                            }
                        }
                    }
                    SuccessAction::Message { data } => {
                        Some(SuccessActionProcessed::Message { data })
                    }
                    SuccessAction::Url { data } => Some(SuccessActionProcessed::Url { data }),
                }
            }
            None => None,
        };

        let description = payment
            .details
            .get_description()
            .or_else(|| extract_description_from_metadata(&prepare_response.data));

        let lnurl_pay_domain = match prepare_response.data.ln_address {
            Some(_) => None,
            None => Some(prepare_response.data.domain),
        };
        if let (Some(tx_id), Some(destination)) =
            (payment.tx_id.clone(), payment.destination.clone())
        {
            self.persister
                .insert_or_update_payment_details(PaymentTxDetails {
                    tx_id: tx_id.clone(),
                    destination,
                    description,
                    lnurl_info: Some(LnUrlInfo {
                        ln_address: prepare_response.data.ln_address,
                        lnurl_pay_comment: prepare_response.comment,
                        lnurl_pay_domain,
                        lnurl_pay_metadata: Some(prepare_response.data.metadata_str),
                        lnurl_pay_success_action: maybe_sa_processed.clone(),
                        lnurl_pay_unprocessed_success_action: prepare_response.success_action,
                        lnurl_withdraw_endpoint: None,
                    }),
                    bip353_address: None,
                })?;
            // Get the payment with the lnurl_info details
            payment = self.persister.get_payment(&tx_id)?.unwrap_or(payment);
        }

        Ok(LnUrlPayResult::EndpointSuccess {
            data: model::LnUrlPaySuccessData {
                payment,
                success_action: maybe_sa_processed,
            },
        })
    }

    /// Second step of LNURL-withdraw. The first step is [LiquidSdk::parse], which also validates the LNURL destination
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
                    amount: Some(ReceiveAmount::Bitcoin {
                        payer_amount_sat: req.amount_msat / 1_000,
                    }),
                }
            })
            .await?;
        let receive_res = self
            .receive_payment(&ReceivePaymentRequest {
                prepare_response,
                description: req.description.clone(),
                use_description_hash: Some(false),
            })
            .await?;

        let Ok(invoice) = parse_invoice(&receive_res.destination) else {
            return Err(LnUrlWithdrawError::Generic {
                err: "Received unexpected output from receive request".to_string(),
            });
        };

        let res =
            validate_lnurl_withdraw(self.rest_client.as_ref(), req.data.clone(), invoice.clone())
                .await?;
        if let LnUrlWithdrawResult::Ok { data: _ } = res {
            if let Some(ReceiveSwap {
                claim_tx_id: Some(tx_id),
                ..
            }) = self
                .persister
                .fetch_receive_swap_by_invoice(&invoice.bolt11)?
            {
                self.persister
                    .insert_or_update_payment_details(PaymentTxDetails {
                        tx_id,
                        destination: receive_res.destination,
                        description: req.description,
                        lnurl_info: Some(LnUrlInfo {
                            lnurl_withdraw_endpoint: Some(req.data.callback),
                            ..Default::default()
                        }),
                        bip353_address: None,
                    })?;
            }
        }
        Ok(res)
    }

    /// Third and last step of LNURL-auth. The first step is [LiquidSdk::parse], which also validates the LNURL destination
    /// and generates the [LnUrlAuthRequestData] payload needed here. The second step is user approval of auth action.
    ///
    /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
    /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        Ok(perform_lnurl_auth(
            self.rest_client.as_ref(),
            &req_data,
            &SdkLnurlAuthSigner::new(self.signer.clone()),
        )
        .await?)
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
        Ok(self.bitcoin_chain_service.recommended_fees().await?)
    }

    /// Get the full default [Config] for specific [LiquidNetwork].
    pub fn default_config(
        network: LiquidNetwork,
        breez_api_key: Option<String>,
    ) -> Result<Config, SdkError> {
        let config = match network {
            LiquidNetwork::Mainnet => Config::mainnet(breez_api_key),
            LiquidNetwork::Testnet => Config::testnet(breez_api_key),
            LiquidNetwork::Regtest => Config::regtest(),
        };
        Ok(config)
    }

    /// Parses a string into an [InputType]. See [input_parser::parse].
    ///
    /// Can optionally be configured to use external input parsers by providing `external_input_parsers` in [Config].
    pub async fn parse(&self, input: &str) -> Result<InputType, PaymentError> {
        let external_parsers = &self.external_input_parsers;
        let input_type =
            parse_with_rest_client(self.rest_client.as_ref(), input, Some(external_parsers))
                .await
                .map_err(|e| PaymentError::generic(&e.to_string()))?;

        let res = match input_type {
            InputType::LiquidAddress { ref address } => match &address.asset_id {
                Some(asset_id) if asset_id.ne(&self.config.lbtc_asset_id()) => {
                    let asset_metadata = self.persister.get_asset_metadata(asset_id)?.ok_or(
                        PaymentError::AssetError {
                            err: format!("Asset {asset_id} is not supported"),
                        },
                    )?;
                    let mut address = address.clone();
                    address.set_amount_precision(asset_metadata.precision.into());
                    InputType::LiquidAddress { address }
                }
                _ => input_type,
            },
            _ => input_type,
        };
        Ok(res)
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

/// Extracts `description` from `metadata_str`
fn extract_description_from_metadata(request_data: &LnUrlPayRequestData) -> Option<String> {
    let metadata = request_data.metadata_vec().ok()?;
    metadata
        .iter()
        .find(|item| item.key == "text/plain")
        .map(|item| {
            info!("Extracted payment description: '{}'", item.value);
            item.value.clone()
        })
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use anyhow::{anyhow, Result};
    use boltz_client::{
        boltz::{self, TransactionInfo},
        swaps::boltz::{ChainSwapStates, RevSwapStates, SubSwapStates},
    };
    use lwk_wollet::{elements::Txid, hashes::hex::DisplayHex};

    use crate::chain_swap::ESTIMATED_BTC_LOCKUP_TX_VSIZE;
    use crate::test_utils::chain_swap::{
        TEST_BITCOIN_OUTGOING_SERVER_LOCKUP_TX, TEST_LIQUID_INCOMING_SERVER_LOCKUP_TX,
        TEST_LIQUID_OUTGOING_USER_LOCKUP_TX,
    };
    use crate::test_utils::swapper::ZeroAmountSwapMockConfig;
    use crate::test_utils::wallet::TEST_LIQUID_RECEIVE_LOCKUP_TX;
    use crate::{
        model::{Direction, PaymentState, Swap},
        sdk::LiquidSdk,
        test_utils::{
            chain::{MockBitcoinChainService, MockHistory, MockLiquidChainService},
            chain_swap::{new_chain_swap, TEST_BITCOIN_INCOMING_USER_LOCKUP_TX},
            persist::{create_persister, new_receive_swap, new_send_swap},
            sdk::{new_liquid_sdk, new_liquid_sdk_with_chain_services},
            status_stream::MockStatusStream,
            swapper::MockSwapper,
        },
    };
    use paste::paste;

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct NewSwapArgs {
        direction: Direction,
        accepts_zero_conf: bool,
        initial_payment_state: Option<PaymentState>,
        receiver_amount_sat: Option<u64>,
        user_lockup_tx_id: Option<String>,
        zero_amount: bool,
        set_actual_payer_amount: bool,
    }

    impl Default for NewSwapArgs {
        fn default() -> Self {
            Self {
                accepts_zero_conf: false,
                initial_payment_state: None,
                direction: Direction::Outgoing,
                receiver_amount_sat: None,
                user_lockup_tx_id: None,
                zero_amount: false,
                set_actual_payer_amount: false,
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

        pub fn set_receiver_amount_sat(mut self, receiver_amount_sat: Option<u64>) -> Self {
            self.receiver_amount_sat = receiver_amount_sat;
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

        pub fn set_zero_amount(mut self, zero_amount: bool) -> Self {
            self.zero_amount = zero_amount;
            self
        }

        pub fn set_set_actual_payer_amount(mut self, set_actual_payer_amount: bool) -> Self {
            self.set_actual_payer_amount = set_actual_payer_amount;
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
                        $args.zero_amount,
                        $args.set_actual_payer_amount,
                        $args.receiver_amount_sat,
                    );
                    $persister.insert_or_update_chain_swap(&swap).unwrap();
                    Swap::Chain(swap)
                }
                "send" => {
                    let swap =
                        new_send_swap($args.initial_payment_state, $args.receiver_amount_sat);
                    $persister.insert_or_update_send_swap(&swap).unwrap();
                    Swap::Send(swap)
                }
                "receive" => {
                    let swap =
                        new_receive_swap($args.initial_payment_state, $args.receiver_amount_sat);
                    $persister.insert_or_update_receive_swap(&swap).unwrap();
                    Swap::Receive(swap)
                }
                _ => panic!(),
            };

            $status_stream
                .clone()
                .send_mock_update(boltz::SwapStatus {
                    id: swap.id(),
                    status: $status.to_string(),
                    transaction: $transaction,
                    zero_conf_rejected: $zero_conf_rejected,
                    ..Default::default()
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

    #[sdk_macros::async_test_all]
    async fn test_receive_swap_update_tracking() -> Result<()> {
        create_persister!(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(MockLiquidChainService::new());
        let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());

        let sdk = new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
            None,
        )?;

        LiquidSdk::track_swap_updates(&sdk);

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
                let mock_tx = TEST_LIQUID_RECEIVE_LOCKUP_TX.clone();
                let mock_tx_id = mock_tx.txid();
                let height = (serde_json::to_string(&status).unwrap()
                    == serde_json::to_string(&RevSwapStates::TransactionConfirmed).unwrap())
                    as i32;
                liquid_chain_service.set_history(vec![MockHistory {
                    txid: mock_tx_id,
                    height,
                    block_hash: None,
                    block_timestamp: None,
                }]);

                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    Some(TransactionInfo {
                        id: mock_tx_id.to_string(),
                        hex: Some(
                            lwk_wollet::elements::encode::serialize(&mock_tx).to_lower_hex_string()
                        ),
                        eta: None,
                    }),
                    None
                );
                assert!(persisted_swap.claim_tx_id.is_some());
            }

            // Check that `TransactionMempool` and `TransactionConfirmed` checks the lockup amount
            // and doesn't claim if not verified
            for status in [
                RevSwapStates::TransactionMempool,
                RevSwapStates::TransactionConfirmed,
            ] {
                let mock_tx = TEST_LIQUID_RECEIVE_LOCKUP_TX.clone();
                let mock_tx_id = mock_tx.txid();
                let height = (serde_json::to_string(&status).unwrap()
                    == serde_json::to_string(&RevSwapStates::TransactionConfirmed).unwrap())
                    as i32;
                liquid_chain_service.set_history(vec![MockHistory {
                    txid: mock_tx_id,
                    height,
                    block_hash: None,
                    block_timestamp: None,
                }]);

                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default().set_receiver_amount_sat(Some(1000)),
                    persister,
                    status_stream,
                    status,
                    Some(TransactionInfo {
                        id: mock_tx_id.to_string(),
                        hex: Some(
                            lwk_wollet::elements::encode::serialize(&mock_tx).to_lower_hex_string()
                        ),
                        eta: None
                    }),
                    None
                );
                assert!(persisted_swap.claim_tx_id.is_none());
            }
        })
        .await
        .unwrap();

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_send_swap_update_tracking() -> Result<()> {
        create_persister!(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());

        let sdk = Arc::new(new_liquid_sdk(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk);

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

    #[sdk_macros::async_test_all]
    async fn test_chain_swap_update_tracking() -> Result<()> {
        create_persister!(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(MockLiquidChainService::new());
        let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());

        let sdk = new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
            None,
        )?;

        LiquidSdk::track_swap_updates(&sdk);

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

                let (mock_user_lockup_tx_hex, mock_user_lockup_tx_id) = match direction {
                    Direction::Outgoing => {
                        let tx = TEST_LIQUID_OUTGOING_USER_LOCKUP_TX.clone();
                        (
                            lwk_wollet::elements::encode::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                    Direction::Incoming => {
                        let tx = TEST_BITCOIN_INCOMING_USER_LOCKUP_TX.clone();
                        (
                            sdk_common::bitcoin::consensus::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                };

                let (mock_server_lockup_tx_hex, mock_server_lockup_tx_id) = match direction {
                    Direction::Incoming => {
                        let tx = TEST_LIQUID_INCOMING_SERVER_LOCKUP_TX.clone();
                        (
                            lwk_wollet::elements::encode::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                    Direction::Outgoing => {
                        let tx = TEST_BITCOIN_OUTGOING_SERVER_LOCKUP_TX.clone();
                        (
                            sdk_common::bitcoin::consensus::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                };

                // Verify that `TransactionLockupFailed` correctly sets the state as
                // `RefundPending`/`Refundable` or as `Failed` depending on whether or not
                // `user_lockup_tx_id` is present
                for user_lockup_tx_id in &[None, Some(mock_user_lockup_tx_id.clone())] {
                    if let Some(user_lockup_tx_id) = user_lockup_tx_id {
                        match direction {
                            Direction::Incoming => {
                                bitcoin_chain_service.set_history(vec![MockHistory {
                                    txid: Txid::from_str(user_lockup_tx_id).unwrap(),
                                    height: 0,
                                    block_hash: None,
                                    block_timestamp: None,
                                }]);
                            }
                            Direction::Outgoing => {
                                liquid_chain_service.set_history(vec![MockHistory {
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
                    if direction == Direction::Incoming {
                        bitcoin_chain_service.set_history(vec![MockHistory {
                            txid: Txid::from_str(&mock_user_lockup_tx_id).unwrap(),
                            height: 0,
                            block_hash: None,
                            block_timestamp: None,
                        }]);
                        bitcoin_chain_service.set_transactions(&[&mock_user_lockup_tx_hex]);
                    }
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default().set_direction(direction),
                        persister,
                        status_stream,
                        status,
                        Some(TransactionInfo {
                            id: mock_user_lockup_tx_id.clone(),
                            hex: Some(mock_user_lockup_tx_hex.clone()),
                            eta: None
                        }), // sets `update.transaction`
                        Some(true) // sets `update.zero_conf_rejected`
                    );
                    assert_eq!(
                        persisted_swap.user_lockup_tx_id,
                        Some(mock_user_lockup_tx_id.clone())
                    );
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
                            .set_accepts_zero_conf(accepts_zero_conf)
                            .set_set_actual_payer_amount(true),
                        persister,
                        status_stream,
                        ChainSwapStates::TransactionServerMempool,
                        Some(TransactionInfo {
                            id: mock_server_lockup_tx_id.clone(),
                            hex: Some(mock_server_lockup_tx_hex.clone()),
                            eta: None,
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
                    NewSwapArgs::default()
                        .set_direction(direction)
                        .set_set_actual_payer_amount(true),
                    persister,
                    status_stream,
                    ChainSwapStates::TransactionServerConfirmed,
                    Some(TransactionInfo {
                        id: mock_server_lockup_tx_id,
                        hex: Some(mock_server_lockup_tx_hex),
                        eta: None,
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

    #[sdk_macros::async_test_all]
    async fn test_zero_amount_chain_swap_zero_leeway() -> Result<()> {
        let user_lockup_sat = 50_000;

        create_persister!(persister);
        let swapper = Arc::new(MockSwapper::new());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(MockLiquidChainService::new());
        let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());

        let sdk = new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
            None,
        )?;

        LiquidSdk::track_swap_updates(&sdk);

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify that `TransactionLockupFailed` correctly:
            // 1. does not affect state when swapper doesn't increase fees
            // 2. triggers a change to WaitingFeeAcceptance when there is a fee increase > 0
            for fee_increase in [0, 1] {
                swapper.set_zero_amount_swap_mock_config(ZeroAmountSwapMockConfig {
                    user_lockup_sat,
                    onchain_fee_increase_sat: fee_increase,
                });
                bitcoin_chain_service.set_script_balance_sat(user_lockup_sat);
                let persisted_swap = trigger_swap_update!(
                    "chain",
                    NewSwapArgs::default()
                        .set_direction(Direction::Incoming)
                        .set_accepts_zero_conf(false)
                        .set_zero_amount(true),
                    persister,
                    status_stream,
                    ChainSwapStates::TransactionLockupFailed,
                    None,
                    None
                );
                match fee_increase {
                    0 => {
                        assert_eq!(persisted_swap.state, PaymentState::Created);
                    }
                    1 => {
                        assert_eq!(persisted_swap.state, PaymentState::WaitingFeeAcceptance);
                    }
                    _ => panic!("Unexpected fee_increase"),
                }
            }
        })
        .await?;

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_zero_amount_chain_swap_with_leeway() -> Result<()> {
        let user_lockup_sat = 50_000;
        let onchain_fee_rate_leeway_sat_per_vbyte = 5;

        create_persister!(persister);
        let swapper = Arc::new(MockSwapper::new());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(MockLiquidChainService::new());
        let bitcoin_chain_service = Arc::new(MockBitcoinChainService::new());

        let sdk = new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
            Some(onchain_fee_rate_leeway_sat_per_vbyte),
        )?;

        LiquidSdk::track_swap_updates(&sdk);

        let max_fee_increase_for_auto_accept_sat =
            onchain_fee_rate_leeway_sat_per_vbyte as u64 * ESTIMATED_BTC_LOCKUP_TX_VSIZE;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify that `TransactionLockupFailed` correctly:
            // 1. does not affect state when swapper increases fee by up to sat/vbyte leeway * tx size
            // 2. triggers a change to WaitingFeeAcceptance when it is any higher
            for fee_increase in [
                max_fee_increase_for_auto_accept_sat,
                max_fee_increase_for_auto_accept_sat + 1,
            ] {
                swapper.set_zero_amount_swap_mock_config(ZeroAmountSwapMockConfig {
                    user_lockup_sat,
                    onchain_fee_increase_sat: fee_increase,
                });
                bitcoin_chain_service.set_script_balance_sat(user_lockup_sat);
                let persisted_swap = trigger_swap_update!(
                    "chain",
                    NewSwapArgs::default()
                        .set_direction(Direction::Incoming)
                        .set_accepts_zero_conf(false)
                        .set_zero_amount(true),
                    persister,
                    status_stream,
                    ChainSwapStates::TransactionLockupFailed,
                    None,
                    None
                );
                match fee_increase {
                    val if val == max_fee_increase_for_auto_accept_sat => {
                        assert_eq!(persisted_swap.state, PaymentState::Created);
                    }
                    val if val == (max_fee_increase_for_auto_accept_sat + 1) => {
                        assert_eq!(persisted_swap.state, PaymentState::WaitingFeeAcceptance);
                    }
                    _ => panic!("Unexpected fee_increase"),
                }
            }
        })
        .await?;

        Ok(())
    }
}
