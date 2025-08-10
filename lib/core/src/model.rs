use anyhow::{anyhow, bail, Result};
use bitcoin::{bip32, ScriptBuf};
use boltz_client::{
    boltz::{ChainPair, BOLTZ_MAINNET_URL_V2, BOLTZ_TESTNET_URL_V2},
    network::{BitcoinChain, Chain, LiquidChain},
    swaps::boltz::{
        CreateChainResponse, CreateReverseResponse, CreateSubmarineResponse, Leaf, Side, SwapTree,
    },
    BtcSwapScript, Keypair, LBtcSwapScript,
};
use derivative::Derivative;
use elements::AssetId;
use lwk_wollet::ElementsNetwork;
use maybe_sync::{MaybeSend, MaybeSync};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use sdk_common::prelude::*;
use sdk_common::utils::Arc;
use sdk_common::{bitcoin::hashes::hex::ToHex, lightning_with_bolt12::offers::offer::Offer};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use tokio_with_wasm::alias as tokio;

use crate::{
    bitcoin,
    chain::bitcoin::esplora::EsploraBitcoinChainService,
    chain::liquid::esplora::EsploraLiquidChainService,
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    elements,
    error::{PaymentError, SdkError, SdkResult},
    persist::model::PaymentTxBalance,
    prelude::DEFAULT_EXTERNAL_INPUT_PARSERS,
    receive_swap::DEFAULT_ZERO_CONF_MAX_SAT,
    side_swap::api::{SIDESWAP_MAINNET_URL, SIDESWAP_TESTNET_URL},
    utils,
};

// Uses f64 for the maximum precision when converting between units
pub const LIQUID_FEE_RATE_SAT_PER_VBYTE: f64 = 0.1;
pub const LIQUID_FEE_RATE_MSAT_PER_VBYTE: f32 = (LIQUID_FEE_RATE_SAT_PER_VBYTE * 1000.0) as f32;
pub const BREEZ_SYNC_SERVICE_URL: &str = "https://datasync.breez.technology";
pub const BREEZ_LIQUID_ESPLORA_URL: &str = "https://lq1.breez.technology/liquid/api";
pub const BREEZ_SWAP_PROXY_URL: &str = "https://swap.breez.technology/v2";
pub const DEFAULT_ONCHAIN_FEE_RATE_LEEWAY_SAT: u64 = 500;

const SIDESWAP_API_KEY: &str = "97fb6a1dfa37ee6656af92ef79675cc03b8ac4c52e04655f41edbd5af888dcc2";

#[derive(Clone, Debug, Serialize)]
pub enum BlockchainExplorer {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    Electrum { url: String },
    Esplora {
        url: String,
        /// Whether or not to use the "waterfalls" extension
        use_waterfalls: bool,
    },
}

/// Configuration for the Liquid SDK
#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub liquid_explorer: BlockchainExplorer,
    pub bitcoin_explorer: BlockchainExplorer,
    /// Directory in which the DB and log files are stored.
    ///
    /// Prefix can be a relative or absolute path to this directory.
    pub working_dir: String,
    pub network: LiquidNetwork,
    /// Send payment timeout. See [LiquidSdk::send_payment](crate::sdk::LiquidSdk::send_payment)
    pub payment_timeout_sec: u64,
    /// The url of the real-time sync service. Defaults to [BREEZ_SYNC_SERVICE_URL]
    /// Setting this field to `None` will disable the service
    pub sync_service_url: Option<String>,
    /// Maximum amount in satoshi to accept zero-conf payments with
    /// Defaults to [DEFAULT_ZERO_CONF_MAX_SAT]
    pub zero_conf_max_amount_sat: Option<u64>,
    /// The Breez API key used for making requests to the sync service
    pub breez_api_key: Option<String>,
    /// A set of external input parsers that are used by [LiquidSdk::parse](crate::sdk::LiquidSdk::parse) when the input
    /// is not recognized. See [ExternalInputParser] for more details on how to configure
    /// external parsing.
    pub external_input_parsers: Option<Vec<ExternalInputParser>>,
    /// The SDK includes some default external input parsers
    /// ([DEFAULT_EXTERNAL_INPUT_PARSERS](crate::sdk::DEFAULT_EXTERNAL_INPUT_PARSERS)).
    /// Set this to false in order to prevent their use.
    pub use_default_external_input_parsers: bool,
    /// For payments where the onchain fees can only be estimated on creation, this can be used
    /// in order to automatically allow slightly more expensive fees. If the actual fee ends up
    /// being above the sum of the initial estimate and this leeway, the payment will require
    /// user fee acceptance. See [WaitingFeeAcceptance](PaymentState::WaitingFeeAcceptance).
    ///
    /// Defaults to [DEFAULT_ONCHAIN_FEE_RATE_LEEWAY_SAT].
    pub onchain_fee_rate_leeway_sat: Option<u64>,
    /// A set of asset metadata used by [LiquidSdk::parse](crate::sdk::LiquidSdk::parse) when the input is a
    /// [LiquidAddressData] and the [asset_id](LiquidAddressData::asset_id) differs from the Liquid Bitcoin asset.
    /// See [AssetMetadata] for more details on how define asset metadata.
    /// By default the asset metadata for Liquid Bitcoin and Tether USD are included.
    pub asset_metadata: Option<Vec<AssetMetadata>>,
    /// The SideSwap API key used for making requests to the SideSwap payjoin service
    pub sideswap_api_key: Option<String>,
    /// Set this to false to disable the use of Magic Routing Hints (MRH) to send payments. Enabled by default.
    pub use_magic_routing_hints: bool,
}

impl Config {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub fn mainnet(breez_api_key: Option<String>) -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Electrum {
                url: "elements-mainnet.breez.technology:50002".to_string(),
            },
            bitcoin_explorer: BlockchainExplorer::Electrum {
                url: "bitcoin-mainnet.blockstream.info:50002".to_string(),
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Mainnet,
            payment_timeout_sec: 15,
            sync_service_url: Some(BREEZ_SYNC_SERVICE_URL.to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: Some(SIDESWAP_API_KEY.to_string()),
            use_magic_routing_hints: true,
        }
    }

