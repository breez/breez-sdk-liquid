use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{
    model::Bolt12Offer,
    persist::model::PaymentTxDetails,
    prelude::{ChainSwap, Direction, LnUrlInfo, PaymentState, ReceiveSwap, SendSwap, Swap},
};

pub(crate) const LAST_DERIVATION_INDEX_DATA_ID: &str = "last-derivation-index";

#[derive(Serialize, Clone, Debug)]
pub(crate) struct ChainSyncData {
    pub(crate) swap_id: String,
    pub(crate) preimage: String,
    pub(crate) pair_fees_json: String,
    pub(crate) create_response_json: String,
    pub(crate) direction: Direction,
    pub(crate) claim_address: Option<String>,
    pub(crate) lockup_address: String,
    pub(crate) claim_fees_sat: u64,
    pub(crate) claim_private_key: String,
    pub(crate) refund_private_key: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) claim_timeout_block_height: u32,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) accepted_receiver_amount_sat: Option<u64>,
    pub(crate) accept_zero_conf: bool,
    pub(crate) created_at: u32,
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) auto_accepted_fees: bool,
}

// Custom deserialization to derive claim_timeout_block_height from create_response_json when missing
impl<'de> Deserialize<'de> for ChainSyncData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawChainSyncData {
            swap_id: String,
            preimage: String,
            pair_fees_json: String,
            create_response_json: String,
            direction: Direction,
            claim_address: Option<String>,
            lockup_address: String,
            claim_fees_sat: u64,
            claim_private_key: String,
            refund_private_key: String,
            timeout_block_height: u32,
            #[serde(default)]
            claim_timeout_block_height: Option<u32>,
            payer_amount_sat: u64,
            receiver_amount_sat: u64,
            accepted_receiver_amount_sat: Option<u64>,
            accept_zero_conf: bool,
            created_at: u32,
            description: Option<String>,
            #[serde(default)]
            auto_accepted_fees: Option<bool>,
        }

        let raw = RawChainSyncData::deserialize(deserializer)?;

        // Prefer explicit field, otherwise derive from JSON. Fallback to 0.
        let derived_claim_timeout = raw
            .claim_timeout_block_height
            .or_else(|| extract_claim_timeout_block_height(&raw.create_response_json))
            .unwrap_or(0);

        Ok(ChainSyncData {
            swap_id: raw.swap_id,
            preimage: raw.preimage,
            pair_fees_json: raw.pair_fees_json,
            create_response_json: raw.create_response_json,
            direction: raw.direction,
            claim_address: raw.claim_address,
            lockup_address: raw.lockup_address,
            claim_fees_sat: raw.claim_fees_sat,
            claim_private_key: raw.claim_private_key,
            refund_private_key: raw.refund_private_key,
            timeout_block_height: raw.timeout_block_height,
            claim_timeout_block_height: derived_claim_timeout,
            payer_amount_sat: raw.payer_amount_sat,
            receiver_amount_sat: raw.receiver_amount_sat,
            accepted_receiver_amount_sat: raw.accepted_receiver_amount_sat,
            accept_zero_conf: raw.accept_zero_conf,
            created_at: raw.created_at,
            description: raw.description,
            auto_accepted_fees: raw.auto_accepted_fees.unwrap_or(false),
        })
    }
}

fn extract_claim_timeout_block_height(create_response_json: &str) -> Option<u32> {
    let value: serde_json::Value = serde_json::from_str(create_response_json).ok()?;
    let timeout = value
        .get("claim_details")?
        .get("timeoutBlockHeight")?
        .as_u64()?;
    u32::try_from(timeout).ok()
}

