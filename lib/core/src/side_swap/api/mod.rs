use anyhow::Result;
use futures_util::{SinkExt as _, StreamExt};
use log::{error, info, warn};
use maybe_sync::{MaybeSend, MaybeSync};
use notifications::SideSwapNotificationsHandler;
use request_handler::SideSwapRequestHandler;
use response_handler::SideSwapResponseHandler;
use sdk_common::utils::Arc;
use sideswap_api::Request;
use sideswap_api::{RequestId, RequestMessage, ResponseMessage};
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::watch;

use boltz_client::boltz::tokio_tungstenite_wasm;
use tokio_tungstenite_wasm::{connect, Message, WebSocketStream};

use crate::wallet::OnchainWallet;

mod notifications;
mod request_handler;
mod response_handler;

pub const SIDESWAP_MAINNET_URL: &str = "wss://api.sideswap.io/json-rpc-ws";
pub const SIDESWAP_TESTNET_URL: &str = "wss://api-testnet.sideswap.io/json-rpc-ws";

#[sdk_macros::async_trait]
pub trait SideSwapStream: MaybeSend + MaybeSync {
    fn start(self: Arc<Self>, shutdown: watch::Receiver<()>);
}

pub(crate) struct SideSwapService {
    url: String,
    onchain_wallet: Arc<dyn OnchainWallet>,
    request_handler: SideSwapRequestHandler,
    response_handler: SideSwapResponseHandler,
    notifications_handler: SideSwapNotificationsHandler,
}

impl SideSwapService {
    pub(crate) fn new(url: String, onchain_wallet: Arc<dyn OnchainWallet>) -> Self {
        Self {
            url,
            request_handler: SideSwapRequestHandler::new(),
            response_handler: SideSwapResponseHandler::new(),
            notifications_handler: SideSwapNotificationsHandler::new(),
            onchain_wallet,
        }
    }

    async fn connect_ws(&self) -> Result<WebSocketStream, tokio_tungstenite_wasm::Error> {
        connect(&self.url).await
    }

    pub(crate) async fn handle_message(&self, msg: &str, resp_sender: &UnboundedSender<i64>) {
        info!("Received text message: {msg:?}");
        match serde_json::from_str::<ResponseMessage>(msg) {
            Ok(ResponseMessage::Response(req_id, res)) => {
                self.response_handler
                    .handle_response(req_id.clone(), res)
                    .await;
                if let Some(RequestId::Int(req_id)) = req_id {
                    resp_sender.send(req_id);
                }
            }
            Ok(ResponseMessage::Notification(_notif)) => todo!(),
            // Either an invalid response, or an error
            Err(e) => error!("Failed to parse websocket response: {e:?} - response: {msg}"),
        }
    }
}

#[sdk_macros::async_trait]
impl SideSwapStream for SideSwapService {
    fn start(self: Arc<Self>, mut shutdown: watch::Receiver<()>) {
        let keep_alive_ping_interval = Duration::from_secs(15);
        let reconnect_delay = Duration::from_secs(2);

        let (resp_sender, resp_receiver) = unbounded_channel::<i64>();
        let (req_sender, mut req_receiver) = unbounded_channel::<RequestMessage>();

        self.response_handler.start(resp_receiver);
        if let Err(err) = self.request_handler.start(req_sender) {
            error!("Could not start SideSwap service: {err}");
            return;
        };

        let cloned = Arc::clone(&self);
        tokio::spawn(async move {
            loop {
                let ws = match cloned.connect_ws().await {
                    Ok(ws) => ws,
                    Err(err) => {
                        warn!("Could not connect to SideSwap API: {err:?}");
                        tokio::time::sleep(reconnect_delay).await;
                        continue;
                    }
                };

                let (mut ws_sender, mut ws_receiver) = ws.split();
                loop {
                    tokio::select! {
                        _ = shutdown.changed() => {
                            info!("Received shutdown signal, exiting Status Stream loop");
                            return;
                        },

                        _ = tokio::time::sleep(keep_alive_ping_interval) => cloned.request_handler.send_ws(
                            &mut ws_sender,
                            RequestMessage::Request(RequestId::String("ping".to_string()), Request::Ping(None))
                        ).await,

                        maybe_req_msg = req_receiver.recv() => match maybe_req_msg {
                            Some(req_msg) => cloned.request_handler.send_ws(&mut ws_sender, req_msg).await,
                            None => {
                                warn!("Request channel has been closed, exiting socket loop");
                                break;
                            }
                        },

                        maybe_next = ws_receiver.next() => match maybe_next {
                            Some(msg) => match msg {
                                Ok(Message::Close(_)) => {
                                    warn!("Received close msg, exiting socket loop");
                                    tokio::time::sleep(reconnect_delay).await;
                                    break;
                                },
                                Ok(Message::Text(payload)) => cloned.handle_message(payload.as_str(), &resp_sender).await,
                                Ok(msg) => warn!("Unhandled msg: {msg:?}"),
                                Err(e) => {
                                    error!("Received stream error: {e:?}");
                                    let _ = ws_sender.close().await;
                                    break;
                                }
                            }
                            None => {
                                warn!("Received nothing from the stream");
                                let _ = ws_sender.close().await;
                                tokio::time::sleep(reconnect_delay).await;
                                break;
                            },
                        }
                    }
                }
            }
        });
    }
}