    pub fn mainnet_esplora(breez_api_key: Option<String>) -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Esplora {
                url: BREEZ_LIQUID_ESPLORA_URL.to_string(),
                use_waterfalls: true,
            },
            bitcoin_explorer: BlockchainExplorer::Esplora {
                url: "https://blockstream.info/api/".to_string(),
                use_waterfalls: false,
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Mainnet,
            payment_timeout_sec: 15,
            sync_service_url: Some(BREEZ_SYNC_SERVICE_URL.to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: Some(SIDESWAP_API_KEY.to_string()),
            use_magic_routing_hints: true,
        }
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub fn testnet(breez_api_key: Option<String>) -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Electrum {
                url: "elements-testnet.blockstream.info:50002".to_string(),
            },
            bitcoin_explorer: BlockchainExplorer::Electrum {
                url: "bitcoin-testnet.blockstream.info:50002".to_string(),
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Testnet,
            payment_timeout_sec: 15,
            sync_service_url: Some(BREEZ_SYNC_SERVICE_URL.to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: Some(SIDESWAP_API_KEY.to_string()),
            use_magic_routing_hints: true,
        }
    }

    pub fn testnet_esplora(breez_api_key: Option<String>) -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Esplora {
                url: "https://blockstream.info/liquidtestnet/api".to_string(),
                use_waterfalls: false,
            },
            bitcoin_explorer: BlockchainExplorer::Esplora {
                url: "https://blockstream.info/testnet/api/".to_string(),
                use_waterfalls: false,
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Testnet,
            payment_timeout_sec: 15,
            sync_service_url: Some(BREEZ_SYNC_SERVICE_URL.to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: Some(SIDESWAP_API_KEY.to_string()),
            use_magic_routing_hints: true,
        }
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub fn regtest() -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Electrum {
                url: "localhost:19002".to_string(),
            },
            bitcoin_explorer: BlockchainExplorer::Electrum {
                url: "localhost:19001".to_string(),
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Regtest,
            payment_timeout_sec: 15,
            sync_service_url: Some("http://localhost:8088".to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key: None,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: None,
            use_magic_routing_hints: true,
        }
    }

    pub fn regtest_esplora() -> Self {
        Config {
            liquid_explorer: BlockchainExplorer::Esplora {
                url: "http://localhost:3120/api".to_string(),
                use_waterfalls: true,
            },
            bitcoin_explorer: BlockchainExplorer::Esplora {
                url: "http://localhost:4002/api".to_string(),
                use_waterfalls: false,
            },
            working_dir: ".".to_string(),
            network: LiquidNetwork::Regtest,
            payment_timeout_sec: 15,
            sync_service_url: Some("http://localhost:8089".to_string()),
            zero_conf_max_amount_sat: None,
            breez_api_key: None,
            external_input_parsers: None,
            use_default_external_input_parsers: true,
            onchain_fee_rate_leeway_sat: None,
            asset_metadata: None,
            sideswap_api_key: None,
            use_magic_routing_hints: true,
        }
    }

    pub fn get_wallet_dir(&self, base_dir: &str, fingerprint_hex: &str) -> anyhow::Result<String> {
        Ok(PathBuf::from(base_dir)
            .join(match self.network {
                LiquidNetwork::Mainnet => "mainnet",
                LiquidNetwork::Testnet => "testnet",
                LiquidNetwork::Regtest => "regtest",
            })
            .join(fingerprint_hex)
            .to_str()
            .ok_or(anyhow::anyhow!(
                "Could not get retrieve current wallet directory"
            ))?
            .to_string())
    }

    pub fn zero_conf_max_amount_sat(&self) -> u64 {
        self.zero_conf_max_amount_sat
            .unwrap_or(DEFAULT_ZERO_CONF_MAX_SAT)
    }

    pub(crate) fn lbtc_asset_id(&self) -> String {
        utils::lbtc_asset_id(self.network).to_string()
    }

    pub(crate) fn get_all_external_input_parsers(&self) -> Vec<ExternalInputParser> {
        let mut external_input_parsers = Vec::new();
        if self.use_default_external_input_parsers {
            let default_parsers = DEFAULT_EXTERNAL_INPUT_PARSERS
                .iter()
                .map(|(id, regex, url)| ExternalInputParser {
                    provider_id: id.to_string(),
                    input_regex: regex.to_string(),
                    parser_url: url.to_string(),
                })
                .collect::<Vec<_>>();
            external_input_parsers.extend(default_parsers);
        }
        external_input_parsers.extend(self.external_input_parsers.clone().unwrap_or_default());

        external_input_parsers
    }

    pub(crate) fn default_boltz_url(&self) -> &str {
        match self.network {
            LiquidNetwork::Mainnet => {
                if self.breez_api_key.is_some() {
                    BREEZ_SWAP_PROXY_URL
                } else {
                    BOLTZ_MAINNET_URL_V2
                }
            }
            LiquidNetwork::Testnet => BOLTZ_TESTNET_URL_V2,
            // On regtest use the swapproxy instance by default
            LiquidNetwork::Regtest => "http://localhost:8387/v2",
        }
    }

    pub fn sync_enabled(&self) -> bool {
        self.sync_service_url.is_some()
    }

    pub(crate) fn bitcoin_chain_service(&self) -> Arc<dyn BitcoinChainService> {
        match self.bitcoin_explorer {
            BlockchainExplorer::Esplora { .. } => {
                Arc::new(EsploraBitcoinChainService::new(self.clone()))
            }
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { .. } => Arc::new(
                crate::chain::bitcoin::electrum::ElectrumBitcoinChainService::new(self.clone()),
            ),
        }
    }

    pub(crate) fn liquid_chain_service(&self) -> Result<Arc<dyn LiquidChainService>> {
        match &self.liquid_explorer {
            BlockchainExplorer::Esplora { url, .. } => {
                if url == BREEZ_LIQUID_ESPLORA_URL && self.breez_api_key.is_none() {
                    bail!("Cannot start the Breez Esplora chain service without providing an API key. See https://sdk-doc-liquid.breez.technology/guide/getting_started.html#api-key")
                }
                Ok(Arc::new(EsploraLiquidChainService::new(self.clone())))
            }
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { .. } => Ok(Arc::new(
                crate::chain::liquid::electrum::ElectrumLiquidChainService::new(self.clone()),
            )),
        }
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub(crate) fn electrum_tls_options(&self) -> (/*tls*/ bool, /*validate_domain*/ bool) {
        match self.network {
            LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
            LiquidNetwork::Regtest => (false, false),
        }
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub(crate) fn electrum_client(
        &self,
        url: &str,
    ) -> Result<lwk_wollet::ElectrumClient, lwk_wollet::Error> {
        let (tls, validate_domain) = self.electrum_tls_options();
        let electrum_url = lwk_wollet::ElectrumUrl::new(url, tls, validate_domain)?;
        lwk_wollet::ElectrumClient::with_options(
            &electrum_url,
            lwk_wollet::ElectrumOptions { timeout: Some(3) },
        )
    }

    pub(crate) fn sideswap_url(&self) -> &'static str {
        match self.network {
            LiquidNetwork::Mainnet => SIDESWAP_MAINNET_URL,
            LiquidNetwork::Testnet => SIDESWAP_TESTNET_URL,
            LiquidNetwork::Regtest => unimplemented!(),
        }
    }
}

/// Network chosen for this Liquid SDK instance. Note that it represents both the Liquid and the
/// Bitcoin network used.
#[derive(Debug, Display, Copy, Clone, PartialEq, Serialize)]
pub enum LiquidNetwork {
    /// Mainnet Bitcoin and Liquid chains
    Mainnet,
    /// Testnet Bitcoin and Liquid chains
    Testnet,
    /// Regtest Bitcoin and Liquid chains
    Regtest,
}
impl LiquidNetwork {
    pub fn as_bitcoin_chain(&self) -> BitcoinChain {
        match self {
            LiquidNetwork::Mainnet => BitcoinChain::Bitcoin,
            LiquidNetwork::Testnet => BitcoinChain::BitcoinTestnet,
            LiquidNetwork::Regtest => BitcoinChain::BitcoinRegtest,
        }
    }
}

impl From<LiquidNetwork> for ElementsNetwork {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => ElementsNetwork::Liquid,
            LiquidNetwork::Testnet => ElementsNetwork::LiquidTestnet,
            LiquidNetwork::Regtest => ElementsNetwork::ElementsRegtest {
                policy_asset: AssetId::from_str(
                    "5ac9f65c0efcc4775e0baec4ec03abdde22473cd3cf33c0419ca290e0751b225",
                )
                .unwrap(),
            },
        }
    }
}

impl From<LiquidNetwork> for Chain {
    fn from(value: LiquidNetwork) -> Self {
        Chain::Liquid(value.into())
    }
}

impl From<LiquidNetwork> for LiquidChain {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => LiquidChain::Liquid,
            LiquidNetwork::Testnet => LiquidChain::LiquidTestnet,
            LiquidNetwork::Regtest => LiquidChain::LiquidRegtest,
        }
    }
}

impl TryFrom<&str> for LiquidNetwork {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<LiquidNetwork, anyhow::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(LiquidNetwork::Mainnet),
            "testnet" => Ok(LiquidNetwork::Testnet),
            "regtest" => Ok(LiquidNetwork::Regtest),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

impl From<LiquidNetwork> for Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
            LiquidNetwork::Regtest => Self::Regtest,
        }
    }
}

impl From<LiquidNetwork> for sdk_common::bitcoin::Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
            LiquidNetwork::Regtest => Self::Regtest,
        }
    }
}

impl From<LiquidNetwork> for boltz_client::bitcoin::Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
            LiquidNetwork::Regtest => Self::Regtest,
        }
    }
}

/// Trait that can be used to react to various [SdkEvent]s emitted by the SDK.
pub trait EventListener: MaybeSend + MaybeSync {
    fn on_event(&self, e: SdkEvent);
}

