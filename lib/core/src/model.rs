use anyhow::{anyhow, Result};
use boltz_client::network::Chain;
use boltz_client::swaps::boltzv2::{
    CreateReverseResponse, CreateSubmarineResponse, Leaf, SwapTree, BOLTZ_MAINNET_URL_V2,
    BOLTZ_TESTNET_URL_V2,
};
use boltz_client::{Keypair, LBtcSwapScriptV2, ToHex};
use lwk_wollet::ElementsNetwork;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use crate::error::PaymentError;
use crate::utils;

/// Configuration for the Liquid SDK
#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub boltz_url: String,
    pub electrum_url: String,
    /// Directory in which all SDK files (DB, log, cache) are stored.
    ///
    /// Prefix can be a relative or absolute path to this directory.
    pub working_dir: String,
    pub network: Network,
    /// Send payment timeout. See [crate::sdk::LiquidSdk::send_payment]
    pub payment_timeout_sec: u64,
}
impl Config {
    pub fn mainnet() -> Self {
        Config {
            boltz_url: BOLTZ_MAINNET_URL_V2.to_owned(),
            electrum_url: "blockstream.info:995".to_string(),
            working_dir: ".".to_string(),
            network: Network::Mainnet,
            payment_timeout_sec: 15,
        }
    }

