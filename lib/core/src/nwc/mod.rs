use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr as _,
    time::Duration,
};

use crate::{
    event::EventManager,
    model::{Config, Payment},
    persist::Persister,
    utils,
};
use anyhow::Result;
use handler::{BreezRelayMessageHandler, RelayMessageHandler};
use log::{info, warn};
use maybe_sync::{MaybeSend, MaybeSync};
use nostr_sdk::{
    nips::nip44::{decrypt, encrypt, Version},
    nips::nip47::{
        ErrorCode, Method, NIP47Error, NostrWalletConnectURI, Notification, NotificationResult,
        NotificationType, PaymentNotification, Request, RequestParams, Response, ResponseResult,
        TransactionType,
    },
    Client as NostrClient, EventBuilder, Filter, Keys, Kind, RelayPoolNotification, RelayUrl,
    SubscriptionId, Tag, Timestamp,
};
use sdk_common::utils::Arc;
use tokio::sync::{mpsc, watch, Mutex};
use tokio::task::JoinHandle;
use tokio_with_wasm::alias as tokio;

use crate::model::{NwcEvent, SdkEvent};

pub(crate) mod handler;
mod persist;

#[sdk_macros::async_trait]
pub trait NWCService: MaybeSend + MaybeSync {
    /// Creates a Nostr Wallet Connect connection string for this service.
    ///
    /// Generates a unique connection URI that external applications can use
    /// to connect to this wallet service. The URI includes the wallet's public key,
    /// relay information, and a randomly generated secret for secure communication.
    ///
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn new_connection_string(&self, name: String) -> Result<String>;

    /// Lists the active Nostr Wallet Connect connections for this service.
    async fn list_connection_strings(&self) -> Result<HashMap<String, String>>;

    /// Removes a Nostr Wallet Connect connection string
    ///
    /// Removes a previously set connection string. Returns error if unset.
    ///
    /// # Arguments
    /// * `name` - The unique identifier for the connection string
    async fn remove_connection_string(&self, name: String) -> Result<()>;

    /// Starts the NWC service event processing loop.
    ///
    /// Establishes connections to Nostr relays and begins listening for incoming
    /// wallet operation requests. The service will:
    /// 1. Connect to configured relays
    /// 2. Broadcast service capability information
    /// 3. Listen for and process incoming requests
    /// 4. Send appropriate responses back through the relays
    ///
    /// The service runs until a shutdown signal is received.
    ///
    /// # Arguments
    /// * `shutdown_receiver` - Channel for receiving shutdown signals
    fn start(self: Arc<Self>, shutdown_receiver: watch::Receiver<()>) -> JoinHandle<()>;

    /// Stops the NWC service and performs cleanup.
    ///
    /// Gracefully shuts down the service by:
    /// 1. Disconnecting from all Nostr relays
    /// 2. Aborting the background event processing task
    /// 3. Releasing any held resources
    async fn stop(&self);
}

pub struct BreezNWCService<Handler: RelayMessageHandler> {
    keys: Keys,
    config: Config,
    client: NostrClient,
    handler: Box<Handler>,
    event_manager: Arc<EventManager>,
    persister: std::sync::Arc<Persister>,
    subscriptions: Mutex<HashMap<String, SubscriptionId>>,
    resubscription_trigger: Mutex<Option<mpsc::Sender<()>>>,
}

impl<Handler: RelayMessageHandler> BreezNWCService<Handler> {
    /// Creates a new BreezNWCService instance.
    ///
    /// Initializes the service with the provided cryptographic keys, handler,
    /// and connects to the specified Nostr relays.
    ///
    /// # Arguments
    /// * `handler` - Handler for processing relay messages
    /// * `relays` - List of relay URLs to connect to
    ///
    /// # Returns
    /// * `Ok(Arc<BreezNWCService>)` - Successfully initialized service
    /// * `Err(anyhow::Error)` - Error adding relays or initializing
    pub(crate) async fn new(
        handler: Box<Handler>,
        config: Config,
        persister: std::sync::Arc<Persister>,
        event_manager: Arc<EventManager>,
    ) -> Result<Arc<Self>> {
        let client = NostrClient::default();
        let relays = config.nwc_relays();
        for relay in &relays {
            client.add_relay(relay).await?;
        }

        let secret_key = Self::get_or_create_secret_key(&config, &persister)?;
        let keys = Keys::parse(&secret_key)?;
        Ok(Arc::new(Self {
            client,
            config,
            handler,
            keys,
            persister,
            event_manager,
            subscriptions: Default::default(),
            resubscription_trigger: Default::default(),
        }))
    }