/// Event emitted by the SDK. Add an [EventListener] by calling [crate::sdk::LiquidSdk::add_event_listener]
/// to listen for emitted events.
#[derive(Clone, Debug, PartialEq)]
pub enum SdkEvent {
    PaymentFailed {
        details: Payment,
    },
    PaymentPending {
        details: Payment,
    },
    PaymentRefundable {
        details: Payment,
    },
    PaymentRefunded {
        details: Payment,
    },
    PaymentRefundPending {
        details: Payment,
    },
    PaymentSucceeded {
        details: Payment,
    },
    PaymentWaitingConfirmation {
        details: Payment,
    },
    PaymentWaitingFeeAcceptance {
        details: Payment,
    },
    /// Synced with mempool and onchain data
    Synced,
    /// Synced with real-time data sync
    DataSynced {
        /// Indicates new data was pulled from other instances.
        did_pull_new_records: bool,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum SignerError {
    #[error("Signer error: {err}")]
    Generic { err: String },
}

impl From<anyhow::Error> for SignerError {
    fn from(err: anyhow::Error) -> Self {
        SignerError::Generic {
            err: err.to_string(),
        }
    }
}

impl From<bip32::Error> for SignerError {
    fn from(err: bip32::Error) -> Self {
        SignerError::Generic {
            err: err.to_string(),
        }
    }
}

/// A trait that can be used to sign messages and verify signatures.
/// The sdk user can implement this trait to use their own signer.
pub trait Signer: MaybeSend + MaybeSync {
    /// The master xpub encoded as 78 bytes length as defined in bip32 specification.
    /// For reference: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Serialization_format>
    fn xpub(&self) -> Result<Vec<u8>, SignerError>;

    /// The derived xpub encoded as 78 bytes length as defined in bip32 specification.
    /// The derivation path is a string represents the shorter notation of the key tree to derive. For example:
    /// m/49'/1'/0'/0/0
    /// m/48'/1'/0'/0/0
    /// For reference: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-The_key_tree>
    fn derive_xpub(&self, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    /// Sign an ECDSA message using the private key derived from the given derivation path
    fn sign_ecdsa(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    /// Sign an ECDSA message using the private key derived from the master key
    fn sign_ecdsa_recoverable(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;

    /// Return the master blinding key for SLIP77: <https://github.com/satoshilabs/slips/blob/master/slip-0077.md>
    fn slip77_master_blinding_key(&self) -> Result<Vec<u8>, SignerError>;

    /// HMAC-SHA256 using the private key derived from the given derivation path
    /// This is used to calculate the linking key of lnurl-auth specification: <https://github.com/lnurl/luds/blob/luds/05.md>
    fn hmac_sha256(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    /// Encrypts a message using (ECIES)[ecies::encrypt]
    fn ecies_encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;

    /// Decrypts a message using (ECIES)[ecies::decrypt]
    fn ecies_decrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;
}

/// An argument when calling [crate::sdk::LiquidSdk::connect].
/// The resquest takes either a `mnemonic` and `passphrase`, or a `seed`.
pub struct ConnectRequest {
    /// The SDK [Config]
    pub config: Config,
    /// The optional Liquid wallet mnemonic
    pub mnemonic: Option<String>,
    /// The optional passphrase for the mnemonic
    pub passphrase: Option<String>,
    /// The optional Liquid wallet seed
    pub seed: Option<Vec<u8>>,
}

pub struct ConnectWithSignerRequest {
    pub config: Config,
}

/// A reserved address. Once an address is reserved, it can only be
/// reallocated to another payment after the block height expiration.
#[derive(Clone, Debug)]
pub(crate) struct ReservedAddress {
    /// The address that is reserved
    pub(crate) address: String,
    /// The block height that the address is reserved until
    pub(crate) expiry_block_height: u32,
}

/// The send/receive methods supported by the SDK
#[derive(Clone, Debug, Serialize)]
pub enum PaymentMethod {
    #[deprecated(since = "0.8.1", note = "Use `Bolt11Invoice` instead")]
    Lightning,
    Bolt11Invoice,
    Bolt12Offer,
    BitcoinAddress,
    LiquidAddress,
}

#[derive(Debug, Serialize, Clone)]
pub enum ReceiveAmount {
    /// The amount in satoshi that should be paid
    Bitcoin { payer_amount_sat: u64 },

    /// The amount of an asset that should be paid
    Asset {
        asset_id: String,
        payer_amount: Option<f64>,
    },
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_receive_payment].
#[derive(Debug, Serialize)]
pub struct PrepareReceiveRequest {
    pub payment_method: PaymentMethod,
    /// The amount to be paid in either Bitcoin or another asset
    pub amount: Option<ReceiveAmount>,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_receive_payment].
#[derive(Debug, Serialize, Clone)]
pub struct PrepareReceiveResponse {
    pub payment_method: PaymentMethod,
    /// Generally represents the total fees that would be paid to send or receive this payment.
    ///
    /// In case of Zero-Amount Receive Chain swaps, the swapper service fee (`swapper_feerate` times
    /// the amount) is paid in addition to `fees_sat`. The swapper service feerate is already known
    /// in the beginning, but the exact swapper service fee will only be known when the
    /// `payer_amount_sat` is known.
    ///
    /// In all other types of swaps, the swapper service fee is included in `fees_sat`.
    pub fees_sat: u64,
    /// The amount to be paid in either Bitcoin or another asset
    pub amount: Option<ReceiveAmount>,
    /// The minimum amount the payer can send for this swap to succeed.
    ///
    /// When the method is [PaymentMethod::LiquidAddress], this is empty.
    pub min_payer_amount_sat: Option<u64>,
    /// The maximum amount the payer can send for this swap to succeed.
    ///
    /// When the method is [PaymentMethod::LiquidAddress], this is empty.
    pub max_payer_amount_sat: Option<u64>,
    /// The percentage of the sent amount that will count towards the service fee.
    ///
    /// When the method is [PaymentMethod::LiquidAddress], this is empty.
    pub swapper_feerate: Option<f64>,
}

/// An argument when calling [crate::sdk::LiquidSdk::receive_payment].
#[derive(Debug, Serialize)]
pub struct ReceivePaymentRequest {
    pub prepare_response: PrepareReceiveResponse,
    /// The description for this payment request
    pub description: Option<String>,
    /// If set to true, then the hash of the description will be used
    pub use_description_hash: Option<bool>,
    /// An optional payer note, typically included in a LNURL-Pay request
    pub payer_note: Option<String>,
}

/// Returned when calling [crate::sdk::LiquidSdk::receive_payment].
#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    /// Either a BIP21 URI (Liquid or Bitcoin), a Liquid address
    /// or an invoice, depending on the [PrepareReceiveResponse] parameters
    pub destination: String,
    /// The Liquid block height at which the payment will expire
    pub liquid_expiration_blockheight: Option<u32>,
    /// The Bitcoin block height at which the payment will expire
    pub bitcoin_expiration_blockheight: Option<u32>,
}

/// An argument when calling [crate::sdk::LiquidSdk::create_bolt12_invoice].
#[derive(Debug, Serialize)]
pub struct CreateBolt12InvoiceRequest {
    /// The BOLT12 offer
    pub offer: String,
    /// The invoice request created from the offer
    pub invoice_request: String,
}

/// Returned when calling [crate::sdk::LiquidSdk::create_bolt12_invoice].
#[derive(Debug, Serialize, Clone)]
pub struct CreateBolt12InvoiceResponse {
    /// The BOLT12 invoice
    pub invoice: String,
}

/// The minimum and maximum in satoshis of a Lightning or onchain payment.
#[derive(Debug, Serialize)]
pub struct Limits {
    pub min_sat: u64,
    pub max_sat: u64,
    pub max_zero_conf_sat: u64,
}

/// Returned when calling [crate::sdk::LiquidSdk::fetch_lightning_limits].
#[derive(Debug, Serialize)]
pub struct LightningPaymentLimitsResponse {
    /// Amount limits for a Send Payment to be valid
    pub send: Limits,
    /// Amount limits for a Receive Payment to be valid
    pub receive: Limits,
}

/// Returned when calling [crate::sdk::LiquidSdk::fetch_onchain_limits].
#[derive(Debug, Serialize)]
pub struct OnchainPaymentLimitsResponse {
    /// Amount limits for a Send Onchain Payment to be valid
    pub send: Limits,
    /// Amount limits for a Receive Onchain Payment to be valid
    pub receive: Limits,
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_send_payment].
#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendRequest {
    /// The destination we intend to pay to.
    /// Supports BIP21 URIs, BOLT11 invoices, BOLT12 offers and Liquid addresses
    pub destination: String,
    /// Should only be set when paying directly onchain or to a BIP21 URI
    /// where no amount is specified, or when the caller wishes to drain
    pub amount: Option<PayAmount>,
}

/// Specifies the supported destinations which can be payed by the SDK
#[derive(Clone, Debug, Serialize)]
pub enum SendDestination {
    LiquidAddress {
        address_data: liquid::LiquidAddressData,
        /// A BIP353 address, in case one was used to resolve this Liquid address
        bip353_address: Option<String>,
    },
    Bolt11 {
        invoice: LNInvoice,
        /// A BIP353 address, in case one was used to resolve this BOLT11
        bip353_address: Option<String>,
    },
    Bolt12 {
        offer: LNOffer,
        receiver_amount_sat: u64,
        /// A BIP353 address, in case one was used to resolve this BOLT12
        bip353_address: Option<String>,
    },
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_send_payment].
#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendResponse {
    pub destination: SendDestination,
    /// The optional amount to be sent in either Bitcoin or another asset
    pub amount: Option<PayAmount>,
    /// The optional estimated fee in satoshi. Is set when there is Bitcoin available
    /// to pay fees. When not set, there are asset fees available to pay fees.
    pub fees_sat: Option<u64>,
    /// The optional estimated fee in the asset. Is set when [PayAmount::Asset::estimate_asset_fees]
    /// is set to `true`, the Payjoin service accepts this asset to pay fees and there
    /// are funds available in this asset to pay fees.
    pub estimated_asset_fees: Option<f64>,
    /// The amount of funds required (in satoshi) to execute a SideSwap payment, excluding fees.
    /// Only present when [PayAmount::Asset::pay_with_bitcoin] is set to `true`.
    pub exchange_amount_sat: Option<u64>,
}

/// An argument when calling [crate::sdk::LiquidSdk::send_payment].
#[derive(Debug, Serialize)]
pub struct SendPaymentRequest {
    pub prepare_response: PrepareSendResponse,
    /// If set to true, the payment will be sent using the SideSwap payjoin service
    pub use_asset_fees: Option<bool>,
    /// An optional payer note, which is to be included in a BOLT12 invoice request
    pub payer_note: Option<String>,
}

/// Returned when calling [crate::sdk::LiquidSdk::send_payment].
#[derive(Debug, Serialize)]
pub struct SendPaymentResponse {
    pub payment: Payment,
}

pub(crate) struct SendPaymentViaSwapRequest {
    pub(crate) invoice: String,
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: String,
    pub(crate) description: Option<String>,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) fees_sat: u64,
}

pub(crate) struct PayLiquidRequest {
    pub address_data: LiquidAddressData,
    pub to_asset: String,
    pub receiver_amount_sat: u64,
    pub asset_pay_fees: bool,
    pub fees_sat: Option<u64>,
}

pub(crate) struct PaySideSwapRequest {
    pub address_data: LiquidAddressData,
    pub to_asset: String,
    pub receiver_amount_sat: u64,
    pub fees_sat: u64,
    pub amount: Option<PayAmount>,
}

/// Used to specify the amount to sent or to send all funds.
#[derive(Debug, Serialize, Clone)]
pub enum PayAmount {
    /// The amount in satoshi that will be received
    Bitcoin { receiver_amount_sat: u64 },

    /// The amount of an asset that will be received
    Asset {
        /// The asset id specifying which asset will be sent
        to_asset: String,
        receiver_amount: f64,
        estimate_asset_fees: Option<bool>,
        /// The asset id whose balance we want to send funds with.
        /// Defaults to the value provided for [PayAmount::Asset::to_asset]
        from_asset: Option<String>,
    },

    /// Indicates that all available Bitcoin funds should be sent
    Drain,
}

impl PayAmount {
    pub(crate) fn is_sideswap_payment(&self) -> bool {
        match self {
            PayAmount::Asset {
                to_asset,
                from_asset,
                ..
            } => from_asset.as_ref().is_some_and(|asset| asset != to_asset),
            _ => false,
        }
    }
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_pay_onchain].
#[derive(Debug, Serialize, Clone)]
pub struct PreparePayOnchainRequest {
    /// The amount to send
    pub amount: PayAmount,
    /// The optional fee rate of the Bitcoin claim transaction in sat/vB. Defaults to the swapper estimated claim fee.
    pub fee_rate_sat_per_vbyte: Option<u32>,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_pay_onchain].
#[derive(Debug, Serialize, Clone)]
pub struct PreparePayOnchainResponse {
    pub receiver_amount_sat: u64,
    pub claim_fees_sat: u64,
    pub total_fees_sat: u64,
}

/// An argument when calling [crate::sdk::LiquidSdk::pay_onchain].
#[derive(Debug, Serialize)]
pub struct PayOnchainRequest {
    pub address: String,
    pub prepare_response: PreparePayOnchainResponse,
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_refund].
#[derive(Debug, Serialize)]
pub struct PrepareRefundRequest {
    /// The address where the swap funds are locked up
    pub swap_address: String,
    /// The address to refund the swap funds to
    pub refund_address: String,
    /// The fee rate in sat/vB for the refund transaction
    pub fee_rate_sat_per_vbyte: u32,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_refund].
#[derive(Debug, Serialize)]
pub struct PrepareRefundResponse {
    pub tx_vsize: u32,
    pub tx_fee_sat: u64,
    /// The txid of the last broadcasted refund tx, if any
    pub last_refund_tx_id: Option<String>,
}

/// An argument when calling [crate::sdk::LiquidSdk::refund].
#[derive(Debug, Serialize)]
pub struct RefundRequest {
    /// The address where the swap funds are locked up
    pub swap_address: String,
    /// The address to refund the swap funds to
    pub refund_address: String,
    /// The fee rate in sat/vB for the refund transaction
    pub fee_rate_sat_per_vbyte: u32,
}

/// Returned when calling [crate::sdk::LiquidSdk::refund].
#[derive(Debug, Serialize)]
pub struct RefundResponse {
    pub refund_tx_id: String,
}

/// An asset balance to denote the balance for each asset.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AssetBalance {
    pub asset_id: String,
    pub balance_sat: u64,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub balance: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlockchainInfo {
    pub liquid_tip: u32,
    pub bitcoin_tip: u32,
}

#[derive(Copy, Clone)]
pub(crate) struct ChainTips {
    pub liquid_tip: u32,
    pub bitcoin_tip: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Usable balance. This is the confirmed onchain balance minus `pending_send_sat`.
    pub balance_sat: u64,
    /// Amount that is being used for ongoing Send swaps
    pub pending_send_sat: u64,
    /// Incoming amount that is pending from ongoing Receive swaps
    pub pending_receive_sat: u64,
    /// The wallet's fingerprint. It is used to build the working directory in [Config::get_wallet_dir].
    pub fingerprint: String,
    /// The wallet's pubkey. Used to verify signed messages.
    pub pubkey: String,
    /// Asset balances of non Liquid Bitcoin assets
    #[serde(default)]
    pub asset_balances: Vec<AssetBalance>,
}

impl WalletInfo {
    pub(crate) fn validate_sufficient_funds(
        &self,
        network: LiquidNetwork,
        amount_sat: u64,
        fees_sat: Option<u64>,
        asset_id: &str,
    ) -> Result<(), PaymentError> {
        let fees_sat = fees_sat.unwrap_or(0);
        if asset_id.eq(&utils::lbtc_asset_id(network).to_string()) {
            ensure_sdk!(
                amount_sat + fees_sat <= self.balance_sat,
                PaymentError::InsufficientFunds
            );
        } else {
            match self
                .asset_balances
                .iter()
                .find(|ab| ab.asset_id.eq(asset_id))
            {
                Some(asset_balance) => ensure_sdk!(
                    amount_sat <= asset_balance.balance_sat && fees_sat <= self.balance_sat,
                    PaymentError::InsufficientFunds
                ),
                None => return Err(PaymentError::InsufficientFunds),
            }
        }
        Ok(())
    }
}

/// Returned when calling [crate::sdk::LiquidSdk::get_info].
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInfoResponse {
    /// The wallet information, such as the balance, fingerprint and public key
    pub wallet_info: WalletInfo,
    /// The latest synced blockchain information, such as the Liquid/Bitcoin tips
    #[serde(default)]
    pub blockchain_info: BlockchainInfo,
}

/// An argument when calling [crate::sdk::LiquidSdk::sign_message].
#[derive(Clone, Debug, PartialEq)]
pub struct SignMessageRequest {
    pub message: String,
}

/// Returned when calling [crate::sdk::LiquidSdk::sign_message].
#[derive(Clone, Debug, PartialEq)]
pub struct SignMessageResponse {
    pub signature: String,
}

/// An argument when calling [crate::sdk::LiquidSdk::check_message].
#[derive(Clone, Debug, PartialEq)]
pub struct CheckMessageRequest {
    /// The message that was signed.
    pub message: String,
    /// The public key of the node that signed the message.
    pub pubkey: String,
    /// The zbase encoded signature to verify.
    pub signature: String,
}

/// Returned when calling [crate::sdk::LiquidSdk::check_message].
#[derive(Clone, Debug, PartialEq)]
pub struct CheckMessageResponse {
    /// Boolean value indicating whether the signature covers the message and
    /// was signed by the given pubkey.
    pub is_valid: bool,
}

/// An argument when calling [crate::sdk::LiquidSdk::backup].
#[derive(Debug, Serialize)]
pub struct BackupRequest {
    /// Path to the backup.
    ///
    /// If not set, it defaults to `backup.sql` for mainnet, `backup-testnet.sql` for testnet,
    /// and `backup-regtest.sql` for regtest.
    ///
    /// The file will be saved in [ConnectRequest]'s `data_dir`.
    pub backup_path: Option<String>,
}

/// An argument when calling [crate::sdk::LiquidSdk::restore].
#[derive(Debug, Serialize)]
pub struct RestoreRequest {
    pub backup_path: Option<String>,
}

/// An argument when calling [crate::sdk::LiquidSdk::list_payments].
#[derive(Default)]
pub struct ListPaymentsRequest {
    pub filters: Option<Vec<PaymentType>>,
    pub states: Option<Vec<PaymentState>>,
    /// Epoch time, in seconds
    pub from_timestamp: Option<i64>,
    /// Epoch time, in seconds
    pub to_timestamp: Option<i64>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub details: Option<ListPaymentDetails>,
    pub sort_ascending: Option<bool>,
}

/// An argument of [ListPaymentsRequest] when calling [crate::sdk::LiquidSdk::list_payments].
#[derive(Debug, Serialize)]
pub enum ListPaymentDetails {
    /// A Liquid payment
    Liquid {
        /// Optional asset id
        asset_id: Option<String>,
        /// Optional BIP21 URI or address
        destination: Option<String>,
    },

    /// A Bitcoin payment
    Bitcoin {
        /// Optional address
        address: Option<String>,
    },
}

/// An argument when calling [crate::sdk::LiquidSdk::get_payment].
#[derive(Debug, Serialize)]
pub enum GetPaymentRequest {
    /// The payment hash of a Lightning payment
    PaymentHash { payment_hash: String },
    /// A swap id or its SHA256 hash
    SwapId { swap_id: String },
}

/// Trait that can be used to react to new blocks from Bitcoin and Liquid chains
#[sdk_macros::async_trait]
pub(crate) trait BlockListener: MaybeSend + MaybeSync {
    async fn on_bitcoin_block(&self, height: u32);
    async fn on_liquid_block(&self, height: u32);
}

// A swap enum variant
#[derive(Clone, Debug)]
pub enum Swap {
    Chain(ChainSwap),
    Send(SendSwap),
    Receive(ReceiveSwap),
}
impl Swap {
    pub(crate) fn id(&self) -> String {
        match &self {
            Swap::Chain(ChainSwap { id, .. })
            | Swap::Send(SendSwap { id, .. })
            | Swap::Receive(ReceiveSwap { id, .. }) => id.clone(),
        }
    }

    pub(crate) fn version(&self) -> u64 {
        match self {
            Swap::Chain(ChainSwap { metadata, .. })
            | Swap::Send(SendSwap { metadata, .. })
            | Swap::Receive(ReceiveSwap { metadata, .. }) => metadata.version,
        }
    }

    pub(crate) fn set_version(&mut self, version: u64) {
        match self {
            Swap::Chain(chain_swap) => {
                chain_swap.metadata.version = version;
            }
            Swap::Send(send_swap) => {
                send_swap.metadata.version = version;
            }
            Swap::Receive(receive_swap) => {
                receive_swap.metadata.version = version;
            }
        }
    }

    pub(crate) fn last_updated_at(&self) -> u32 {
        match self {
            Swap::Chain(ChainSwap { metadata, .. })
            | Swap::Send(SendSwap { metadata, .. })
            | Swap::Receive(ReceiveSwap { metadata, .. }) => metadata.last_updated_at,
        }
    }
}
impl From<ChainSwap> for Swap {
    fn from(swap: ChainSwap) -> Self {
        Self::Chain(swap)
    }
}
impl From<SendSwap> for Swap {
    fn from(swap: SendSwap) -> Self {
        Self::Send(swap)
    }
}
impl From<ReceiveSwap> for Swap {
    fn from(swap: ReceiveSwap) -> Self {
        Self::Receive(swap)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SwapScriptV2 {
    Bitcoin(BtcSwapScript),
    Liquid(LBtcSwapScript),
}
impl SwapScriptV2 {
    pub(crate) fn as_bitcoin_script(&self) -> Result<BtcSwapScript> {
        match self {
            SwapScriptV2::Bitcoin(script) => Ok(script.clone()),
            _ => Err(anyhow!("Invalid chain")),
        }
    }

    pub(crate) fn as_liquid_script(&self) -> Result<LBtcSwapScript> {
        match self {
            SwapScriptV2::Liquid(script) => Ok(script.clone()),
            _ => Err(anyhow!("Invalid chain")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Incoming = 0,
    Outgoing = 1,
}
impl ToSql for Direction {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for Direction {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(Direction::Incoming),
                1 => Ok(Direction::Outgoing),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SwapMetadata {
    /// Version used for optimistic concurrency control within local db
    pub(crate) version: u64,
    pub(crate) last_updated_at: u32,
    pub(crate) is_local: bool,
}

/// A chain swap
///
/// See <https://docs.boltz.exchange/v/api/lifecycle#chain-swaps>
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct ChainSwap {
    pub(crate) id: String,
    pub(crate) direction: Direction,
    /// Always set for Outgoing Chain Swaps. It's the destination Bitcoin address
    /// Only set for Incoming Chain Swaps when a claim is broadcasted. It's a local Liquid wallet address
    pub(crate) claim_address: Option<String>,
    pub(crate) lockup_address: String,
    /// The Liquid refund address is only set for Outgoing Chain Swaps
    pub(crate) refund_address: Option<String>,
    /// The block height at which the swap will expire (origin chain)
    pub(crate) timeout_block_height: u32,
    /// The block height at which the claim will expire (destination chain)
    pub(crate) claim_timeout_block_height: u32,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    /// Payer amount defined at swap creation
    pub(crate) payer_amount_sat: u64,
    /// The actual payer amount as seen on the user lockup tx. Might differ from `payer_amount_sat`
    /// in the case of an over/underpayment
    pub(crate) actual_payer_amount_sat: Option<u64>,
    /// Receiver amount defined at swap creation
    pub(crate) receiver_amount_sat: u64,
    /// The final receiver amount, in case of an amountless swap for which fees have been accepted
    pub(crate) accepted_receiver_amount_sat: Option<u64>,
    pub(crate) claim_fees_sat: u64,
    /// The [ChainPair] chosen on swap creation
    pub(crate) pair_fees_json: String,
    pub(crate) accept_zero_conf: bool,
    /// JSON representation of [crate::persist::chain::InternalCreateChainResponse]
    pub(crate) create_response_json: String,
    /// Persisted only when the server lockup tx is successfully broadcast
    pub(crate) server_lockup_tx_id: Option<String>,
    /// Persisted only when the user lockup tx is successfully broadcast
    pub(crate) user_lockup_tx_id: Option<String>,
    /// Persisted as soon as a claim tx is broadcast
    pub(crate) claim_tx_id: Option<String>,
    /// Persisted as soon as a refund tx is broadcast
    pub(crate) refund_tx_id: Option<String>,
    pub(crate) created_at: u32,
    pub(crate) state: PaymentState,
    pub(crate) claim_private_key: String,
    pub(crate) refund_private_key: String,
    pub(crate) auto_accepted_fees: bool,
    /// Swap metadata that is only valid when reading one from the local database
    #[derivative(PartialEq = "ignore")]
    pub(crate) metadata: SwapMetadata,
}
impl ChainSwap {
    pub(crate) fn get_claim_keypair(&self) -> SdkResult<Keypair> {
        utils::decode_keypair(&self.claim_private_key)
    }

    pub(crate) fn get_refund_keypair(&self) -> SdkResult<Keypair> {
        utils::decode_keypair(&self.refund_private_key)
    }

    pub(crate) fn get_boltz_create_response(&self) -> Result<CreateChainResponse> {
        let internal_create_response: crate::persist::chain::InternalCreateChainResponse =
            serde_json::from_str(&self.create_response_json).map_err(|e| {
                anyhow!("Failed to deserialize InternalCreateSubmarineResponse: {e:?}")
            })?;

        Ok(CreateChainResponse {
            id: self.id.clone(),
            claim_details: internal_create_response.claim_details,
            lockup_details: internal_create_response.lockup_details,
        })
    }

    pub(crate) fn get_boltz_pair(&self) -> Result<ChainPair> {
        let pair: ChainPair = serde_json::from_str(&self.pair_fees_json)
            .map_err(|e| anyhow!("Failed to deserialize ChainPair: {e:?}"))?;

        Ok(pair)
    }

    pub(crate) fn get_claim_swap_script(&self) -> SdkResult<SwapScriptV2> {
        let chain_swap_details = self.get_boltz_create_response()?.claim_details;
        let our_pubkey = self.get_claim_keypair()?.public_key();
        let swap_script = match self.direction {
            Direction::Incoming => SwapScriptV2::Liquid(LBtcSwapScript::chain_from_swap_resp(
                Side::Claim,
                chain_swap_details,
                our_pubkey.into(),
            )?),
            Direction::Outgoing => SwapScriptV2::Bitcoin(BtcSwapScript::chain_from_swap_resp(
                Side::Claim,
                chain_swap_details,
                our_pubkey.into(),
            )?),
        };
        Ok(swap_script)
    }

    pub(crate) fn get_lockup_swap_script(&self) -> SdkResult<SwapScriptV2> {
        let chain_swap_details = self.get_boltz_create_response()?.lockup_details;
        let our_pubkey = self.get_refund_keypair()?.public_key();
        let swap_script = match self.direction {
            Direction::Incoming => SwapScriptV2::Bitcoin(BtcSwapScript::chain_from_swap_resp(
                Side::Lockup,
                chain_swap_details,
                our_pubkey.into(),
            )?),
            Direction::Outgoing => SwapScriptV2::Liquid(LBtcSwapScript::chain_from_swap_resp(
                Side::Lockup,
                chain_swap_details,
                our_pubkey.into(),
            )?),
        };
        Ok(swap_script)
    }

    /// Returns the lockup script pubkey for Receive Chain Swaps
    pub(crate) fn get_receive_lockup_swap_script_pubkey(
        &self,
        network: LiquidNetwork,
    ) -> SdkResult<ScriptBuf> {
        let swap_script = self.get_lockup_swap_script()?.as_bitcoin_script()?;
        let script_pubkey = swap_script
            .to_address(network.as_bitcoin_chain())
            .map_err(|e| SdkError::generic(format!("Error getting script address: {e:?}")))?
            .script_pubkey();
        Ok(script_pubkey)
    }

    pub(crate) fn to_refundable(&self, amount_sat: u64) -> RefundableSwap {
        RefundableSwap {
            swap_address: self.lockup_address.clone(),
            timestamp: self.created_at,
            amount_sat,
            last_refund_tx_id: self.refund_tx_id.clone(),
        }
    }

    pub(crate) fn from_boltz_struct_to_json(
        create_response: &CreateChainResponse,
        expected_swap_id: &str,
    ) -> Result<String, PaymentError> {
        let internal_create_response =
            crate::persist::chain::InternalCreateChainResponse::try_convert_from_boltz(
                create_response,
                expected_swap_id,
            )?;

        let create_response_json =
            serde_json::to_string(&internal_create_response).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to serialize InternalCreateChainResponse: {e:?}"),
                }
            })?;

        Ok(create_response_json)
    }

    pub(crate) fn is_waiting_fee_acceptance(&self) -> bool {
        self.payer_amount_sat == 0 && self.accepted_receiver_amount_sat.is_none()
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ChainSwapUpdate {
    pub(crate) swap_id: String,
    pub(crate) to_state: PaymentState,
    pub(crate) server_lockup_tx_id: Option<String>,
    pub(crate) user_lockup_tx_id: Option<String>,
    pub(crate) claim_address: Option<String>,
    pub(crate) claim_tx_id: Option<String>,
    pub(crate) refund_tx_id: Option<String>,
}

/// A submarine swap, used for Send
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct SendSwap {
    pub(crate) id: String,
    /// Bolt11 or Bolt12 invoice. This is determined by whether `bolt12_offer` is set or not.
    pub(crate) invoice: String,
    /// The bolt12 offer, if this swap sends to a Bolt12 offer
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) destination_pubkey: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) preimage: Option<String>,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    /// The [SubmarinePair] chosen on swap creation
    pub(crate) pair_fees_json: String,
    /// JSON representation of [crate::persist::send::InternalCreateSubmarineResponse]
    pub(crate) create_response_json: String,
    /// Persisted only when the lockup tx is successfully broadcast
    pub(crate) lockup_tx_id: Option<String>,
    /// Persisted only when a refund tx is needed
    pub(crate) refund_address: Option<String>,
    /// Persisted after the refund tx is broadcast
    pub(crate) refund_tx_id: Option<String>,
    pub(crate) created_at: u32,
    pub(crate) timeout_block_height: u64,
    pub(crate) state: PaymentState,
    pub(crate) refund_private_key: String,
    /// Swap metadata that is only valid when reading one from the local database
    #[derivative(PartialEq = "ignore")]
    pub(crate) metadata: SwapMetadata,
}
impl SendSwap {
    pub(crate) fn get_refund_keypair(&self) -> Result<Keypair, SdkError> {
        utils::decode_keypair(&self.refund_private_key)
    }

    pub(crate) fn get_boltz_create_response(&self) -> Result<CreateSubmarineResponse> {
        let internal_create_response: crate::persist::send::InternalCreateSubmarineResponse =
            serde_json::from_str(&self.create_response_json).map_err(|e| {
                anyhow!("Failed to deserialize InternalCreateSubmarineResponse: {e:?}")
            })?;

        let res = CreateSubmarineResponse {
            id: self.id.clone(),
            accept_zero_conf: internal_create_response.accept_zero_conf,
            address: internal_create_response.address.clone(),
            bip21: internal_create_response.bip21.clone(),
            claim_public_key: crate::utils::json_to_pubkey(
                &internal_create_response.claim_public_key,
            )?,
            expected_amount: internal_create_response.expected_amount,
            referral_id: internal_create_response.referral_id,
            swap_tree: internal_create_response.swap_tree.clone().into(),
            timeout_block_height: internal_create_response.timeout_block_height,
            blinding_key: internal_create_response.blinding_key.clone(),
        };
        Ok(res)
    }

    pub(crate) fn get_swap_script(&self) -> Result<LBtcSwapScript, SdkError> {
        LBtcSwapScript::submarine_from_swap_resp(
            &self.get_boltz_create_response()?,
            self.get_refund_keypair()?.public_key().into(),
        )
        .map_err(|e| {
            SdkError::generic(format!(
                "Failed to create swap script for Send Swap {}: {e:?}",
                self.id
            ))
        })
    }

    pub(crate) fn from_boltz_struct_to_json(
        create_response: &CreateSubmarineResponse,
        expected_swap_id: &str,
    ) -> Result<String, PaymentError> {
        let internal_create_response =
            crate::persist::send::InternalCreateSubmarineResponse::try_convert_from_boltz(
                create_response,
                expected_swap_id,
            )?;

        let create_response_json =
            serde_json::to_string(&internal_create_response).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to serialize InternalCreateSubmarineResponse: {e:?}"),
                }
            })?;

        Ok(create_response_json)
    }
}

/// A reverse swap, used for Receive
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct ReceiveSwap {
    pub(crate) id: String,
    pub(crate) preimage: String,
    /// JSON representation of [crate::persist::receive::InternalCreateReverseResponse]
    pub(crate) create_response_json: String,
    pub(crate) claim_private_key: String,
    pub(crate) invoice: String,
    /// The bolt12 offer, if used to create the swap
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) destination_pubkey: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) payer_note: Option<String>,
    /// The amount of the invoice
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    /// The [ReversePair] chosen on swap creation
    pub(crate) pair_fees_json: String,
    pub(crate) claim_fees_sat: u64,
    /// Persisted only when a claim tx is needed
    pub(crate) claim_address: Option<String>,
    /// Persisted after the claim tx is broadcast
    pub(crate) claim_tx_id: Option<String>,
    /// The transaction id of the swapper's tx broadcast
    pub(crate) lockup_tx_id: Option<String>,
    /// The address reserved for a magic routing hint payment
    pub(crate) mrh_address: String,
    /// Persisted only if a transaction is sent to the `mrh_address`
    pub(crate) mrh_tx_id: Option<String>,
    /// Until the lockup tx is seen in the mempool, it contains the swap creation time.
    /// Afterwards, it shows the lockup tx creation time.
    pub(crate) created_at: u32,
    pub(crate) timeout_block_height: u32,
    pub(crate) state: PaymentState,
    /// Swap metadata that is only valid when reading one from the local database
    #[derivative(PartialEq = "ignore")]
    pub(crate) metadata: SwapMetadata,
}
impl ReceiveSwap {
    pub(crate) fn get_claim_keypair(&self) -> Result<Keypair, PaymentError> {
        utils::decode_keypair(&self.claim_private_key).map_err(Into::into)
    }

