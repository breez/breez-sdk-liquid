use anyhow::{anyhow, bail, Result};
use base64::Engine;
use futures_util::{SinkExt as _, StreamExt};
use log::{debug, error, info, warn};
use maybe_sync::{MaybeSend, MaybeSync};
use request_handler::SideSwapRequestHandler;
use response_handler::SideSwapResponseHandler;
use sdk_common::bitcoin::hashes::hex::ToHex as _;
use sdk_common::prelude::RestClient;
use sdk_common::utils::Arc;
use sideswap_api::http_rpc::{
    SwapSignRequest, SwapSignResponse, SwapStartRequest, SwapStartResponse,
};
use sideswap_api::{
    Notification, Request, RequestId, Response, StartSwapWebRequest,
    SubscribePriceStreamRequest, SubscribePriceStreamResponse, UnsubscribePriceStreamRequest,
    WrappedRequest, WrappedResponse,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_with_wasm::alias as tokio;

use boltz_client::boltz::tokio_tungstenite_wasm;
use tokio_tungstenite_wasm::{connect, Message};

use crate::bitcoin::base64;
use crate::elements::{self, pset::PartiallySignedTransaction, AssetId};
use crate::model::{AssetSwap, Config};
use crate::wallet::OnchainWallet;

mod request_handler;
mod response_handler;

pub const SIDESWAP_MAINNET_URL: &str = "wss://api.sideswap.io/json-rpc-ws";
pub const SIDESWAP_TESTNET_URL: &str = "wss://api-testnet.sideswap.io/json-rpc-ws";
pub const SIDESWAP_REGTEST_URL: &str = "wss://api-regtest.sideswap.io/json-rpc-ws";

#[sdk_macros::async_trait]
pub trait SideSwapService: MaybeSend + MaybeSync {
    async fn start(self: Arc<Self>) -> Result<()>;
    async fn stop(&self);
    async fn subscribe_price_stream(
        &self,
        asset_id: AssetId,
        send_amount_sat: u64,
    ) -> Result<AssetSwap>;
    async fn unsubscribe_price_stream(&self) -> Result<()>;
    async fn get_current_price(&self) -> Option<AssetSwap>;
    async fn execute_swap(&self) -> Result<ExecuteSwapResponse>;
}

pub struct ExecuteSwapResponse {
    pub recv_address: String,
    pub tx_id: String,
}

struct ShutdownChannel {
    pub(crate) sender: Sender<()>,
    pub(crate) receiver: Mutex<Receiver<()>>,
}

impl ShutdownChannel {
    pub(crate) fn new((sender, receiver): (Sender<()>, Receiver<()>)) -> Self {
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

pub(crate) struct HybridSideSwapService {
    config: Config,
    is_started: Mutex<bool>,
    shutdown: ShutdownChannel,
    rest_client: Arc<dyn RestClient>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    request_handler: SideSwapRequestHandler,
    response_handler: SideSwapResponseHandler,
    ongoing_swap: Mutex<Option<AssetSwap>>,
}

impl HybridSideSwapService {
    pub(crate) fn new(
        config: Config,
        rest_client: Arc<dyn RestClient>,
        onchain_wallet: Arc<dyn OnchainWallet>,
    ) -> Self {
        Self {
            config,
            is_started: Mutex::new(false),
            shutdown: ShutdownChannel::new(tokio::sync::mpsc::channel::<()>(10)),
            request_handler: SideSwapRequestHandler::new(),
            response_handler: SideSwapResponseHandler::new(),
            ongoing_swap: Mutex::new(None),
            rest_client,
            onchain_wallet,
        }
    }

    async fn set_started(&self, is_started: bool) {
        let mut lock = self.is_started.lock().await;
        *lock = is_started;
    }

    async fn update_ongoing_swap(&self, res: &SubscribePriceStreamResponse) -> Result<AssetSwap> {
        let asset_swap = AssetSwap::try_from_price_stream_response(self.config.network, res)?;
        let mut lock = self.ongoing_swap.lock().await;
        *lock = Some(asset_swap.clone());
        Ok(asset_swap)
    }

    async fn unset_ongoing_swap(&self) {
        let mut lock = self.ongoing_swap.lock().await;
        *lock = None;
    }

    async fn handle_message(&self, msg: &str) {
        info!("Received text message: {msg:?}");
        match serde_json::from_str::<WrappedResponse>(msg) {
            Ok(WrappedResponse::Response { id, response, .. }) => {
                self.response_handler.handle_response(id, response).await;
            }
            Ok(WrappedResponse::Notification { notification, .. }) => match notification {
                Notification::UpdatePriceStream(res) => {
                    if let Err(err) = self.update_ongoing_swap(&res).await {
                        warn!("Could not update ongoing swap: {err:?}");
                    }
                }
                notif => debug!("Received unhandled notification from SideSwap service: {notif:?}"),
            },
            // Either an invalid response, or an error
            Err(e) => error!("Failed to parse websocket response: {e:?} - response: {msg}"),
        }
    }

    fn invalid_response(res: Response) -> anyhow::Error {
        anyhow!("Received invalid response from the server: {res:?}")
    }
}

#[sdk_macros::async_trait]
impl SideSwapService for HybridSideSwapService {
    async fn start(self: Arc<Self>) -> Result<()> {
        if *self.is_started.lock().await {
            return Ok(());
        }

        let ws = connect(self.config.sideswap_url()).await?;
        self.set_started(true).await;

        let (mut ws_sender, mut ws_receiver) = ws.split();
        let keep_alive_ping_interval = Duration::from_secs(15);

        let cloned = Arc::clone(&self);
        tokio::spawn(async move {
            let mut shutdown = cloned.shutdown.receiver.lock().await;
            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        info!("Received shutdown signal, exiting SideSwap service loop");
                        cloned.set_started(false).await;
                        return;
                    }

                    _ = tokio::time::sleep(keep_alive_ping_interval) => cloned.request_handler.send_ws(
                        &mut ws_sender,
                        WrappedRequest {
                            id: RequestId::String("ping".to_string()),
                            request: Request::Ping(None),
                        }
                    ).await,

                    maybe_req_msg = self.request_handler.recv() => match maybe_req_msg {
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
                                cloned.set_started(false).await;
                                break;
                            },
                            Ok(Message::Text(payload)) => cloned.handle_message(payload.as_str()).await,
                            Ok(msg) => warn!("Unhandled msg: {msg:?}"),
                            Err(e) => {
                                error!("Received stream error: {e:?}");
                                let _ = ws_sender.close().await;
                                cloned.set_started(false).await;
                                break;
                            }
                        }
                        None => {
                            warn!("Received nothing from the stream");
                            let _ = ws_sender.close().await;
                            cloned.set_started(false).await;
                            break;
                        },
                    }
                }
            }
        });
        Ok(())
    }

    async fn stop(&self) {
        let _ = self.shutdown.sender.send(()).await;
        self.unset_ongoing_swap().await;
    }

    async fn subscribe_price_stream(
        &self,
        asset_id: AssetId,
        send_amount_sat: u64,
    ) -> Result<AssetSwap> {
        let req = Request::SubscribePriceStream(SubscribePriceStreamRequest {
            subscribe_id: None,
            asset: asset_id,
            send_bitcoins: true,
            send_amount: Some(send_amount_sat as i64),
            recv_amount: None,
        });
        let request_id = self.request_handler.send(req).await?;
        let res = match self.response_handler.recv(request_id).await? {
            Response::SubscribePriceStream(res) => res,
            res => {
                return Err(Self::invalid_response(res));
            }
        };

        self.update_ongoing_swap(&res).await
    }

    async fn unsubscribe_price_stream(&self) -> Result<()> {
        let req =
            Request::UnsubscribePriceStream(UnsubscribePriceStreamRequest { subscribe_id: None });
        let request_id = self.request_handler.send(req).await?;
        match self.response_handler.recv(request_id).await? {
            Response::UnsubscribePriceStream(_) => {}
            res => {
                return Err(Self::invalid_response(res));
            }
        };
        self.unset_ongoing_swap().await;
        Ok(())
    }

    async fn get_current_price(&self) -> Option<AssetSwap> {
        self.ongoing_swap.lock().await.clone()
    }

    async fn execute_swap(&self) -> Result<ExecuteSwapResponse> {
        let Some(ref ongoing_swap) = *self.ongoing_swap.lock().await else {
            bail!("Cannot execute swap without subscribing to price stream first");
        };

        let req = Request::StartSwapWeb(StartSwapWebRequest {
            asset: ongoing_swap.asset.try_to_asset_id(self.config.network)?,
            price: ongoing_swap.exchange_rate,
            send_amount: ongoing_swap.payer_amount_sat as i64,
            recv_amount: ongoing_swap.receiver_amount as i64,
            send_bitcoins: true,
        });
        let request_id = self.request_handler.send(req).await?;
        let start_res = match self.response_handler.recv(request_id).await? {
            Response::StartSwapWeb(res) => Ok(res),
            res => Err(Self::invalid_response(res)),
        }?;

        let recv_addr = self.onchain_wallet.next_unused_address().await?;
        let change_addr = self.onchain_wallet.next_unused_change_address().await?;

        let body = serde_json::to_string(&SwapStartRequest {
            order_id: start_res.order_id,
            inputs: vec![],
            change_addr,
            recv_addr: recv_addr.clone(),
            send_asset: start_res.send_asset,
            send_amount: start_res.send_amount,
            recv_asset: start_res.recv_asset,
            recv_amount: start_res.recv_amount,
        })?;
        let (raw_body, status) = self
            .rest_client
            .post(
                &start_res.upload_url,
                Some(HashMap::from([(
                    "Content-Type".to_string(),
                    "application/json".to_string(),
                )])),
                Some(body),
            )
            .await?;

        if !reqwest::StatusCode::from_u16(status)?.is_success() {
            bail!("Received error status code when executing swap: {status}");
        }

        let response: SwapStartResponse = serde_json::from_str(&raw_body)?;
        let pset = PartiallySignedTransaction::from_str(&response.pset)?;
        // TODO verify pset amounts match

        let tx = self.onchain_wallet.sign_pset(pset).await?;
        let tx = elements::encode::serialize(&tx);
        let body = serde_json::to_string(&SwapSignRequest {
            order_id: start_res.order_id,
            submit_id: response.submit_id,
            pset: base64::engine::general_purpose::STANDARD.encode(&tx),
        })?;

        let (raw_body, status) = self
            .rest_client
            .post(
                &start_res.upload_url,
                Some(HashMap::from([(
                    "Content-Type".to_string(),
                    "application/json".to_string(),
                )])),
                Some(body),
            )
            .await?;

        if !reqwest::StatusCode::from_u16(status)?.is_success() {
            bail!("Received error status code when sending pset: {status}");
        }

        let response: SwapSignResponse = serde_json::from_str(&raw_body)?;
        self.unset_ongoing_swap().await;
        Ok(ExecuteSwapResponse {
            tx_id: response.txid.to_hex(),
            recv_address: recv_addr.to_string(),
        })
    }
}
