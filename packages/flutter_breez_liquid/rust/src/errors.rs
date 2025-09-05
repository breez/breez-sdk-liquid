pub use breez_sdk_liquid::error::*;
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