    fn get_or_create_secret_key(config: &Config, p: &Persister) -> Result<String> {
        // If we have a key from the configuration, use it
        if let Some(key) = config
            .nwc_options
            .as_ref()
            .and_then(|opts| opts.secret_key.clone())
        {
            return Ok(key);
        }

        // Otherwise, try restoring it from the previous session
        if let Ok(Some(key)) = p.get_nwc_seckey() {
            return Ok(key);
        }

        // If none exists, generate a new one
        let key = nostr_sdk::key::SecretKey::generate().to_secret_hex();
        p.set_nwc_seckey(key.clone())?;
        Ok(key)
    }

    fn list_clients(&self) -> Result<HashMap<String, NostrWalletConnectURI>> {
        Ok(self
            .persister
            .list_nwc_uris()?
            .into_iter()
            .filter_map(|(name, uri)| {
                NostrWalletConnectURI::from_str(&uri)
                    .map(|uri| (name, uri))
                    .ok()
            })
            .collect())
    }

    async fn subscribe(
        &self,
        name: String,
        uri: &NostrWalletConnectURI,
        trigger_resub: bool,
    ) -> Result<()> {
        let sub_id = self
            .client
            .subscribe(
                Filter {
                    generic_tags: BTreeMap::from([(
                        nostr_sdk::SingleLetterTag::from_char('p')?,
                        BTreeSet::from([uri.public_key.to_string()]),
                    )]),
                    kinds: Some(BTreeSet::from([Kind::WalletConnectRequest])),
                    ..Default::default()
                },
                None,
            )
            .await?;
        self.subscriptions
            .lock()
            .await
            .insert(name.clone(), sub_id.val);
        if trigger_resub {
            if let Some(ref trigger) = *self.resubscription_trigger.lock().await {
                let _ = trigger.send(()).await;
            }
        }
        info!("Successfully subscribed to `{name}` events");
        Ok(())
    }

    async fn unsubscribe(&self, name: &str) {
        if let Some(sub_id) = self.subscriptions.lock().await.remove(name) {
            self.client.unsubscribe(&sub_id).await;
        }
        if let Some(ref trigger) = *self.resubscription_trigger.lock().await {
            let _ = trigger.send(()).await;
        }
    }
}

impl BreezNWCService<BreezRelayMessageHandler> {
    async fn send_event(&self, eb: EventBuilder) -> Result<(), nostr_sdk::client::Error> {
        let evt = eb.sign_with_keys(&self.keys)?;
        self.client.send_event(&evt).await?;
        Ok(())
    }

    async fn handle_event(
        &self,
        n: &RelayPoolNotification,
        clients: &HashMap<String, NostrWalletConnectURI>,
    ) {
        let RelayPoolNotification::Event {
            event,
            subscription_id,
            ..
        } = n
        else {
            return;
        };
        info!("Received NWC event: {event:?}");

        // Verify event subscription exists and retrieve URI
        let Some(client_name) = self
            .subscriptions
            .lock()
            .await
            .iter()
            .find(|(_, sub_id)| *sub_id == subscription_id)
            .map(|(name, _)| name.clone())
        else {
            warn!(
                "Could not find active event subscription. Skipping event {}",
                event.id
            );
            return;
        };
        let Some(client_uri) = clients.get(&client_name) else {
            warn!("Could not retrieve client URI. Skipping event {}", event.id);
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
        let nwc_client_keypair = Keys::new(client_uri.secret.clone());
        let decrypted_content = match decrypt(
            self.keys.secret_key(),
            &nwc_client_keypair.public_key,
            &event.content,
        ) {
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
            RequestParams::PayInvoice(req) => match self.handler.pay_invoice(req).await {
                Ok(res) => (Some(ResponseResult::PayInvoice(res)), None),
                Err(e) => (None, Some(e)),
            },
            RequestParams::ListTransactions(req) => match self.handler.list_transactions(req).await
            {
                Ok(res) => (Some(ResponseResult::ListTransactions(res)), None),
                Err(e) => (None, Some(e)),
            },
            RequestParams::GetBalance => match self.handler.get_balance().await {
                Ok(res) => (Some(ResponseResult::GetBalance(res)), None),
                Err(e) => (None, Some(e)),
            },
            _ => {
                info!("Received unhandled request: {req:?}");
                return;
            }
        };

        let _ = self.handle_local_notification(&result, &error).await;

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
            self.keys.secret_key(),
            &nwc_client_keypair.public_key,
            &content,
            Version::V2,
        ) {
            Ok(encrypted) => encrypted,
            Err(e) => {
                warn!("Could not encrypt response content: {e:?}");
                return;
            }
        };

        let eb = EventBuilder::new(Kind::WalletConnectResponse, encrypted_content)
            .tags([Tag::event(event.id), Tag::public_key(client_uri.public_key)]);
        if let Err(e) = self.send_event(eb).await {
            warn!("Could not send response event to relay pool: {e:?}");
        }
        info!("sent encrypted NWC response");
    }

