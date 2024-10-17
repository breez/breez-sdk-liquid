use std::sync::Arc;

use lwk_wollet::hashes::hex::DisplayHex;
use openssl::sha::sha256;
use serde::{Deserialize, Serialize};

use self::sync::{ListChangesRequest, ListenChangesRequest, Record, SetRecordRequest};
use crate::model::{
    ChainSwap, Direction, PaymentState, ReceiveSwap, SendSwap, Signer, SignerError,
};

use super::CURRENT_SCHEMA_VERSION;

pub(crate) mod sync;

const MESSAGE_PREFIX: &[u8; 13] = b"realtimesync:";

#[derive(Serialize, Deserialize)]
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

impl From<ChainSwap> for ChainSyncData {
    fn from(value: ChainSwap) -> Self {
        Self {
            swap_id: value.id,
            preimage: value.preimage,
            create_response_json: value.create_response_json,
            direction: value.direction,
            lockup_address: value.lockup_address,
            claim_address: value.claim_address,
            claim_fees_sat: value.claim_fees_sat,
            claim_private_key: value.claim_private_key,
            refund_private_key: value.refund_private_key,
            timeout_block_height: value.timeout_block_height,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            accept_zero_conf: value.accept_zero_conf,
            created_at: value.created_at,
            description: value.description,
        }
    }
}

impl From<ChainSyncData> for ChainSwap {
    fn from(val: ChainSyncData) -> Self {
        ChainSwap {
            id: val.swap_id,
            direction: val.direction,
            timeout_block_height: val.timeout_block_height,
            preimage: val.preimage,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            accept_zero_conf: val.accept_zero_conf,
            created_at: val.created_at,
            lockup_address: val.lockup_address,
            claim_address: val.claim_address,
            claim_fees_sat: val.claim_fees_sat,
            claim_private_key: val.claim_private_key,
            refund_private_key: val.refund_private_key,
            create_response_json: val.create_response_json,
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) create_response_json: String,
    pub(crate) refund_private_key: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) preimage: Option<String>,
    pub(crate) description: Option<String>,
}

impl From<SendSwap> for SendSyncData {
    fn from(value: SendSwap) -> Self {
        Self {
            swap_id: value.id,
            invoice: value.invoice,
            create_response_json: value.create_response_json,
            refund_private_key: value.refund_private_key,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            created_at: value.created_at,
            preimage: value.preimage,
            description: value.description,
        }
    }
}

impl From<SendSyncData> for SendSwap {
    fn from(val: SendSyncData) -> Self {
        SendSwap {
            id: val.swap_id,
            invoice: val.invoice,
            description: val.description,
            preimage: val.preimage,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            create_response_json: val.create_response_json,
            refund_private_key: val.refund_private_key,
            created_at: val.created_at,
            lockup_tx_id: None,
            refund_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_private_key: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) description: Option<String>,
}

impl From<ReceiveSwap> for ReceiveSyncData {
    fn from(value: ReceiveSwap) -> Self {
        Self {
            swap_id: value.id,
            invoice: value.invoice,
            preimage: value.preimage,
            create_response_json: value.create_response_json,
            claim_fees_sat: value.claim_fees_sat,
            claim_private_key: value.claim_private_key,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            created_at: value.created_at,
            description: value.description,
        }
    }
}

impl From<ReceiveSyncData> for ReceiveSwap {
    fn from(val: ReceiveSyncData) -> Self {
        ReceiveSwap {
            id: val.swap_id,
            preimage: val.preimage,
            create_response_json: val.create_response_json,
            claim_private_key: val.claim_private_key,
            invoice: val.invoice,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            claim_fees_sat: val.claim_fees_sat,
            created_at: val.created_at,
            claim_tx_id: None,
            state: PaymentState::Created,
            is_local: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
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

pub(crate) struct DecryptedRecord {
    pub(crate) id: i64,
    pub(crate) version: f32,
    pub(crate) data: SyncData,
}

impl DecryptedRecord {
    pub(crate) fn try_from_record(
        signer: Arc<Box<dyn Signer>>,
        record: &Record,
    ) -> anyhow::Result<Self> {
        let dec_data = signer.ecies_decrypt(record.data.as_slice())?;
        let data = serde_json::from_slice(&dec_data)?;
        Ok(Self {
            id: record.id,
            version: record.version,
            data,
        })
    }
}

impl Record {
    pub(crate) fn new(
        id: i64,
        data: SyncData,
        signer: Arc<Box<dyn Signer>>,
    ) -> anyhow::Result<Self> {
        let bytes = data.to_bytes()?;
        let data = signer
            .ecies_encrypt(&bytes)
            .map_err(|err| anyhow::anyhow!("Could not encrypt sync data: {err:?}"))?;
        Ok(Record {
            id,
            version: CURRENT_SCHEMA_VERSION,
            data,
        })
    }
}

impl SetRecordRequest {
    pub(crate) fn new(
        record: Record,
        request_time: u32,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, SignerError> {
        let msg = format!(
            "{}-{}-{}-{}",
            record.id,
            record.version,
            record.data.to_lower_hex_string(),
            request_time,
        );
        let signature = sign_message(msg.as_bytes(), signer)?;
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
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, SignerError> {
        let msg = format!("{}-{}", from_id, request_time);
        let signature = sign_message(msg.as_bytes(), signer)?;
        Ok(Self {
            from_id,
            request_time,
            signature,
        })
    }
}

impl ListenChangesRequest {
    pub(crate) fn new(
        request_time: u32,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, SignerError> {
        let msg = format!("{}", request_time);
        let signature = sign_message(msg.as_bytes(), signer)?;
        Ok(Self {
            request_time,
            signature,
        })
    }
}

fn sign_message(msg: &[u8], signer: Arc<Box<dyn Signer>>) -> Result<String, SignerError> {
    let msg = [MESSAGE_PREFIX, msg].concat();
    let digest = sha256(&sha256(&msg));
    signer
        .sign_ecdsa_recoverable(digest.into())
        .map(|bytes| zbase32::encode_full_bytes(&bytes))
}
