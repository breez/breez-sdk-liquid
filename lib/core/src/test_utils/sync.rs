#![cfg(test)]

use crate::{
    prelude::Direction,
    sync::{
        client::SyncerClient,
        model::{
            data::{ChainSyncData, ReceiveSyncData, SendSyncData},
            sync::{
                ListChangesReply, ListChangesRequest, Record, SetRecordReply, SetRecordRequest,
            },
        },
    },
};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::{mpsc::Receiver, Mutex};

pub(crate) struct MockSyncerClient {
    pub(crate) incoming_rx: Mutex<Receiver<Record>>,
}

impl MockSyncerClient {
    pub(crate) fn new(incoming_rx: Receiver<Record>) -> Self {
        Self {
            incoming_rx: Mutex::new(incoming_rx),
        }
    }
}

#[async_trait]
impl SyncerClient for MockSyncerClient {
    async fn connect(&self, _connect_url: String) -> Result<()> {
        todo!()
    }

    async fn push(&self, _req: SetRecordRequest) -> Result<SetRecordReply> {
        todo!()
    }

    async fn pull(&self, _req: ListChangesRequest) -> Result<ListChangesReply> {
        let mut rx = self.incoming_rx.lock().await;
        let mut changes = Vec::with_capacity(3);
        rx.recv_many(&mut changes, 3).await;
        Ok(ListChangesReply { changes })
    }

    async fn disconnect(&self) -> Result<()> {
        todo!()
    }
}

pub(crate) fn new_receive_sync_data() -> ReceiveSyncData {
    ReceiveSyncData {
        swap_id: "receive-swap".to_string(),
        invoice: "".to_string(),
        create_response_json: "".to_string(),
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        created_at: 0,
        claim_fees_sat: 0,
        claim_private_key: "".to_string(),
        mrh_address: "".to_string(),
        preimage: "".to_string(),
        payment_hash: None,
        description: None,
    }
}

pub(crate) fn new_send_sync_data(preimage: Option<String>) -> SendSyncData {
    SendSyncData {
        swap_id: "send-swap".to_string(),
        invoice: "".to_string(),
        create_response_json: "".to_string(),
        refund_private_key: "".to_string(),
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        created_at: 0,
        preimage,
        payment_hash: None,
        description: None,
    }
}

pub(crate) fn new_chain_sync_data(accept_zero_conf: Option<bool>) -> ChainSyncData {
    ChainSyncData {
        swap_id: "chain-swap".to_string(),
        preimage: "".to_string(),
        create_response_json: "".to_string(),
        direction: Direction::Incoming,
        lockup_address: "".to_string(),
        claim_fees_sat: 0,
        claim_private_key: "".to_string(),
        refund_private_key: "".to_string(),
        timeout_block_height: 0,
        payer_amount_sat: 0,
        receiver_amount_sat: 0,
        accept_zero_conf: accept_zero_conf.unwrap_or(true),
        created_at: 0,
        description: None,
    }
}
