use std::{
    collections::HashMap,
    str::FromStr as _,
    sync::{Arc, Weak},
    time::Duration,
};

use crate::{
    context::RuntimeContext,
    error::NwcResult,
    event::{EventManager, NwcEvent, NwcEventDetails, NwcEventListener},
    handler::{RelayMessageHandler, SdkRelayMessageHandler},
    persist::Persister,
    sdk_event::SdkEventListener,
};
use anyhow::{bail, Result};
use breez_sdk_liquid::{
    plugin::{Plugin, PluginStorage},
    prelude::*,
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
use tokio::sync::{mpsc, Mutex, OnceCell};
use tokio_with_wasm::alias as tokio;

pub(crate) mod context;
pub mod error;
pub mod event;
pub(crate) mod handler;
mod persist;
pub(crate) mod sdk_event;

pub const DEFAULT_RELAY_URLS: [&str; 1] = ["wss://relay.getalbypro.com/breez"];

#[sdk_macros::async_trait]
pub trait NwcService: Send + Sync {
    /// Creates a Nostr Wallet Connect connection string for this service.
    ///
    /// Generates a unique connection URI that external applications can use
    /// to connect to this wallet service. The URI includes the wallet's public key,
    /// relay information, and a randomly generated secret for secure communication.
    ///
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn add_connection_string(&self, name: String) -> NwcResult<String>;

    /// Lists the active Nostr Wallet Connect connections for this service.
    async fn list_connection_strings(&self) -> NwcResult<HashMap<String, String>>;

    /// Removes a Nostr Wallet Connect connection string
    ///
    /// Removes a previously set connection string. Returns error if unset.
    ///
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn remove_connection_string(&self, name: String) -> NwcResult<()>;

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

pub struct NwcConfig {
    /// Custom relays urls to be used
    pub relay_urls: Option<Vec<String>>,
    /// Custom Nostr secret key for the wallet node, hex-encoded
    pub secret_key_hex: Option<String>,
}

impl NwcConfig {
    pub fn relays(&self) -> Vec<String> {
        self.relay_urls
            .clone()
            .unwrap_or(DEFAULT_RELAY_URLS.iter().map(|s| s.to_string()).collect())
    }
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
        sdk: Weak<LiquidSdk>,
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

    async fn handle_event(ctx: &RuntimeContext, notification: &RelayPoolNotification) {
        let RelayPoolNotification::Event { event, .. } = notification else {
            return;
        };
        info!("Received NWC event: {event:?}");

        let client_pubkey = event.pubkey;

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

        let (result, error) = match req.params {
            RequestParams::PayInvoice(req) => match ctx.handler.pay_invoice(req).await {
                Ok(res) => (Some(ResponseResult::PayInvoice(res)), None),
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
}

#[sdk_macros::async_trait]
impl NwcService for SdkNwcService {
    async fn add_connection_string(&self, name: String) -> NwcResult<String> {
        let random_secret_key = nostr_sdk::SecretKey::generate();
        let relays = self
            .config
            .relays()
            .into_iter()
            .filter_map(|r| RelayUrl::from_str(&r).ok())
            .collect();

        let ctx = self.runtime_ctx().await?;
        let uri =
            NostrWalletConnectURI::new(ctx.our_keys.public_key, relays, random_secret_key, None);
        ctx.persister.set_nwc_uri(name.clone(), uri.to_string())?;
        ctx.trigger_resubscription().await;
        Ok(uri.to_string())
    }

    async fn list_connection_strings(&self) -> NwcResult<HashMap<String, String>> {
        self.runtime_ctx().await?.persister.list_nwc_uris()
    }

    async fn remove_connection_string(&self, name: String) -> NwcResult<()> {
        let ctx = self.runtime_ctx().await?;
        ctx.persister.remove_nwc_uri(name)?;
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

    async fn on_start(&self, sdk: Weak<LiquidSdk>, storage: PluginStorage) {
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

            loop {
                let clients = match thread_ctx.list_clients().await {
                    Ok(clients) => clients,
                    Err(err) => {
                        warn!("Could not retreive active clients from database: {err:?}");
                        return;
                    }
                };
                if let Err(err) = thread_ctx.resubscribe(&clients).await {
                    warn!("Could not resubscribe to events: {err:?}");
                    return;
                };

                let sdk_listener_id = match sdk.upgrade() {
                    Some(sdk) => match sdk
                        .add_event_listener(Box::new(SdkEventListener::new(
                            thread_ctx.clone(),
                            clients,
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
                    },
                    None => {
                        warn!("SDK is not running. Exiting NWC service loop.");
                        return;
                    }
                };

                let mut notifications_listener = thread_ctx.client.notifications();
                loop {
                    tokio::select! {
                        Ok(notification) = notifications_listener.recv() => Self::handle_event(&thread_ctx, &notification).await,
                        Some(_) = resub_rx.recv() => {
                            info!("Resubscribing to notifications.");
                            if let Some(listener_id) = sdk_listener_id {
                                if let Some(sdk) = sdk.upgrade() {
                                    if let Err(err) = sdk.remove_event_listener(listener_id).await {
                                        warn!("Could not remove payment event listener: {err:?}");
                                    }
                                }
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
