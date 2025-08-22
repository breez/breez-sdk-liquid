use std::{collections::HashMap, time::Duration};

use super::model::{RequestId, Response};
use anyhow::{bail, Result};
use log::warn;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tokio_with_wasm::alias as tokio;

const RECV_TIMEOUT_SECS: u64 = 10;

pub(crate) struct SideSwapResponseHandler {
    sender: UnboundedSender<i64>,
    receiver: Mutex<UnboundedReceiver<i64>>,
    received: Mutex<HashMap<i64, Response>>,
}

impl SideSwapResponseHandler {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<i64>();
        Self {
            sender,
            receiver: Mutex::new(receiver),
            received: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn handle_response(&self, res_id: RequestId, res: Response) {
        let RequestId::Int(res_id) = res_id else {
            warn!("Could not handle response - invalid id received from server: {res_id:?}");
            return;
        };
        self.received.lock().await.insert(res_id, res);
        let _ = self.sender.send(res_id);
    }

    pub(crate) async fn recv(&self, res_id: i64) -> Result<Response> {
        let mut received = self.received.lock().await;
        if let Some(maybe_res) = received.remove(&res_id) {
            return Ok(maybe_res);
        }
        drop(received);

        tokio::time::timeout(Duration::from_secs(RECV_TIMEOUT_SECS), async {
            let mut receiver = self.receiver.lock().await;
            while let Some(new_id) = receiver.recv().await {
                if new_id != res_id {
                    continue;
                }
                let mut received = self.received.lock().await;
                match received.remove(&res_id) {
                    Some(maybe_res) => return Ok(maybe_res),
                    None => bail!("Expected response from the server"),
                }
            }
            bail!("Receive channel has been closed")
        })
        .await?
    }
}
