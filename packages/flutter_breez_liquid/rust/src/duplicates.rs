/// External structs that cannot be mirrored for FRB, so are therefore duplicated instead
use sdk_common::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum LnUrlPayError {
    /// This error is raised when attempting to pay an invoice that has already being paid.
    #[error("Invoice already paid")]
    AlreadyPaid,

    /// This error is raised when a general error occurs not specific to other error variants
    /// in this enum.
    #[error("Generic: {err}")]
    Generic { err: String },

    /// This error is raised when the node does not have enough funds to make the payment.
    #[error("Insufficient balance: {err}")]
    InsufficientBalance { err: String },

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
    fn from(value: sdk_common::prelude::LnUrlPayError) -> Self {
        match value {
            sdk_common::prelude::LnUrlPayError::AlreadyPaid => Self::AlreadyPaid,
            sdk_common::prelude::LnUrlPayError::Generic { err } => Self::Generic { err },
            sdk_common::prelude::LnUrlPayError::InsufficientBalance { err } => {
                Self::InsufficientBalance { err }
            }
            sdk_common::prelude::LnUrlPayError::InvalidAmount { err } => {
                Self::InvalidAmount { err }
            }
            sdk_common::prelude::LnUrlPayError::InvalidInvoice { err } => {
                Self::InvalidInvoice { err }
            }
            sdk_common::prelude::LnUrlPayError::InvalidNetwork { err } => {
                Self::InvalidNetwork { err }
            }
            sdk_common::prelude::LnUrlPayError::InvalidUri { err } => Self::InvalidUri { err },
            sdk_common::prelude::LnUrlPayError::InvoiceExpired { err } => {
                Self::InvoiceExpired { err }
            }
            sdk_common::prelude::LnUrlPayError::PaymentFailed { err } => {
                Self::PaymentFailed { err }
            }
            sdk_common::prelude::LnUrlPayError::PaymentTimeout { err } => {
                Self::PaymentTimeout { err }
            }
            sdk_common::prelude::LnUrlPayError::RouteNotFound { err } => {
                Self::RouteNotFound { err }
            }
            sdk_common::prelude::LnUrlPayError::RouteTooExpensive { err } => {
                Self::RouteTooExpensive { err }
            }
            sdk_common::prelude::LnUrlPayError::ServiceConnectivity { err } => {
                Self::ServiceConnectivity { err }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum LnUrlWithdrawError {
    /// This error is raised when a general error occurs not specific to other error variants
    /// in this enum.
    #[error("Generic: {err}")]
    Generic { err: String },

    /// This error is raised when the amount is zero or the amount does not cover
    /// the cost to open a new channel.
    #[error("Invalid amount: {err}")]
    InvalidAmount { err: String },

    /// This error is raised when the lightning invoice cannot be parsed.
    #[error("Invalid invoice: {err}")]
    InvalidInvoice { err: String },

    /// This error is raised when the decoded LNURL URI is not compliant to the specification.
    #[error("Invalid uri: {err}")]
    InvalidUri { err: String },

    /// This error is raised when no routing hints were able to be added to the invoice
    /// while trying to receive a payment.
    #[error("No routing hints: {err}")]
    InvoiceNoRoutingHints { err: String },

    /// This error is raised when a connection to an external service fails.
    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },
}

impl From<sdk_common::prelude::LnUrlWithdrawError> for LnUrlWithdrawError {
    fn from(value: sdk_common::prelude::LnUrlWithdrawError) -> Self {
        match value {
            sdk_common::prelude::LnUrlWithdrawError::Generic { err } => Self::Generic { err },
            sdk_common::prelude::LnUrlWithdrawError::InvalidAmount { err } => {
                Self::InvalidAmount { err }
            }
            sdk_common::prelude::LnUrlWithdrawError::InvalidInvoice { err } => {
                Self::InvalidInvoice { err }
            }
            sdk_common::prelude::LnUrlWithdrawError::InvalidUri { err } => Self::InvalidUri { err },
            sdk_common::prelude::LnUrlWithdrawError::InvoiceNoRoutingHints { err } => {
                Self::InvoiceNoRoutingHints { err }
            }
            sdk_common::prelude::LnUrlWithdrawError::ServiceConnectivity { err } => {
                Self::ServiceConnectivity { err }
            }
        }
    }
}

#[derive(Clone, Serialize)]
pub enum LnUrlWithdrawResult {
    Ok { data: LnUrlWithdrawSuccessData },
    Timeout { data: LnUrlWithdrawSuccessData },
    ErrorStatus { data: LnUrlErrorData },
}
impl From<sdk_common::prelude::LnUrlWithdrawResult> for LnUrlWithdrawResult {
    fn from(value: sdk_common::prelude::LnUrlWithdrawResult) -> Self {
        match value {
            sdk_common::prelude::LnUrlWithdrawResult::Ok { data } => Self::Ok { data: data.into() },
            sdk_common::prelude::LnUrlWithdrawResult::Timeout { data } => {
                Self::Timeout { data: data.into() }
            }
            sdk_common::prelude::LnUrlWithdrawResult::ErrorStatus { data } => {
                Self::ErrorStatus { data }
            }
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct LnUrlWithdrawSuccessData {
    pub invoice: LNInvoice,
}
impl From<sdk_common::prelude::LnUrlWithdrawSuccessData> for LnUrlWithdrawSuccessData {
    fn from(value: sdk_common::prelude::LnUrlWithdrawSuccessData) -> Self {
        Self {
            invoice: value.invoice,
        }
    }
}

#[derive(Debug, Error)]
pub enum LnUrlAuthError {
    /// This error is raised when a general error occurs not specific to other error variants
    /// in this enum.
    #[error("Generic: {err}")]
    Generic { err: String },

    /// This error is raised when the decoded LNURL URI is not compliant to the specification.
    #[error("Invalid uri: {err}")]
    InvalidUri { err: String },

    /// This error is raised when a connection to an external service fails.
    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },
}
impl From<sdk_common::prelude::LnUrlAuthError> for LnUrlAuthError {
    fn from(value: prelude::LnUrlAuthError) -> Self {
        match value {
            sdk_common::prelude::LnUrlAuthError::Generic { err } => Self::Generic { err },
            sdk_common::prelude::LnUrlAuthError::InvalidUri { err } => Self::InvalidUri { err },
            sdk_common::prelude::LnUrlAuthError::ServiceConnectivity { err } => {
                Self::ServiceConnectivity { err }
            }
        }
    }
}

/// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
///
/// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
///  - verified the client signature in the case of LNURL-auth,
/// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
///   description of the reason.
///
/// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "status")]
pub enum LnUrlCallbackStatus {
    /// On-wire format is: `{"status": "OK"}`
    Ok,
    /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
    #[serde(rename = "ERROR")]
    ErrorStatus {
        #[serde(flatten)]
        data: LnUrlErrorData,
    },
}
impl From<sdk_common::prelude::LnUrlCallbackStatus> for LnUrlCallbackStatus {
    fn from(value: prelude::LnUrlCallbackStatus) -> Self {
        match value {
            sdk_common::prelude::LnUrlCallbackStatus::Ok => Self::Ok,
            sdk_common::prelude::LnUrlCallbackStatus::ErrorStatus { data } => {
                Self::ErrorStatus { data }
            }
        }
    }
}
