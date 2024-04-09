use boltz_client::error::Error;
use boltz_client::network::Chain;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork, WolletDescriptor};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug)]
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
    /// Custom Electrum URL. If set, it must match the specified network.
    ///
    /// If not set, it defaults to a Blockstream instance.
    pub electrum_url: Option<ElectrumUrl>,
}
impl WalletOptions {
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

#[derive(Debug)]
pub struct ReceivePaymentRequest {
    pub invoice_amount_sat: Option<u64>,
    pub onchain_amount_sat: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    pub id: String,
    pub invoice: String,
}

#[derive(Debug)]
pub struct PreparePaymentResponse {
    pub id: String,
    pub funding_amount: u64,
    pub funding_address: String,
}

#[derive(Debug, Serialize)]
pub struct SendPaymentResponse {
    pub txid: String,
}

#[derive(thiserror::Error, Debug)]
pub enum PaymentError {
    #[error("Invoice amount is out of range")]
    AmountOutOfRange,

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

impl From<Error> for PaymentError {
    fn from(err: Error) -> Self {
        PaymentError::BoltzGeneric {
            err: format!("{err:?}"),
        }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PaymentType {
    Sent,
    Received,
    PendingReceive,
    PendingSend,
}

#[derive(Debug, Clone, Serialize)]
pub struct Payment {
    pub id: Option<String>,
    pub timestamp: Option<u32>,
    pub amount_sat: u64,
    #[serde(rename(serialize = "type"))]
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
