use lwk_wollet::{bitcoin, elements};
use sdk_common::prelude::ServiceConnectivityError;

use crate::error::PaymentError;

pub type PayjoinResult<T, E = PayjoinError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum PayjoinError {
    #[error("{0}")]
    Generic(String),

    #[error("Cannot pay: not enough funds")]
    InsufficientFunds,

    #[error("{0}")]
    ServiceConnectivity(String),
}

impl PayjoinError {
    pub(crate) fn generic<S: AsRef<str>>(err: S) -> Self {
        Self::Generic(err.as_ref().to_string())
    }

    pub(crate) fn service_connectivity<S: AsRef<str>>(err: S) -> Self {
        Self::ServiceConnectivity(err.as_ref().to_string())
    }
}

impl From<anyhow::Error> for PayjoinError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<bitcoin::base64::DecodeError> for PayjoinError {
    fn from(err: bitcoin::base64::DecodeError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<elements::encode::Error> for PayjoinError {
    fn from(err: elements::encode::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<elements::hashes::hex::HexToArrayError> for PayjoinError {
    fn from(err: elements::hashes::hex::HexToArrayError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<PaymentError> for PayjoinError {
    fn from(value: PaymentError) -> Self {
        match value {
            PaymentError::InsufficientFunds => Self::InsufficientFunds,
            _ => Self::Generic(value.to_string()),
        }
    }
}

impl From<serde_json::error::Error> for PayjoinError {
    fn from(value: serde_json::error::Error) -> Self {
        Self::Generic(value.to_string())
    }
}

impl From<ServiceConnectivityError> for PayjoinError {
    fn from(value: ServiceConnectivityError) -> Self {
        Self::ServiceConnectivity(value.err)
    }
}
