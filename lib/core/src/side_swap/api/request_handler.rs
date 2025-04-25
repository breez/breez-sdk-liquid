use anyhow::{anyhow, Result};
use boltz_client::boltz::tokio_tungstenite_wasm::{Message, WebSocketStream};
use futures_util::{stream::SplitSink, SinkExt as _};
use log::{debug, error, warn};
use sideswap_api::{Request, RequestId, RequestMessage};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};

pub(crate) struct SideSwapRequestHandler {
    latest_request_id: Mutex<i64>,
    sender: UnboundedSender<RequestMessage>,
    receiver: Mutex<UnboundedReceiver<RequestMessage>>,
}

impl SideSwapRequestHandler {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<RequestMessage>();
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
        let request_id = self.next_request_id().await;
        let msg = RequestMessage::Request(RequestId::Int(request_id), req);

        self.sender
            .send(msg)
            .map_err(|err| anyhow!("Could not forward message to SideSwap service: {err}"))?;
        Ok(request_id)
    }

    pub(crate) async fn recv(&self) -> Option<RequestMessage> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }
}