    pub(crate) fn claim_script(&self) -> Result<elements::Script> {
        Ok(self
            .get_swap_script()?
            .funding_addrs
            .ok_or(anyhow!("No funding address found"))?
            .script_pubkey())
    }

    pub(crate) fn get_boltz_create_response(&self) -> Result<CreateReverseResponse, PaymentError> {
        let internal_create_response: crate::persist::receive::InternalCreateReverseResponse =
            serde_json::from_str(&self.create_response_json).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to deserialize InternalCreateReverseResponse: {e:?}"),
                }
            })?;

        let res = CreateReverseResponse {
            id: self.id.clone(),
            invoice: Some(self.invoice.clone()),
            swap_tree: internal_create_response.swap_tree.clone().into(),
            lockup_address: internal_create_response.lockup_address.clone(),
            refund_public_key: crate::utils::json_to_pubkey(
                &internal_create_response.refund_public_key,
            )?,
            timeout_block_height: internal_create_response.timeout_block_height,
            onchain_amount: internal_create_response.onchain_amount,
            blinding_key: internal_create_response.blinding_key.clone(),
        };
        Ok(res)
    }

    pub(crate) fn get_swap_script(&self) -> Result<LBtcSwapScript, PaymentError> {
        let keypair = self.get_claim_keypair()?;
        let create_response =
            self.get_boltz_create_response()
                .map_err(|e| PaymentError::Generic {
                    err: format!(
                        "Failed to create swap script for Receive Swap {}: {e:?}",
                        self.id
                    ),
                })?;
        LBtcSwapScript::reverse_from_swap_resp(&create_response, keypair.public_key().into())
            .map_err(|e| PaymentError::Generic {
                err: format!(
                    "Failed to create swap script for Receive Swap {}: {e:?}",
                    self.id
                ),
            })
    }

    pub(crate) fn from_boltz_struct_to_json(
        create_response: &CreateReverseResponse,
        expected_swap_id: &str,
        expected_invoice: Option<&str>,
    ) -> Result<String, PaymentError> {
        let internal_create_response =
            crate::persist::receive::InternalCreateReverseResponse::try_convert_from_boltz(
                create_response,
                expected_swap_id,
                expected_invoice,
            )?;

        let create_response_json =
            serde_json::to_string(&internal_create_response).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to serialize InternalCreateReverseResponse: {e:?}"),
                }
            })?;

        Ok(create_response_json)
    }
}

