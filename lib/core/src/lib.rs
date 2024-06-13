#[cfg(feature = "frb")]
pub(crate) mod bindings;
pub mod error;
pub(crate) mod event;
#[cfg(feature = "frb")]
pub(crate) mod frb_generated;
pub mod logger;
pub mod model;
pub mod persist;
pub(crate) mod receive_swap;
pub mod sdk;
pub(crate) mod send_swap;
pub(crate) mod swapper;
pub(crate) mod utils;
pub(crate) mod wallet;

pub use sdk_common::prelude::*;

// === FRB mirroring
//
// This section contains frb "mirroring" structs and enums.
// These are needed by the flutter bridge in order to use structs defined in an external crate.
// See <https://cjycode.com/flutter_rust_bridge/v1/feature/lang_external.html#types-in-other-crates>

use flutter_rust_bridge::frb;

#[frb(mirror(Network))]
pub enum _Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[frb(mirror(LNInvoice))]
pub struct _LNInvoice {
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

#[frb(mirror(RouteHint))]
pub struct _RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[frb(mirror(RouteHintHop))]
pub struct _RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: u64,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}

#[frb(mirror(InputType))]
pub enum _InputType {
    BitcoinAddress { address: BitcoinAddressData },
    Bolt11 { invoice: LNInvoice },
    NodeId { node_id: String },
    Url { url: String },
    LnUrlPay { data: LnUrlPayRequestData },
    LnUrlWithdraw { data: LnUrlWithdrawRequestData },
    LnUrlAuth { data: LnUrlAuthRequestData },
    LnUrlEndpointError { data: LnUrlErrorData },
}

#[frb(mirror(BitcoinAddressData))]
pub struct _BitcoinAddressData {
    pub address: String,
    pub network: crate::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[frb(mirror(LnUrlPayRequestData))]
pub struct _LnUrlPayRequestData {
    pub callback: String,
    pub min_sendable: u64,
    pub max_sendable: u64,
    pub metadata_str: String,
    pub comment_allowed: u16,
    pub domain: String,
    pub allows_nostr: bool,
    pub nostr_pubkey: Option<String>,
    pub ln_address: Option<String>,
}

// #[frb(mirror(LnUrlPayError))]
// pub enum _LnUrlPayError {
//     AlreadyPaid,
//     Generic { err: String },
//     InvalidAmount { err: String },
//     InvalidInvoice { err: String },
//     InvalidNetwork { err: String },
//     InvalidUri { err: String },
//     InvoiceExpired { err: String },
//     PaymentFailed { err: String },
//     PaymentTimeout { err: String },
//     RouteNotFound { err: String },
//     RouteTooExpensive { err: String },
//     ServiceConnectivity { err: String },
// }

use thiserror::Error;
use crate::error::PaymentError;

#[derive(Clone, Debug, Error)]
pub enum LnUrlPayError {
    /// This error is raised when attempting to pay an invoice that has already being paid.
    #[error("Invoice already paid")]
    AlreadyPaid,

    /// This error is raised when a general error occurs not specific to other error variants
    /// in this enum.
    #[error("Generic: {err}")]
    Generic { err: String },

    /// This error is raised when the amount from the parsed invoice is not set.
    #[error("Invalid amount: {err}")]
    InvalidAmount { err: String },

    /// This error is raised when the lightning invoice cannot be parsed.
    #[error("Invalid invoice: {err}")]
    InvalidInvoice { err: String },

    /// This error is raised when the lightning invoice is for a different Bitcoin network.
    #[error("Invalid network: {err}")]
    InvalidNetwork { err: String },

    /// This error is raised when the decoded LNURL URI is not compliant to the specification.
    #[error("Invalid uri: {err}")]
    InvalidUri { err: String },

    /// This error is raised when the lightning invoice has passed it's expiry time.
    #[error("Invoice expired: {err}")]
    InvoiceExpired { err: String },

    /// This error is raised when attempting to make a payment by the node fails.
    #[error("Payment failed: {err}")]
    PaymentFailed { err: String },

