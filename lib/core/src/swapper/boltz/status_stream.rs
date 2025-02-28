use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz::{self, Subscription, SwapUpdate};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, watch};
use tokio::time::MissedTickBehavior;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::model::Config;
use crate::swapper::{SubscriptionHandler, SwapperStatusStream};

use super::{split_proxy_url, ProxyUrlFetcher};

pub(crate) struct BoltzStatusStream {
    config: Config,
    proxy_url: Arc<dyn ProxyUrlFetcher>,
    subscription_notifier: broadcast::Sender<String>,
    update_notifier: broadcast::Sender<boltz::Update>,
}

impl BoltzStatusStream {
    pub(crate) fn new(config: Config, proxy_url: Arc<dyn ProxyUrlFetcher>) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        let (update_notifier, _) = broadcast::channel::<boltz::Update>(30);

        Self {
            config,
            proxy_url,
            subscription_notifier,
            update_notifier,
        }
    }

    async fn connect(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let default_url = self.config.default_boltz_url().to_string();
        let url = match self.proxy_url.fetch().await {
            Ok(Some(url)) => split_proxy_url(url).0.unwrap_or(default_url),
            _ => default_url,
        };
        let url = url.replace("http", "ws") + "/ws";
        let (socket, _) = connect_async(Url::parse(&url)?)
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

impl SwapperStatusStream for BoltzStatusStream {
    fn track_swap_id(&self, swap_id: &str) -> Result<()> {
        let _ = self.subscription_notifier.send(swap_id.to_string());
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::Update> {
        self.update_notifier.subscribe()
    }

    fn start(
        self: Arc<Self>,
        callback: Box<dyn SubscriptionHandler>,
        mut shutdown: watch::Receiver<()>,
    ) {
        let keep_alive_ping_interval = Duration::from_secs(15);
        let reconnect_delay = Duration::from_secs(2);

        tokio::spawn(async move {
            loop {
                debug!("Start of ws stream loop");
                match self.connect().await {
                    Ok(mut ws_stream) => {
                        let mut tracked_swap_ids: HashSet<String> = HashSet::new();
                        let mut subscription_stream = self.subscription_notifier.subscribe();

                        callback.subscribe_swaps().await;

                        let mut interval = tokio::time::interval(keep_alive_ping_interval);
                        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

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
                                    Ok(swap_id) => {
                                      if !tracked_swap_ids.contains(&swap_id) {
                                        self.send_subscription(swap_id.clone(), &mut ws_stream).await;
                                        tracked_swap_ids.insert(swap_id.clone());
                                      }
                                    },
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