/// Returned when calling [crate::sdk::LiquidSdk::list_refundables].
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RefundableSwap {
    pub swap_address: String,
    pub timestamp: u32,
    /// Amount that is refundable, from all UTXOs
    pub amount_sat: u64,
    /// The txid of the last broadcasted refund tx, if any
    pub last_refund_tx_id: Option<String>,
}

/// A BOLT12 offer
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub(crate) struct Bolt12Offer {
    /// The BOLT12 offer string
    pub(crate) id: String,
    /// The description of the offer
    pub(crate) description: String,
    /// The private key used to create the offer
    pub(crate) private_key: String,
    /// The optional webhook URL where the offer invoice request will be sent
    pub(crate) webhook_url: Option<String>,
    /// The creation time of the offer
    pub(crate) created_at: u32,
}
impl Bolt12Offer {
    pub(crate) fn get_keypair(&self) -> Result<Keypair, SdkError> {
        utils::decode_keypair(&self.private_key)
    }
}
impl TryFrom<Bolt12Offer> for Offer {
    type Error = SdkError;

    fn try_from(val: Bolt12Offer) -> Result<Self, Self::Error> {
        Offer::from_str(&val.id)
            .map_err(|e| SdkError::generic(format!("Failed to parse BOLT12 offer: {e:?}")))
    }
}

