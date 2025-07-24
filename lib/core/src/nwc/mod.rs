use std::{collections::BTreeSet, str::FromStr as _};

use anyhow::Result;
use bip39::rand::{self, RngCore};
use handler::{BreezRelayMessageHandler, RelayMessageHandler};
use log::{info, warn};
use maybe_sync::{MaybeSend, MaybeSync};
use nostr_sdk::{
    nips::nip04::{decrypt, encrypt},
    nips::nip47::{
        ErrorCode, Method, NIP47Error, NostrWalletConnectURI, Notification, NotificationResult,
        NotificationType, PaymentNotification, Request, RequestParams, Response, ResponseResult,
        TransactionType,
    },
    Client as NostrClient, EventBuilder, Filter, Keys, Kind, RelayPoolNotification, RelayUrl, Tag,
    Timestamp,
};
use sdk_common::utils::Arc;
use tokio::sync::{broadcast, watch, OnceCell};
use tokio::task::JoinHandle;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use tokio::task::spawn as platform_spawn;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use wasm_bindgen_futures::spawn_local as platform_spawn;

use crate::model::{NwcEvent, SdkEvent};

pub(crate) mod handler;

#[sdk_macros::async_trait]
pub trait NWCService: MaybeSend + MaybeSync {
    /// Creates a Nostr Wallet Connect connection string for this service.
    ///
    /// Generates a unique connection URI that external applications can use
    /// to connect to this wallet service. The URI includes the wallet's public key,
    /// relay information, and a randomly generated secret for secure communication.
    async fn get_connection_string(&self) -> String;

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
    /// * `notifier` - Broadcast sender for emitting SDK events
    fn start(
        &self,
        shutdown_receiver: watch::Receiver<()>,
        listener: broadcast::Receiver<SdkEvent>,
        notifier: broadcast::Sender<SdkEvent>,
    );

    /// Stops the NWC service and performs cleanup.
    ///
    /// Gracefully shuts down the service by:
    /// 1. Disconnecting from all Nostr relays
    /// 2. Aborting the background event processing task
    /// 3. Releasing any held resources
    async fn stop(self);
}

pub struct BreezNWCService<Handler: RelayMessageHandler> {
    keys: Keys,
    handler: Arc<Handler>,
    nwc_uri: NostrWalletConnectURI,
    client: std::sync::Arc<NostrClient>,
    event_loop_handle: OnceCell<JoinHandle<()>>,
}

impl<Handler: RelayMessageHandler> BreezNWCService<Handler> {
    /// Creates a new BreezNWCService instance.
    ///
    /// Initializes the service with the provided cryptographic keys, handler,
    /// and connects to the specified Nostr relays.
    ///
    /// # Arguments
    /// * `secret_key` - The secret key for signing Nostr events
    /// * `handler` - Handler for processing relay messages
    /// * `relays` - List of relay URLs to connect to
    ///
    /// # Returns
    /// * `Ok(BreezNWCService)` - Successfully initialized service
    /// * `Err(anyhow::Error)` - Error adding relays or initializing
    pub(crate) async fn new(handler: Arc<Handler>, relays: &[String]) -> Result<Self> {
        let client = std::sync::Arc::new(NostrClient::default());
        for relay in relays {
            client.add_relay(relay).await?;
        }
        let mut rng = rand::thread_rng();
        let keys = Keys::generate_with_rng(&mut rng);

        let relays = relays
            .iter()
            .filter_map(|r| RelayUrl::from_str(r).ok())
            .collect();
        let nwc_uri = Self::new_connection_uri(&keys, relays)?;

        Ok(Self {
            client,
            handler,
            keys,
            nwc_uri,
            event_loop_handle: OnceCell::new(),
        })
    }

    fn new_connection_uri(keys: &Keys, relays: Vec<RelayUrl>) -> Result<NostrWalletConnectURI> {
        let mut random_bytes = [0u8; 32];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut random_bytes);
        let random_secret_key = nostr_sdk::SecretKey::from_slice(&random_bytes).unwrap();
        Ok(NostrWalletConnectURI::new(
            keys.public_key(),
            relays,
            random_secret_key,
            None,
        ))
    }
}

impl BreezNWCService<BreezRelayMessageHandler> {
    async fn send_event(
        eb: EventBuilder,
        keys: &Keys,
        client: std::sync::Arc<NostrClient>,
    ) -> Result<(), nostr_sdk::client::Error> {
        let evt = eb.sign_with_keys(keys)?;
        client.send_event(&evt).await?;
        Ok(())
    }

    fn handle_notification(
        notifier: &broadcast::Sender<SdkEvent>,
        result: &Option<ResponseResult>,
        error: &Option<NIP47Error>,
    ) -> Result<()> {
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
        notifier.send(event)?;
        Ok(())
    }
}

#[sdk_macros::async_trait]
impl NWCService for BreezNWCService<BreezRelayMessageHandler> {
    async fn get_connection_string(&self) -> String {
        self.nwc_uri.to_string()
    }

