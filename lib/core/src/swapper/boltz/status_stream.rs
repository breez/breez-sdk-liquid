use std::collections::HashSet;
use std::time::Duration;

use crate::swapper::{
    boltz::BoltzSwapper, ProxyUrlFetcher, SubscriptionHandler, SwapperStatusStream,
};
use anyhow::{anyhow, Result};
use boltz_client::boltz::{
    self,
    tokio_tungstenite_wasm::{Message, WebSocketStream},
    InvoiceRequestParams, WsRequest, WsResponse,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use sdk_common::utils::Arc;
use tokio::sync::{broadcast, watch};
use tokio_with_wasm::alias as tokio;

impl<P: ProxyUrlFetcher> BoltzSwapper<P> {
    async fn send_request(
        &self,
        ws_request: WsRequest,
        sender: &mut SplitSink<WebSocketStream, Message>,
    ) {
        debug!("Sending request: {ws_request:?}");
        match serde_json::to_string(&ws_request) {
            Ok(ref req_json) => match sender.send(Message::Text(req_json.into())).await {
                Ok(_) => debug!("Sent request: {req_json}"),
                Err(e) => error!("Failed to send {req_json}: {e:?}"),
            },
            Err(e) => error!("Error encoding request: {e:?}"),
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

                        let mut tracked_ids: HashSet<String> = HashSet::new();
                        let mut request_stream = self.request_notifier.subscribe();

                        callback.track_subscriptions().await;

                        loop {
                            tokio::select! {
                                _ = shutdown.changed() => {
                                    info!("Received shutdown signal, exiting Status Stream loop");
                                    return;
                                },

                                _ = tokio::time::sleep(keep_alive_ping_interval) => {
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

                                ws_request_res = request_stream.recv() => match ws_request_res {
                                    Ok(WsRequest::Subscribe(subscribe)) => {
                                        let id = match subscribe.clone() {
                                            boltz::SubscribeRequest::SwapUpdate { args } => args.first().cloned(),
                                            boltz::SubscribeRequest::InvoiceRequest { args } => args.first().map(|p| p.offer.clone()),
                                        };
                                        if let Some(id) = id {
                                            if !tracked_ids.contains(&id) {
                                                self.send_request(WsRequest::Subscribe(subscribe), &mut sender).await;
                                                tracked_ids.insert(id);
                                            }
                                        }
                                    },
                                    Ok(ws_request) => self.send_request(ws_request, &mut sender).await,
                                    Err(e) => error!("Received error on request stream: {e:?}"),
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
                                            debug!("Received text msg: {payload:?}");
                                            match serde_json::from_str::<WsResponse>(payload) {
                                                // Subscribing/unsubscribing confirmation
                                                Ok(WsResponse::Subscribe { .. }) | Ok(WsResponse::Unsubscribe { .. }) => {}

                                                // Swap status update(s)
                                                Ok(WsResponse::Update(update)) => {
                                                    for update in update.args {
                                                        let _ = self.update_notifier.send(update);
                                                    }
                                                }

                                                // Invoice requests(s)
                                                Ok(WsResponse::InvoiceRequest(invoice_request)) => {
                                                    for invoice_request in invoice_request.args {
                                                        let _ = self.invoice_request_notifier.send(invoice_request);
                                                    }
                                                }

                                                // Error response
                                                Ok(WsResponse::Error(error)) => {
                                                    error!("Received error msg: {error:?}");
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
        match self.request_notifier.send(WsRequest::Subscribe(
            boltz::SubscribeRequest::SwapUpdate {
                args: vec![swap_id.to_string()],
            },
        )) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send subscribe swap: {e:?}");
                Err(anyhow!("Failed to send subscribe swap: {e:?}"))
            }
        }
    }

    fn track_offer(&self, offer: &str, signature: &str) -> Result<()> {
        match self.request_notifier.send(WsRequest::Subscribe(
            boltz::SubscribeRequest::InvoiceRequest {
                args: vec![InvoiceRequestParams {
                    offer: offer.to_string(),
                    signature: signature.to_string(),
                }],
            },
        )) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send subscribe offer: {e:?}");
                Err(anyhow!("Failed to send subscribe offer: {e:?}"))
            }
        }
    }

    fn send_invoice_created(&self, id: &str, invoice: &str) -> Result<()> {
        debug!("Sending invoice created: id: {id}");
        match self
            .request_notifier
            .send(WsRequest::Invoice(boltz::InvoiceCreated {
                id: id.to_string(),
                invoice: invoice.to_string(),
            })) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send invoice created: {e:?}");
                Err(anyhow!("Failed to send invoice created: {e:?}"))
            }
        }
    }

    fn send_invoice_error(&self, id: &str, error: &str) -> Result<()> {
        match self
            .request_notifier
            .send(WsRequest::InvoiceError(boltz::InvoiceError {
                id: id.to_string(),
                error: error.to_string(),
            })) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send invoice error: {e:?}");
                Err(anyhow!("Failed to send invoice error: {e:?}"))
            }
        }
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::SwapStatus> {
        self.update_notifier.subscribe()
    }

    fn subscribe_invoice_requests(&self) -> broadcast::Receiver<boltz::InvoiceRequest> {
        self.invoice_request_notifier.subscribe()
    }
}