/// The payment state of an individual payment.
#[derive(Clone, Copy, Debug, Default, EnumString, Eq, PartialEq, Serialize, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum PaymentState {
    #[default]
    Created = 0,

    /// ## Receive Swaps
    ///
    /// Covers the cases when
    /// - the lockup tx is seen in the mempool or
    /// - our claim tx is broadcast
    ///
    /// When the claim tx is broadcast, `claim_tx_id` is set in the swap.
    ///
    /// ## Send Swaps
    ///
    /// This is the status when our lockup tx was broadcast
    ///
    /// ## Chain Swaps
    ///
    /// This is the status when the user lockup tx was broadcast
    ///
    /// ## No swap data available
    ///
    /// If no associated swap is found, this indicates the underlying tx is not confirmed yet.
    Pending = 1,

    /// ## Receive Swaps
    ///
    /// Covers the case when the claim tx is confirmed.
    ///
    /// ## Send and Chain Swaps
    ///
    /// This is the status when the claim tx is broadcast and we see it in the mempool.
    ///
    /// ## No swap data available
    ///
    /// If no associated swap is found, this indicates the underlying tx is confirmed.
    Complete = 2,

    /// ## Receive Swaps
    ///
    /// This is the status when the swap failed for any reason and the Receive could not complete.
    ///
    /// ## Send and Chain Swaps
    ///
    /// This is the status when a swap refund was initiated and the refund tx is confirmed.
    Failed = 3,

    /// ## Send and Outgoing Chain Swaps
    ///
    /// This covers the case when the swap state is still Created and the swap fails to reach the
    /// Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
    TimedOut = 4,

    /// ## Incoming Chain Swaps
    ///
    /// This covers the case when the swap failed for any reason and there is a user lockup tx.
    /// The swap in this case has to be manually refunded with a provided Bitcoin address
    Refundable = 5,

    /// ## Send and Chain Swaps
    ///
    /// This is the status when a refund was initiated and/or our refund tx was broadcast
    ///
    /// When the refund tx is broadcast, `refund_tx_id` is set in the swap.
    RefundPending = 6,

    /// ## Chain Swaps
    ///
    /// This is the state when the user needs to accept new fees before the payment can proceed.
    ///
    /// Use [LiquidSdk::fetch_payment_proposed_fees](crate::sdk::LiquidSdk::fetch_payment_proposed_fees)
    /// to find out the current fees and
    /// [LiquidSdk::accept_payment_proposed_fees](crate::sdk::LiquidSdk::accept_payment_proposed_fees)
    /// to accept them, allowing the payment to proceed.
    ///
    /// Otherwise, this payment can be immediately refunded using
    /// [prepare_refund](crate::sdk::LiquidSdk::prepare_refund)/[refund](crate::sdk::LiquidSdk::refund).
    WaitingFeeAcceptance = 7,
}

