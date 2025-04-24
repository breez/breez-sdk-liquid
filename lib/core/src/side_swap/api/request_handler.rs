use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use boltz_client::boltz::tokio_tungstenite_wasm::{Message, WebSocketStream};
use futures_util::{stream::SplitSink, SinkExt as _};
use log::{debug, error, warn};
use sideswap_api::{Request, RequestId, RequestMessage};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

pub(crate) struct SideSwapRequestHandler {
    latest_request_id: Mutex<i64>,
    req_sender: OnceLock<UnboundedSender<RequestMessage>>,
}

impl SideSwapRequestHandler {
    pub(crate) fn new() -> Self {
        Self {
            latest_request_id: Mutex::new(0),
            req_sender: OnceLock::new(),
        }
    }

    pub(crate) fn start(&self, req_sender: UnboundedSender<RequestMessage>) -> Result<()> {
        self.req_sender
            .set(req_sender)
            .map_err(|_| anyhow!("Could not create outbound request channel"))
    }

    pub(crate) async fn next_request_id(&self) -> i64 {
        let mut request_id = self.latest_request_id.lock().await;
        *request_id += 1;
        *request_id
    }

    pub(crate) async fn send_ws(
        &self,
        ws_sender: &mut SplitSink<WebSocketStream, Message>,
        msg: RequestMessage,
    ) {
        match serde_json::to_string(&msg) {
            Ok(msg) => match ws_sender.send(Message::Text(msg.into())).await {
                Ok(_) => debug!("Sent message to SideSwap service"),
                Err(e) => warn!("Failed to send message: {e:?}"),
            },
            Err(e) => error!("Failed to serialize message: {e:?}"),
        }
    }

    pub(crate) async fn send(&self, req: Request) -> Result<i64> {
        let sender = self.req_sender.get().context("Handler is not active")?;

        let request_id = self.next_request_id().await;
        let msg = RequestMessage::Request(RequestId::Int(request_id), req);

        sender
            .send(msg)
            .map_err(|err| anyhow!("Could not forward message to SideSwap service: {err}"))?;
        Ok(request_id)
    }
}
