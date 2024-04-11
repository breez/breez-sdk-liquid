use boltz_client::network::Chain;
use boltz_client::{error::Error, Bolt11Invoice};
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork, WolletDescriptor};
use serde::Serialize;

use crate::get_invoice_amount;

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

#[derive(Debug, Serialize)]
pub struct PrepareReceiveRequest {
    pub payer_amount_sat: Option<u64>,
    pub receiver_amount_sat: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PrepareReceiveResponse {
    pub pair_hash: String,
    pub payer_amount_sat: u64,
    pub fees_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    pub id: String,
    pub invoice: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendResponse {
    pub id: String,
    pub invoice_amount_sat: u64,
    pub onchain_amount_sat: u64,
    pub funding_address: String,
    pub invoice: String,
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

    #[error("The specified funds have already been claimed")]
    AlreadyClaimed,

    #[error("Boltz error: {err}")]
    BoltzError { err: String },
}

impl From<Error> for PaymentError {
    fn from(err: Error) -> Self {
        match err {
            Error::Protocol(msg) => {
                if msg == "Could not find utxos for script" {
                    return PaymentError::AlreadyClaimed;
                }

                PaymentError::BoltzError { err: msg }
            }
            _ => PaymentError::BoltzError {
                err: format!("{err:?}"),
            },
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
        funding_address: String,
        invoice: String,
        onchain_amount_sat: u64,
        txid: Option<String>,
    },
    Receive {
        id: String,
        preimage: String,
        redeem_script: String,
        blinding_key: String,
        invoice: String,
        receiver_amount_sat: u64,
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
    pub fees_sat: Option<u64>,
    #[serde(rename(serialize = "type"))]
    pub payment_type: PaymentType,

    /// Only for [PaymentType::PendingReceive]
    pub invoice: Option<String>,
}

impl From<OngoingSwap> for Payment {
    fn from(swap: OngoingSwap) -> Self {
        match swap {
            OngoingSwap::Send {
                invoice,
                onchain_amount_sat,
                ..
            } => {
                let payer_amount_sat = get_invoice_amount!(invoice);
                Payment {
                    id: None,
                    timestamp: None,
                    payment_type: PaymentType::PendingSend,
                    amount_sat: payer_amount_sat,
                    invoice: Some(invoice),
                    fees_sat: Some(onchain_amount_sat - payer_amount_sat),
                }
            }
            OngoingSwap::Receive {
                receiver_amount_sat,
                invoice,
                ..
            } => {
                let payer_amount_sat = get_invoice_amount!(invoice);
                Payment {
                    id: None,
                    timestamp: None,
                    payment_type: PaymentType::PendingReceive,
                    amount_sat: receiver_amount_sat,
                    invoice: Some(invoice),
                    fees_sat: Some(payer_amount_sat - receiver_amount_sat),
                }
            }
        }
    }
}

pub(crate) struct PaymentData {
    pub invoice_amount_sat: u64,
}
