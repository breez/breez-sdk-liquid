use anyhow::Result;
use bip39::rand::{self, RngCore};
use handler::{BreezRelayMessageHandler, RelayMessageHandler};
use log::{info, warn};
use nostr_sdk::{
    nips::nip04::decrypt,
    nips::nip47::{
        ErrorCode, Method, NIP47Error, NostrWalletConnectURI, Request, RequestParams, Response, ResponseResult,
    },
    Client as NostrClient, EventBuilder, Keys, Kind, RelayPoolNotification, SecretKey,
};
use maybe_sync::{MaybeSend, MaybeSync};
// use std::sync::Arc;
use sdk_common::utils::Arc;
use tokio::sync::{broadcast, watch, OnceCell};
use tokio::task::JoinHandle;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use tokio::task::spawn as platform_spawn;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use tokio::task::spawn_local as platform_spawn;

use crate::model::{SdkEvent, NwcEvent};

pub(crate) mod handler;

#[sdk_macros::async_trait]
pub trait NWCService: MaybeSend+MaybeSync {
    /// Creates a Nostr Wallet Connect connection string for this service.
    /// 
    /// Generates a unique connection URI that external applications can use
    /// to connect to this wallet service. The URI includes the wallet's public key,
    /// relay information, and a randomly generated secret for secure communication.
    /// 
    /// # Returns
    /// * `Ok(String)` - The connection string that clients can use
    /// * `Err(anyhow::Error)` - Error generating the connection string
    async fn create_connection_string(&self) -> Result<String>;

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
    fn start(&self, shutdown_receiver: watch::Receiver<()>, notifier: broadcast::Sender<SdkEvent>);

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
    client: Arc<NostrClient>,
    handler: Arc<Handler>,
    event_loop_handle: OnceCell<JoinHandle<()>>,
    nwc_connection_string: OnceCell<NostrWalletConnectURI>,
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
    pub(crate) async fn new(
        handler: Arc<Handler>,
        relays: &[String],
    ) -> Result<Self> {
        let client = Arc::new(NostrClient::default());
        for relay in relays {
            client.add_relay(relay).await?;
        }

        Ok(Self {
            client,
            handler,
            keys: Keys::generate(),
            event_loop_handle: OnceCell::new(),
            nwc_connection_string: OnceCell::new(),
        })
    }
}

impl BreezNWCService<BreezRelayMessageHandler> {
    async fn send_event(
        eb: EventBuilder,
        keys: &Keys,
        client: Arc<NostrClient>,
    ) -> Result<(), nostr_sdk::client::Error> {
        let evt = eb.sign_with_keys(&keys)?;
        client.send_event(&evt).await?;
        Ok(())
    }

    fn handle_notification(
        notifier: &broadcast::Sender<SdkEvent>,
        result: &Option<ResponseResult>,
        error: &Option<NIP47Error>,
    ) -> Result<()> {
        let event: SdkEvent = match (result, error) {
            (Some(ResponseResult::PayInvoice(response)), None) => {
                SdkEvent::NWC {
                    details: NwcEvent::PayInvoice {
                        success: true,
                        preimage: Some(response.preimage.clone()),
                        fees_sat: response.fees_paid.map(|f| f / 1000),
                        error: None,
                    },
                }
            }
            (None, Some(error)) => {
                match error.code {
                    ErrorCode::PaymentFailed => {
                        SdkEvent::NWC {
                            details: NwcEvent::PayInvoice {
                                success: false,
                                preimage: None,
                                fees_sat: None,
                                error: Some(error.message.clone()),
                            },
                        }
                    }
                    _ => {
                        warn!("Unhandled error code: {:?}", error.code);
                        return Ok(());
                    }
                }
            }
            (Some(ResponseResult::ListTransactions(_)), None) => {
                SdkEvent::NWC {
                    details: NwcEvent::ListTransactions,
                }
            }
            (Some(ResponseResult::GetBalance(_)), None) => {
                SdkEvent::NWC {
                    details: NwcEvent::GetBalance,
                }
            }
            _ => {
                warn!("Unexpected combination");
                return Ok(());
            }
        };
        notifier.send(event);
        Ok(())
    }
}

#[sdk_macros::async_trait]
impl NWCService for BreezNWCService<BreezRelayMessageHandler> {
    async fn create_connection_string(&self) -> Result<String> {
        let connection_uri = self.nwc_connection_string.get_or_init(|| async {
            let public_key = self.keys.public_key();
            let relays = self.client.relays().await.keys().cloned().collect();

            let mut random_bytes = [0u8; 32];
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut random_bytes);
            let random_secret_key = nostr_sdk::SecretKey::from_slice(&random_bytes).unwrap();

            NostrWalletConnectURI::new(public_key, relays, random_secret_key, None)
        }).await;
        
        Ok(connection_uri.to_string())
    }

    fn start(&self, mut shutdown_receiver: watch::Receiver<()>, notifier: broadcast::Sender<SdkEvent>) {
        let client = self.client.clone();
        let handler = self.handler.clone();
        let keys = self.keys.clone();
        let nwc_connection_string = self.nwc_connection_string.clone();

        let handle = platform_spawn(async move {
            client.connect().await;

            // Broadcast info event
            let content = &[
                Method::PayInvoice,
                Method::ListTransactions,
                Method::GetBalance,
            ]
            .map(|m| m.to_string())
            .join(" ");
            if let Err(err) = Self::send_event(
                EventBuilder::new(Kind::WalletConnectInfo, content),
                &keys,
                client.clone(),
            )
            .await
            {
                warn!("Could not send info event to relay pool: {err:?}");
            }

            let mut notifications_listener = client.notifications();
            loop {
                tokio::select! {
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting NWC service loop");
                        client.disconnect().await;
                        return;
                    }

                    Ok(notification) = notifications_listener.recv() => {
                        let RelayPoolNotification::Event { event, .. } = notification else {
                            continue;
                        };

                        let connection_uri = match nwc_connection_string.get() {
                            Some(uri) => uri,
                            None => {
                                warn!("NWC connection not initialized, ignoring event");
                                continue;
                            }
                        };

                        let client_keys = Keys::new(connection_uri.secret.clone());
                        
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
                            &connection_uri.secret, 
                            &event.pubkey, 
                            &event.content
                        ) {
                            Ok(content) => content,
                            Err(e) => {
                                warn!("Failed to decrypt event content: {e:?}");
                                continue;
                            }
                        };

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

                        if let Err(e) = Self::send_event(EventBuilder::new(Kind::WalletConnectResponse, content), &keys, client.clone()).await {
                            warn!("Could not send response event to relay pool: {e:?}");
                        }
                    },
                }
            }
        });

        let _ = self.event_loop_handle.set(handle);
    }

    async fn stop(self) {
        self.client.disconnect().await;
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
    }
}
