use anyhow::{anyhow, Context as _, Result};
use base64::Engine;
use futures_util::{SinkExt as _, StreamExt};
use log::{error, info, warn};
use maybe_sync::{MaybeSend, MaybeSync};
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
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::watch;

use boltz_client::boltz::tokio_tungstenite_wasm;
use tokio_tungstenite_wasm::{connect, Message, WebSocketStream};

use crate::bitcoin::base64;
use crate::elements::{self, pset::PartiallySignedTransaction, Txid};
use crate::wallet::OnchainWallet;

mod notifications;
mod request_handler;
mod response_handler;

pub const SIDESWAP_MAINNET_URL: &str = "wss://api.sideswap.io/json-rpc-ws";
pub const SIDESWAP_TESTNET_URL: &str = "wss://api-testnet.sideswap.io/json-rpc-ws";

#[sdk_macros::async_trait]
pub trait SideSwapStream: MaybeSend + MaybeSync {
    fn start(self: Arc<Self>, shutdown: watch::Receiver<()>);
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

    async fn handle_message(&self, msg: &str, resp_sender: &UnboundedSender<i64>) {
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