impl ChainSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "accept_zero_conf" => self.accept_zero_conf = other.accept_zero_conf,
                "accepted_receiver_amount_sat" => {
                    self.accepted_receiver_amount_sat = other.accepted_receiver_amount_sat
                }
                "auto_accepted_fees" => {
                    self.auto_accepted_fees = other.auto_accepted_fees;
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
                if update.auto_accepted_fees != swap.auto_accepted_fees {
                    updated_fields.push("auto_accepted_fees".to_string());
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
            claim_address: value.claim_address,
            lockup_address: value.lockup_address,
            claim_fees_sat: value.claim_fees_sat,
            claim_private_key: value.claim_private_key,
            refund_private_key: value.refund_private_key,
            timeout_block_height: value.timeout_block_height,
            claim_timeout_block_height: value.claim_timeout_block_height,
            payer_amount_sat: value.payer_amount_sat,
            receiver_amount_sat: value.receiver_amount_sat,
            accepted_receiver_amount_sat: value.accepted_receiver_amount_sat,
            accept_zero_conf: value.accept_zero_conf,
            created_at: value.created_at,
            description: value.description,
            auto_accepted_fees: value.auto_accepted_fees,
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
            claim_timeout_block_height: val.claim_timeout_block_height,
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
            claim_address: val.claim_address,
            refund_address: None,
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            auto_accepted_fees: val.auto_accepted_fees,
            metadata: Default::default(),
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
    pub(crate) preimage: Option<String>,
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) destination_pubkey: Option<String>,
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

    pub(crate) fn updated_fields(swap: Option<SendSwap>, update: &SendSwap) -> Option<Vec<String>> {
        match swap {
            Some(swap) => {
                let mut updated_fields = vec![];
                if update.preimage.is_some() && update.preimage != swap.preimage {
                    updated_fields.push("preimage".to_string());
                }
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
            preimage: value.preimage,
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
            preimage: val.preimage,
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
            refund_address: None,
            refund_tx_id: None,
            metadata: Default::default(),
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
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) payer_note: Option<String>,
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
            bolt12_offer: value.bolt12_offer,
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
            payer_note: value.payer_note,
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
            bolt12_offer: val.bolt12_offer,
            payment_hash: val.payment_hash,
            destination_pubkey: val.destination_pubkey,
            description: val.description,
            payer_note: val.payer_note,
            payer_amount_sat: val.payer_amount_sat,
            receiver_amount_sat: val.receiver_amount_sat,
            claim_fees_sat: val.claim_fees_sat,
            mrh_address: val.mrh_address,
            timeout_block_height: val.timeout_block_height,
            created_at: val.created_at,
            state: PaymentState::Created,
            claim_address: None,
            claim_tx_id: None,
            lockup_tx_id: None,
            mrh_tx_id: None,
            metadata: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct PaymentDetailsSyncData {
    pub(crate) tx_id: String,
    pub(crate) destination: String,
    pub(crate) description: Option<String>,
    pub(crate) lnurl_info: Option<LnUrlInfo>,
    pub(crate) bip353_address: Option<String>,
    pub(crate) payer_note: Option<String>,
    pub(crate) asset_fees: Option<u64>,
}

impl PaymentDetailsSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "destination" => self.destination.clone_from(&other.destination),
                "description" => clone_if_set(&mut self.description, &other.description),
                "lnurl_info" => clone_if_set(&mut self.lnurl_info, &other.lnurl_info),
                "bip353_address" => clone_if_set(&mut self.bip353_address, &other.bip353_address),
                "payer_note" => clone_if_set(&mut self.payer_note, &other.payer_note),
                "asset_fees" => self.asset_fees = other.asset_fees,
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
            bip353_address: value.bip353_address,
            payer_note: value.payer_note,
            asset_fees: value.asset_fees,
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
            bip353_address: val.bip353_address,
            payer_note: val.payer_note,
            asset_fees: val.asset_fees,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Bolt12OfferSyncData {
    pub(crate) id: String,
    pub(crate) description: String,
    pub(crate) private_key: String,
    pub(crate) webhook_url: Option<String>,
    pub(crate) created_at: u32,
}

impl Bolt12OfferSyncData {
    pub(crate) fn merge(&mut self, other: &Self, updated_fields: &[String]) {
        for field in updated_fields {
            match field.as_str() {
                "webhook_url" => self.webhook_url.clone_from(&other.webhook_url),
                _ => continue,
            }
        }
    }

    pub(crate) fn updated_fields(
        bolt12_offer: Option<Bolt12Offer>,
        update: &Bolt12Offer,
    ) -> Option<Vec<String>> {
        match bolt12_offer {
            Some(bolt12_offer) => {
                let mut updated_fields = vec![];
                if update.webhook_url != bolt12_offer.webhook_url {
                    updated_fields.push("webhook_url".to_string());
                }
                Some(updated_fields)
            }
            None => None,
        }
    }
}

impl From<Bolt12Offer> for Bolt12OfferSyncData {
    fn from(value: Bolt12Offer) -> Self {
        Self {
            id: value.id,
            description: value.description,
            private_key: value.private_key,
            webhook_url: value.webhook_url,
            created_at: value.created_at,
        }
    }
}

impl From<Bolt12OfferSyncData> for Bolt12Offer {
    fn from(val: Bolt12OfferSyncData) -> Self {
        Bolt12Offer {
            id: val.id,
            description: val.description,
            private_key: val.private_key,
            webhook_url: val.webhook_url,
            created_at: val.created_at,
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
    Bolt12Offer(Bolt12OfferSyncData),
}

impl SyncData {
    pub(crate) fn id(&self) -> &str {
        match self {
            SyncData::Chain(chain_data) => &chain_data.swap_id,
            SyncData::Send(send_data) => &send_data.swap_id,
            SyncData::Receive(receive_data) => &receive_data.swap_id,
            SyncData::LastDerivationIndex(_) => LAST_DERIVATION_INDEX_DATA_ID,
            SyncData::PaymentDetails(payment_details) => &payment_details.tx_id,
            SyncData::Bolt12Offer(bolt12_offer_data) => &bolt12_offer_data.id,
        }
    }

    pub(crate) fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    /// Whether the data is a swap
    pub(crate) fn is_swap(&self) -> bool {
        match self {
            SyncData::Bolt12Offer(_)
            | SyncData::LastDerivationIndex(_)
            | SyncData::PaymentDetails(_) => false,
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
                bail!("Merge not supported for sync data of type Receive")
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
            (SyncData::Bolt12Offer(ref mut base), SyncData::Bolt12Offer(other)) => {
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
