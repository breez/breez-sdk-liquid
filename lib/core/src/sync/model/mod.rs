use serde::{Deserialize, Serialize};

use crate::model::{Direction, InternalSwapTree};

pub(crate) mod sync;

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) refund_private_key: String,
    pub(crate) swap_tree: InternalSwapTree,
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
    pub(crate) swap_tree: InternalSwapTree,
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