    /// This error is raised when attempting to make a payment takes too long.
    #[error("Payment timeout: {err}")]
    PaymentTimeout { err: String },

    /// This error is raised when no route can be found when attempting to make a
    /// payment by the node.
    #[error("Route not found: {err}")]
    RouteNotFound { err: String },

    /// This error is raised when the route is considered too expensive when
    /// attempting to make a payment by the node.
    #[error("Route too expensive: {err}")]
    RouteTooExpensive { err: String },

    /// This error is raised when a connection to an external service fails.
    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },
}
impl From<sdk_common::prelude::LnUrlPayError> for LnUrlPayError {
    fn from(value: prelude::LnUrlPayError) -> Self {
        match value {
            sdk_common::prelude::LnUrlPayError::AlreadyPaid => Self::AlreadyPaid,
            sdk_common::prelude::LnUrlPayError::Generic { err } => Self::Generic { err },
            sdk_common::prelude::LnUrlPayError::InvalidAmount { err } => Self::InvalidAmount { err },
            sdk_common::prelude::LnUrlPayError::InvalidInvoice { err } => Self::InvalidInvoice { err },
            sdk_common::prelude::LnUrlPayError::InvalidNetwork { err } => Self::InvalidNetwork { err },
            sdk_common::prelude::LnUrlPayError::InvalidUri { err } => Self::InvalidUri { err },
            sdk_common::prelude::LnUrlPayError::InvoiceExpired { err } => Self::InvoiceExpired { err },
            sdk_common::prelude::LnUrlPayError::PaymentFailed { err } => Self::PaymentFailed { err },
            sdk_common::prelude::LnUrlPayError::PaymentTimeout { err } => Self::PaymentTimeout { err },
            sdk_common::prelude::LnUrlPayError::RouteNotFound { err } => Self::RouteNotFound { err },
            sdk_common::prelude::LnUrlPayError::RouteTooExpensive { err } => Self::RouteTooExpensive { err },
            sdk_common::prelude::LnUrlPayError::ServiceConnectivity { err } => Self::ServiceConnectivity { err },
        }
    }
}

impl From<PaymentError> for sdk_common::prelude::LnUrlPayError {
    fn from(value: PaymentError) -> Self {
        Self::Generic {
            err: format!("{value}")
        }
    }
}

#[frb(mirror(LnUrlPayRequest))]
pub struct _LnUrlPayRequest {
    pub data: LnUrlPayRequestData,
    pub amount_msat: u64,
    pub comment: Option<String>,
    pub payment_label: Option<String>,
}

#[frb(mirror(SuccessActionProcessed))]
pub enum _SuccessActionProcessed {
    Aes { result: AesSuccessActionDataResult },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[frb(mirror(AesSuccessActionDataResult))]
pub enum _AesSuccessActionDataResult {
    Decrypted { data: AesSuccessActionDataDecrypted },
    ErrorStatus { reason: String },
}

#[frb(mirror(AesSuccessActionDataDecrypted))]
pub struct _AesSuccessActionDataDecrypted {
    pub description: String,
    pub plaintext: String,
}

#[frb(mirror(MessageSuccessActionData))]
pub struct _MessageSuccessActionData {
    pub message: String,
}

#[frb(mirror(UrlSuccessActionData))]
pub struct _UrlSuccessActionData {
    pub description: String,
    pub url: String,
}

#[frb(mirror(LnUrlPayErrorData))]
pub struct _LnUrlPayErrorData {
    pub payment_hash: String,
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawRequestData))]
pub struct _LnUrlWithdrawRequestData {
    pub callback: String,
    pub k1: String,
    pub default_description: String,
    pub min_withdrawable: u64,
    pub max_withdrawable: u64,
}

#[frb(mirror(LnUrlAuthRequestData))]
pub struct _LnUrlAuthRequestData {
    pub k1: String,
    pub action: Option<String>,
    pub domain: String,
    pub url: String,
}

#[frb(mirror(LnUrlErrorData))]
pub struct _LnUrlErrorData {
    pub reason: String,
}
