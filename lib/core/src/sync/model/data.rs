use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{
    persist::model::PaymentTxDetails,
    prelude::{ChainSwap, Direction, LnUrlInfo, PaymentState, ReceiveSwap, SendSwap, Swap},
};

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
    pub(crate) accepted_receiver_amount_sat: Option<u64>,
    pub(crate) accept_zero_conf: bool,
    pub(crate) created_at: u32,
    pub(crate) description: Option<String>,
}

impl ChainSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "accept_zero_conf" => self.accept_zero_conf = other.accept_zero_conf,
                "accepted_receiver_amount_sat" => {
                    self.accepted_receiver_amount_sat = other.accepted_receiver_amount_sat
                }
                _ => continue,
            }
        }
    }

    pub(crate) fn updated_fields(
        swap: Option<ChainSwap>,
        update: &ChainSwap,
    ) -> Option<Vec<String>> {
        match swap {
            Some(swap) => {
                let mut updated_fields = vec![];
                if update.accept_zero_conf != swap.accept_zero_conf {
                    updated_fields.push("accept_zero_conf".to_string());
                }
                if update.accepted_receiver_amount_sat != swap.accepted_receiver_amount_sat {
                    updated_fields.push("accepted_receiver_amount_sat".to_string());
                }
                Some(updated_fields)
            }
            None => None,
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
            accepted_receiver_amount_sat: value.accepted_receiver_amount_sat,
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
            actual_payer_amount_sat: None,
            receiver_amount_sat: val.receiver_amount_sat,
            accepted_receiver_amount_sat: val.accepted_receiver_amount_sat,
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
    #[serde(default)]
    pub(crate) timeout_block_height: u64,
    pub(crate) created_at: u32,
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) destination_pubkey: Option<String>,
}

impl SendSyncData {
    pub(crate) fn updated_fields(
        swap: Option<SendSwap>,
        _update: &SendSwap,
    ) -> Option<Vec<String>> {
        match swap {
            Some(_swap) => {
                let updated_fields = vec![];
                Some(updated_fields)
            }
            None => None,
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
            timeout_block_height: value.timeout_block_height,
            created_at: value.created_at,
            description: value.description,
            bolt12_offer: value.bolt12_offer,
            destination_pubkey: value.destination_pubkey,
        }
    }
}

impl From<SendSyncData> for SendSwap {
    fn from(val: SendSyncData) -> Self {
        SendSwap {
            id: val.swap_id,
            invoice: val.invoice,
            payment_hash: val.payment_hash,
            destination_pubkey: val.destination_pubkey,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            pair_fees_json: val.pair_fees_json,
            create_response_json: val.create_response_json,
            created_at: val.created_at,
            timeout_block_height: val.timeout_block_height,
            refund_private_key: val.refund_private_key,
            bolt12_offer: val.bolt12_offer,
            state: PaymentState::Created,
            lockup_tx_id: None,
            refund_tx_id: None,
            preimage: None,
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
    #[serde(default)]
    pub(crate) timeout_block_height: u32,
    pub(crate) created_at: u32,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) destination_pubkey: Option<String>,
}

impl ReceiveSyncData {
    pub(crate) fn updated_fields(
        swap: Option<ReceiveSwap>,
        _update: &ReceiveSwap,
    ) -> Option<Vec<String>> {
        match swap {
            Some(_swap) => {
                let updated_fields = vec![];
                Some(updated_fields)
            }
            None => None,
        }
    }
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
            timeout_block_height: value.timeout_block_height,
            created_at: value.created_at,
            description: value.description,
            destination_pubkey: value.destination_pubkey,
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
            destination_pubkey: val.destination_pubkey,
            description: val.description,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            claim_fees_sat: val.claim_fees_sat,
            mrh_address: val.mrh_address,
            timeout_block_height: val.timeout_block_height,
            created_at: val.created_at,
            state: PaymentState::Created,
            claim_tx_id: None,
            lockup_tx_id: None,
            mrh_tx_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct PaymentDetailsSyncData {
    pub(crate) tx_id: String,
    pub(crate) destination: String,
    pub(crate) description: Option<String>,
    pub(crate) lnurl_info: Option<LnUrlInfo>,
}

impl PaymentDetailsSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "destination" => self.destination.clone_from(&other.destination),
                "description" => clone_if_set(&mut self.description, &other.description),
                "lnurl_info" => clone_if_set(&mut self.lnurl_info, &other.lnurl_info),
                _ => continue,
            }
        }
    }
}

impl From<PaymentTxDetails> for PaymentDetailsSyncData {
    fn from(value: PaymentTxDetails) -> Self {
        Self {
            tx_id: value.tx_id,
            destination: value.destination,
            description: value.description,
            lnurl_info: value.lnurl_info,
        }
    }
}

impl From<PaymentDetailsSyncData> for PaymentTxDetails {
    fn from(val: PaymentDetailsSyncData) -> Self {
        PaymentTxDetails {
            tx_id: val.tx_id,
            destination: val.destination,
            description: val.description,
            lnurl_info: val.lnurl_info,
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
    PaymentDetails(PaymentDetailsSyncData),
}

impl SyncData {
    pub(crate) fn id(&self) -> &str {
        match self {
            SyncData::Chain(chain_data) => &chain_data.swap_id,
            SyncData::Send(send_data) => &send_data.swap_id,
            SyncData::Receive(receive_data) => &receive_data.swap_id,
            SyncData::LastDerivationIndex(_) => LAST_DERIVATION_INDEX_DATA_ID,
            SyncData::PaymentDetails(payment_details) => &payment_details.tx_id,
        }
    }

    pub(crate) fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    /// Whether the data is a swap
    pub(crate) fn is_swap(&self) -> bool {
        match self {
            SyncData::LastDerivationIndex(_) | SyncData::PaymentDetails(_) => false,
            SyncData::Chain(_) | SyncData::Send(_) | SyncData::Receive(_) => true,
        }
    }

    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) -> anyhow::Result<()> {
        match (self, other) {
            (SyncData::Chain(ref mut base), SyncData::Chain(other)) => {
                base.merge(other, updated_fields)
            }
            (SyncData::Send(_), SyncData::Send(_))
            | (SyncData::Receive(_), SyncData::Receive(_)) => {
                bail!("Merge not supported for sync data of type `Receive` and `Send`")
            }
            (
                SyncData::LastDerivationIndex(our_index),
                SyncData::LastDerivationIndex(their_index),
            ) => {
                *our_index = std::cmp::max(*their_index, *our_index);
            }
            (SyncData::PaymentDetails(ref mut base), SyncData::PaymentDetails(other)) => {
                base.merge(other, updated_fields)
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
