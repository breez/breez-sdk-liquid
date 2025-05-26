use anyhow::{anyhow, Result};
use boltz_client::boltz::tokio_tungstenite_wasm::{Message, WebSocketStream};
use futures_util::{stream::SplitSink, SinkExt as _};
use log::{debug, error, warn};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};

use super::model::{Request, WrappedRequest, RequestId};

pub(crate) struct SideSwapRequestHandler {
    latest_request_id: Mutex<i64>,
    sender: UnboundedSender<WrappedRequest>,
    receiver: Mutex<UnboundedReceiver<WrappedRequest>>,
}

impl SideSwapRequestHandler {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<WrappedRequest>();
        Self {
            sender,
            latest_request_id: Mutex::new(0),
            receiver: Mutex::new(receiver),
        }
    }

    pub(crate) async fn next_request_id(&self) -> i64 {
        let mut request_id = self.latest_request_id.lock().await;
        *request_id += 1;
        *request_id
    }

    pub(crate) async fn send_ws(
        &self,
        ws_sender: &mut SplitSink<WebSocketStream, Message>,
        msg: WrappedRequest,
    ) {
        match serde_json::to_string(&msg) {
            Ok(msg) => match ws_sender.send(Message::Text(msg.clone().into())).await {
                Ok(_) => debug!("Sent message to SideSwap service: {msg:?}"),
                Err(e) => warn!("Failed to send message: {e:?}"),
            },
            Err(e) => error!("Failed to serialize message: {e:?}"),
        }
    }

    pub(crate) async fn send(&self, request: Request) -> Result<i64> {
        let request_id = self.next_request_id().await;
        let msg = WrappedRequest {
            id: RequestId::Int(request_id),
            request,
        };

        self.sender
            .send(msg)
            .map_err(|err| anyhow!("Could not forward message to SideSwap service: {err}"))?;
        Ok(request_id)
    }

    pub(crate) async fn recv(&self) -> Option<WrappedRequest> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }
}
