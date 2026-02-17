use std::{
    collections::{HashMap, HashSet},
    str::FromStr as _,
    sync::Arc,
    time::Duration,
};

use crate::{
    context::RuntimeContext,
    encrypt::EncryptionHandler,
    error::{NwcError, NwcResult},
    event::{EventManager, NwcEvent, NwcEventDetails, NwcEventListener},
    handler::{RelayMessageHandler, SdkRelayMessageHandler},
    model::{
        AddConnectionRequest, AddConnectionResponse, EditConnectionRequest, EditConnectionResponse,
        NostrServiceInfo, NwcConfig, NwcConnection, NwcConnectionInner, PeriodicBudgetInner,
    },
    persist::Persister,
};
use anyhow::{bail, Result};
use breez_sdk_liquid::{
    model::{ListPaymentsRequest, Payment, PaymentDetails, PaymentType},
    plugin::{Plugin, PluginSdk, PluginStorage},
    InputType,
};
use log::{debug, error, info, warn};
use nostr_sdk::{
    nips::nip47::{NostrWalletConnectURI, Request, RequestParams, Response, ResponseResult},
    Client as NostrClient, Event, EventBuilder, EventId, Filter, Keys, Kind, RelayMessage,
    RelayPoolNotification, RelayUrl, Tag, Timestamp,
};
use tokio::{
    sync::{mpsc, Mutex, OnceCell},
    time::Interval,
};
use tokio_with_wasm::alias as tokio;

pub(crate) mod context;
mod encrypt;
pub mod error;
pub mod event;
pub(crate) mod handler;
pub mod model;
mod persist;
pub(crate) mod sdk_event;
pub(crate) mod utils;

pub const MIN_REFRESH_INTERVAL_SEC: u64 = 60; // 1 minute
pub const DEFAULT_PERIODIC_BUDGET_TIME_SEC: u32 = 60 * 60 * 24 * 30; // 30 days
pub const DEFAULT_EVENT_HANDLING_INTERVAL_SEC: u64 = 10;
pub const DEFAULT_RELAY_URLS: [&str; 4] = [
    "wss://relay.getalbypro.com/breez",
    "wss://nos.lol/",
    "wss://nostr.land/",
    "wss://nostr.wine/",
];

#[sdk_macros::async_trait]
pub trait NwcService: Send + Sync {
    /// Creates a Nostr Wallet Connect connection for this service.
    ///
    /// Generates a unique connection URI that external applications can use
    /// to connect to this wallet service. The URI includes the wallet's public key,
    /// relay information, and a randomly generated secret for secure communication.
    ///
    /// # Arguments
    /// * `req` - The [add connection request](AddConnectionRequest), including:
    ///     * `name` - the **unique** identifier of the connection
    ///     * `expiry_time_min` - the expiry time of the connection string. If None, it will **not**
    ///     expire
    ///     * `periodic_budget_req` - the periodic budget paremeters of the connection if any.
    ///     You can specify the [maximum amount \(in satoshi\) per period](crate::model::PeriodicBudgetRequest::max_budget_sat)
    ///     and the [period renewal time \(in minutes\)](crate::model::PeriodicBudgetRequest::renewal_time_mins)
    ///
    /// # Returns
    /// * `res` - The [AddConnectionResponse], including:
    ///     * `connection` - the generated NWC connection
    async fn add_connection(&self, req: AddConnectionRequest) -> NwcResult<AddConnectionResponse>;

    /// Modifies a Nostr Wallet Connect connection for this service.
    ///
    /// # Arguments
    /// * `req` - The [edit connection request](EditConnectionRequest), including:
    ///     * `name` - the already existing identifier of the connection
    ///     * `expiry_time_min` - the expiry time of the connection string. If None, it will **not**
    ///     expire
    ///     * `periodic_budget_req` - the periodic budget paremeters of the connection if any.
    ///     You can specify the [maximum amount \(in satoshi\) per period](crate::model::PeriodicBudgetRequest::max_budget_sat)
    ///     and the [period renewal time \(in minutes\)](crate::model::PeriodicBudgetRequest::renewal_time_mins)
    ///
    /// # Returns
    /// * `res` - The [EditConnectionResponse], including:
    ///     * `connection` - the modified NWC connection
    async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> NwcResult<EditConnectionResponse>;

    /// Lists the active Nostr Wallet Connect connections for this service.
    async fn list_connections(&self) -> NwcResult<HashMap<String, NwcConnection>>;