impl ToSql for PaymentState {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentState::Created),
                1 => Ok(PaymentState::Pending),
                2 => Ok(PaymentState::Complete),
                3 => Ok(PaymentState::Failed),
                4 => Ok(PaymentState::TimedOut),
                5 => Ok(PaymentState::Refundable),
                6 => Ok(PaymentState::RefundPending),
                7 => Ok(PaymentState::WaitingFeeAcceptance),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl PaymentState {
    pub(crate) fn is_refundable(&self) -> bool {
        matches!(
            self,
            PaymentState::Refundable
                | PaymentState::RefundPending
                | PaymentState::WaitingFeeAcceptance
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, EnumString, Display, Hash, PartialEq, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum PaymentType {
    Receive = 0,
    Send = 1,
}
impl From<Direction> for PaymentType {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Incoming => Self::Receive,
            Direction::Outgoing => Self::Send,
        }
    }
}
impl ToSql for PaymentType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentType::Receive),
                1 => Ok(PaymentType::Send),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PaymentStatus {
    Pending = 0,
    Complete = 1,
}
impl ToSql for PaymentStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}
impl FromSql for PaymentStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentStatus::Pending),
                1 => Ok(PaymentStatus::Complete),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentTxData {
    /// The tx ID of the transaction
    pub tx_id: String,

    /// The point in time when the underlying tx was included in a block.
    pub timestamp: Option<u32>,

    /// The onchain fees of this tx
    pub fees_sat: u64,

    /// Onchain tx status
    pub is_confirmed: bool,

    /// Data to use in the `blinded` param when unblinding the transaction in an explorer.
    /// See: <https://docs.liquid.net/docs/unblinding-transactions>
    pub unblinding_data: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum PaymentSwapType {
    Receive,
    Send,
    Chain,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSwapData {
    pub swap_id: String,

    pub swap_type: PaymentSwapType,

    /// Swap creation timestamp
    pub created_at: u32,

    /// The height of the block at which the swap will no longer be valid
    /// For chain swaps, this is the timeout on the origin chain
    pub expiration_blockheight: u32,

    /// For chain swaps, this is the timeout on the destination chain
    pub claim_expiration_blockheight: Option<u32>,

    pub preimage: Option<String>,
    pub invoice: Option<String>,
    pub bolt12_offer: Option<String>,
    pub payment_hash: Option<String>,
    pub destination_pubkey: Option<String>,
    pub description: String,
    pub payer_note: Option<String>,

    /// Amount sent by the swap payer
    pub payer_amount_sat: u64,

    /// Amount received by the swap receiver
    pub receiver_amount_sat: u64,

    /// The swapper service fee
    pub swapper_fees_sat: u64,

    pub refund_tx_id: Option<String>,
    pub refund_tx_amount_sat: Option<u64>,

    /// Present only for chain swaps.
    /// It's the Bitcoin address that receives funds.
    pub bitcoin_address: Option<String>,

    /// Payment status derived from the swap status
    pub status: PaymentState,
}

/// Represents the payment LNURL info
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LnUrlInfo {
    pub ln_address: Option<String>,
    pub lnurl_pay_comment: Option<String>,
    pub lnurl_pay_domain: Option<String>,
    pub lnurl_pay_metadata: Option<String>,
    pub lnurl_pay_success_action: Option<SuccessActionProcessed>,
    pub lnurl_pay_unprocessed_success_action: Option<SuccessAction>,
    pub lnurl_withdraw_endpoint: Option<String>,
}

/// Configuration for asset metadata. Each asset metadata item represents an entry in the
/// [Liquid Asset Registry](https://docs.liquid.net/docs/blockstream-liquid-asset-registry).
/// An example Liquid Asset in the registry would be [Tether USD](https://assets.blockstream.info/ce091c998b83c78bb71a632313ba3760f1763d9cfcffae02258ffa9865a37bd2.json>).
#[derive(Debug, Clone, Serialize)]
pub struct AssetMetadata {
    /// The asset id of the registered asset
    pub asset_id: String,
    /// The name of the asset
    pub name: String,
    /// The ticker of the asset
    pub ticker: String,
    /// The precision used to display the asset amount.
    /// For example, precision of 2 shifts the decimal 2 places left from the satoshi amount.
    pub precision: u8,
    /// The optional ID of the fiat currency used to represent the asset
    pub fiat_id: Option<String>,
}

impl AssetMetadata {
    pub fn amount_to_sat(&self, amount: f64) -> u64 {
        (amount * (10_u64.pow(self.precision.into()) as f64)) as u64
    }

    pub fn amount_from_sat(&self, amount_sat: u64) -> f64 {
        amount_sat as f64 / (10_u64.pow(self.precision.into()) as f64)
    }
}

/// Represents the Liquid payment asset info. The asset info is derived from
/// the available [AssetMetadata] that is set in the [Config].
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssetInfo {
    /// The name of the asset
    pub name: String,
    /// The ticker of the asset
    pub ticker: String,
    /// The amount calculated from the satoshi amount of the transaction, having its
    /// decimal shifted to the left by the [precision](AssetMetadata::precision)
    pub amount: f64,
    /// The optional fees when paid using the asset, having its
    /// decimal shifted to the left by the [precision](AssetMetadata::precision)
    pub fees: Option<f64>,
}

/// The specific details of a payment, depending on its type
#[derive(Debug, Clone, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum PaymentDetails {
    /// Swapping to or from Lightning
    Lightning {
        swap_id: String,

        /// Represents the invoice description
        description: String,

        /// The height of the block at which the swap will no longer be valid
        liquid_expiration_blockheight: u32,

        /// The preimage of the paid invoice (proof of payment).
        preimage: Option<String>,

        /// Represents the Bolt11/Bolt12 invoice associated with a payment
        /// In the case of a Send payment, this is the invoice paid by the swapper
        /// In the case of a Receive payment, this is the invoice paid by the user
        invoice: Option<String>,

        bolt12_offer: Option<String>,

        /// The payment hash of the invoice
        payment_hash: Option<String>,

        /// The invoice destination/payee pubkey
        destination_pubkey: Option<String>,

        /// The payment LNURL info
        lnurl_info: Option<LnUrlInfo>,

        /// The BIP353 address used to resolve this payment
        bip353_address: Option<String>,

        /// The payer note
        payer_note: Option<String>,

        /// For a Receive payment, this is the claim tx id in case it has already been broadcast
        claim_tx_id: Option<String>,

        /// For a Send swap which was refunded, this is the refund tx id
        refund_tx_id: Option<String>,

        /// For a Send swap which was refunded, this is the refund amount
        refund_tx_amount_sat: Option<u64>,
    },
    /// Direct onchain payment to a Liquid address
    Liquid {
        /// Represents either a Liquid BIP21 URI or pure address
        destination: String,

        /// Represents the BIP21 `message` field
        description: String,

        /// The asset id
        asset_id: String,

        /// The asset info derived from the [AssetMetadata]
        asset_info: Option<AssetInfo>,

        /// The payment LNURL info
        lnurl_info: Option<LnUrlInfo>,

        /// The BIP353 address used to resolve this payment
        bip353_address: Option<String>,

        /// The payer note
        payer_note: Option<String>,
    },
    /// Swapping to or from the Bitcoin chain
    Bitcoin {
        swap_id: String,

        /// The Bitcoin address that receives funds.
        bitcoin_address: String,

        /// Represents the invoice description
        description: String,

        /// For an amountless receive swap, this indicates if fees were automatically accepted.
        /// Fees are auto accepted when the swapper proposes fees that are within the initial
        /// estimate, plus the `onchain_fee_rate_leeway_sat_per_vbyte` set in the [Config], if any.
        auto_accepted_fees: bool,

        /// The height of the Liquid block at which the swap will no longer be valid
        liquid_expiration_blockheight: u32,

        /// The height of the Bitcoin block at which the swap will no longer be valid
        bitcoin_expiration_blockheight: u32,

        /// The lockup tx id that initiates the swap
        lockup_tx_id: Option<String>,

        /// The claim tx id that claims the server lockup tx
        claim_tx_id: Option<String>,

        /// For a Send swap which was refunded, this is the refund tx id
        refund_tx_id: Option<String>,

        /// For a Send swap which was refunded, this is the refund amount
        refund_tx_amount_sat: Option<u64>,
    },
}

impl PaymentDetails {
    pub(crate) fn get_swap_id(&self) -> Option<String> {
        match self {
            Self::Lightning { swap_id, .. } | Self::Bitcoin { swap_id, .. } => {
                Some(swap_id.clone())
            }
            Self::Liquid { .. } => None,
        }
    }

    pub(crate) fn get_refund_tx_amount_sat(&self) -> Option<u64> {
        match self {
            Self::Lightning {
                refund_tx_amount_sat,
                ..
            }
            | Self::Bitcoin {
                refund_tx_amount_sat,
                ..
            } => *refund_tx_amount_sat,
            Self::Liquid { .. } => None,
        }
    }

    pub(crate) fn get_description(&self) -> Option<String> {
        match self {
            Self::Lightning { description, .. }
            | Self::Bitcoin { description, .. }
            | Self::Liquid { description, .. } => Some(description.clone()),
        }
    }

    pub(crate) fn is_lbtc_asset_id(&self, network: LiquidNetwork) -> bool {
        match self {
            Self::Liquid { asset_id, .. } => {
                asset_id.eq(&utils::lbtc_asset_id(network).to_string())
            }
            _ => true,
        }
    }
}

/// Represents an SDK payment.
///
/// By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Payment {
    /// The destination associated with the payment, if it was created via our SDK.
    /// Can be either a Liquid/Bitcoin address, a Liquid BIP21 URI or an invoice
    pub destination: Option<String>,

    pub tx_id: Option<String>,

    /// Data to use in the `blinded` param when unblinding the transaction in an explorer.
    /// See: <https://docs.liquid.net/docs/unblinding-transactions>
    pub unblinding_data: Option<String>,

    /// Composite timestamp that can be used for sorting or displaying the payment.
    ///
    /// If this payment has an associated swap, it is the swap creation time. Otherwise, the point
    /// in time when the underlying tx was included in a block. If there is no associated swap
    /// available and the underlying tx is not yet confirmed, the value is `now()`.
    pub timestamp: u32,

    /// The payment amount, which corresponds to the onchain tx amount.
    ///
    /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
    pub amount_sat: u64,

    /// Represents the fees paid by this wallet for this payment.
    ///
    /// ### Swaps
    /// If there is an associated Send Swap, these fees represent the total fees paid by this wallet
    /// (the sender). It is the difference between the amount that was sent and the amount received.
    ///
    /// If there is an associated Receive Swap, these fees represent the total fees paid by this wallet
    /// (the receiver). It is also the difference between the amount that was sent and the amount received.
    ///
    /// ### Pure onchain txs
    /// If no swap is associated with this payment:
    /// - for Send payments, this is the onchain tx fee
    /// - for Receive payments, this is zero
    pub fees_sat: u64,

    /// Service fees paid to the swapper service. This is only set for swaps (i.e. doesn't apply to
    /// direct Liquid payments).
    pub swapper_fees_sat: Option<u64>,

    /// If it is a `Send` or `Receive` payment
    pub payment_type: PaymentType,

    /// Composite status representing the overall status of the payment.
    ///
    /// If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
    ///
    /// If the tx has an associated swap, this is determined by the swap status (pending or complete).
    pub status: PaymentState,

    /// The details of a payment, depending on its [destination](Payment::destination) and
    /// [type](Payment::payment_type)
    pub details: PaymentDetails,
}
impl Payment {
    pub(crate) fn from_pending_swap(
        swap: PaymentSwapData,
        payment_type: PaymentType,
        payment_details: PaymentDetails,
    ) -> Payment {
        let amount_sat = match payment_type {
            PaymentType::Receive => swap.receiver_amount_sat,
            PaymentType::Send => swap.payer_amount_sat,
        };

        Payment {
            destination: swap.invoice.clone(),
            tx_id: None,
            unblinding_data: None,
            timestamp: swap.created_at,
            amount_sat,
            fees_sat: swap
                .payer_amount_sat
                .saturating_sub(swap.receiver_amount_sat),
            swapper_fees_sat: Some(swap.swapper_fees_sat),
            payment_type,
            status: swap.status,
            details: payment_details,
        }
    }

    pub(crate) fn from_tx_data(
        tx: PaymentTxData,
        balance: PaymentTxBalance,
        swap: Option<PaymentSwapData>,
        details: PaymentDetails,
    ) -> Payment {
        let (amount_sat, fees_sat) = match swap.as_ref() {
            Some(s) => match balance.payment_type {
                // For receive swaps, to avoid some edge case issues related to potential past
                // overpayments, we use the actual claim value as the final received amount
                // for fee calculation.
                PaymentType::Receive => (
                    balance.amount,
                    s.payer_amount_sat.saturating_sub(balance.amount),
                ),
                PaymentType::Send => (
                    s.receiver_amount_sat,
                    s.payer_amount_sat.saturating_sub(s.receiver_amount_sat),
                ),
            },
            None => {
                let (amount_sat, fees_sat) = match balance.payment_type {
                    PaymentType::Receive => (balance.amount, 0),
                    PaymentType::Send => (balance.amount, tx.fees_sat),
                };
                // If the payment is a Liquid payment, we only show the amount if the asset
                // is LBTC and only show the fees if the asset info has no set fees
                match details {
                    PaymentDetails::Liquid {
                        asset_info: Some(ref asset_info),
                        ..
                    } if asset_info.ticker != "BTC" => (0, asset_info.fees.map_or(fees_sat, |_| 0)),
                    _ => (amount_sat, fees_sat),
                }
            }
        };
        Payment {
            tx_id: Some(tx.tx_id),
            unblinding_data: tx.unblinding_data,
            // When the swap is present and of type send and receive, we retrieve the destination from the invoice.
            // If it's a chain swap instead, we use the `bitcoin_address` field from the swap data.
            // Otherwise, we specify the Liquid address (BIP21 or pure), set in `payment_details.address`.
            destination: match &swap {
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Receive,
                    invoice,
                    ..
                }) => invoice.clone(),
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Send,
                    invoice,
                    bolt12_offer,
                    ..
                }) => bolt12_offer.clone().or(invoice.clone()),
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Chain,
                    bitcoin_address,
                    ..
                }) => bitcoin_address.clone(),
                _ => match &details {
                    PaymentDetails::Liquid { destination, .. } => Some(destination.clone()),
                    _ => None,
                },
            },
            timestamp: tx
                .timestamp
                .or(swap.as_ref().map(|s| s.created_at))
                .unwrap_or(utils::now()),
            amount_sat,
            fees_sat,
            swapper_fees_sat: swap.as_ref().map(|s| s.swapper_fees_sat),
            payment_type: balance.payment_type,
            status: match &swap {
                Some(swap) => swap.status,
                None => match tx.is_confirmed {
                    true => PaymentState::Complete,
                    false => PaymentState::Pending,
                },
            },
            details,
        }
    }

    pub(crate) fn get_refund_tx_id(&self) -> Option<String> {
        match self.details.clone() {
            PaymentDetails::Lightning { refund_tx_id, .. } => Some(refund_tx_id),
            PaymentDetails::Bitcoin { refund_tx_id, .. } => Some(refund_tx_id),
            PaymentDetails::Liquid { .. } => None,
        }
        .flatten()
    }
}

