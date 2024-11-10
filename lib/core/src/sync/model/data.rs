use serde::{Deserialize, Serialize};

use crate::prelude::{ChainSwap, Direction, ReceiveSwap, SendSwap};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) direction: Direction,
    pub(crate) lockup_address: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) create_response_json: String,
    pub(crate) refund_private_key: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) preimage: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
}

impl From<SendSwap> for SendSyncData {
    fn from(value: SendSwap) -> Self {
        Self {
            swap_id: value.id,
            payment_hash: value.payment_hash,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_private_key: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) mrh_address: String,
    pub(crate) created_at: u32,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
}

impl From<ReceiveSwap> for ReceiveSyncData {
    fn from(value: ReceiveSwap) -> Self {
        Self {
            swap_id: value.id,
            payment_hash: value.payment_hash,
            invoice: value.invoice,
            preimage: value.preimage,
            create_response_json: value.create_response_json,
            claim_fees_sat: value.claim_fees_sat,
            claim_private_key: value.claim_private_key,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            mrh_address: value.mrh_address,
            created_at: value.created_at,
            description: value.description,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "data_type", content = "data")]
pub(crate) enum SyncData {
    Chain(ChainSyncData),
    Send(SendSyncData),
    Receive(ReceiveSyncData),
}

impl SyncData {
    pub(crate) fn id(&self) -> &str {
        match self {
            SyncData::Chain(chain_data) => &chain_data.swap_id,
            SyncData::Send(send_data) => &send_data.swap_id,
            SyncData::Receive(receive_data) => &receive_data.swap_id,
        }
    }

    pub(crate) fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }
}