    /// Removes a Nostr Wallet Connect connection string
    ///
    /// Removes a previously set connection string. Returns error if unset.
    ///
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn remove_connection(&self, name: String) -> NwcResult<()>;

    /// Lists the payments associated to a specific connection
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn list_connection_payments(&self, name: String) -> NwcResult<Vec<Payment>>;

    /// Fetches and handles a Nostr WalletRequest event
    ///
    /// # Arguments
    /// * `id` - the ID of the Nostr event
    async fn handle_event(&self, event_id: String) -> NwcResult<()>;

    /// Adds an event listener to the service, where all [NwcEvent]s will be emitted to.
    /// The event listener can be removed be calling [NwcService::remove_event_listener].
    ///
    /// # Arguments
    ///
    /// * `listener` - The listener which is an implementation of the [NwcEventListener] trait
    async fn add_event_listener(&self, listener: Box<dyn NwcEventListener>) -> String;

    /// Removes an event listener from the service
    ///
    /// # Arguments
    ///
    /// * `id` - the event listener id returned by [NwcService::add_event_listener]
    async fn remove_event_listener(&self, id: &str);

    /// Returns runtime information about the Nostr service
    async fn get_info(&self) -> Option<NostrServiceInfo>;

    /// Tracks an incoming zap until the payment is complete, and broadcasts the associated
    /// zap_receipt
    ///
    /// # Arguments
    ///
    /// * `invoice` - The invoice related to the zap request
    /// * `zap_request` - the URL- and JSON-encoded zap request
    async fn track_zap(&self, invoice: String, zap_request: String) -> NwcResult<()>;
}

pub struct SdkNwcService {
    config: NwcConfig,
    event_manager: Arc<EventManager>,
    runtime_ctx: Mutex<Option<Arc<RuntimeContext>>>,
}

impl SdkNwcService {
    /// Creates a new SdkNwcService instance.
    ///
    /// Initializes the service with the provided cryptographic keys
    /// and connects to the specified Nostr relays.
    ///
    /// # Arguments
    /// * `config` - Configuration containing the relay URLs and secret key
    ///
    /// # Returns
    /// * `Arc<SdkNwcService>` - Successfully initialized service
    /// * `Err(anyhow::Error)` - Error adding relays or initializing
    pub fn new(config: NwcConfig) -> Self {
        Self {
            config,
            runtime_ctx: Default::default(),
            event_manager: Arc::new(EventManager::new()),
        }
    }

    async fn new_ctx(
        &self,
        sdk: PluginSdk,
        storage: PluginStorage,
        resub_tx: mpsc::Sender<()>,
    ) -> Result<RuntimeContext> {
        let persister = Persister::new(storage);
        let client = NostrClient::default();
        for relay in self.config.relays() {
            if let Err(err) = client.add_relay(&relay).await {
                warn!("Could not add nwc relay {relay}: {err:?}");
            }
        }
        let our_keys = match self.get_or_create_keypair(&persister).await {
            Ok(keypair) => keypair,
            Err(err) => {
                bail!("Could not fetch/create Nostr secret key: {err:?}");
            }
        };
        let handler: Box<dyn RelayMessageHandler> =
            Box::new(SdkRelayMessageHandler::new(sdk.clone()));
        let ctx = RuntimeContext {
            sdk,
            client,
            our_keys,
            persister,
            handler,
            resubscription_trigger: resub_tx,
            event_loop_handle: OnceCell::new(),
            sdk_listener_id: Mutex::new(None),
            event_manager: self.event_manager.clone(),
            replied_event_ids: Mutex::new(HashSet::new()),
        };
        Ok(ctx)
    }

    async fn runtime_ctx(&self) -> Result<Arc<RuntimeContext>> {
        for _ in 0..3 {
            match *self.runtime_ctx.lock().await {
                Some(ref ctx) => return Ok(ctx.clone()),
                None => tokio::time::sleep(Duration::from_millis(500)).await,
            };
        }
        bail!("NWC service is not running.")
    }

    async fn get_or_create_keypair(&self, persister: &Persister) -> Result<Keys> {
        let get_secret_key = async || -> Result<String> {
            // If we have a key from the configuration, use it
            if let Some(key) = &self.config.secret_key_hex {
                return Ok(key.clone());
            }

            // Otherwise, try restoring it from the previous session
            if let Ok(Some(key)) = persister.get_nwc_seckey() {
                return Ok(key);
            }

            // If none exists, generate a new one
            let key = nostr_sdk::key::SecretKey::generate().to_secret_hex();
            persister.set_nwc_seckey(key.clone())?;
            Ok(key)
        };
        let secret_key = get_secret_key().await?;
        Ok(Keys::parse(&secret_key)?)
    }

