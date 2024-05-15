use anyhow::anyhow;
use boltz_client::network::Chain;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork, WolletDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
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

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Network, anyhow::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Liquid),
            "testnet" => Ok(Network::LiquidTestnet),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

#[derive(Debug)]
pub struct LiquidSdkOptions {
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
impl LiquidSdkOptions {
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
pub struct ConnectRequest {
    pub mnemonic: String,
    pub data_dir: Option<String>,
    pub network: Network,
}

#[derive(Debug, Serialize)]
pub struct PrepareReceiveRequest {
    pub payer_amount_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct PrepareReceiveResponse {
    pub payer_amount_sat: u64,
    pub fees_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    pub id: String,
    pub invoice: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendRequest {
    pub invoice: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendResponse {
    pub invoice: String,
    pub fees_sat: u64,
}

#[derive(Debug, Serialize)]
pub struct SendPaymentResponse {
    pub txid: String,
}

#[derive(Debug, Serialize)]
pub struct GetInfoRequest {
    pub with_scan: bool,
}

#[derive(Debug, Serialize)]
pub struct GetInfoResponse {
    /// Usable balance. This is the confirmed onchain balance minus `pending_send_sat`.
    pub balance_sat: u64,
    /// Amount that is being used for ongoing Send swaps
    pub pending_send_sat: u64,
    /// Incoming amount that is pending from ongoing Receive swaps
    pub pending_receive_sat: u64,
    pub pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct RestoreRequest {
    pub backup_path: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum Swap {
    Send(SwapIn),
    Receive(SwapOut),
}
impl Swap {
    pub(crate) fn id(&self) -> String {
        match &self {
            Swap::Send(SwapIn { id, .. }) | Swap::Receive(SwapOut { id, .. }) => id.clone(),
        }
    }
}

/// A submarine swap, used for swap-in (Receive)
#[derive(Clone, Debug)]
pub(crate) struct SwapIn {
    pub(crate) id: String,
    pub(crate) invoice: String,
    pub(crate) payer_amount_sat: u64,
    pub(crate) create_response_json: String,
    /// Persisted only when the lockup tx is successfully broadcasted
    pub(crate) lockup_txid: Option<String>,
    /// Whether or not the claim tx was seen in the mempool
    pub(crate) is_claim_tx_seen: bool,
}
impl SwapIn {
    pub(crate) fn calculate_status(&self) -> SubmarineSwapStatus {
        match (&self.lockup_txid, &self.is_claim_tx_seen) {
            (None, _) => SubmarineSwapStatus::Initial,
            (Some(_), false) => SubmarineSwapStatus::Pending,
            (Some(_), true) => SubmarineSwapStatus::Completed,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SubmarineSwapStatus {
    /// The swap was created, but the lockup tx was not broadcast sucecessfully.
    Initial = 0,

    /// The lockup tx was broadcasted successfully, but the claim tx was not seen in the mempool yet.
    Pending = 1,

    // TODO Separate between CompletedSeen vs CompletedConfirmed?
    Completed = 2,
}

/// A reverse swap, used for swap-out (Send)
#[derive(Clone, Debug)]
pub(crate) struct SwapOut {
    pub(crate) id: String,
    pub(crate) preimage: String,
    pub(crate) redeem_script: String,
    pub(crate) blinding_key: String,
    pub(crate) invoice: String,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) claim_fees_sat: u64,
    /// Persisted as soon as claim tx is broadcasted
    pub(crate) claim_txid: Option<String>,
}
impl SwapOut {
    pub(crate) fn calculate_status(&self) -> ReverseSwapStatus {
        match self.claim_txid {
            None => ReverseSwapStatus::Pending,
            Some(_) => ReverseSwapStatus::Completed,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ReverseSwapStatus {
    /// Reverse swap created, but lockup tx not yet seen and claim tx not yet broadcasted
    Pending = 0,

    /// Claim tx was broadcasted
    Completed = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PaymentType {
    Sent,
    Received,
    PendingReceive,
    PendingSend,
}
impl PaymentType {
    pub(crate) fn is_pending(&self) -> bool {
        match &self {
            Self::Sent | Self::Received => false,
            Self::PendingSend | Self::PendingReceive => true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Payment {
    /// The txid of the transaction
    pub id: String,
    pub timestamp: Option<u32>,
    pub amount_sat: u64,
    pub fees_sat: Option<u64>,
    #[serde(rename(serialize = "type"))]
    pub payment_type: PaymentType,

    pub invoice: Option<String>,
}

pub(crate) struct PaymentData {
    pub payer_amount_sat: u64,
    pub receiver_amount_sat: u64,
}

#[macro_export]
macro_rules! get_invoice_amount {
    ($invoice:expr) => {
        $invoice
            .parse::<Bolt11Invoice>()
            .expect("Expecting valid invoice")
            .amount_milli_satoshis()
            .expect("Expecting valid amount")
            / 1000
    };
}
