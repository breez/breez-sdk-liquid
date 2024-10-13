use std::sync::Arc;

use lwk_wollet::hashes::hex::DisplayHex as _;
use serde::{Deserialize, Serialize};

use self::sync::{ListChangesRequest, ListenChangesRequest, Record, SetRecordRequest};
use crate::{
    model::{ChainSwap, Direction, PaymentState, ReceiveSwap, SendSwap, Signer, SignerError},
    utils,
};

pub(crate) mod sync;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) direction: Direction,
    pub(crate) lockup_address: String,
    pub(crate) claim_address: String,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_private_key: String,
    pub(crate) refund_private_key: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) accept_zero_conf: bool,
    pub(crate) created_at: u32,
    pub(crate) description: Option<String>,
}

impl ChainSyncData {
    pub(crate) fn to_swap(self) -> ChainSwap {
        ChainSwap {
            id: self.swap_id,
            direction: self.direction,
            timeout_block_height: self.timeout_block_height,
            preimage: self.preimage,
            description: self.description,
            payer_amount_sat: self.payer_amount_sat,
            receiver_amount_sat: self.receiver_amount_sat,
            accept_zero_conf: self.accept_zero_conf,
            created_at: self.created_at,
            lockup_address: self.lockup_address,
            claim_address: self.claim_address,
            claim_fees_sat: self.claim_fees_sat,
            claim_private_key: self.claim_private_key,
            refund_private_key: self.refund_private_key,
            create_response_json: self.create_response_json,
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) create_response_json: String,
    pub(crate) refund_private_key: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) preimage: Option<String>,
    pub(crate) description: Option<String>,
}

impl SendSyncData {
    pub(crate) fn to_swap(self) -> SendSwap {
        SendSwap {
            id: self.swap_id,
            invoice: self.invoice,
            description: self.description,
            preimage: self.preimage,
            payer_amount_sat: self.payer_amount_sat,
            receiver_amount_sat: self.receiver_amount_sat,
            create_response_json: self.create_response_json,
            refund_private_key: self.refund_private_key,
            created_at: self.created_at,
            lockup_tx_id: None,
            refund_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_private_key: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) description: Option<String>,
}

impl ReceiveSyncData {
    pub(crate) fn to_swap(self) -> ReceiveSwap {
        ReceiveSwap {
            id: self.swap_id,
            preimage: self.preimage,
            create_response_json: self.create_response_json,
            claim_private_key: self.claim_private_key,
            invoice: self.invoice,
            description: self.description,
            payer_amount_sat: self.payer_amount_sat,
            receiver_amount_sat: self.receiver_amount_sat,
            claim_fees_sat: self.claim_fees_sat,
            created_at: self.created_at,
            claim_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "data_type", content = "data")]
pub(crate) enum SyncData {
    Chain(ChainSyncData),
    Send(SendSyncData),
    Receive(ReceiveSyncData),
}

impl SyncData {
    pub(crate) fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }
}

#[derive(Clone)]
pub(crate) struct DecryptedRecord {
    pub(crate) id: i64,
    pub(crate) version: f32,
    pub(crate) data: SyncData,
}

impl DecryptedRecord {
    pub(crate) fn try_from_record(private_key: &[u8], record: &Record) -> anyhow::Result<Self> {
        let dec_data = utils::decrypt(private_key, record.data.as_slice())?;
        let data = serde_json::from_slice(&dec_data)?;
        Ok(Self {
            id: record.id,
            version: record.version,
            data,
        })
    }
}

impl SetRecordRequest {
    pub(crate) fn new(
        record: Record,
        request_time: u32,
        signer: Arc<dyn Signer>,
    ) -> Result<Self, SignerError> {
        let msg = format!(
            "{}-{}-{}-{}",
            record.id,
            record.version,
            record.data.to_lower_hex_string(),
            request_time,
        );
        let signature = signer
            .sign_ecdsa_recoverable(msg.as_bytes().into())
            .map(|bytes| bytes.to_lower_hex_string())?;

        Ok(Self {
            record: Some(record),
            request_time,
            signature,
        })
    }
}

impl ListChangesRequest {
    pub(crate) fn new(
        from_id: i64,
        request_time: u32,
        signer: Arc<dyn Signer>,
    ) -> Result<Self, SignerError> {
        let msg = format!("{}-{}", from_id, request_time);
        let signature = signer
            .sign_ecdsa_recoverable(msg.as_bytes().into())
            .map(|bytes| bytes.to_lower_hex_string())?;

        Ok(Self {
            from_id,
            request_time,
            signature,
        })
    }
}

impl ListenChangesRequest {
    pub(crate) fn new(request_time: u32, signer: Arc<dyn Signer>) -> Result<Self, SignerError> {
        let msg = format!("{}", request_time);
        let signature = signer
            .sign_ecdsa_recoverable(msg.as_bytes().into())
            .map(|bytes| bytes.to_lower_hex_string())?;

        Ok(Self {
            request_time,
            signature,
        })
    }
}
