use serde::{Deserialize, Serialize};

use crate::prelude::{ChainSwap, Direction, PaymentState, ReceiveSwap, SendSwap, Swap};

pub(crate) const LAST_DERIVATION_INDEX_DATA_ID: &str = "last-derivation-index";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) pair_fees_json: String,
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

impl ChainSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "accept_zero_conf" => self.accept_zero_conf = other.accept_zero_conf,
                _ => continue,
            }
        }
    }
}

impl From<ChainSwap> for ChainSyncData {
    fn from(value: ChainSwap) -> Self {
        Self {
            swap_id: value.id,
            preimage: value.preimage,
            pair_fees_json: value.pair_fees_json,
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

impl From<ChainSyncData> for ChainSwap {
    fn from(val: ChainSyncData) -> Self {
        ChainSwap {
            id: val.swap_id,
            direction: val.direction,
            lockup_address: val.lockup_address,
            timeout_block_height: val.timeout_block_height,
            preimage: val.preimage,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            claim_fees_sat: val.claim_fees_sat,
            accept_zero_conf: val.accept_zero_conf,
            pair_fees_json: val.pair_fees_json,
            create_response_json: val.create_response_json,
            created_at: val.created_at,
            claim_private_key: val.claim_private_key,
            refund_private_key: val.refund_private_key,
            state: PaymentState::Created,
            claim_address: None,
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SendSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) pair_fees_json: String,
    pub(crate) create_response_json: String,
    pub(crate) refund_private_key: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) created_at: u32,
    pub(crate) preimage: Option<String>,
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
}

impl SendSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "preimage" => clone_if_set(&mut self.preimage, &other.preimage),
                _ => continue,
            }
        }
    }
}

impl From<SendSwap> for SendSyncData {
    fn from(value: SendSwap) -> Self {
        Self {
            swap_id: value.id,
            payment_hash: value.payment_hash,
            invoice: value.invoice,
            pair_fees_json: value.pair_fees_json,
            create_response_json: value.create_response_json,
            refund_private_key: value.refund_private_key,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            created_at: value.created_at,
            preimage: value.preimage,
            description: value.description,
            bolt12_offer: value.bolt12_offer,
        }
    }
}

impl From<SendSyncData> for SendSwap {
    fn from(val: SendSyncData) -> Self {
        SendSwap {
            id: val.swap_id,
            invoice: val.invoice,
            payment_hash: val.payment_hash,
            description: val.description,
            preimage: val.preimage,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            pair_fees_json: val.pair_fees_json,
            create_response_json: val.create_response_json,
            created_at: val.created_at,
            refund_private_key: val.refund_private_key,
            bolt12_offer: val.bolt12_offer,
            state: PaymentState::Created,
            lockup_tx_id: None,
            refund_tx_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ReceiveSyncData {
    pub(crate) swap_id: String,
    pub(crate) invoice: String,
    pub(crate) preimage: String,
    pub(crate) pair_fees_json: String,
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
            pair_fees_json: value.pair_fees_json,
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

impl From<ReceiveSyncData> for ReceiveSwap {
    fn from(val: ReceiveSyncData) -> Self {
        ReceiveSwap {
            id: val.swap_id,
            preimage: val.preimage,
            create_response_json: val.create_response_json,
            pair_fees_json: val.pair_fees_json,
            claim_private_key: val.claim_private_key,
            invoice: val.invoice,
            payment_hash: val.payment_hash,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            claim_fees_sat: val.claim_fees_sat,
            mrh_address: val.mrh_address,
            created_at: val.created_at,
            state: PaymentState::Created,
            claim_tx_id: None,
            lockup_tx_id: None,
            mrh_tx_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "data_type", content = "data")]
pub(crate) enum SyncData {
    Chain(ChainSyncData),
    Send(SendSyncData),
    Receive(ReceiveSyncData),
    LastDerivationIndex(u32),
}

impl SyncData {
    pub(crate) fn id(&self) -> &str {
        match self {
            SyncData::Chain(chain_data) => &chain_data.swap_id,
            SyncData::Send(send_data) => &send_data.swap_id,
            SyncData::Receive(receive_data) => &receive_data.swap_id,
            SyncData::LastDerivationIndex(_) => LAST_DERIVATION_INDEX_DATA_ID,
        }
    }

    pub(crate) fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    /// Whether the data is a swap
    pub(crate) fn is_swap(&self) -> bool {
        match self {
            SyncData::LastDerivationIndex(_) => false,
            SyncData::Chain(_) | SyncData::Send(_) | SyncData::Receive(_) => true,
        }
    }

    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) -> anyhow::Result<()> {
        match (self, other) {
            (SyncData::Chain(ref mut base), SyncData::Chain(other)) => {
                base.merge(other, updated_fields)
            }
            (SyncData::Send(ref mut base), SyncData::Send(other)) => {
                base.merge(other, updated_fields)
            }
            (SyncData::Receive(ref mut _base), SyncData::Receive(_other)) => {
                log::warn!("Attempting to merge for unnecessary type SyncData::Receive");
            }
            (
                SyncData::LastDerivationIndex(our_index),
                SyncData::LastDerivationIndex(their_index),
            ) => {
                *our_index = std::cmp::max(*their_index, *our_index);
            }
            _ => return Err(anyhow::anyhow!("Cannot merge data from two separate types")),
        };
        Ok(())
    }
}

impl TryInto<Swap> for SyncData {
    type Error = anyhow::Error;
    fn try_into(self) -> std::result::Result<Swap, Self::Error> {
        match self {
            SyncData::Chain(chain_data) => Ok(Swap::Chain(chain_data.into())),
            SyncData::Send(send_data) => Ok(Swap::Send(send_data.into())),
            SyncData::Receive(receive_data) => Ok(Swap::Receive(receive_data.into())),
            _ => Err(anyhow::anyhow!(
                "Cannot convert this sync data type to a swap"
            )),
        }
    }
}

fn clone_if_set<T: Clone>(s: &mut Option<T>, other: &Option<T>) {
    if other.is_some() {
        s.clone_from(other)
    }
}
