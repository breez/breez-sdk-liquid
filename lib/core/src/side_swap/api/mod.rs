use anyhow::{anyhow, Context as _, Result};
use base64::Engine;
use futures_util::{SinkExt as _, StreamExt};
use log::{error, info, warn};
use notifications::SideSwapNotificationsHandler;
use request_handler::SideSwapRequestHandler;
use response_handler::SideSwapResponseHandler;
use sdk_common::utils::Arc;
use sideswap_api::mkt::{
    AssetPair, AssetType, GetQuoteRequest, QuoteNotif, QuoteStatus, QuoteSubId, StartQuotesRequest,
    StartQuotesResponse, StopQuotesRequest, TakerSignRequest, TradeDir,
};
use sideswap_api::{mkt::Request as MarketRequest, mkt::Response as MarketResponse, Request};
use sideswap_api::{RequestId, RequestMessage, Response, ResponseMessage};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_with_wasm::alias as tokio;

use boltz_client::boltz::tokio_tungstenite_wasm;
use tokio_tungstenite_wasm::{connect, Message};

use crate::bitcoin::base64;
use crate::elements::{self, pset::PartiallySignedTransaction, Txid};
use crate::wallet::OnchainWallet;

mod notifications;
mod request_handler;
mod response_handler;

pub const SIDESWAP_MAINNET_URL: &str = "wss://api.sideswap.io/json-rpc-ws";
pub const SIDESWAP_TESTNET_URL: &str = "wss://api-testnet.sideswap.io/json-rpc-ws";

#[sdk_macros::async_trait]
pub trait SideSwapStream {
    async fn start_fetching_quotes(
        &self,
        asset_pair: AssetPair,
        amount: u64,
    ) -> Result<StartQuotesResponse>;
    async fn stop_fetching_quotes(&self) -> Result<()>;
    async fn get_quote(
        &self,
        quote_sub_id: QuoteSubId,
        successful_only: bool,
    ) -> Result<QuoteNotif>;
    async fn sign_quote_pset(&self, quote_sub_id: QuoteSubId) -> Result<Txid>;
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

pub(crate) struct SideSwapService {
    url: String,
    is_started: Mutex<bool>,
    shutdown: ShutdownChannel,
    onchain_wallet: Arc<dyn OnchainWallet>,
    request_handler: SideSwapRequestHandler,
    response_handler: SideSwapResponseHandler,
    notifications_handler: SideSwapNotificationsHandler,
}

impl SideSwapService {
    pub(crate) fn new(url: String, onchain_wallet: Arc<dyn OnchainWallet>) -> Self {
        Self {
            url,
            is_started: Mutex::new(false),
            shutdown: ShutdownChannel::new(tokio::sync::mpsc::channel::<()>(10)),
            request_handler: SideSwapRequestHandler::new(),
            response_handler: SideSwapResponseHandler::new(),
            notifications_handler: SideSwapNotificationsHandler::new(),
            onchain_wallet,
        }
    }

    async fn set_started(&self, is_started: bool) {
        let mut lock = self.is_started.lock().await;
        *lock = is_started;
    }

    async fn start(self: Arc<Self>) -> Result<()> {
        let ws = connect(&self.url).await?;
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
                        RequestMessage::Request(RequestId::String("ping".to_string()), Request::Ping(None))
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

    pub(crate) async fn stop(&self) {
        let _ = self.shutdown.sender.send(()).await;
    }

    async fn handle_message(&self, msg: &str) {
        info!("Received text message: {msg:?}");
        match serde_json::from_str::<ResponseMessage>(msg) {
            Ok(ResponseMessage::Response(req_id, res)) => {
                self.response_handler
                    .handle_response(req_id.clone(), res)
                    .await;
            }
            Ok(ResponseMessage::Notification(notif)) => {
                self.notifications_handler.handle_notification(notif).await;
            }
            // Either an invalid response, or an error
            Err(e) => error!("Failed to parse websocket response: {e:?} - response: {msg}"),
        }
    }

    fn invalid_response(res: Result<Response, sideswap_api::Error>) -> anyhow::Error {
        match res {
            Ok(res) => anyhow!("Received invalid response from the server: {res:?}"),
            Err(err) => anyhow!("Received error response from the server: {err:?}"),
        }
    }
}

#[sdk_macros::async_trait]
impl SideSwapStream for SideSwapService {
    async fn start_fetching_quotes(
        &self,
        asset_pair: AssetPair,
        amount: u64,
    ) -> Result<StartQuotesResponse> {
        let receive_address = self.onchain_wallet.next_unused_address().await?;
        let change_address = self.onchain_wallet.next_unused_change_address().await?;

        let req = Request::Market(MarketRequest::StartQuotes(StartQuotesRequest {
            asset_pair,
            asset_type: AssetType::Base,
            amount,
            trade_dir: TradeDir::Buy,
            utxos: vec![],
            receive_address,
            change_address,
            order_id: None,
            private_id: None,
            instant_swap: false,
        }));

        let request_id = self.request_handler.send(req).await?;
        match self.response_handler.recv(request_id).await? {
            Ok(Response::Market(MarketResponse::StartQuotes(res))) => Ok(res),
            res => Err(Self::invalid_response(res)),
        }
    }

    async fn stop_fetching_quotes(&self) -> Result<()> {
        let req = Request::Market(MarketRequest::StopQuotes(StopQuotesRequest {}));
        let request_id = self.request_handler.send(req).await?;
        match self.response_handler.recv(request_id).await? {
            Ok(Response::Market(MarketResponse::StopQuotes(_))) => Ok(()),
            res => Err(Self::invalid_response(res)),
        }
    }

    async fn get_quote(
        &self,
        quote_sub_id: QuoteSubId,
        successful_only: bool,
    ) -> Result<QuoteNotif> {
        let maybe_quote = self
            .notifications_handler
            .wait_for_quote(
                quote_sub_id,
                Duration::from_millis(500),
                10,
                successful_only,
            )
            .await;
        maybe_quote.context("Did not receive any quotes from server")
    }

    async fn sign_quote_pset(&self, quote_sub_id: QuoteSubId) -> Result<Txid> {
        let quote = self.get_quote(quote_sub_id, true).await?;
        let quote_id = match quote.status {
            QuoteStatus::Success { quote_id, .. } => quote_id,
            _ => anyhow::bail!("Expected quote with success status, got: {quote:?}"),
        };

        let req = Request::Market(MarketRequest::GetQuote(GetQuoteRequest { quote_id }));
        let request_id = self.request_handler.send(req).await?;
        let get_quote_res = match self.response_handler.recv(request_id).await? {
            Ok(Response::Market(MarketResponse::GetQuote(res))) => res,
            res => return Err(Self::invalid_response(res)),
        };

        let pset = PartiallySignedTransaction::from_str(&get_quote_res.pset)?;
        // TODO verify pset amounts match

        let tx = self.onchain_wallet.sign_pset(pset).await?;
        let tx = elements::encode::serialize(&tx);
        let req = Request::Market(MarketRequest::TakerSign(TakerSignRequest {
            quote_id,
            pset: base64::engine::general_purpose::STANDARD.encode(&tx),
        }));
        let request_id = self.request_handler.send(req).await?;
        match self.response_handler.recv(request_id).await? {
            Ok(Response::Market(MarketResponse::TakerSign(res))) => Ok(res.txid),
            res => Err(Self::invalid_response(res)),
        }
    }
}
