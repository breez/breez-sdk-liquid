use std::{collections::HashMap, str::FromStr as _, sync::Arc, time::Duration};

use crate::{
    context::RuntimeContext,
    error::NwcResult,
    event::{EventManager, NwcEvent, NwcEventDetails, NwcEventListener},
    handler::{RelayMessageHandler, SdkRelayMessageHandler},
    model::{
        ActiveConnection, AddConnectionRequest, AddConnectionResponse, EditConnectionRequest,
        EditConnectionResponse, NwcConfig, NwcConnection, PeriodicBudget,
    },
    persist::Persister,
    sdk_event::SdkEventListener,
};
use anyhow::{bail, Result};
use breez_sdk_liquid::{
    plugin::{Plugin, PluginSdk, PluginStorage},
    InputType,
};
use log::{debug, error, info, warn};
use nostr_sdk::{
    nips::nip44::{decrypt, encrypt, Version},
    nips::nip47::{
        ErrorCode, NIP47Error, NostrWalletConnectURI, Request, RequestParams, Response,
        ResponseResult,
    },
    Client as NostrClient, EventBuilder, Keys, Kind, RelayPoolNotification, RelayUrl, Tag,
    Timestamp,
};
use tokio::{
    sync::{mpsc, Mutex, OnceCell},
    time::Interval,
};
use tokio_with_wasm::alias as tokio;

pub(crate) mod context;
pub mod error;
pub mod event;
pub(crate) mod handler;
pub mod model;
mod persist;
pub(crate) mod sdk_event;
pub(crate) mod utils;