    async fn handle_local_notification(
        &self,
        result: &Option<ResponseResult>,
        error: &Option<NIP47Error>,
    ) -> Result<()> {
        info!("Handling notification: {result:?} {error:?}");
        let event: SdkEvent = match (result, error) {
            (Some(ResponseResult::PayInvoice(response)), None) => SdkEvent::NWC {
                details: NwcEvent::PayInvoice {
                    success: true,
                    preimage: Some(response.preimage.clone()),
                    fees_sat: response.fees_paid.map(|f| f / 1000),
                    error: None,
                },
            },
            (None, Some(error)) => match error.code {
                ErrorCode::PaymentFailed => SdkEvent::NWC {
                    details: NwcEvent::PayInvoice {
                        success: false,
                        preimage: None,
                        fees_sat: None,
                        error: Some(error.message.clone()),
                    },
                },
                _ => {
                    warn!("Unhandled error code: {:?}", error.code);
                    return Ok(());
                }
            },
            (Some(ResponseResult::ListTransactions(_)), None) => SdkEvent::NWC {
                details: NwcEvent::ListTransactions,
            },
            (Some(ResponseResult::GetBalance(_)), None) => SdkEvent::NWC {
                details: NwcEvent::GetBalance,
            },
            _ => {
                warn!("Unexpected combination");
                return Ok(());
            }
        };
        info!("Sending event: {event:?}");
        self.event_manager.notify(event).await;
        Ok(())
    }

    async fn forward_payment_to_clients(
        &self,
        p: &Payment,
        clients: &HashMap<String, NostrWalletConnectURI>,
    ) {
        let (invoice, description, preimage, payment_hash) = match &p.details {
            crate::model::PaymentDetails::Lightning {
                invoice,
                description,
                preimage,
                payment_hash,
                ..
            } => (
                invoice.clone().unwrap_or_default(),
                description.clone(),
                preimage.clone().unwrap_or_default(),
                payment_hash.clone().unwrap_or_default(),
            ),
            _ => {
                return;
            }
        };

        let payment_notification = PaymentNotification {
            transaction_type: Some(if p.payment_type == crate::model::PaymentType::Send {
                TransactionType::Outgoing
            } else {
                TransactionType::Incoming
            }),
            invoice,
            description: Some(description),
            description_hash: None,
            preimage,
            payment_hash,
            amount: p.amount_sat * 1000,
            fees_paid: p.fees_sat * 1000,
            created_at: Timestamp::from_secs(p.timestamp as u64),
            expires_at: None,
            settled_at: Timestamp::from_secs(p.timestamp as u64),
            metadata: None,
        };

        let notification = if p.payment_type == crate::model::PaymentType::Send {
            Notification {
                notification_type: NotificationType::PaymentSent,
                notification: NotificationResult::PaymentSent(payment_notification),
            }
        } else {
            Notification {
                notification_type: NotificationType::PaymentReceived,
                notification: NotificationResult::PaymentReceived(payment_notification),
            }
        };

        let notification_content = match serde_json::to_string(&notification) {
            Ok(content) => content,
            Err(e) => {
                warn!("Could not serialize notification: {e:?}");
                return;
            }
        };

        for (_, uri) in clients {
            let nwc_client_keypair = Keys::new(uri.secret.clone());
            let encrypted_content = match encrypt(
                self.keys.secret_key(),
                &nwc_client_keypair.public_key,
                &notification_content,
                Version::V2,
            ) {
                Ok(encrypted) => encrypted,
                Err(e) => {
                    warn!("Could not encrypt notification content: {e:?}");
                    continue;
                }
            };

            let eb = EventBuilder::new(Kind::Custom(23196), encrypted_content)
                .tags([Tag::public_key(uri.public_key)]);

            if let Err(e) = self.send_event(eb).await {
                warn!("Could not send notification event to relay: {e:?}");
            } else {
                info!("Sent payment notification to relay");
            }
        }
    }
}

