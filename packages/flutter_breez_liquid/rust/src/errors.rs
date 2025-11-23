pub use breez_sdk_liquid::error::*;
pub use breez_sdk_liquid::plugin::PluginStorageError;
pub use breez_sdk_liquid_nwc::error::NwcError;
use flutter_rust_bridge::frb;

#[frb(mirror(PaymentError))]
pub enum _PaymentError {
    AlreadyClaimed,
    AlreadyPaid,
    PaymentInProgress,
    AmountOutOfRange { min: u64, max: u64 },
    AmountMissing { err: String },
    AssetError { err: String },
    InvalidNetwork { err: String },
    Generic { err: String },
    InvalidOrExpiredFees,
    InsufficientFunds,
    InvalidDescription { err: String },
    InvalidInvoice { err: String },
    InvalidPreimage,
    PairsNotFound,
    PaymentTimeout,
    PersistError,
    ReceiveError { err: String },
    Refunded { err: String, refund_tx_id: String },
    SelfTransferNotSupported,
    SendError { err: String },
    SignerError { err: String },
}

#[frb(mirror(SdkError))]
pub enum _SdkError {
    AlreadyStarted,
    Generic { err: String },
    NotStarted,
    ServiceConnectivity { err: String },
}

#[frb(mirror(PluginStorageError))]
pub enum _PluginStorageError {
    DataTooOld,
    Encryption { err: String },
    Generic { err: String },
}

#[frb(mirror(NwcError))]
pub enum _NwcError {
    Persist { err: String },
    Generic { err: String },
    Network { err: String },
    PubkeyNotFound { pubkey: String },
    InvalidSignature { err: String },
    Encryption { err: String },
    EventNotFound,
    EventExpired,
    AlreadyReplied,
    InvoiceExpired,
    InvoiceWithoutAmount,
    MaxBudgetExceeded,
}
