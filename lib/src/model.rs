use boltz_client::network::Chain;
use boltz_client::error::Error;
use boltz_client::error::Error::*;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork, WolletDescriptor};

#[derive(Copy, Clone, PartialEq)]
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

pub struct WalletOptions {
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

    #[error("Could not sign/send the transaction: {err}")]
    SendError { err: String },

    #[error("Could not fetch the required wallet information")]
    WalletError,

    #[error("Could not store the swap details locally")]
    PersistError,

    #[error("The generated preimage is not valid")]
    InvalidPreimage,

    #[error("Generic boltz error: {err}")]
    BoltzGeneric { err: String },
}

impl From<Error> for SwapError {
    fn from(err: Error) -> Self {
        match err {
            // TODO Better mapping
            // TODO Since there is no more clear ServersUnreachable, do we need that type?
            _ => SwapError::BoltzGeneric { err: format!("{err:?}") },
            // boltz_client::util::error::ErrorKind::Network | boltz_client::util::error::ErrorKind::BoltzApi => {
            //     SwapError::ServersUnreachable { err: err.message }
            // }
            // boltz_client::util::error::ErrorKind::Input => SwapError::BadResponse,
            // _ => SwapError::BoltzGeneric { err: err.message },
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
pub struct OngoingReceiveSwap {
    pub id: String,
    pub preimage: String,
    pub redeem_script: String,
    pub blinding_key: String,
    pub invoice_amount_sat: u64,
    pub onchain_amount_sat: u64,
}

pub struct OngoingSendSwap {
    pub id: String,
    // pub preimage: String,
    // pub redeem_script: String,
    // pub blinding_key: String,
    // pub invoice_amount_sat: Option<u64>,
    // pub onchain_amount_sat: Option<u64>,
}

#[derive(Debug)]
pub enum PaymentType {
    Sent,
    Received,
    PendingReceive,
}

#[derive(Debug)]
pub struct Payment {
    pub id: Option<String>,
    pub timestamp: Option<u32>,
    pub amount_sat: u64,
    pub payment_type: PaymentType,
}
