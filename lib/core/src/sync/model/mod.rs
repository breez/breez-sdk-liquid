use boltz_client::boltz::ChainSwapDetails;
use serde::{Deserialize, Serialize};

use self::sync::Record;
use crate::{
    model::{Direction, InternalSwapTree},
    utils,
};

pub(crate) mod sync;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    pub(crate) direction: Direction,
    pub(crate) claim_swap_tree: InternalSwapTree,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_address: String,
    pub(crate) claim_private_key: String,
    pub(crate) claim_timeout_block_height: u32,
    pub(crate) lockup_swap_tree: InternalSwapTree,
    pub(crate) lockup_address: String,
    pub(crate) refund_private_key: String,
    pub(crate) lockup_timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) accept_zero_conf: bool,
    pub(crate) created_at: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) refund_private_key: String,
    pub(crate) swap_tree: InternalSwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    pub(crate) claim_private_key: String,
    pub(crate) swap_tree: InternalSwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
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
