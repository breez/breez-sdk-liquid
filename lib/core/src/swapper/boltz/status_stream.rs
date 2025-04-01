use std::collections::HashSet;
use std::time::Duration;

use crate::swapper::{
    boltz::BoltzSwapper, ProxyUrlFetcher, SubscriptionHandler, SwapperStatusStream,
};
use anyhow::Result;
use boltz_client::boltz::{
    self,
    tokio_tungstenite_wasm::{Message, WebSocketStream},
    WsRequest, WsResponse,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use sdk_common::utils::Arc;
use tokio::sync::{broadcast, watch};
use tokio_with_wasm::alias as tokio;

impl<P: ProxyUrlFetcher> BoltzSwapper<P> {
    async fn send_subscription(
        &self,
        swap_id: String,
        sender: &mut SplitSink<WebSocketStream, Message>,
    ) {
        info!("Subscribing to status updates for swap ID {swap_id}");

        let subscription = WsRequest::subscribe_swap_request(&swap_id);
        match serde_json::to_string(&subscription) {
            Ok(subscribe_json) => match sender.send(Message::Text(subscribe_json.into())).await {
                Ok(_) => info!("Subscribed"),
                Err(e) => error!("Failed to subscribe to {swap_id}: {e:?}"),
            },
            Err(e) => error!("Invalid subscription msg: {e:?}"),
        }
    }
}

impl<P: ProxyUrlFetcher> SwapperStatusStream for BoltzSwapper<P> {
    fn start(
        self: Arc<Self>,
        callback: Box<dyn SubscriptionHandler>,
        mut shutdown: watch::Receiver<()>,
    ) {
        let keep_alive_ping_interval = Duration::from_secs(15);
        let reconnect_delay = Duration::from_secs(2);

        let swapper = Arc::clone(&self);
        tokio::spawn(async move {
            loop {
                debug!("Start of ws stream loop");
                let client = match swapper.get_boltz_client().await {
                    Ok(client) => client,
                    Err(e) => {
                        warn!("Failed to get swapper client: {e:?}");
                        tokio::time::sleep(reconnect_delay).await;
                        continue;
                    }
                };
                match client.inner.connect_ws().await {
                    Ok(ws_stream) => {
                        let (mut sender, mut receiver) = ws_stream.split();

                        let mut tracked_swap_ids: HashSet<String> = HashSet::new();
                        let mut subscription_stream = self.subscription_notifier.subscribe();

                        callback.subscribe_swaps().await;

                        let mut interval = tokio::time::interval(keep_alive_ping_interval);
                        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
                        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

                        loop {
                            tokio::select! {
                                _ = shutdown.changed() => {
                                    info!("Received shutdown signal, exiting Status Stream loop");
                                    return;
                                },

                                _ = interval.tick() => {
                                    match serde_json::to_string(&WsRequest::Ping) {
                                        Ok(ping_msg) => {
                                            match sender.send(Message::Text(ping_msg.into())).await {
                                                Ok(_) => debug!("Sent keep-alive ping"),
                                                Err(e) => warn!("Failed to send keep-alive ping: {e:?}"),
                                            }
                                        },
                                        Err(e) => error!("Failed to serialize ping message: {e:?}"),
                                    }
                                },


                                swap_res = subscription_stream.recv() => match swap_res {
                                    Ok(swap_id) => {
                                      if !tracked_swap_ids.contains(&swap_id) {
                                        self.send_subscription(swap_id.clone(), &mut sender).await;
                                        tracked_swap_ids.insert(swap_id.clone());
                                      }
                                    },
                                    Err(e) => error!("Received error on subscription stream: {e:?}"),
                                },

                                maybe_next = receiver.next() => match maybe_next {
                                    Some(msg) => match msg {
                                        Ok(Message::Close(_)) => {
                                            warn!("Received close msg, exiting socket loop");
                                            tokio::time::sleep(reconnect_delay).await;
                                            break;
                                        },
                                        Ok(Message::Text(payload)) => {
                                            let payload = payload.as_str();
                                            info!("Received text msg: {payload:?}");
                                            match serde_json::from_str::<WsResponse>(payload) {
                                                // Subscribing/unsubscribing confirmation
                                                Ok(WsResponse::Subscribe { .. }) | Ok(WsResponse::Unsubscribe { .. }) => {}

                                                // Status update(s)
                                                Ok(WsResponse::Update(update)) => {
                                                    for update in update.args {
                                                        let _ = self.update_notifier.send(update);
                                                    }
                                                }

                                                // A response to one of our pings
                                                Ok(WsResponse::Pong) => debug!("Received pong"),

                                                // Either an invalid response, or an error related to subscription
                                                Err(e) => error!("Failed to parse websocket response: {e:?} - response: {payload}"),
                                            }
                                        },
                                        Ok(msg) => warn!("Unhandled msg: {msg:?}"),
                                        Err(e) => {
                                            error!("Received stream error: {e:?}");
                                            let _ = sender.close().await;
                                            break;
                                        }
                                    },
                                    None => {
                                        warn!("Received nothing from the stream");
                                        let _ = sender.close().await;
                                        tokio::time::sleep(reconnect_delay).await;
                                        break;
                                    },
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error connecting to stream: {e:?}");
                        tokio::time::sleep(reconnect_delay).await;
                    }
                }
            }
        });
    }

    fn track_swap_id(&self, swap_id: &str) -> Result<()> {
        let _ = self.subscription_notifier.send(swap_id.to_string());
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::SwapStatus> {
        self.update_notifier.subscribe()
    }
}
