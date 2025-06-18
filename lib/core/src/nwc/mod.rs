use anyhow::Result;
use bip39::rand::{self, RngCore};
use handler::{BreezRelayMessageHandler, RelayMessageHandler};
use log::{info, warn};
use nostr_sdk::{
    nips::nip47::{
        Method, NostrWalletConnectURI, Request, RequestParams, Response, ResponseResult,
    },
    Client as NostrClient, EventBuilder, Keys, Kind, RelayPoolNotification, SecretKey,
};
use std::sync::Arc;
use tokio::sync::watch;
use tokio::{sync::OnceCell, task::JoinHandle};

use crate::sdk::LiquidSdk;

mod handler;

#[sdk_macros::async_trait]
pub trait NWCService {
    async fn create_connection_string(&self) -> Result<String>;
    fn start(&self, shutdown_receiver: watch::Receiver<()>) -> Result<()>;
    async fn stop(self);
}

pub struct BreezNWCService<Handler: RelayMessageHandler> {
    keys: Keys,
    client: Arc<NostrClient>,
    handler: Arc<Handler>,
    event_loop_handle: OnceCell<JoinHandle<()>>,
}

impl BreezNWCService<BreezRelayMessageHandler> {
    pub(crate) async fn new(
        secret_key: SecretKey,
        sdk: Arc<LiquidSdk>,
        relays: &[String],
    ) -> Result<Self> {
        let client = Arc::new(NostrClient::default());
        for relay in relays {
            client.add_relay(relay).await?;
        }

        let handler = Arc::new(BreezRelayMessageHandler::new(sdk));
        Ok(Self {
            client,
            handler,
            keys: Keys::new(secret_key),
            event_loop_handle: OnceCell::new(),
        })
    }

    async fn send_event(
        eb: EventBuilder,
        keys: &Keys,
        client: Arc<NostrClient>,
    ) -> Result<(), nostr_sdk::client::Error> {
        let evt = eb.sign_with_keys(&keys)?;
        client.send_event(&evt).await?;
        Ok(())
    }
}

#[sdk_macros::async_trait]
impl NWCService for BreezNWCService<BreezRelayMessageHandler> {
    async fn create_connection_string(&self) -> Result<String> {
        let public_key = self.keys.public_key();
        let relays = self.client.relays().await.keys().cloned().collect();

        let mut random_bytes = [0u8; 32];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut random_bytes);
        let random_secret_key = nostr_sdk::SecretKey::from_slice(&random_bytes)?;

        Ok(NostrWalletConnectURI::new(public_key, relays, random_secret_key, None).to_string())
    }

    fn start(&self, mut shutdown_receiver: watch::Receiver<()>) -> Result<()> {
        let client = self.client.clone();
        let handler = self.handler.clone();
        let keys = self.keys.clone();

        let handle = tokio::spawn(async move {
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

                        let req = match serde_json::from_str::<Request>(&event.content) {
                            Ok(r) => r,
                            Err(e) => {
                                warn!("Received unexpected request from relay pool: {event:?} err {e:?}");
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
        Ok(())
    }

    async fn stop(self) {
        self.client.disconnect().await;
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
    }
}