/// Returned when calling [crate::sdk::LiquidSdk::recommended_fees].
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedFees {
    pub fastest_fee: u64,
    pub half_hour_fee: u64,
    pub hour_fee: u64,
    pub economy_fee: u64,
    pub minimum_fee: u64,
}

/// An argument of [PrepareBuyBitcoinRequest] when calling [crate::sdk::LiquidSdk::prepare_buy_bitcoin].
#[derive(Debug, Clone, Copy, EnumString, PartialEq, Serialize)]
pub enum BuyBitcoinProvider {
    #[strum(serialize = "moonpay")]
    Moonpay,
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_buy_bitcoin].
#[derive(Debug, Serialize)]
pub struct PrepareBuyBitcoinRequest {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_buy_bitcoin].
#[derive(Clone, Debug, Serialize)]
pub struct PrepareBuyBitcoinResponse {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
    pub fees_sat: u64,
}

/// An argument when calling [crate::sdk::LiquidSdk::buy_bitcoin].
#[derive(Clone, Debug, Serialize)]
pub struct BuyBitcoinRequest {
    pub prepare_response: PrepareBuyBitcoinResponse,
    /// The optional URL to redirect to after completing the buy.
    ///
    /// For Moonpay, see <https://dev.moonpay.com/docs/on-ramp-configure-user-journey-params>
    pub redirect_url: Option<String>,
}

/// Internal SDK log entry used in the Uniffi and Dart bindings
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InternalLeaf {
    pub output: String,
    pub version: u8,
}
impl From<InternalLeaf> for Leaf {
    fn from(value: InternalLeaf) -> Self {
        Leaf {
            output: value.output,
            version: value.version,
        }
    }
}
impl From<Leaf> for InternalLeaf {
    fn from(value: Leaf) -> Self {
        InternalLeaf {
            output: value.output,
            version: value.version,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct InternalSwapTree {
    claim_leaf: InternalLeaf,
    refund_leaf: InternalLeaf,
}
impl From<InternalSwapTree> for SwapTree {
    fn from(value: InternalSwapTree) -> Self {
        SwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}
impl From<SwapTree> for InternalSwapTree {
    fn from(value: SwapTree) -> Self {
        InternalSwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_lnurl_pay].
#[derive(Debug, Serialize)]
pub struct PrepareLnUrlPayRequest {
    /// The [LnUrlPayRequestData] returned by [parse]
    pub data: LnUrlPayRequestData,
    /// The amount to send
    pub amount: PayAmount,
    /// A BIP353 address, in case one was used in order to fetch the LNURL Pay request data.
    /// Returned by [parse].
    pub bip353_address: Option<String>,
    /// An optional comment LUD-12 to be stored with the payment. The comment is included in the
    /// invoice request sent to the LNURL endpoint.
    pub comment: Option<String>,
    /// Validates that, if there is a URL success action, the URL domain matches
    /// the LNURL callback domain. Defaults to `true`
    pub validate_success_action_url: Option<bool>,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_lnurl_pay].
#[derive(Debug, Serialize)]
pub struct PrepareLnUrlPayResponse {
    /// The destination of the payment
    pub destination: SendDestination,
    /// The fees in satoshis to send the payment
    pub fees_sat: u64,
    /// The [LnUrlPayRequestData] returned by [parse]
    pub data: LnUrlPayRequestData,
    /// The amount to send
    pub amount: PayAmount,
    /// An optional comment LUD-12 to be stored with the payment. The comment is included in the
    /// invoice request sent to the LNURL endpoint.
    pub comment: Option<String>,
    /// The unprocessed LUD-09 success action. This will be processed and decrypted if
    /// needed after calling [crate::sdk::LiquidSdk::lnurl_pay]
    pub success_action: Option<SuccessAction>,
}

/// An argument when calling [crate::sdk::LiquidSdk::lnurl_pay].
#[derive(Debug, Serialize)]
pub struct LnUrlPayRequest {
    /// The response from calling [crate::sdk::LiquidSdk::prepare_lnurl_pay]
    pub prepare_response: PrepareLnUrlPayResponse,
}

/// Contains the result of the entire LNURL-pay interaction, as reported by the LNURL endpoint.
///
/// * `EndpointSuccess` indicates the payment is complete. The endpoint may return a `SuccessActionProcessed`,
///   in which case, the wallet has to present it to the user as described in
///   <https://github.com/lnurl/luds/blob/luds/09.md>
///
/// * `EndpointError` indicates a generic issue the LNURL endpoint encountered, including a freetext
///   field with the reason.
///
/// * `PayError` indicates that an error occurred while trying to pay the invoice from the LNURL endpoint.
///   This includes the payment hash of the failed invoice and the failure reason.
#[derive(Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum LnUrlPayResult {
    EndpointSuccess { data: LnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[derive(Serialize)]
pub struct LnUrlPaySuccessData {
    pub payment: Payment,
    pub success_action: Option<SuccessActionProcessed>,
}

#[derive(Debug, Clone)]
pub enum Transaction {
    Liquid(boltz_client::elements::Transaction),
    Bitcoin(boltz_client::bitcoin::Transaction),
}

impl Transaction {
    pub(crate) fn txid(&self) -> String {
        match self {
            Transaction::Liquid(tx) => tx.txid().to_hex(),
            Transaction::Bitcoin(tx) => tx.compute_txid().to_hex(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Utxo {
    Liquid(
        Box<(
            boltz_client::elements::OutPoint,
            boltz_client::elements::TxOut,
        )>,
    ),
    Bitcoin(
        (
            boltz_client::bitcoin::OutPoint,
            boltz_client::bitcoin::TxOut,
        ),
    ),
}

impl Utxo {
    pub(crate) fn as_bitcoin(
        &self,
    ) -> Option<&(
        boltz_client::bitcoin::OutPoint,
        boltz_client::bitcoin::TxOut,
    )> {
        match self {
            Utxo::Liquid(_) => None,
            Utxo::Bitcoin(utxo) => Some(utxo),
        }
    }

    pub(crate) fn as_liquid(
        &self,
    ) -> Option<
        Box<(
            boltz_client::elements::OutPoint,
            boltz_client::elements::TxOut,
        )>,
    > {
        match self {
            Utxo::Bitcoin(_) => None,
            Utxo::Liquid(utxo) => Some(utxo.clone()),
        }
    }
}

/// An argument when calling [crate::sdk::LiquidSdk::fetch_payment_proposed_fees].
#[derive(Debug, Clone)]
pub struct FetchPaymentProposedFeesRequest {
    pub swap_id: String,
}

/// Returned when calling [crate::sdk::LiquidSdk::fetch_payment_proposed_fees].
#[derive(Debug, Clone, Serialize)]
pub struct FetchPaymentProposedFeesResponse {
    pub swap_id: String,
    pub fees_sat: u64,
    /// Amount sent by the swap payer
    pub payer_amount_sat: u64,
    /// Amount that will be received if these fees are accepted
    pub receiver_amount_sat: u64,
}

/// An argument when calling [crate::sdk::LiquidSdk::accept_payment_proposed_fees].
#[derive(Debug, Clone)]
pub struct AcceptPaymentProposedFeesRequest {
    pub response: FetchPaymentProposedFeesResponse,
}

#[derive(Clone, Debug)]
pub struct History<T> {
    pub txid: T,
    /// Confirmation height of txid
    ///
    /// -1 means unconfirmed with unconfirmed parents
    ///  0 means unconfirmed with confirmed parents
    pub height: i32,
}
pub(crate) type LBtcHistory = History<elements::Txid>;
pub(crate) type BtcHistory = History<bitcoin::Txid>;

impl<T> History<T> {
    pub(crate) fn confirmed(&self) -> bool {
        self.height > 0
    }
}
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
impl From<electrum_client::GetHistoryRes> for BtcHistory {
    fn from(value: electrum_client::GetHistoryRes) -> Self {
        Self {
            txid: value.tx_hash,
            height: value.height,
        }
    }
}
impl From<lwk_wollet::History> for LBtcHistory {
    fn from(value: lwk_wollet::History) -> Self {
        Self::from(&value)
    }
}
impl From<&lwk_wollet::History> for LBtcHistory {
    fn from(value: &lwk_wollet::History) -> Self {
        Self {
            txid: value.txid,
            height: value.height,
        }
    }
}
pub(crate) type BtcScript = bitcoin::ScriptBuf;
pub(crate) type LBtcScript = elements::Script;

#[derive(Clone, Debug)]
pub struct BtcScriptBalance {
    /// Confirmed balance in Satoshis for the address.
    pub confirmed: u64,
    /// Unconfirmed balance in Satoshis for the address.
    ///
    /// Some servers (e.g. `electrs`) return this as a negative value.
    pub unconfirmed: i64,
}
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
impl From<electrum_client::GetBalanceRes> for BtcScriptBalance {
    fn from(val: electrum_client::GetBalanceRes) -> Self {
        Self {
            confirmed: val.confirmed,
            unconfirmed: val.unconfirmed,
        }
    }
}

pub(crate) struct GetSyncContextRequest {
    pub partial_sync: Option<bool>,
    pub last_liquid_tip: u32,
    pub last_bitcoin_tip: u32,
}

pub(crate) struct SyncContext {
    pub maybe_liquid_tip: Option<u32>,
    pub maybe_bitcoin_tip: Option<u32>,
    pub recoverable_swaps: Vec<Swap>,
    pub is_new_liquid_block: bool,
    pub is_new_bitcoin_block: bool,
}

pub(crate) struct TaskHandle {
    pub name: String,
    pub handle: tokio::task::JoinHandle<()>,
}

#[macro_export]
macro_rules! get_updated_fields {
    ($($var:ident),* $(,)?) => {{
        let mut options = Vec::new();
        $(
            if $var.is_some() {
                options.push(stringify!($var).to_string());
            }
        )*
        match options.len() > 0 {
            true => Some(options),
            false => None,
        }
    }};
}
