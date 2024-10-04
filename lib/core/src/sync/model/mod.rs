use boltz_client::boltz::SwapTree;
use serde::{Deserialize, Serialize};

use crate::{prelude::Direction, utils};

use self::sync::Record;

pub(crate) mod sync;

#[derive(Serialize, Deserialize)]
pub(crate) struct ClaimDetails {
    pub(crate) swap_tree: SwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_address: String,
    pub(crate) claim_private_key: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LockupDetails {
    pub(crate) swap_tree: SwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) lockup_address: String,
    pub(crate) refund_private_key: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    pub(crate) direction: Direction,
    pub(crate) claim_details: ClaimDetails,
    pub(crate) lockup_details: LockupDetails,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) accept_zero_conf: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) refund_private_key: String,
    pub(crate) swap_tree: SwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    pub(crate) claim_private_key: String,
    pub(crate) swap_tree: SwapTree,
    pub(crate) timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
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
