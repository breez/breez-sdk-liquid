#![cfg(test)]

use std::{collections::HashMap, sync::Arc};

use crate::{
    persist::Persister,
    prelude::{Direction, Signer},
    sync::{
        client::SyncerClient,
        model::{
            data::{ChainSyncData, ReceiveSyncData, SendSyncData},
            sync::{
                ListChangesReply, ListChangesRequest, Record, SetRecordReply, SetRecordRequest,
                SetRecordStatus,
            },
        },
        SyncService,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub(crate) struct MockSyncerClient {
    pub(crate) incoming_rx: Mutex<Receiver<Record>>,
    pub(crate) outgoing_records: Arc<Mutex<HashMap<String, Record>>>,
}

impl MockSyncerClient {
    pub(crate) fn new(
        incoming_rx: Receiver<Record>,
        outgoing_records: Arc<Mutex<HashMap<String, Record>>>,
    ) -> Self {
        Self {
            incoming_rx: Mutex::new(incoming_rx),
            outgoing_records,
        }
    }
}

#[async_trait]
impl SyncerClient for MockSyncerClient {
    async fn connect(&self, _connect_url: String) -> Result<()> {
        todo!()
    }

    async fn push(&self, req: SetRecordRequest) -> Result<SetRecordReply> {
        if let Some(mut record) = req.record {
            let mut outgoing_records = self.outgoing_records.lock().await;

            if let Some(existing_record) = outgoing_records.get(&record.id) {
                if existing_record.revision != record.revision {
                    return Ok(SetRecordReply {
                        status: SetRecordStatus::Conflict as i32,
                        new_revision: 0,
                    });
                }
            }

            record.revision = outgoing_records.len() as u64 + 1;
            let record_revision = record.revision;

            outgoing_records.insert(record.id.clone(), record);
            return Ok(SetRecordReply {
                status: SetRecordStatus::Success as i32,
                new_revision: record_revision,
            });
        }

        return Err(anyhow::anyhow!("No record was sent"));
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

#[allow(clippy::type_complexity)]
pub(crate) fn new_sync_service(
    persister: Arc<Persister>,
    signer: Arc<Box<dyn Signer>>,
) -> Result<(
    Sender<Record>,
    Arc<Mutex<HashMap<String, Record>>>,
    SyncService,
)> {
    let (_, sync_trigger_rx) = mpsc::channel::<()>(30);
    let (incoming_tx, incoming_rx) = mpsc::channel::<Record>(10);
    let outgoing_records = Arc::new(Mutex::new(HashMap::new()));
    let client = Box::new(MockSyncerClient::new(incoming_rx, outgoing_records.clone()));
    let sync_service = SyncService::new(
        "".to_string(),
        persister.clone(),
        signer.clone(),
        client,
        sync_trigger_rx,
    );

    Ok((incoming_tx, outgoing_records, sync_service))
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