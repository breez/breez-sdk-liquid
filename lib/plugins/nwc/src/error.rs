use std::string::FromUtf8Error;

use breez_sdk_liquid::{
    error::{PaymentError, SdkError},
    plugin::PluginStorageError,
};
use nostr_sdk::nips::nip47::{ErrorCode, NIP47Error};

#[derive(thiserror::Error, Debug, Clone)]
pub enum NwcError {
    #[error("Generic error: {err}")]
    Generic { err: String },

    #[error("Plugin storage operation failed: {err}")]
    Persist { err: String },

    #[error("Could not contact relays: {err}")]
    Network { err: String },

    #[error("Event is from an unrecognized public key: {pubkey:?}")]
    PubkeyNotFound { pubkey: String },

    #[error("Invalid event signature: {err}")]
    InvalidSignature { err: String },

    #[error("Could not encrypt/decrypt event: {err}")]
    Encryption { err: String },

    #[error("Event not found")]
    EventNotFound,

    #[error("Event has expired")]
    EventExpired,

    #[error("A reply for this event has already been broadcast")]
    AlreadyReplied,

    #[error("Invoice has expired")]
    InvoiceExpired,

    #[error("Cannot pay an amountless invoice")]
    InvoiceWithoutAmount,

    #[error("Could not pay invoice: max budget has been exceeded")]
    MaxBudgetExceeded,

    #[error("Connection not found")]
    ConnectionNotFound,

    #[error("Connection already exists")]
    ConnectionExists,
}

impl NwcError {
    pub fn generic<T: ToString>(err: T) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }

    pub fn persist<T: ToString>(err: T) -> Self {
        Self::Persist {
            err: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for NwcError {
    fn from(err: anyhow::Error) -> Self {
        Self::generic(err)
    }
}

impl From<nostr_sdk::client::Error> for NwcError {
    fn from(err: nostr_sdk::client::Error) -> Self {
        Self::Network {
            err: err.to_string(),
        }
    }
}

impl From<nostr_sdk::event::Error> for NwcError {
    fn from(err: nostr_sdk::event::Error) -> Self {
        Self::generic(err)
    }
}

impl From<serde_json::Error> for NwcError {
    fn from(err: serde_json::Error) -> Self {
        Self::generic(err)
    }
}

impl From<FromUtf8Error> for NwcError {
    fn from(err: FromUtf8Error) -> Self {
        Self::generic(err)
    }
}

impl From<PluginStorageError> for NwcError {
    fn from(err: PluginStorageError) -> Self {
        Self::Persist {
            err: err.to_string(),
        }
    }
}

impl From<PaymentError> for NwcError {
    fn from(err: PaymentError) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

impl From<SdkError> for NwcError {
    fn from(err: SdkError) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<NIP47Error> for NwcError {
    fn into(self) -> NIP47Error {
        let code = match &self {
            Self::PubkeyNotFound { .. } | Self::EventNotFound => ErrorCode::NotFound,
            Self::MaxBudgetExceeded => ErrorCode::QuotaExceeded,
            _ => ErrorCode::PaymentFailed,
        };
        NIP47Error {
            code,
            message: self.to_string(),
        }
    }
}

pub type NwcResult<T> = Result<T, NwcError>;
