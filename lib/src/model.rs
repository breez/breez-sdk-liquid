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

pub struct SendPaymentResponse {
    pub txid: String,
}

pub struct WalletInfo {
    pub balance_sat: u64,
    pub pubkey: String,
    pub active_address: String,
}

pub enum PaymentType {
    Sent,
    Received,
    Pending,
}

impl ToString for PaymentType {
    fn to_string(&self) -> String {
        match self {
            PaymentType::Sent => "Sent",
            PaymentType::Received => "Received",
            PaymentType::Pending => "Pending",
        }
        .to_string()
    }
}

pub struct Payment {
    pub id: Option<String>,
    pub timestamp: Option<u32>,
    pub amount_sat: u64,
    pub payment_type: PaymentType,
}
