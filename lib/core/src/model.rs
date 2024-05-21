use anyhow::anyhow;
use boltz_client::network::Chain;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork, WolletDescriptor};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum Network {
    Liquid,
    LiquidTestnet,
}

impl From<Network> for ElementsNetwork {
    fn from(value: Network) -> Self {
        match value {
            Network::Liquid => ElementsNetwork::Liquid,
            Network::LiquidTestnet => ElementsNetwork::LiquidTestnet,
        }
    }
}

impl From<Network> for Chain {
    fn from(value: Network) -> Self {
        match value {
            Network::Liquid => Chain::Liquid,
            Network::LiquidTestnet => Chain::LiquidTestnet,
        }
    }
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Network, anyhow::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Liquid),
            "testnet" => Ok(Network::LiquidTestnet),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

#[derive(Debug)]
pub struct LiquidSdkOptions {
    pub signer: SwSigner,
    pub network: Network,
    /// Output script descriptor
    ///
    /// See <https://github.com/bitcoin/bips/pull/1143>
    pub descriptor: WolletDescriptor,
    /// Absolute or relative path to the data dir, including the dir name.
    ///
    /// If not set, it defaults to [crate::DEFAULT_DATA_DIR].
    pub data_dir_path: Option<String>,
    /// Custom Electrum URL. If set, it must match the specified network.
    ///
    /// If not set, it defaults to a Blockstream instance.
    pub electrum_url: Option<ElectrumUrl>,
}
impl LiquidSdkOptions {
    pub(crate) fn get_electrum_url(&self) -> ElectrumUrl {
        self.electrum_url.clone().unwrap_or({
            let (url, validate_domain, tls) = match &self.network {
                Network::Liquid => ("blockstream.info:995", true, true),
                Network::LiquidTestnet => ("blockstream.info:465", true, true),
            };
            ElectrumUrl::new(url, tls, validate_domain)
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ConnectRequest {
    pub mnemonic: String,
    pub data_dir: Option<String>,
    pub network: Network,
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
    pub txid: String,
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
pub struct RestoreRequest {
    pub backup_path: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum Swap {
    Send(SwapIn),
    Receive(SwapOut),
}
impl Swap {
    pub(crate) fn id(&self) -> String {
        match &self {
            Swap::Send(SwapIn { id, .. }) | Swap::Receive(SwapOut { id, .. }) => id.clone(),
        }
    }
}

/// A submarine swap, used for swap-in (Send)
#[derive(Clone, Debug)]
pub(crate) struct SwapIn {
    pub(crate) id: String,
    pub(crate) invoice: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) create_response_json: String,
    /// Persisted only when the lockup tx is successfully broadcasted
    pub(crate) lockup_tx_id: Option<String>,
    /// Whether or not the claim tx was seen in the mempool
    pub(crate) is_claim_tx_seen: bool,
    pub(crate) created_at: u32,
}
impl SwapIn {
    pub(crate) fn calculate_status(&self) -> SwapInStatus {
        match (&self.lockup_tx_id, &self.is_claim_tx_seen) {
            (None, _) => SwapInStatus::Created,
            (Some(_), false) => SwapInStatus::Pending,
            (Some(_), true) => SwapInStatus::Completed,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SwapInStatus {
    /// The swap was created, but the lockup tx was not broadcast sucecessfully.
    Created,

    /// The lockup tx was broadcasted successfully, but the claim tx was not seen in the mempool yet.
    Pending,

    /// The claim tx has been seen in the mempool.
    Completed,
}

/// A reverse swap, used for swap-out (Receive)
#[derive(Clone, Debug)]
pub(crate) struct SwapOut {
    pub(crate) id: String,
    pub(crate) preimage: String,
    pub(crate) create_response_json: String,
    pub(crate) blinding_key: String,
    pub(crate) invoice: String,
    /// The amount of the invoice
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) claim_fees_sat: u64,
    /// Persisted as soon as claim tx is broadcasted
    pub(crate) claim_tx_id: Option<String>,
    pub(crate) created_at: u32,

    /// Whether or not the claim tx is confirmed.
    ///
    /// This is a derived field (e.g. reflects data from the payments table, is not persisted when
    /// the swap is inserted or updated)
    pub(crate) is_claim_tx_confirmed: bool,
}
impl SwapOut {
    pub(crate) fn calculate_status(&self) -> SwapOutStatus {
        match self.claim_tx_id {
            None => SwapOutStatus::Created,
            Some(_) => match self.is_claim_tx_confirmed {
                false => SwapOutStatus::Pending,
                true => SwapOutStatus::Completed,
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SwapOutStatus {
    /// Reverse swap was created, but the claim tx was not yet broadcasted
    Created,

    /// The claim tx was broadcasted
    Pending,

    /// The claim tx is confirmed
    Completed,
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

    pub payment_type: PaymentType,

    /// Onchain tx status
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSwapData {
    pub swap_id: String,

    /// Swap creation timestamp
    pub created_at: u32,

    /// Amount sent by the swap payer
    pub payer_amount_sat: u64,

    /// Amount received by the swap receiver
    pub receiver_amount_sat: u64,

    /// Payment status derived from the swap status
    pub status: PaymentStatus,
}

/// Represents an SDK payment.
///
/// By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
#[derive(Debug, Clone, Serialize)]
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

    /// If a swap is associated with this payment, this represents the total fees paid by the
    /// sender. In other words, it's the delta between the amount that was sent and the amount
    /// received.
    pub fees_sat: Option<u64>,

    pub payment_type: PaymentType,

    /// Composite status representing the overall status of the payment.
    ///
    /// If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
    ///
    /// If the tx has an associated swap, this is determined by the swap status (pending or complete).
    pub status: PaymentStatus,
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
            fees_sat: swap
                .as_ref()
                .map(|s| s.payer_amount_sat - s.receiver_amount_sat),
            payment_type: tx.payment_type,
            status: match swap {
                Some(swap) => swap.status,
                None => tx.status,
            },
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