pub const MIN_REFRESH_INTERVAL_SEC: u64 = 60; // 1 minute
pub const DEFAULT_PERIODIC_BUDGET_TIME_SEC: u32 = 60 * 60 * 24 * 30; // 30 days
pub const DEFAULT_RELAY_URLS: [&str; 1] = ["wss://relay.getalbypro.com/breez"];

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
    ///     * `expiry_time_sec` - the expiry time of the connection string. If None, it will **not**
    ///     expire
    ///     * `periodic_budget_req` - the periodic budget paremeters of the connection if any.
    ///     You can specify the [maximum amount \(in satoshi\) per period](crate::model::PeriodicBudget::max_budget_sat)
    ///     and the [period reset time \(in seconds\)](crate::model::PeriodicBudget::reset_time_sec)
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
    ///     * `expiry_time_sec` - the expiry time of the connection string. If None, it will **not**
    ///     expire
    ///     * `periodic_budget_req` - the periodic budget paremeters of the connection if any.
    ///     You can specify the [maximum amount \(in satoshi\) per period](crate::model::PeriodicBudget::max_budget_sat)
    ///     and the [period reset time \(in seconds\)](crate::model::PeriodicBudget::reset_time_sec)
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

    async fn handle_event(
        ctx: &RuntimeContext,
        active_connections: &mut HashMap<String, ActiveConnection>,
        notification: &RelayPoolNotification,
    ) {
        let RelayPoolNotification::Event { event, .. } = notification else {
            return;
        };
        info!("Received NWC event: {event:?}");

        let client_pubkey = event.pubkey;

        let Some((connection_name, client)) = active_connections
            .iter_mut()
            .find(|(_, con)| con.uri.public_key == client_pubkey)
        else {
            info!("Received event from unrecognized public key: {client_pubkey:?}. Skipping.");
            return;
        };

        // Verify the event has not expired
        if event
            .tags
            .expiration()
            .is_some_and(|t| *t > Timestamp::now())
        {
            warn!("Event {} has expired. Skipping.", event.id);
            return;
        }

        // Verify the event signature and event id
        if let Err(e) = event.verify() {
            warn!("Event signature verification failed: {e:?}");
            return;
        }

        // Decrypt the event content
        let decrypted_content =
            match decrypt(ctx.our_keys.secret_key(), &client_pubkey, &event.content) {
                Ok(content) => content,
                Err(e) => {
                    warn!("Failed to decrypt event content: {e:?}");
                    return;
                }
            };

        info!("Decrypted NWC notification: {decrypted_content}");

        let req = match serde_json::from_str::<Request>(&decrypted_content) {
            Ok(r) => r,
            Err(e) => {
                warn!("Received unexpected request from relay pool: {decrypted_content} err {e:?}");
                return;
            }
        };

        if client.connection.receive_only && !matches!(req.params, RequestParams::MakeInvoice(_)) {
            warn!(
                "Could not execute command: {:?}: connection is receive-only. Skipping.",
                req.params
            );
            return;
        }

        let (result, error) = match req.params {
            RequestParams::PayInvoice(req) => {
                let Ok(InputType::Bolt11 { invoice }) =
                    sdk_common::input_parser::parse(&req.invoice, None).await
                else {
                    warn!(
                        "Could not parse pay_invoice invoice: {}. Skipping command.",
                        req.invoice
                    );
                    return;
                };
                let Some(req_amount_sat) = req
                    .amount
                    .or(invoice.amount_msat)
                    .map(|amount| amount.div_ceil(1000))
                else {
                    warn!(
                        "Cannot pay an amountless invoice: {}. Skipping.",
                        req.invoice
                    );
                    return;
                };

                if let Some(ref mut periodic_budget) = client.connection.periodic_budget {
                    if periodic_budget.used_budget_sat + req_amount_sat
                        > periodic_budget.max_budget_sat
                    {
                        warn!("Cannot pay invoice: max periodic budget exceeded on connection \"{connection_name}\".");
                        return;
                    }
                    // We modify the connection's budget before executing the payment to avoid any race
                    // conditions
                    periodic_budget.used_budget_sat += req_amount_sat;
                    if let Err(err) = ctx
                        .persister
                        .update_periodic_budget(connection_name, periodic_budget.clone())
                        .await
                    {
                        warn!("Cannot pay invoice: could not update periodic budget on connection \"{connection_name}\": {err}.");
                        return;
                    }
                }
                match ctx.handler.pay_invoice(req).await {
                    Ok(res) => (Some(ResponseResult::PayInvoice(res)), None),
                    Err(e) => {
                        // In case of payment failure, we want to undo the periodic budget changes
                        if let Some(ref mut periodic_budget) = client.connection.periodic_budget {
                            periodic_budget.used_budget_sat -= req_amount_sat;
                            if let Err(err) = ctx
                                .persister
                                .update_periodic_budget(connection_name, periodic_budget.clone())
                                .await
                            {
                                warn!("Cannot pay invoice: could not update periodic budget on connection \"{connection_name}\": {err}.");
                                return;
                            }
                        }
                        (None, Some(e))
                    }
                }
            }
            RequestParams::MakeInvoice(req) => match ctx.handler.make_invoice(req).await {
                Ok(res) => (Some(ResponseResult::MakeInvoice(res)), None),
                Err(e) => (None, Some(e)),
            },
            RequestParams::ListTransactions(req) => {
                match ctx.handler.list_transactions(req).await {
                    Ok(res) => (Some(ResponseResult::ListTransactions(res)), None),
                    Err(e) => (None, Some(e)),
                }
            }
            RequestParams::GetBalance => match ctx.handler.get_balance().await {
                Ok(res) => (Some(ResponseResult::GetBalance(res)), None),
                Err(e) => (None, Some(e)),
            },
            _ => {
                info!("Received unhandled request: {req:?}");
                return;
            }
        };

        Self::handle_local_notification(ctx, &result, &error, &event.id.to_string()).await;

        let content = match serde_json::to_string(&Response {
            result_type: req.method,
            result,
            error,
        }) {
            Ok(c) => c,
            Err(e) => {
                warn!("Could not serialize Nostr response: {e:?}");
                return;
            }
        };
        info!("NWC Response content: {content}");
        info!("encrypting NWC response");
        let encrypted_content = match encrypt(
            ctx.our_keys.secret_key(),
            &client_pubkey,
            &content,
            Version::V2,
        ) {
            Ok(encrypted) => encrypted,
            Err(e) => {
                warn!("Could not encrypt response content: {e:?}");
                return;
            }
        };

        let event_builder = EventBuilder::new(Kind::WalletConnectResponse, encrypted_content)
            .tags([Tag::event(event.id), Tag::public_key(client_pubkey)]);
        if let Err(e) = ctx.send_event(event_builder).await {
            warn!("Could not send response event to relay pool: {e:?}");
        }
        info!("sent encrypted NWC response");
    }

    async fn handle_local_notification(
        ctx: &RuntimeContext,
        result: &Option<ResponseResult>,
        error: &Option<NIP47Error>,
        event_id: &str,
    ) {
        debug!("Handling notification: {result:?} {error:?}");
        let event = match (result, error) {
            (Some(ResponseResult::PayInvoice(response)), None) => NwcEvent {
                details: NwcEventDetails::PayInvoice {
                    success: true,
                    preimage: Some(response.preimage.clone()),
                    fees_sat: response.fees_paid.map(|f| f / 1000),
                    error: None,
                },
                event_id: Some(event_id.to_string()),
            },
            (None, Some(error)) => match error.code {
                ErrorCode::PaymentFailed => NwcEvent {
                    details: NwcEventDetails::PayInvoice {
                        success: false,
                        preimage: None,
                        fees_sat: None,
                        error: Some(error.message.clone()),
                    },
                    event_id: Some(event_id.to_string()),
                },
                _ => {
                    warn!("Unhandled error code: {:?}", error.code);
                    return;
                }
            },
            (Some(ResponseResult::ListTransactions(_)), None) => NwcEvent {
                details: NwcEventDetails::ListTransactions,
                event_id: Some(event_id.to_string()),
            },
            (Some(ResponseResult::GetBalance(_)), None) => NwcEvent {
                details: NwcEventDetails::GetBalance,
                event_id: Some(event_id.to_string()),
            },
            _ => {
                warn!("Unexpected combination");
                return;
            }
        };
        info!("Sending event: {event:?}");
        ctx.event_manager.notify(event).await;
    }

    fn connections_have_changed(
        active_connections: &mut HashMap<String, ActiveConnection>,
    ) -> (bool, Vec<String>, Vec<String>) {
        let now = utils::now();
        let mut to_delete = vec![];
        let mut to_refresh = vec![];
        for (name, ActiveConnection { connection, .. }) in active_connections.iter_mut() {
            // If the connection has expired, mark it for deletion
            if let Some(expiry) = connection.expiry_time_sec {
                if now >= connection.created_at + expiry {
                    to_delete.push(name.clone());
                    continue;
                }
            }
            // If the connection's periodic budget has to be updated
            if let Some(ref mut budget) = connection.periodic_budget {
                if now >= budget.updated_at + budget.reset_time_sec {
                    budget.used_budget_sat = 0;
                    budget.updated_at = now;
                    to_refresh.push(name.clone());
                }
            }
        }
        for name in &to_delete {
            active_connections.remove(name);
        }
        (
            !to_delete.is_empty() || !to_refresh.is_empty(),
            to_delete,
            to_refresh,
        )
    }

    async fn new_maybe_interval(ctx: &RuntimeContext) -> Option<Interval> {
        ctx.persister
            .get_min_interval()
            .await
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
        let random_secret_key = nostr_sdk::SecretKey::generate();
        let relays = self
            .config
            .relays()
            .into_iter()
            .filter_map(|r| RelayUrl::from_str(&r).ok())
            .collect();

        let ctx = self.runtime_ctx().await?;
        let now = utils::now();
        let connection = NwcConnection {
            connection_string: NostrWalletConnectURI::new(
                ctx.our_keys.public_key,
                relays,
                random_secret_key,
                None,
            )
            .to_string(),
            created_at: now,
            expiry_time_sec: req.expiry_time_sec,
            receive_only: req.receive_only.unwrap_or(false),
            periodic_budget: req
                .periodic_budget_req
                .map(|req| PeriodicBudget::from_budget_request(req, now)),
        };
        ctx.persister
            .add_nwc_connection(req.name.clone(), connection.clone())
            .await?;
        ctx.trigger_resubscription().await;
        Ok(AddConnectionResponse { connection })
    }

    async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> NwcResult<EditConnectionResponse> {
        let connection = self
            .runtime_ctx()
            .await?
            .persister
            .edit_nwc_connection(
                &req.name,
                req.expiry_time_sec,
                req.receive_only,
                req.periodic_budget_req
                    .map(|req| PeriodicBudget::from_budget_request(req, utils::now())),
            )
            .await?;
        self.runtime_ctx().await?.trigger_resubscription().await;
        Ok(EditConnectionResponse { connection })
    }

    async fn list_connections(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        self.runtime_ctx()
            .await?
            .persister
            .list_nwc_connections()
            .await
    }

    async fn remove_connection(&self, name: String) -> NwcResult<()> {
        let ctx = self.runtime_ctx().await?;
        ctx.persister.remove_nwc_connection(name).await?;
        ctx.trigger_resubscription().await;
        Ok(())
    }

    async fn add_event_listener(&self, listener: Box<dyn NwcEventListener>) -> String {
        self.event_manager.add(listener).await
    }

    async fn remove_event_listener(&self, id: &str) {
        self.event_manager.remove(id).await
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

        let thread_ctx = ctx.clone();
        let event_loop_handle = tokio::spawn(async move {
            thread_ctx.client.connect().await;
            thread_ctx
                .event_manager
                .notify(NwcEvent {
                    details: NwcEventDetails::Connected,
                    event_id: None,
                })
                .await;
            info!("Successfully connected NWC client");

            thread_ctx.send_info_event().await;

            let mut maybe_expiry_interval = Self::new_maybe_interval(&thread_ctx).await;
            loop {
                let mut active_connections = match thread_ctx.list_active_connections().await {
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

                let sdk_listener_id = match sdk
                    .add_event_listener(Box::new(SdkEventListener::new(
                        thread_ctx.clone(),
                        active_connections
                            .values()
                            .map(|con| con.uri.clone())
                            .collect(),
                    )))
                    .await
                {
                    Ok(listener_id) => {
                        *thread_ctx.sdk_listener_id.lock().await = Some(listener_id.clone());
                        Some(listener_id)
                    }
                    Err(err) => {
                        warn!("Could not set payment event listener: {err:?}");
                        None
                    }
                };

                let mut notifications_listener = thread_ctx.client.notifications();
                loop {
                    tokio::select! {
                        Ok(notification) = notifications_listener.recv() => Self::handle_event(&thread_ctx, &mut active_connections, &notification).await,
                        Some(_) = Self::min_refresh_interval(&mut maybe_expiry_interval) => {
                            let (have_changed, deleted, refreshed) = Self::connections_have_changed(&mut active_connections);
                            if have_changed {
                                let connections = active_connections.into_iter().map(|(name, con)| (name, con.connection)).collect();
                                if let Err(err) = thread_ctx.persister.set_connections_raw(connections) {
                                    warn!("Could not save active connections: {err:?}");
                                    return;
                                }
                                for name in deleted {
                                    thread_ctx.event_manager.notify(NwcEvent { event_id: None, details: NwcEventDetails::ConnectionExpired { name } }).await;
                                }
                                for name in refreshed {
                                    thread_ctx.event_manager.notify(NwcEvent { event_id: None, details: NwcEventDetails::ConnectionRefreshed { name } }).await;
                                }
                                break;
                            }
                        }
                        Some(_) = resub_rx.recv() => {
                            info!("Resubscribing to notifications.");
                            if let Some(listener_id) = sdk_listener_id {
                                if let Err(err) = sdk.remove_event_listener(listener_id).await {
                                    warn!("Could not remove payment event listener: {err:?}");
                                }
                            }
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