    fn start(
        &self,
        mut shutdown_receiver: watch::Receiver<()>,
        mut listener: broadcast::Receiver<SdkEvent>,
        notifier: broadcast::Sender<SdkEvent>,
    ) {
        let client = self.client.clone();
        let handler = self.handler.clone();
        let our_keys = self.keys.clone();
        let client_keys = Keys::new(self.nwc_uri.secret.clone());

        let handle = platform_spawn(async move {
            client.connect().await;

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

            if let Err(err) = Self::send_event(
                EventBuilder::new(Kind::WalletConnectInfo, content),
                &our_keys,
                client.clone(),
            )
            .await
            {
                warn!("Could not send info event to relay pool: {err:?}");
            }

            let sub_id = match client
                .subscribe(
                    Filter {
                        authors: Some(BTreeSet::from([client_keys.public_key()])),
                        kinds: Some(BTreeSet::from([Kind::WalletConnectRequest])),
                        ..Default::default()
                    },
                    None,
                )
                .await
            {
                Ok(sub_id) => sub_id,
                Err(err) => {
                    warn!("Could not subscribe to relay notifications: {err:?}");
                    return;
                }
            };

            let mut notifications_listener = client.notifications();
            loop {
                tokio::select! {
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting NWC service loop");
                        client.disconnect().await;
                        return;
                    }

                    Ok(SdkEvent::PaymentSucceeded { details }) = listener.recv() => {
                        let (invoice, description, preimage, payment_hash) = match &details.details {
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
                                warn!("Payment details not available for NWC notification");
                                continue;
                            }
                        };

                        let payment_notification = PaymentNotification {
                            transaction_type: Some(
                                if details.payment_type == crate::model::PaymentType::Send {
                                    TransactionType::Outgoing
                                } else {
                                    TransactionType::Incoming
                                }
                            ),
                            invoice,
                            description: Some(description),
                            description_hash: None,
                            preimage,
                            payment_hash,
                            amount: details.amount_sat * 1000,
                            fees_paid: details.fees_sat * 1000,
                            created_at: Timestamp::from_secs(details.timestamp as u64),
                            expires_at: None,
                            settled_at: Timestamp::from_secs(details.timestamp as u64),
                            metadata: None,
                        };

                        let notification = if details.payment_type == crate::model::PaymentType::Send {
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
                                continue;
                            }
                        };

                        let encrypted_content = match encrypt(
                            our_keys.secret_key(),
                            &client_keys.public_key(),
                            &notification_content,
                        ){
                            Ok(encrypted) => encrypted,
                            Err(e) => {
                                warn!("Could not encrypt notification content: {e:?}");
                                continue;
                            }
                        };

                        let eb = EventBuilder::new(Kind::Custom(23196), encrypted_content)
                            .tags([
                                Tag::public_key(client_keys.public_key())
                            ]);

                        if let Err(e) = Self::send_event(eb, &our_keys, client.clone()).await {
                            warn!("Could not send notification event to relay: {e:?}");
                        }else {
                            info!("Sent payment notification to relay");
                        }
                    }

                    Ok(notification) = notifications_listener.recv() => {
                        let RelayPoolNotification::Event { event, subscription_id, .. } = notification else {
                            continue;
                        };
                        if subscription_id != *sub_id.id() {
                            continue;
                        }

                        info!("Received NWC notification: {event:?}");
                        // Verify event pubkey matches expected pubkey
                        if event.pubkey != client_keys.public_key() {
                            warn!("Event pubkey mismatch: expected {}, got {}",
                                  client_keys.public_key(), event.pubkey);
                            continue;
                        }

                        // Verify the event signature and event id
                        if let Err(e) = event.verify() {
                            warn!("Event signature verification failed: {e:?}");
                            continue;
                        }

                        // Decrypt the event content
                        let decrypted_content = match decrypt(
                            our_keys.secret_key(),
                            &client_keys.public_key(),
                            &event.content
                        ) {
                            Ok(content) => content,
                            Err(e) => {
                                warn!("Failed to decrypt event content: {e:?}");
                                continue;
                            }
                        };

                        info!("Decrypted NWC notification: {decrypted_content}");

                        let req = match serde_json::from_str::<Request>(&decrypted_content) {
                            Ok(r) => r,
                            Err(e) => {
                                warn!("Received unexpected request from relay pool: {decrypted_content} err {e:?}");
                                continue;
                            }
                        };

                        let (result, error) = match req.params {
                            RequestParams::PayInvoice(req) => match handler.pay_invoice(req).await {
                                Ok(res) => (Some(ResponseResult::PayInvoice(res)), None),
                                Err(e) => (None, Some(e))
                            },
                            RequestParams::ListTransactions(req) => match handler.list_transactions(req).await {
                                Ok(res) => (Some(ResponseResult::ListTransactions(res)), None),
                                Err(e) => (None, Some(e))
                            }
                            RequestParams::GetBalance => match handler.get_balance().await {
                                Ok(res) => (Some(ResponseResult::GetBalance(res)), None),
                                Err(e) => (None, Some(e))
                            }
                            _ => {
                                info!("Received unhandled request: {req:?}");
                                continue;
                            }
                        };

                        let _ = Self::handle_notification(&notifier, &result, &error);

                        let content = match serde_json::to_string(&Response {
                            result_type: req.method,
                            result,
                            error
                        }) {
                            Ok(c) => c,
                            Err(e) => {
                                warn!("Could not serialize Nostr response: {e:?}");
                                continue;
                            }
                        };
                        info!("NWC Response content: {content}");
                        info!("encrypting NWC response");
                        let encrypted_content = match encrypt(
                            our_keys.secret_key(),
                            &client_keys.public_key(),
                            &content
                        ) {
                            Ok(encrypted) => encrypted,
                            Err(e) => {
                                warn!("Could not encrypt response content: {e:?}");
                                continue;
                            }
                        };

                        let eb = EventBuilder::new(Kind::WalletConnectResponse, encrypted_content)
                            .tags([
                                Tag::event(event.id),
                                Tag::public_key(client_keys.public_key()),
                            ]);
                        if let Err(e) = Self::send_event(eb, &our_keys, client.clone()).await {
                            warn!("Could not send response event to relay pool: {e:?}");
                        }
                        info!("sent encrypted NWC response");
                    },
                }
            }
        });

        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        let _ = self.event_loop_handle.set(handle);
    }

    async fn stop(self) {
        self.client.disconnect().await;
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
    }
}
