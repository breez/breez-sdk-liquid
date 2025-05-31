use anyhow::{anyhow, bail, Result};
use base64::Engine;
use futures_util::{SinkExt as _, StreamExt};
use log::{debug, error, info, warn};
use request_handler::SideSwapRequestHandler;
use response_handler::SideSwapResponseHandler;
use sdk_common::bitcoin::hashes::hex::ToHex as _;
use sdk_common::prelude::{parse_json, RestClient};
use sdk_common::utils::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sideswap_api::http_rpc::{SwapSignRequest, SwapStartRequest};
use sideswap_api::{StartSwapWebRequest, SubscribePriceStreamRequest, Utxo};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::{watch, OnceCell};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_with_wasm::alias as tokio;

use boltz_client::boltz::tokio_tungstenite_wasm;
use tokio_tungstenite_wasm::{connect, Message};

use model::*;

use crate::bitcoin::base64;
use crate::elements::{self, pset::PartiallySignedTransaction, Address, AssetId};
use crate::error::PaymentError;
use crate::model::Config;
use crate::wallet::OnchainWallet;
use crate::{ensure_sdk, utils};

pub(crate) mod model;
mod request_handler;
mod response_handler;

pub const SIDESWAP_MAINNET_URL: &str = "wss://api.sideswap.io/json-rpc-ws";
pub const SIDESWAP_TESTNET_URL: &str = "wss://api-testnet.sideswap.io/json-rpc-ws";

pub(crate) struct SideSwapService {
    config: Config,
    rest_client: Arc<dyn RestClient>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    request_handler: SideSwapRequestHandler,
    response_handler: SideSwapResponseHandler,
    event_loop_handle: OnceCell<JoinHandle<()>>,
}

impl SideSwapService {
    pub(crate) fn from_sdk(sdk: &crate::sdk::LiquidSdk) -> Arc<Self> {
        Self::new(
            sdk.config.clone(),
            sdk.rest_client.clone(),
            sdk.onchain_wallet.clone(),
            sdk.shutdown_receiver.clone(),
        )
    }

