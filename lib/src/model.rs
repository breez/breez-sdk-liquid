use boltz_client::util::error::S5Error;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork};

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

pub struct WalletOptions {
    pub signer: SwSigner,
    pub network: Network,
    /// Output script descriptor
    ///
    /// See <https://github.com/bitcoin/bips/pull/1143>
    pub descriptor: String,
    pub db_root_path: Option<String>,
    pub chain_cache_path: Option<String>,
    pub electrum_url: Option<ElectrumUrl>,
}

#[derive(Debug)]
pub struct SwapLbtcResponse {
    pub id: String,
    pub invoice: String,
}

pub enum SwapStatus {
    Created,
    Mempool,
    Completed,
}

pub struct ReceivePaymentRequest {
    pub invoice_amount_sat: Option<u64>,
    pub onchain_amount_sat: Option<u64>,
}

pub struct SendPaymentResponse {
    pub txid: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SwapError {
    #[error("Could not contact Boltz servers: {err}")]
    ServersUnreachable { err: String },

    #[error("Invoice amount is out of range")]
    AmountOutOfRange,

    #[error("Wrong response received from Boltz servers")]
    BadResponse,

    #[error("The specified invoice is not valid")]
    InvalidInvoice,

    #[error("Could not sign/send the transaction")]
    SendError,

    #[error("Could not fetch the required wallet information")]
    WalletError,

    #[error("Could not store the swap details locally")]
    PersistError,

    #[error("The generated preimage is not valid")]
    InvalidPreimage,

    #[error("Generic boltz error: {err}")]
    BoltzGeneric { err: String },
}

impl From<S5Error> for SwapError {
    fn from(err: S5Error) -> Self {
        match err.kind {
            boltz_client::util::error::ErrorKind::Network
            | boltz_client::util::error::ErrorKind::BoltzApi => {
                SwapError::ServersUnreachable { err: err.message }
            }
            boltz_client::util::error::ErrorKind::Input => SwapError::BadResponse,
            _ => SwapError::BoltzGeneric { err: err.message },
        }
    }
}

#[derive(Debug)]
pub struct WalletInfo {
    pub balance_sat: u64,
    pub pubkey: String,
    pub active_address: String,
}

#[derive(Debug)]
pub(crate) enum OngoingSwap {
    Send {
        id: String,
        amount_sat: u64,
        funding_address: String,
    },
    Receive {
        id: String,
        preimage: String,
        redeem_script: String,
        blinding_key: String,
        invoice_amount_sat: u64,
        onchain_amount_sat: u64,
    },
}

#[derive(Debug)]
pub enum PaymentType {
    Sent,
    Received,
    PendingReceive,
    PendingSend,
}

#[derive(Debug)]
pub struct Payment {
    pub id: Option<String>,
    pub timestamp: Option<u32>,
    pub amount_sat: u64,
    pub payment_type: PaymentType,
}

impl From<OngoingSwap> for Payment {
    fn from(swap: OngoingSwap) -> Self {
        match swap {
            OngoingSwap::Send { amount_sat, .. } => Payment {
                id: None,
                timestamp: None,
                payment_type: PaymentType::PendingSend,
                amount_sat,
            },
            OngoingSwap::Receive {
                onchain_amount_sat, ..
            } => Payment {
                id: None,
                timestamp: None,
                payment_type: PaymentType::PendingReceive,
                amount_sat: onchain_amount_sat,
            },
        }
    }
}

#[derive(Debug)]
pub struct PreparePaymentResponse {
    pub id: String,
    pub funding_amount: u64,
    pub funding_address: String,
}
