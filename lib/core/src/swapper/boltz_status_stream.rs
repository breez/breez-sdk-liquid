use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::swaps::boltzv2::Subscription;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, watch};
use tokio::time::MissedTickBehavior;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::model::{SwapUpdate, Update};

use super::{ReconnectHandler, SwapperStatusStream};

pub(crate) struct BoltzStatusStream {
    url: String,
    subscription_notifier: broadcast::Sender<String>,
    update_notifier: broadcast::Sender<Update>,
}

impl BoltzStatusStream {
    pub(crate) fn new(url: &str) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        let (update_notifier, _) = broadcast::channel::<Update>(30);

        Self {
            url: url.replace("http", "ws") + "/ws",
            subscription_notifier,
            update_notifier,
        }
    }

    async fn connect(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let (socket, _) = connect_async(Url::parse(&self.url)?)
            .await
            .map_err(|e| anyhow!("Failed to connect to websocket: {e:?}"))?;
        Ok(socket)
    }

    async fn send_subscription(
        &self,
        swap_id: String,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) {
        info!("Subscribing to status updates for swap ID {swap_id}");

        let subscription = Subscription::new(&swap_id);
        match serde_json::to_string(&subscription) {
            Ok(subscribe_json) => match ws_stream.send(Message::Text(subscribe_json)).await {
                Ok(_) => info!("Subscribed"),
                Err(e) => error!("Failed to subscribe to {swap_id}: {e:?}"),
            },
            Err(e) => error!("Invalid subscription msg: {e:?}"),
        }
    }
}

#[async_trait]
impl SwapperStatusStream for BoltzStatusStream {
    fn track_swap_id(&self, swap_id: &str) -> Result<()> {
        let _ = self.subscription_notifier.send(swap_id.to_string());
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<Update> {
        self.update_notifier.subscribe()
    }

    async fn start(
        self: Arc<Self>,
        callback: Box<dyn ReconnectHandler>,
        mut shutdown: watch::Receiver<()>,
    ) {
        let keep_alive_ping_interval = Duration::from_secs(15);
        let reconnect_delay = Duration::from_secs(2);

        tokio::spawn(async move {
            loop {
                debug!("Start of ws stream loop");
                match self.connect().await {
                    Ok(mut ws_stream) => {
                        callback.on_stream_reconnect().await;

                        let mut interval = tokio::time::interval(keep_alive_ping_interval);
                        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

                        let mut subscription_stream = self.subscription_notifier.subscribe();

                        loop {
                            tokio::select! {
                                _ = shutdown.changed() => {
                                    info!("Received shutdown signal, exiting Status Stream loop");
                                    return;
                                },

                                _ = interval.tick() => {
                                    match ws_stream.send(Message::Ping(vec![])).await {
                                        Ok(_) => debug!("Sent keep-alive ping"),
                                        Err(e) => warn!("Failed to send keep-alive ping: {e:?}"),
                                    }
                                },

                                swap_res = subscription_stream.recv() => match swap_res {
                                    Ok(swap_id) => self.send_subscription(swap_id, &mut ws_stream).await,
                                    Err(e) => error!("Received error on subscription stream: {e:?}"),
                                },

                                maybe_next = ws_stream.next() => match maybe_next {
                                    Some(msg) => match msg {
                                        Ok(Message::Close(_)) => {
                                            warn!("Received close msg, exiting socket loop");
                                            tokio::time::sleep(reconnect_delay).await;
                                            break;
                                        },
                                        Ok(Message::Text(payload)) => {
                                            info!("Received text msg: {payload:?}");
                                            match serde_json::from_str::<SwapUpdate>(&payload) {
                                                // Subscription confirmation
                                                Ok(SwapUpdate::Subscription { .. }) => {}

                                                // Status update(s)
                                                Ok(SwapUpdate::Update {
                                                    args,
                                                    ..
                                                }) => {
                                                    for update in args {
                                                        let _ = self.update_notifier.send(update);
                                                    }
                                                }

                                                // Error related to subscription, like "Unknown swap ID"
                                                Ok(SwapUpdate::Error {
                                                    args,
                                                    ..
                                                }) => error!("Received a status update error: {args:?}"),

                                                Err(e) => warn!("WS response is invalid SwapUpdate: {e:?}"),
                                            }
                                        },
                                        Ok(Message::Ping(_)) => debug!("Received ping"),
                                        Ok(Message::Pong(_)) => debug!("Received pong"),
                                        Ok(msg) => warn!("Unhandled msg: {msg:?}"),
                                        Err(e) => {
                                            error!("Received stream error: {e:?}");
                                            let _ = ws_stream.close(None).await;
                                            break;
                                        }
                                    },
                                    None => {
                                        warn!("Received nothing from the stream");
                                        let _ = ws_stream.close(None).await;
                                        tokio::time::sleep(reconnect_delay).await;
                                        break;
                                    },
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error connecting to stream: {e}");
                        tokio::time::sleep(reconnect_delay).await;
                    }
                }
            }
        });
    }
}