    async fn handle_event_inner(ctx: &RuntimeContext, event: &Event) -> NwcResult<()> {
        let event_id = event.id.to_string();
        let client_pubkey = event.pubkey;

        let (connection_name, mut client) = ctx
            .list_active_connections()
            .await?
            .into_iter()
            .find(|(_, con)| con.pubkey == client_pubkey)
            .ok_or(NwcError::PubkeyNotFound {
                pubkey: client_pubkey.to_string(),
            })?;
        if ctx.check_replied_event(event_id.clone()).await {
            info!("Event {event_id} has already been replied to. Skipping.");
            return Ok(());
        }

        // Verify the event has not expired
        if event
            .tags
            .expiration()
            .is_some_and(|t| *t < Timestamp::now())
        {
            return Err(NwcError::EventExpired);
        }

        // Verify the event signature and event id
        event.verify().map_err(|err| NwcError::InvalidSignature {
            err: err.to_string(),
        })?;

        // Decrypt the event content
        let encryption_handler = EncryptionHandler::new(ctx.our_keys.secret_key(), &client_pubkey);
        let decrypted_content = encryption_handler.decrypt(event)?;
        info!("Decrypted NWC notification");

        // Build response
        let req = serde_json::from_str::<Request>(&decrypted_content)?;
        let mut compute_result = async || -> NwcResult<ResponseResult> {
            if client.connection.receive_only
                && !matches!(req.params, RequestParams::MakeInvoice(_))
            {
                return Err(NwcError::generic("Connection is receive-only."));
            }

            match &req.params {
                RequestParams::PayInvoice(req) => {
                    let Ok(InputType::Bolt11 { invoice }) =
                        sdk_common::input_parser::parse(&req.invoice, None).await
                    else {
                        return Err(NwcError::generic(format!(
                            "Could not parse pay_invoice invoice: {}",
                            req.invoice
                        )));
                    };
                    let Some(req_amount_sat) = req
                        .amount
                        .or(invoice.amount_msat)
                        .map(|amount| amount.div_ceil(1000))
                    else {
                        return Err(NwcError::InvoiceWithoutAmount);
                    };

                    if let Some(ref mut periodic_budget) = client.connection.periodic_budget {
                        if periodic_budget.used_budget_sat + req_amount_sat
                            > periodic_budget.max_budget_sat
                        {
                            return Err(NwcError::MaxBudgetExceeded);
                        }
                        // We modify the connection's budget before executing the payment to avoid any race
                        // conditions
                        if let Err(err) = ctx
                            .persister
                            .update_budget(&connection_name, req_amount_sat as i64)
                        {
                            return Err(NwcError::generic(format!(
                            "Cannot pay invoice: could not update periodic budget on connection \"{connection_name}\": {err}"
                        )));
                        }
                    }
                    match ctx.handler.pay_invoice(req).await {
                        Ok(res) => {
                            ctx.persister
                                .add_paid_invoice(&connection_name, invoice.bolt11)
                                .map_err(|err| {
                                    NwcError::persist(format!(
                                        "Could not persist paid invoice: {err}"
                                    ))
                                })?;
                            Ok(ResponseResult::PayInvoice(res))
                        }
                        Err(e) => {
                            // In case of payment failure, we want to undo the periodic budget changes
                            if client.connection.periodic_budget.is_some() {
                                if let Err(err) = ctx
                                    .persister
                                    .update_budget(&connection_name, -(req_amount_sat as i64))
                                {
                                    return Err(NwcError::generic(format!(
                                    "Cannot pay invoice: could not update periodic budget on connection \"{connection_name}\": {err}."
                                )));
                                }
                            }
                            Err(e)
                        }
                    }
                }
                RequestParams::MakeInvoice(req) => ctx
                    .handler
                    .make_invoice(req)
                    .await
                    .map(ResponseResult::MakeInvoice),
                RequestParams::ListTransactions(req) => ctx
                    .handler
                    .list_transactions(req)
                    .await
                    .map(ResponseResult::ListTransactions),
                RequestParams::GetBalance => ctx
                    .handler
                    .get_balance()
                    .await
                    .map(ResponseResult::GetBalance),
                RequestParams::GetInfo => ctx.handler.get_info().await.map(ResponseResult::GetInfo),
                _ => Err(NwcError::generic(format!(
                    "Received unhandled request: {req:?}"
                ))),
            }
        };
        let res = compute_result().await;
        debug!("Got result {res:?} for event {event_id}");

        // Notify SDK
        Self::handle_local_notification(ctx, connection_name.to_string(), &res, &event_id).await;

        // Serialize and encrypt the response
        let content = serde_json::to_string(&Response {
            result_type: req.method,
            result: res.as_ref().ok().cloned(),
            error: res.as_ref().err().cloned().map(Into::into),
        })
        .map_err(|err| NwcError::generic(format!("Could not serialize Nostr response: {err:?}")))?;

        let encrypted_content = encryption_handler.encrypt(event, &content)?;
        info!("Encrypted NWC response");
        let event_builder = EventBuilder::new(Kind::WalletConnectResponse, encrypted_content)
            .tags([Tag::event(event.id), Tag::public_key(client_pubkey)]);

        // Broadcast the response
        ctx.send_event(event_builder)
            .await
            .map_err(|err| NwcError::Network {
                err: err.to_string(),
            })?;
        info!("Sent encrypted NWC response");

        Ok(())
    }