    pub(crate) fn new(
        config: Config,
        rest_client: Arc<dyn RestClient>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        mut shutdown_rx: watch::Receiver<()>,
    ) -> Arc<Self> {
        let service = Arc::new(Self {
            config,
            request_handler: SideSwapRequestHandler::new(),
            response_handler: SideSwapResponseHandler::new(),
            rest_client,
            onchain_wallet,
            event_loop_handle: OnceCell::new(),
        });
        let cloned = service.clone();

        let handle = tokio::spawn(async move {
            info!("Starting SideSwap service event loop");
            loop {
                if shutdown_rx.has_changed().unwrap_or(true) {
                    return;
                }

                let ws = match connect(cloned.config.sideswap_url()).await {
                    Ok(ws) => ws,
                    Err(err) => {
                        error!("Could not connect to SideSwap websocket: {err}");
                        sleep(Duration::from_secs(3)).await;
                        continue;
                    }
                };

                let (mut ws_sender, mut ws_receiver) = ws.split();
                let keep_alive_ping_interval = Duration::from_secs(15);
                loop {
                    tokio::select! {
                        _ = shutdown_rx.changed() => {
                            info!("Received shutdown signal from SDK, exiting SideSwap service loop");
                            return;
                        }

                        _ = tokio::time::sleep(keep_alive_ping_interval) => {
                            if let Err(err) = cloned.request_handler.send(Request::Ping(None)).await {
                                warn!("Could not send ping message to SideSwap server: {err:?}");
                            };
                        },

                        maybe_req_msg = cloned.request_handler.recv() => match maybe_req_msg {
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
                                    break; // break the inner loop which will start the main loop by reconnecting
                                },
                                Ok(Message::Text(payload)) => cloned.handle_message(payload.as_str()).await,
                                Ok(msg) => warn!("Unhandled msg: {msg:?}"),
                                Err(e) => {
                                    error!("Received stream error: {e:?}");
                                    let _ = ws_sender.close().await;
                                    break; // break the inner loop which will start the main loop by reconnecting
                                }
                            }
                            None => {
                                warn!("Received nothing from the stream");
                                let _ = ws_sender.close().await;
                                break; // break the inner loop which will start the main loop by reconnecting
                            },
                        }
                    }
                }
            }
        });
        let _ = service.event_loop_handle.set(handle);
        service
    }

    async fn post_request<I: Serialize, O: DeserializeOwned>(
        &self,
        url: &str,
        body: &I,
    ) -> Result<O> {
        let headers = HashMap::from([("Content-Type".to_string(), "application/json".to_string())]);
        let body = serde_json::to_string(body)?;
        debug!("Posting request to SideSwap API: {body}");
        let (response, status_code) = self
            .rest_client
            .post(url, Some(headers), Some(body))
            .await?;
        if status_code != 200 {
            error!("Received status code {status_code} response from SideSwap API");
            bail!("Failed to post request to SideSwap API: {response}")
        }
        debug!("Received response from SideSwap API: {response}");
        Ok(parse_json(&response)?)
    }

    pub(crate) fn stop(&self) {
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
    }

    async fn handle_message(&self, msg: &str) {
        info!("Received text message: {msg}");
        match serde_json::from_str::<WrappedResponse>(msg) {
            Ok(WrappedResponse::Response { id, response, .. }) => {
                self.response_handler.handle_response(id, response).await;
            }
            Ok(WrappedResponse::Notification { notification, .. }) => {
                debug!("Received unhandled notification from SideSwap service: {notification:?}");
            }
            // Either an error or an invalid response
            Ok(WrappedResponse::Error { error, .. }) => {
                error!("Received error response from the server: {error:?}")
            }
            Err(e) => error!("Failed to parse websocket response: {e:?} - response: {msg}"),
        }
    }

    fn invalid_response(res: Response) -> anyhow::Error {
        anyhow!("Received invalid response from the server: {res:?}")
    }

    pub(crate) async fn get_asset_swap(
        &self,
        asset_id: AssetId,
        receiver_amount_sat: u64,
    ) -> Result<AssetSwap> {
        let req = Request::SubscribePriceStream(SubscribePriceStreamRequest {
            subscribe_id: None,
            asset: asset_id,
            send_bitcoins: true,
            send_amount: None,
            recv_amount: Some(receiver_amount_sat as i64),
        });
        let request_id = self.request_handler.send(req).await?;
        let res = match self.response_handler.recv(request_id).await? {
            Response::SubscribePriceStream(res) => res,
            res => {
                return Err(Self::invalid_response(res));
            }
        };

        res.try_into()
    }

    pub(crate) async fn execute_swap(
        &self,
        receiver_address: Address,
        asset_swap: &AssetSwap,
    ) -> Result<String> {
        let req = Request::StartSwapWeb(StartSwapWebRequest {
            asset: AssetId::from_str(&asset_swap.asset_id)?,
            price: asset_swap.exchange_rate,
            send_amount: asset_swap.payer_amount_sat as i64,
            recv_amount: asset_swap.receiver_amount_sat as i64,
            send_bitcoins: true,
        });
        let request_id = self.request_handler.send(req).await?;
        let start_res = match self.response_handler.recv(request_id).await? {
            Response::StartSwapWeb(res) => Ok(res),
            res => Err(Self::invalid_response(res)),
        }?;

        let change_addr = self.onchain_wallet.next_unused_change_address().await?;

        let wallet_utxos = self
            .onchain_wallet
            .asset_utxos(&utils::lbtc_asset_id(self.config.network))
            .await?;

        ensure_sdk!(
            !wallet_utxos.is_empty(),
            PaymentError::InsufficientFunds.into()
        );

        let mut inputs = vec![];
        let mut send_value = 0i64;
        for wallet_utxo in wallet_utxos {
            if wallet_utxo.is_spent {
                continue;
            }
            send_value += wallet_utxo.unblinded.value as i64;
            inputs.push(Utxo {
                txid: wallet_utxo.outpoint.txid,
                vout: wallet_utxo.outpoint.vout,
                asset: wallet_utxo.unblinded.asset,
                asset_bf: wallet_utxo.unblinded.asset_bf,
                value: wallet_utxo.unblinded.value,
                value_bf: wallet_utxo.unblinded.value_bf,
                redeem_script: None,
            });
            if send_value >= start_res.send_amount {
                break;
            }
        }
        ensure_sdk!(
            send_value >= start_res.send_amount,
            PaymentError::InsufficientFunds.into()
        );

        let body = HttpRequest::SwapStart(SwapStartRequest {
            order_id: start_res.order_id,
            inputs,
            recv_addr: receiver_address.clone(),
            change_addr: change_addr.clone(),
            send_asset: start_res.send_asset,
            send_amount: start_res.send_amount,
            recv_asset: start_res.recv_asset,
            recv_amount: start_res.recv_amount,
        });
        let HttpResponse::SwapStart(response) =
            self.post_request(&start_res.upload_url, &body).await?
        else {
            bail!("Received unexpected response from SideSwap API.");
        };

        let mut pset = PartiallySignedTransaction::from_str(&response.pset)?;
        let Some(recv_output) = pset
            .outputs()
            .iter()
            .find(|o| o.script_pubkey == receiver_address.script_pubkey())
        else {
            bail!("PSET verification error: Could not find receive address script pubkey among outputs");
        };

        ensure_sdk!(
            recv_output.amount.is_some_and(|a| a == start_res.recv_amount as u64) &&
            recv_output.asset.is_some_and(|a| a == start_res.recv_asset),
            anyhow!(
                "PSET verification error: (receive output) expected amount: {}, asset: {} / received: amount: {:?}, asset: {:?}",
                start_res.recv_amount, start_res.recv_asset,
                recv_output.amount, recv_output.asset,
            )
        );

        let Some(change_output) = pset
            .outputs()
            .iter()
            .find(|o| o.script_pubkey == change_addr.script_pubkey())
        else {
            bail!("PSET verification error: Could not find change address script pubkey among outputs");
        };
        let change_amount = (send_value - start_res.send_amount) as u64;
        let change_asset = utils::lbtc_asset_id(self.config.network);
        ensure_sdk!(
            change_output.amount.is_some_and(|a| a == change_amount) &&
            change_output.asset.is_some_and(|a| a == change_asset),
            anyhow!(
                "PSET verification error: (change output) expected amount: {}, asset: {} / received: amount: {:?}, asset: {:?}",
                change_amount, change_asset,
                change_output.amount, change_output.asset,
            )
        );

        self.onchain_wallet.sign_pset(&mut pset).await?;
        let pset = elements::encode::serialize(&pset);
        let body = HttpRequest::SwapSign(SwapSignRequest {
            order_id: start_res.order_id,
            submit_id: response.submit_id,
            pset: base64::engine::general_purpose::STANDARD.encode(&pset),
        });

        let HttpResponse::SwapSign(response) =
            self.post_request(&start_res.upload_url, &body).await?
        else {
            bail!("Received unexpected response from SideSwap API.");
        };

        Ok(response.txid.to_hex())
    }
}