#[sdk_macros::async_trait]
impl NWCService for BreezNWCService<BreezRelayMessageHandler> {
    async fn new_connection_string(&self, name: String) -> Result<String> {
        let random_secret_key = nostr_sdk::SecretKey::generate();
        let relays = self
            .config
            .nwc_relays()
            .into_iter()
            .filter_map(|r| RelayUrl::from_str(&r).ok())
            .collect();
        let uri = NostrWalletConnectURI::new(self.keys.public_key, relays, random_secret_key, None);
        self.persister.set_nwc_uri(name.clone(), uri.to_string())?;
        self.subscribe(name, &uri, true).await?;
        Ok(uri.to_string())
    }

    async fn list_connection_strings(&self) -> Result<HashMap<String, String>> {
        self.persister.list_nwc_uris()
    }

    async fn remove_connection_string(&self, name: String) -> Result<()> {
        self.unsubscribe(&name).await;
        self.persister.remove_nwc_uri(name)?;
        Ok(())
    }

    fn start(self: Arc<Self>, shutdown: watch::Receiver<()>) -> JoinHandle<()> {
        let s = self.clone();
        let mut sdk_event_listener = self.event_manager.subscribe();
        let nwc_service_future = async move {
            s.client.connect().await;

            info!("Successfully connected NWC client");

            // Broadcast info event
            let mut content: String = [
                Method::PayInvoice,
                Method::ListTransactions,
                Method::GetBalance,
            ]
            .map(|m| m.to_string())
            .join(" ");
            content.push_str("notifications");

            if let Err(err) = s
                .send_event(
                    EventBuilder::new(Kind::WalletConnectInfo, content)
                        .tag(Tag::custom("encryption".into(), ["nip44_v2".to_string()])),
                )
                .await
            {
                warn!("Could not send info event to relay pool: {err:?}");
            }

            // Load the clients from the database and susbcribe to each pubkey
            let clients = match s.list_clients() {
                Ok(clients) => clients,
                Err(err) => {
                    warn!("Could not load active NWC clients: {err:?}");
                    return;
                }
            };
            for (name, uri) in &clients {
                if let Err(err) = s.subscribe(name.clone(), &uri, false).await {
                    warn!("Could not subscribe to persisted client: {err:?}");
                    continue;
                };
            }

            let (resub_tx, mut resub_rx) = mpsc::channel::<()>(10);
            *s.resubscription_trigger.lock().await = Some(resub_tx);
            loop {
                let mut notifications_listener = s.client.notifications();
                loop {
                    tokio::select! {
                        Ok(SdkEvent::PaymentSucceeded { details: payment }) = sdk_event_listener.recv() => s.forward_payment_to_clients(&payment, &clients).await,
                        Ok(notification) = notifications_listener.recv() => s.handle_event(&notification, &clients).await,
                        Some(_) = resub_rx.recv() => {
                            info!("URI list has changed. Resubscribing to notifications.");
                            break;
                        }
                    }
                }
            }
        };

        tokio::task::spawn(async move {
            utils::run_with_shutdown_and_cleanup(
                shutdown,
                "Received shutdown signal, exiting NWC service loop",
                nwc_service_future,
                || async move {
                    match tokio::time::timeout(Duration::from_secs(2), self.stop()).await {
                        Ok(_) => {
                            info!("Successfully disconnected NWC client");
                        }
                        Err(err) => {
                            warn!("Could not disconnect NWC client within timeout: {err:?}");
                        }
                    }
                },
            )
            .await
        })
    }

    async fn stop(&self) {
        self.client.disconnect().await;
        *self.resubscription_trigger.lock().await = None;
    }
}