    async fn handle_local_notification(
        ctx: &RuntimeContext,
        connection_name: String,
        result: &NwcResult<ResponseResult>,
        event_id: &str,
    ) {
        debug!("Handling notification: {result:?}");
        let event = match result {
            Ok(ResponseResult::PayInvoice(response)) => NwcEvent {
                details: NwcEventDetails::PayInvoice {
                    success: true,
                    preimage: Some(response.preimage.clone()),
                    fees_sat: response.fees_paid.map(|f| f / 1000),
                    error: None,
                },
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            Ok(ResponseResult::MakeInvoice(_)) => NwcEvent {
                details: NwcEventDetails::MakeInvoice,
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            Ok(ResponseResult::ListTransactions(_)) => NwcEvent {
                details: NwcEventDetails::ListTransactions,
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            Ok(ResponseResult::GetBalance(_)) => NwcEvent {
                details: NwcEventDetails::GetBalance,
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            Ok(ResponseResult::GetInfo(_)) => NwcEvent {
                details: NwcEventDetails::GetInfo,
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            Err(
                err @ NwcError::InvoiceExpired
                | err @ NwcError::InvoiceWithoutAmount
                | err @ NwcError::MaxBudgetExceeded,
            ) => NwcEvent {
                details: NwcEventDetails::PayInvoice {
                    success: false,
                    preimage: None,
                    fees_sat: None,
                    error: Some(err.to_string()),
                },
                connection_name: Some(connection_name),
                event_id: Some(event_id.to_string()),
            },
            _ => {
                return;
            }
        };
        info!("Sending event: {event:?}");
        ctx.event_manager.notify(event).await;
    }

    async fn new_maybe_interval(ctx: &RuntimeContext) -> Option<Interval> {
        ctx.persister
            .get_min_interval()
            .map(|interval| tokio::time::interval(Duration::from_secs(interval)))
    }

    async fn min_refresh_interval(maybe_interval: &mut Option<Interval>) -> Option<()> {
        match maybe_interval {
            Some(interval) => {
                interval.tick().await;
                Some(())
            }
            None => None,
        }
    }
}

#[sdk_macros::async_trait]
impl NwcService for SdkNwcService {
    async fn add_connection(&self, req: AddConnectionRequest) -> NwcResult<AddConnectionResponse> {
        let ctx = self.runtime_ctx().await?;
        let random_secret_key = nostr_sdk::SecretKey::generate();
        let relays = self
            .config
            .relays()
            .into_iter()
            .filter_map(|r| RelayUrl::from_str(&r).ok())
            .collect();

        let now = utils::now();
        let connection = NwcConnectionInner {
            connection_string: NostrWalletConnectURI::new(
                ctx.our_keys.public_key,
                relays,
                random_secret_key,
                None,
            )
            .to_string(),
            created_at: now,
            expiry_time_sec: req.expiry_time_mins.map(utils::mins_to_seconds),
            receive_only: req.receive_only.unwrap_or(false),
            paid_amount_sat: 0,
            periodic_budget: req
                .periodic_budget_req
                .map(|req| PeriodicBudgetInner::from_budget_request(req, now)),
        };
        ctx.persister
            .add_nwc_connection(req.name.clone(), connection.clone())?;
        ctx.trigger_resubscription().await;
        Ok(AddConnectionResponse {
            connection: connection.into(),
        })
    }

    async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> NwcResult<EditConnectionResponse> {
        let ctx = self.runtime_ctx().await?;
        let connection = ctx.persister.edit_nwc_connection(req)?;
        ctx.trigger_resubscription().await;
        Ok(EditConnectionResponse {
            connection: connection.into(),
        })
    }

    async fn list_connections(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        let connections = self.runtime_ctx().await?.persister.list_nwc_connections()?;
        Ok(connections
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect())
    }

    async fn remove_connection(&self, name: String) -> NwcResult<()> {
        let ctx = self.runtime_ctx().await?;
        ctx.persister.remove_nwc_connection(name)?;
        ctx.trigger_resubscription().await;
        Ok(())
    }

    async fn list_connection_payments(&self, name: String) -> NwcResult<Vec<Payment>> {
        let ctx = self.runtime_ctx().await?;
        if !ctx.persister.list_nwc_connections()?.contains_key(&name) {
            return Err(NwcError::ConnectionNotFound);
        }

        let paid_invoices = ctx.persister.list_paid_invoices()?;
        let Some(paid_invoices) = paid_invoices.get(&name) else {
            return Ok(vec![]);
        };
        let payment_list = ctx
            .sdk
            .list_payments(&ListPaymentsRequest {
                filters: Some(vec![PaymentType::Send]),
                ..Default::default()
            })
            .await
            .map_err(|err| NwcError::generic(format!("Could not list payments: {err}")))?;

        Ok(payment_list
            .into_iter()
            .filter(|payment| match &payment.details {
                PaymentDetails::Lightning { invoice, .. } => invoice
                    .as_ref()
                    .is_some_and(|invoice| paid_invoices.contains(invoice)),
                _ => false,
            })
            .collect())
    }

    async fn add_event_listener(&self, listener: Box<dyn NwcEventListener>) -> String {
        self.event_manager.add(listener).await
    }

    async fn remove_event_listener(&self, id: &str) {
        self.event_manager.remove(id).await
    }

    async fn handle_event(&self, event_id: String) -> NwcResult<()> {
        let ctx = self.runtime_ctx().await?;
        let event_id = EventId::from_str(&event_id)?;
        ctx.client.connect().await;

        // Retry fetching the event with exponential backoff
        // Relays may take time to sync or connection may be slow
        let mut retry_delay = Duration::from_secs(1);
        let max_retries = 3;
        let mut events = None;

        for attempt in 0..max_retries {
            match ctx
                .client
                .fetch_events(
                    Filter::new().id(event_id),
                    Duration::from_secs(30), // Increased timeout from 10s to 30s
                )
                .await
            {
                Ok(fetched) if !fetched.is_empty() => {
                    events = Some(fetched);
                    break;
                }
                Ok(_) => {
                    warn!(
                        "Event {} not found on attempt {}/{}",
                        event_id,
                        attempt + 1,
                        max_retries
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to fetch event {} on attempt {}/{}: {}",
                        event_id,
                        attempt + 1,
                        max_retries,
                        e
                    );
                }
            }

            if attempt < max_retries - 1 {
                info!("Retrying event fetch in {:?}", retry_delay);
                tokio::time::sleep(retry_delay).await;
                retry_delay *= 2; // Exponential backoff
            }
        }

        let Some(event_list) = events else {
            return Err(NwcError::EventNotFound);
        };

        let Some(event) = event_list.first() else {
            return Err(NwcError::EventNotFound);
        };

        Self::handle_event_inner(&ctx, event).await?;
        Ok(())
    }

    async fn get_info(&self) -> Option<NostrServiceInfo> {
        let lock = self.runtime_ctx.lock().await;
        let ctx = (*lock).as_ref()?;
        Some(NostrServiceInfo {
            wallet_pubkey: ctx.our_keys.public_key().to_hex(),
            connected_relays: self.config.relays(),
        })
    }

    async fn track_zap(&self, invoice: String, zap_request: String) -> NwcResult<()> {
        let ctx = self.runtime_ctx().await?;
        let zap_request = urlencoding::decode(&zap_request)?.into_owned();
        let zap_request_event: Event = serde_json::from_str(&zap_request)?;
        zap_request_event.verify()?;
        ctx.persister
            .add_tracked_zap(invoice.clone(), zap_request)?;
        info!("Successfully added zap tracking for invoice {invoice}");
        Ok(())
    }
}

#[sdk_macros::async_trait]
impl Plugin for SdkNwcService {
    fn id(&self) -> String {
        "breez-nwc-plugin".to_string()
    }

    async fn on_start(&self, sdk: PluginSdk, storage: PluginStorage) {
        let mut ctx_lock = self.runtime_ctx.lock().await;
        if ctx_lock.is_some() {
            warn!("Called on_start when service was already running.");
            return;
        }

        let (resub_tx, mut resub_rx) = mpsc::channel::<()>(10);
        let ctx = match self.new_ctx(sdk.clone(), storage, resub_tx).await {
            Ok(ctx) => Arc::new(ctx),
            Err(err) => {
                error!("Could not create NWC service runtime context: {err:?}");
                return;
            }
        };
        self.event_manager.resume_notifications();

        ctx.client.connect().await;

        if self.config.listen_to_events.is_some_and(|listen| !listen) {
            let res = match ctx.list_active_connections().await {
                Ok(connections) => ctx.new_sdk_listener(&connections).await,
                Err(err) => Err(err),
            };
            if let Err(err) = res {
                warn!("Could not create payment event listener: {err:?}");
            }
            *ctx_lock = Some(ctx);
            return;
        }

        let thread_ctx = ctx.clone();
        let event_loop_handle = tokio::spawn(async move {
            thread_ctx
                .event_manager
                .notify(NwcEvent {
                    details: NwcEventDetails::Connected,
                    connection_name: None,
                    event_id: None,
                })
                .await;
            info!("Successfully connected NWC client");

            thread_ctx.send_info_event().await;

            let mut maybe_expiry_interval = Self::new_maybe_interval(&thread_ctx).await;
            loop {
                let active_connections = match thread_ctx.list_active_connections().await {
                    Ok(clients) => clients,
                    Err(err) => {
                        warn!("Could not retreive active connections from database: {err:?}");
                        return;
                    }
                };

                if let Err(err) = thread_ctx.resubscribe(&active_connections).await {
                    warn!("Could not resubscribe to events: {err:?}");
                    return;
                };

                if let Err(err) = thread_ctx.new_sdk_listener(&active_connections).await {
                    warn!("Could not create payment event listener: {err:?}");
                }

                let mut notifications_listener = thread_ctx.client.notifications();
                loop {
                    tokio::select! {
                        Ok(notification) = notifications_listener.recv() => match notification {
                            RelayPoolNotification::Message { message: RelayMessage::Event { event, .. }, .. } => {
                                    info!("Received NWC event: {event:?}");
                                    let ctx = thread_ctx.clone();
                                    tokio::spawn(async move {
                                        if let Err(err) = Self::handle_event_inner(&ctx, &event).await {
                                            warn!("Could not handle NWC event {}: {}", event.id, err);
                                        }
                                    });
                            },
                            RelayPoolNotification::Message { message: RelayMessage::EndOfStoredEvents(_), .. } => notifications_listener = notifications_listener.resubscribe(),
                            _ => {},
                        },
                        Some(_) = Self::min_refresh_interval(&mut maybe_expiry_interval) => {
                            let result = match thread_ctx.persister.refresh_connections() {
                                Ok(result) => result,
                                Err(err) => {
                                    warn!("Could not refresh connections: {err}");
                                    continue;
                                }
                            };
                            for connection_name in result.deleted {
                                thread_ctx.event_manager.notify(NwcEvent { event_id: None, connection_name: Some(connection_name) , details: NwcEventDetails::ConnectionExpired }).await;
                            }
                            for connection_name in result.refreshed {
                                thread_ctx.event_manager.notify(NwcEvent { event_id: None, connection_name: Some(connection_name), details: NwcEventDetails::ConnectionRefreshed  }).await;
                            }
                        }
                        Some(_) = resub_rx.recv() => {
                            info!("Resubscribing to notifications.");
                            // We update the interval in case any connections have been
                            // added/removed
                            maybe_expiry_interval = Self::new_maybe_interval(&thread_ctx).await;
                            if let Some(ref mut interval) = maybe_expiry_interval {
                                // First time ticks instantly
                                interval.tick().await;
                            }
                            break;
                        }
                    }
                }
            }
        });

        if let Err(err) = ctx.event_loop_handle.set(event_loop_handle) {
            error!("Could not set NWC service event loop handle: {err:?}");
        }
        *ctx_lock = Some(ctx);
    }

    async fn on_stop(&self) {
        let mut ctx_lock = self.runtime_ctx.lock().await;
        if let Some(ref ctx) = *ctx_lock {
            ctx.clear().await;
            *ctx_lock = None;
        }
    }
}