    pub fn testnet() -> Self {
        Config {
            boltz_url: BOLTZ_TESTNET_URL_V2.to_owned(),
            electrum_url: "blockstream.info:465".to_string(),
            working_dir: ".".to_string(),
            network: Network::Testnet,
            payment_timeout_sec: 15,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum Network {
    /// Mainnet Bitcoin and Liquid chains
    Mainnet,
    /// Testnet Bitcoin and Liquid chains
    Testnet,
}

impl From<Network> for ElementsNetwork {
    fn from(value: Network) -> Self {
        match value {
            Network::Mainnet => ElementsNetwork::Liquid,
            Network::Testnet => ElementsNetwork::LiquidTestnet,
        }
    }
}

impl From<Network> for Chain {
    fn from(value: Network) -> Self {
        match value {
            Network::Mainnet => Chain::Liquid,
            Network::Testnet => Chain::LiquidTestnet,
        }
    }
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Network, anyhow::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Mainnet),
            "testnet" => Ok(Network::Testnet),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

impl TryFrom<boltz_client::lightning_invoice::Currency> for Network {
    type Error = anyhow::Error;

    fn try_from(
        value: boltz_client::lightning_invoice::Currency,
    ) -> Result<Network, anyhow::Error> {
        match value {
            boltz_client::lightning_invoice::Currency::Bitcoin => Ok(Network::Mainnet),
            boltz_client::lightning_invoice::Currency::BitcoinTestnet => Ok(Network::Testnet),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

/// Trait that can be used to react to various [LiquidSdkEvent]s emitted by the SDK.
pub trait EventListener: Send + Sync {
    fn on_event(&self, e: LiquidSdkEvent);
}

/// Event emitted by the SDK. To listen for and react to these events, use an [EventListener] when
/// initializing the [LiquidSdk].
#[derive(Clone, Debug, PartialEq)]
pub enum LiquidSdkEvent {
    PaymentFailed { details: Payment },
    PaymentPending { details: Payment },
    PaymentRefunded { details: Payment },
    PaymentRefundPending { details: Payment },
    PaymentSucceeded { details: Payment },
    PaymentWaitingConfirmation { details: Payment },
    Synced,
}

#[derive(Debug, Serialize)]
pub struct ConnectRequest {
    pub mnemonic: String,
    pub config: Config,
}

#[derive(Debug, Serialize)]
pub struct PrepareReceiveRequest {
    pub payer_amount_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct PrepareReceiveResponse {
    pub payer_amount_sat: u64,
    pub fees_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    pub id: String,
    pub invoice: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendRequest {
    pub invoice: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendResponse {
    pub invoice: String,
    pub fees_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct SendPaymentResponse {
    pub payment: Payment,
}

#[derive(Debug, Serialize)]
pub struct GetInfoRequest {
    pub with_scan: bool,
}

#[derive(Debug, Serialize)]
pub struct GetInfoResponse {
    /// Usable balance. This is the confirmed onchain balance minus `pending_send_sat`.
    pub balance_sat: u64,
    /// Amount that is being used for ongoing Send swaps
    pub pending_send_sat: u64,
    /// Incoming amount that is pending from ongoing Receive swaps
    pub pending_receive_sat: u64,
    pub pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct BackupRequest {
    /// Path to the backup.
    ///
    /// If not set, it defaults to `backup.sql` for mainnet and `backup-testnet.sql` for testnet.
    /// The file will be saved in [ConnectRequest]'s `data_dir`.
    pub backup_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RestoreRequest {
    pub backup_path: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum Swap {
    Send(SendSwap),
    Receive(ReceiveSwap),
}
impl Swap {
    pub(crate) fn id(&self) -> String {
        match &self {
            Swap::Send(SendSwap { id, .. }) | Swap::Receive(ReceiveSwap { id, .. }) => id.clone(),
        }
    }
}

/// A submarine swap, used for Send
#[derive(Clone, Debug)]
pub(crate) struct SendSwap {
    pub(crate) id: String,
    pub(crate) invoice: String,
    pub(crate) preimage: Option<String>,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    /// JSON representation of [crate::persist::send::InternalCreateSubmarineResponse]
    pub(crate) create_response_json: String,
    /// Persisted only when the lockup tx is successfully broadcast
    pub(crate) lockup_tx_id: Option<String>,
    /// Persisted as soon as a refund tx is broadcast
    pub(crate) refund_tx_id: Option<String>,
    pub(crate) created_at: u32,
    pub(crate) state: PaymentState,
    pub(crate) refund_private_key: String,
}
impl SendSwap {
    pub(crate) fn get_refund_keypair(&self) -> Result<Keypair, PaymentError> {
        utils::decode_keypair(&self.refund_private_key).map_err(Into::into)
    }

    pub(crate) fn get_boltz_create_response(&self) -> Result<CreateSubmarineResponse> {
        let internal_create_response: crate::persist::send::InternalCreateSubmarineResponse =
            serde_json::from_str(&self.create_response_json).map_err(|e| {
                anyhow!("Failed to deserialize InternalCreateSubmarineResponse: {e:?}")
            })?;

        let res = CreateSubmarineResponse {
            id: self.id.clone(),
            accept_zero_conf: internal_create_response.accept_zero_conf,
            address: internal_create_response.address.clone(),
            bip21: internal_create_response.bip21.clone(),
            claim_public_key: crate::utils::json_to_pubkey(
                &internal_create_response.claim_public_key,
            )?,
            expected_amount: internal_create_response.expected_amount,
            swap_tree: internal_create_response.swap_tree.clone().into(),
            blinding_key: internal_create_response.blinding_key.clone(),
        };
        Ok(res)
    }

    pub(crate) fn get_swap_script(&self) -> Result<LBtcSwapScriptV2, PaymentError> {
        LBtcSwapScriptV2::submarine_from_swap_resp(
            &self.get_boltz_create_response()?,
            self.get_refund_keypair()?.public_key().into(),
        )
        .map_err(|e| PaymentError::Generic {
            err: format!(
                "Failed to create swap script for Send Swap {}: {e:?}",
                self.id
            ),
        })
    }

    pub(crate) fn from_boltz_struct_to_json(
        create_response: &CreateSubmarineResponse,
        expected_swap_id: &str,
    ) -> Result<String, PaymentError> {
        let internal_create_response =
            crate::persist::send::InternalCreateSubmarineResponse::try_convert_from_boltz(
                create_response,
                expected_swap_id,
            )?;

        let create_response_json =
            serde_json::to_string(&internal_create_response).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to serialize InternalCreateSubmarineResponse: {e:?}"),
                }
            })?;

        Ok(create_response_json)
    }
}

/// A reverse swap, used for Receive
#[derive(Clone, Debug)]
pub(crate) struct ReceiveSwap {
    pub(crate) id: String,
    pub(crate) preimage: String,
    /// JSON representation of [crate::persist::receive::InternalCreateReverseResponse]
    pub(crate) create_response_json: String,
    pub(crate) claim_private_key: String,
    pub(crate) invoice: String,
    /// The amount of the invoice
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) claim_fees_sat: u64,
    /// Persisted as soon as a claim tx is broadcast
    pub(crate) claim_tx_id: Option<String>,
    pub(crate) created_at: u32,
    pub(crate) state: PaymentState,
}
impl ReceiveSwap {
    pub(crate) fn get_claim_keypair(&self) -> Result<Keypair, PaymentError> {
        utils::decode_keypair(&self.claim_private_key).map_err(Into::into)
    }

    pub(crate) fn get_boltz_create_response(&self) -> Result<CreateReverseResponse, PaymentError> {
        let internal_create_response: crate::persist::receive::InternalCreateReverseResponse =
            serde_json::from_str(&self.create_response_json).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to deserialize InternalCreateReverseResponse: {e:?}"),
                }
            })?;

        let res = CreateReverseResponse {
            id: self.id.clone(),
            invoice: self.invoice.clone(),
            swap_tree: internal_create_response.swap_tree.clone().into(),
            lockup_address: internal_create_response.lockup_address.clone(),
            refund_public_key: crate::utils::json_to_pubkey(
                &internal_create_response.refund_public_key,
            )?,
            timeout_block_height: internal_create_response.timeout_block_height,
            onchain_amount: internal_create_response.onchain_amount,
            blinding_key: internal_create_response.blinding_key.clone(),
        };
        Ok(res)
    }

    pub(crate) fn get_swap_script(&self) -> Result<LBtcSwapScriptV2, PaymentError> {
        let keypair = self.get_claim_keypair()?;
        let create_response =
            self.get_boltz_create_response()
                .map_err(|e| PaymentError::Generic {
                    err: format!(
                        "Failed to create swap script for Receive Swap {}: {e:?}",
                        self.id
                    ),
                })?;
        LBtcSwapScriptV2::reverse_from_swap_resp(&create_response, keypair.public_key().into())
            .map_err(|e| PaymentError::Generic {
                err: format!(
                    "Failed to create swap script for Receive Swap {}: {e:?}",
                    self.id
                ),
            })
    }

    pub(crate) fn from_boltz_struct_to_json(
        create_response: &CreateReverseResponse,
        expected_swap_id: &str,
        expected_invoice: &str,
    ) -> Result<String, PaymentError> {
        let internal_create_response =
            crate::persist::receive::InternalCreateReverseResponse::try_convert_from_boltz(
                create_response,
                expected_swap_id,
                expected_invoice,
            )?;

        let create_response_json =
            serde_json::to_string(&internal_create_response).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to serialize InternalCreateReverseResponse: {e:?}"),
                }
            })?;

        Ok(create_response_json)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum PaymentState {
    Created = 0,

    /// ## Receive Swaps
    ///
    /// Covers the cases when
    /// - the lockup tx is seen in the mempool or
    /// - our claim tx is broadcast
    ///
    /// When the claim tx is broadcast, `claim_tx_id` is set in the swap.
    ///
    /// ## Send Swaps
    ///
    /// Covers the cases when
    /// - our lockup tx was broadcast or
    /// - a refund was initiated and our refund tx was broadcast
    ///
    /// When the refund tx is broadcast, `refund_tx_id` is set in the swap.
    ///
    /// ## No swap data available
    ///
    /// If no associated swap is found, this indicates the underlying tx is not confirmed yet.
    Pending = 1,

    /// ## Receive Swaps
    ///
    /// Covers the case when the claim tx is confirmed.
    ///
    /// ## Send Swaps
    ///
    /// This is the status when the claim tx is broadcast and we see it in the mempool.
    ///
    /// ## No swap data available
    ///
    /// If no associated swap is found, this indicates the underlying tx is confirmed.
    Complete = 2,

    /// ## Receive Swaps
    ///
    /// This is the status when the swap failed for any reason and the Receive could not complete.
    ///
    /// ## Send Swaps
    ///
    /// This is the status when a swap refund was initiated and the refund tx is confirmed.
    Failed = 3,

    /// ## Send Swaps
    ///
    /// This covers the case when the swap state is still Created and the swap fails to reach the
    /// Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
    TimedOut = 4,
}
impl ToSql for PaymentState {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentState::Created),
                1 => Ok(PaymentState::Pending),
                2 => Ok(PaymentState::Complete),
                3 => Ok(PaymentState::Failed),
                4 => Ok(PaymentState::TimedOut),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum PaymentType {
    Receive = 0,
    Send = 1,
}
impl ToSql for PaymentType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentType::Receive),
                1 => Ok(PaymentType::Send),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PaymentStatus {
    Pending = 0,
    Complete = 1,
}
impl ToSql for PaymentStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentStatus::Pending),
                1 => Ok(PaymentStatus::Complete),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentTxData {
    /// The tx ID of the transaction
    pub tx_id: String,

    /// The point in time when the underlying tx was included in a block.
    pub timestamp: Option<u32>,

    /// The onchain tx amount.
    ///
    /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
    pub amount_sat: u64,

    /// The onchain fees of this tx
    pub fees_sat: u64,

    pub payment_type: PaymentType,

    /// Onchain tx status
    pub is_confirmed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSwapData {
    pub swap_id: String,

    /// Swap creation timestamp
    pub created_at: u32,

    pub preimage: Option<String>,

    /// Amount sent by the swap payer
    pub payer_amount_sat: u64,

    /// Amount received by the swap receiver
    pub receiver_amount_sat: u64,

    pub refund_tx_id: Option<String>,
    pub refund_tx_amount_sat: Option<u64>,

    /// Payment status derived from the swap status
    pub status: PaymentState,
}

/// Represents an SDK payment.
///
/// By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Payment {
    /// The tx ID of the onchain transaction
    pub tx_id: String,

    /// The swap ID, if any swap is associated with this payment
    pub swap_id: Option<String>,

    /// Composite timestamp that can be used for sorting or displaying the payment.
    ///
    /// If this payment has an associated swap, it is the swap creation time. Otherwise, the point
    /// in time when the underlying tx was included in a block. If there is no associated swap
    /// available and the underlying tx is not yet confirmed, the value is `now()`.
    pub timestamp: u32,

    /// The payment amount, which corresponds to the onchain tx amount.
    ///
    /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
    pub amount_sat: u64,

    /// Represents the fees paid by this wallet for this payment.
    ///
    /// ### Swaps
    /// If there is an associated Send Swap, these fees represent the total fees paid by this wallet
    /// (the sender). It is the difference between the amount that was sent and the amount received.
    ///
    /// If there is an associated Receive Swap, these fees represent the total fees paid by this wallet
    /// (the receiver). It is also the difference between the amount that was sent and the amount received.
    ///
    /// ### Pure onchain txs
    /// If no swap is associated with this payment:
    /// - for Send payments, this is the onchain tx fee
    /// - for Receive payments, this is zero
    pub fees_sat: u64,

    /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
    pub preimage: Option<String>,

    /// For a Send swap which was refunded, this is the refund tx id
    pub refund_tx_id: Option<String>,

    /// For a Send swap which was refunded, this is the refund amount
    pub refund_tx_amount_sat: Option<u64>,

    pub payment_type: PaymentType,

    /// Composite status representing the overall status of the payment.
    ///
    /// If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
    ///
    /// If the tx has an associated swap, this is determined by the swap status (pending or complete).
    pub status: PaymentState,
}
impl Payment {
    pub(crate) fn from(tx: PaymentTxData, swap: Option<PaymentSwapData>) -> Payment {
        Payment {
            tx_id: tx.tx_id,
            swap_id: swap.as_ref().map(|s| s.swap_id.clone()),
            timestamp: match swap {
                Some(ref swap) => swap.created_at,
                None => tx.timestamp.unwrap_or(utils::now()),
            },
            amount_sat: tx.amount_sat,
            fees_sat: match swap.as_ref() {
                Some(s) => s.payer_amount_sat - s.receiver_amount_sat,
                None => match tx.payment_type {
                    PaymentType::Receive => 0,
                    PaymentType::Send => tx.fees_sat,
                },
            },
            preimage: swap.as_ref().and_then(|s| s.preimage.clone()),
            refund_tx_id: swap.as_ref().and_then(|s| s.refund_tx_id.clone()),
            refund_tx_amount_sat: swap.as_ref().and_then(|s| s.refund_tx_amount_sat),
            payment_type: tx.payment_type,
            status: match swap {
                Some(swap) => swap.status,
                None => match tx.is_confirmed {
                    true => PaymentState::Complete,
                    false => PaymentState::Pending,
                },
            },
        }
    }
}

/// Internal SDK log entry used in the Uniffi and Dart bindings
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

/// Wrapper for a BOLT11 LN invoice
#[derive(Clone, Debug, PartialEq)]
pub struct LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

/// A route hint for a LN payment
#[derive(Clone, Debug, PartialEq)]
pub struct RouteHint {
    pub hops: Vec<RouteHintHop>,
}

impl RouteHint {
    pub fn from_ldk_hint(hint: &boltz_client::lightning_invoice::RouteHint) -> RouteHint {
        let mut hops = Vec::new();
        for hop in hint.0.iter() {
            let pubkey_res = hop.src_node_id.serialize().to_hex();

            let router_hop = RouteHintHop {
                src_node_id: pubkey_res,
                short_channel_id: hop.short_channel_id,
                fees_base_msat: hop.fees.base_msat,
                fees_proportional_millionths: hop.fees.proportional_millionths,
                cltv_expiry_delta: u64::from(hop.cltv_expiry_delta),
                htlc_minimum_msat: hop.htlc_minimum_msat,
                htlc_maximum_msat: hop.htlc_maximum_msat,
            };
            hops.push(router_hop);
        }
        RouteHint { hops }
    }
}

/// Details of a specific hop in a larger route hint
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHintHop {
    /// The node_id of the non-target end of the route
    pub src_node_id: String,
    /// The short_channel_id of this channel
    pub short_channel_id: u64,
    /// The fees which must be paid to use this channel
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,

    /// The difference in CLTV values between this node and the next node.
    pub cltv_expiry_delta: u64,
    /// The minimum value, in msat, which must be relayed to the next hop.
    pub htlc_minimum_msat: Option<u64>,
    /// The maximum value in msat available for routing with a single HTLC.
    pub htlc_maximum_msat: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InternalLeaf {
    pub output: String,
    pub version: u8,
}
impl From<InternalLeaf> for Leaf {
    fn from(value: InternalLeaf) -> Self {
        Leaf {
            output: value.output,
            version: value.version,
        }
    }
}
impl From<Leaf> for InternalLeaf {
    fn from(value: Leaf) -> Self {
        InternalLeaf {
            output: value.output,
            version: value.version,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct InternalSwapTree {
    claim_leaf: InternalLeaf,
    refund_leaf: InternalLeaf,
}
impl From<InternalSwapTree> for SwapTree {
    fn from(value: InternalSwapTree) -> Self {
        SwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}
impl From<SwapTree> for InternalSwapTree {
    fn from(value: SwapTree) -> Self {
        InternalSwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}

#[macro_export]
macro_rules! get_invoice_amount {
    ($invoice:expr) => {
        $invoice
            .parse::<Bolt11Invoice>()
            .expect("Expecting valid invoice")
            .amount_milli_satoshis()
            .expect("Expecting valid amount")
            / 1000
    };
}
