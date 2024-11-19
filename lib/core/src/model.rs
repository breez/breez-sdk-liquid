use std::path::PathBuf;

use anyhow::{anyhow, Result};

use boltz_client::{
    bitcoin::ScriptBuf,
    network::Chain,
    swaps::boltz::{
        CreateChainResponse, CreateReverseResponse, CreateSubmarineResponse, Leaf, Side, SwapTree,
    },
    ToHex,
};
use boltz_client::{BtcSwapScript, Keypair, LBtcSwapScript};
use lwk_wollet::{bitcoin::bip32, ElementsNetwork};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use sdk_common::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::error::{PaymentError, SdkError, SdkResult};
use crate::receive_swap::{
    DEFAULT_ZERO_CONF_MAX_SAT, DEFAULT_ZERO_CONF_MIN_FEE_RATE_MAINNET,
    DEFAULT_ZERO_CONF_MIN_FEE_RATE_TESTNET,
};
use crate::utils;

// Both use f64 for the maximum precision when converting between units
pub const STANDARD_FEE_RATE_SAT_PER_VBYTE: f64 = 0.1;
pub const LOWBALL_FEE_RATE_SAT_PER_VBYTE: f64 = 0.01;

/// Configuration for the Liquid SDK
#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub liquid_electrum_url: String,
    pub bitcoin_electrum_url: String,
    /// The mempool.space API URL, has to be in the format: `https://mempool.space/api`
    pub mempoolspace_url: String,
    /// Directory in which the DB and log files are stored.
    ///
    /// Prefix can be a relative or absolute path to this directory.
    pub working_dir: String,
    /// Directory in which the Liquid wallet cache is stored. Defaults to `working_dir`
    pub cache_dir: Option<String>,
    pub network: LiquidNetwork,
    /// Send payment timeout. See [crate::sdk::LiquidSdk::send_payment]
    pub payment_timeout_sec: u64,
    /// Zero-conf minimum accepted fee-rate in millisatoshis per vbyte
    pub zero_conf_min_fee_rate_msat: u32,
    /// Maximum amount in satoshi to accept zero-conf payments with
    /// Defaults to [crate::receive_swap::DEFAULT_ZERO_CONF_MAX_SAT]
    pub zero_conf_max_amount_sat: Option<u64>,
    /// The Breez API key used for making requests to their mempool service
    pub breez_api_key: Option<String>,
}

impl Config {
    pub fn mainnet(breez_api_key: String) -> Self {
        Config {
            liquid_electrum_url: "elements-mainnet.breez.technology:50002".to_string(),
            bitcoin_electrum_url: "bitcoin-mainnet.blockstream.info:50002".to_string(),
            mempoolspace_url: "https://mempool.space/api".to_string(),
            working_dir: ".".to_string(),
            cache_dir: None,
            network: LiquidNetwork::Mainnet,
            payment_timeout_sec: 15,
            zero_conf_min_fee_rate_msat: DEFAULT_ZERO_CONF_MIN_FEE_RATE_MAINNET,
            zero_conf_max_amount_sat: None,
            breez_api_key: Some(breez_api_key),
        }
    }

    pub fn testnet(breez_api_key: Option<String>) -> Self {
        Config {
            liquid_electrum_url: "elements-testnet.blockstream.info:50002".to_string(),
            bitcoin_electrum_url: "bitcoin-testnet.blockstream.info:50002".to_string(),
            mempoolspace_url: "https://mempool.space/testnet/api".to_string(),
            working_dir: ".".to_string(),
            cache_dir: None,
            network: LiquidNetwork::Testnet,
            payment_timeout_sec: 15,
            zero_conf_min_fee_rate_msat: DEFAULT_ZERO_CONF_MIN_FEE_RATE_TESTNET,
            zero_conf_max_amount_sat: None,
            breez_api_key,
        }
    }

    pub(crate) fn get_wallet_dir(
        &self,
        base_dir: &str,
        fingerprint_hex: &str,
    ) -> anyhow::Result<String> {
        Ok(PathBuf::from(base_dir)
            .join(match self.network {
                LiquidNetwork::Mainnet => "mainnet",
                LiquidNetwork::Testnet => "testnet",
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

    pub(crate) fn lowball_fee_rate_msat_per_vbyte(&self) -> Option<f32> {
        match self.network {
            LiquidNetwork::Mainnet => Some((LOWBALL_FEE_RATE_SAT_PER_VBYTE * 1000.0) as f32),
            LiquidNetwork::Testnet => None,
        }
    }
}

/// Network chosen for this Liquid SDK instance. Note that it represents both the Liquid and the
/// Bitcoin network used.
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum LiquidNetwork {
    /// Mainnet Bitcoin and Liquid chains
    Mainnet,
    /// Testnet Bitcoin and Liquid chains
    Testnet,
}
impl LiquidNetwork {
    pub fn as_bitcoin_chain(&self) -> Chain {
        match self {
            LiquidNetwork::Mainnet => Chain::Bitcoin,
            LiquidNetwork::Testnet => Chain::BitcoinTestnet,
        }
    }
}

impl From<LiquidNetwork> for ElementsNetwork {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => ElementsNetwork::Liquid,
            LiquidNetwork::Testnet => ElementsNetwork::LiquidTestnet,
        }
    }
}

impl From<LiquidNetwork> for Chain {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Chain::Liquid,
            LiquidNetwork::Testnet => Chain::LiquidTestnet,
        }
    }
}

impl TryFrom<&str> for LiquidNetwork {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<LiquidNetwork, anyhow::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(LiquidNetwork::Mainnet),
            "testnet" => Ok(LiquidNetwork::Testnet),
            _ => Err(anyhow!("Invalid network")),
        }
    }
}

impl From<LiquidNetwork> for sdk_common::prelude::Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
        }
    }
}

impl From<LiquidNetwork> for sdk_common::bitcoin::Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
        }
    }
}

impl From<LiquidNetwork> for boltz_client::bitcoin::Network {
    fn from(value: LiquidNetwork) -> Self {
        match value {
            LiquidNetwork::Mainnet => Self::Bitcoin,
            LiquidNetwork::Testnet => Self::Testnet,
        }
    }
}

/// Trait that can be used to react to various [SdkEvent]s emitted by the SDK.
pub trait EventListener: Send + Sync {
    fn on_event(&self, e: SdkEvent);
}

/// Event emitted by the SDK. Add an [EventListener] by calling [crate::sdk::LiquidSdk::add_event_listener]
/// to listen for emitted events.
#[derive(Clone, Debug, PartialEq)]
pub enum SdkEvent {
    PaymentFailed { details: Payment },
    PaymentPending { details: Payment },
    PaymentRefunded { details: Payment },
    PaymentRefundPending { details: Payment },
    PaymentSucceeded { details: Payment },
    PaymentWaitingConfirmation { details: Payment },
    Synced,
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
pub trait Signer: Send + Sync {
    /// The master xpub encoded as 78 bytes length as defined in bip32 specification.
    /// For reference: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Serialization_format
    fn xpub(&self) -> Result<Vec<u8>, SignerError>;

    /// The derived xpub encoded as 78 bytes length as defined in bip32 specification.
    /// The derivation path is a string represents the shorter notation of the key tree to derive. For example:
    /// m/49'/1'/0'/0/0
    /// m/48'/1'/0'/0/0
    /// For reference: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-The_key_tree
    fn derive_xpub(&self, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    /// Sign an ECDSA message using the private key derived from the given derivation path
    fn sign_ecdsa(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    /// Sign an ECDSA message using the private key derived from the master key
    fn sign_ecdsa_recoverable(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;

    /// Return the master blinding key for SLIP77: https://github.com/satoshilabs/slips/blob/master/slip-0077.md
    fn slip77_master_blinding_key(&self) -> Result<Vec<u8>, SignerError>;

    /// HMAC-SHA256 using the private key derived from the given derivation path
    /// This is used to calculate the linking key of lnurl-auth specification: https://github.com/lnurl/luds/blob/luds/05.md
    fn hmac_sha256(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;
}

/// An argument when calling [crate::sdk::LiquidSdk::connect].
pub struct ConnectRequest {
    pub config: Config,
    pub mnemonic: String,
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
#[derive(Clone, Debug, EnumString, Serialize, Eq, PartialEq)]
pub enum PaymentMethod {
    #[strum(serialize = "lightning")]
    Lightning,
    #[strum(serialize = "bitcoin")]
    BitcoinAddress,
    #[strum(serialize = "liquid")]
    LiquidAddress,
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_receive_payment].
#[derive(Debug, Serialize)]
pub struct PrepareReceiveRequest {
    pub payer_amount_sat: Option<u64>,
    pub payment_method: PaymentMethod,
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_receive_payment].
#[derive(Debug, Serialize)]
pub struct PrepareReceiveResponse {
    pub payment_method: PaymentMethod,
    pub payer_amount_sat: Option<u64>,
    pub fees_sat: u64,
}

/// An argument when calling [crate::sdk::LiquidSdk::receive_payment].
#[derive(Debug, Serialize)]
pub struct ReceivePaymentRequest {
    pub prepare_response: PrepareReceiveResponse,
    /// The description for this payment request.
    pub description: Option<String>,
    /// If set to true, then the hash of the description will be used.
    pub use_description_hash: Option<bool>,
}

/// Returned when calling [crate::sdk::LiquidSdk::receive_payment].
#[derive(Debug, Serialize)]
pub struct ReceivePaymentResponse {
    /// Either a BIP21 URI (Liquid or Bitcoin), a Liquid address
    /// or an invoice, depending on the [PrepareReceivePaymentResponse] parameters
    pub destination: String,
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
    },
    Bolt11 {
        invoice: LNInvoice,
    },
    Bolt12 {
        offer: LNOffer,
        receiver_amount_sat: u64,
    },
}

/// Returned when calling [crate::sdk::LiquidSdk::prepare_send_payment].
#[derive(Debug, Serialize, Clone)]
pub struct PrepareSendResponse {
    pub destination: SendDestination,
    pub fees_sat: u64,
}

/// An argument when calling [crate::sdk::LiquidSdk::send_payment].
#[derive(Debug, Serialize)]
pub struct SendPaymentRequest {
    pub prepare_response: PrepareSendResponse,
}

/// Returned when calling [crate::sdk::LiquidSdk::send_payment].
#[derive(Debug, Serialize)]
pub struct SendPaymentResponse {
    pub payment: Payment,
}

#[derive(Debug, Serialize, Clone)]
pub enum PayAmount {
    /// The amount in satoshi that will be received
    Receiver { amount_sat: u64 },
    /// Indicates that all available funds should be sent
    Drain,
}

/// An argument when calling [crate::sdk::LiquidSdk::prepare_pay_onchain].
#[derive(Debug, Serialize, Clone)]
pub struct PreparePayOnchainRequest {
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
    pub refund_tx_id: Option<String>,
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

/// Returned when calling [crate::sdk::LiquidSdk::get_info].
#[derive(Debug, Serialize)]
pub struct GetInfoResponse {
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
    /// If not set, it defaults to `backup.sql` for mainnet and `backup-testnet.sql` for testnet.
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
    /// Epoch time, in seconds
    pub from_timestamp: Option<i64>,
    /// Epoch time, in seconds
    pub to_timestamp: Option<i64>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub details: Option<ListPaymentDetails>,
}

/// An argument of [ListPaymentsRequest] when calling [crate::sdk::LiquidSdk::list_payments].
#[derive(Debug, Serialize)]
pub enum ListPaymentDetails {
    /// The Liquid BIP21 URI or address of the payment
    Liquid { destination: String },

    /// The Bitcoin address of the payment
    Bitcoin { address: String },
}

/// An argument when calling [crate::sdk::LiquidSdk::get_payment].
#[derive(Debug, Serialize)]
pub enum GetPaymentRequest {
    /// The Lightning payment hash of the payment
    Lightning { payment_hash: String },
}

// A swap enum variant
#[derive(Clone, Debug)]
pub(crate) enum Swap {
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
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

/// A chain swap
///
/// See <https://docs.boltz.exchange/v/api/lifecycle#chain-swaps>
#[derive(Clone, Debug)]
pub(crate) struct ChainSwap {
    pub(crate) id: String,
    pub(crate) direction: Direction,
    /// The Bitcoin claim address is only set for Outgoing Chain Swaps
    pub(crate) claim_address: Option<String>,
    pub(crate) lockup_address: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) preimage: String,
    pub(crate) description: Option<String>,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) claim_fees_sat: u64,
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
}
impl ChainSwap {
    pub(crate) fn get_claim_keypair(&self) -> SdkResult<Keypair> {
        utils::decode_keypair(&self.claim_private_key).map_err(Into::into)
    }

    pub(crate) fn get_refund_keypair(&self) -> SdkResult<Keypair> {
        utils::decode_keypair(&self.refund_private_key).map_err(Into::into)
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

    pub(crate) fn to_refundable(&self, refundable_amount_sat: u64) -> RefundableSwap {
        RefundableSwap {
            swap_address: self.lockup_address.clone(),
            timestamp: self.created_at,
            amount_sat: refundable_amount_sat,
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
}

/// A submarine swap, used for Send
#[derive(Clone, Debug)]
pub(crate) struct SendSwap {
    pub(crate) id: String,
    /// Bolt11 or Bolt12 invoice. This is determined by whether `bolt12_offer` is set or not.
    pub(crate) invoice: String,
    /// The bolt12 offer, if this swap sends to a Bolt12 offer
    pub(crate) bolt12_offer: Option<String>,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) preimage: Option<String>,
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    /// JSON representation of [crate::persist::send::InternalCreateSubmarineResponse]
    pub(crate) create_response_json: String,
    /// Persisted only when the lockup tx is successfully broadcast
    pub(crate) lockup_tx_id: Option<String>,
    /// Persisted as soon as a refund tx is broadcast
    pub(crate) refund_tx_id: Option<String>,
    pub(crate) created_at: u32,
    pub(crate) state: PaymentState,
    pub(crate) refund_private_key: String,
}
impl SendSwap {
    pub(crate) fn get_refund_keypair(&self) -> Result<Keypair, SdkError> {
        utils::decode_keypair(&self.refund_private_key).map_err(Into::into)
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
#[derive(Clone, Debug)]
pub(crate) struct ReceiveSwap {
    pub(crate) id: String,
    pub(crate) preimage: String,
    /// JSON representation of [crate::persist::receive::InternalCreateReverseResponse]
    pub(crate) create_response_json: String,
    pub(crate) claim_private_key: String,
    pub(crate) invoice: String,
    pub(crate) payment_hash: Option<String>,
    pub(crate) description: Option<String>,
    /// The amount of the invoice
    pub(crate) payer_amount_sat: u64,
    pub(crate) receiver_amount_sat: u64,
    pub(crate) claim_fees_sat: u64,
    /// Persisted as soon as a claim tx is broadcast
    pub(crate) claim_tx_id: Option<String>,
    /// Persisted only when the lockup tx is broadcast
    pub(crate) lockup_tx_id: Option<String>,
    /// The address reserved for a magic routing hint payment
    pub(crate) mrh_address: String,
    /// The script pubkey for a magic routing hint payment
    pub(crate) mrh_script_pubkey: String,
    /// Persisted only if a transaction is sent to the `mrh_address`
    pub(crate) mrh_tx_id: Option<String>,
    /// Until the lockup tx is seen in the mempool, it contains the swap creation time.
    /// Afterwards, it shows the lockup tx creation time.
    pub(crate) created_at: u32,
    pub(crate) state: PaymentState,
}
impl ReceiveSwap {
    pub(crate) fn get_claim_keypair(&self) -> Result<Keypair, PaymentError> {
        utils::decode_keypair(&self.claim_private_key).map_err(Into::into)
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
            invoice: self.invoice.clone(),
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
        expected_invoice: &str,
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
}

/// The payment state of an individual payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Hash)]
pub enum PaymentState {
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
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
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

    /// The onchain tx amount.
    ///
    /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
    pub amount_sat: u64,

    /// The onchain fees of this tx
    pub fees_sat: u64,

    pub payment_type: PaymentType,

    /// Onchain tx status
    pub is_confirmed: bool,
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

    pub preimage: Option<String>,
    pub bolt11: Option<String>,
    pub bolt12_offer: Option<String>,
    pub payment_hash: Option<String>,
    pub description: String,

    /// Amount sent by the swap payer
    pub payer_amount_sat: u64,

    /// Amount received by the swap receiver
    pub receiver_amount_sat: u64,

    pub refund_tx_id: Option<String>,
    pub refund_tx_amount_sat: Option<u64>,

    /// Present only for chain swaps.
    /// In case of an outgoing chain swap, it's the Bitcoin address which will receive the funds
    /// In case of an incoming chain swap, it's the Liquid address which will receive the funds
    pub claim_address: Option<String>,

    /// Payment status derived from the swap status
    pub status: PaymentState,
}

/// The specific details of a payment, depending on its type
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PaymentDetails {
    /// Swapping to or from Lightning
    Lightning {
        swap_id: String,

        /// Represents the invoice description
        description: String,

        /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
        preimage: Option<String>,

        /// Represents the Bolt11 invoice associated with a payment
        /// In the case of a Send payment, this is the invoice paid by the swapper
        /// In the case of a Receive payment, this is the invoice paid by the user
        bolt11: Option<String>,

        bolt12_offer: Option<String>,

        /// The payment hash of the invoice
        payment_hash: Option<String>,

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
    },
    /// Swapping to or from the Bitcoin chain
    Bitcoin {
        swap_id: String,

        /// Represents the invoice description
        description: String,

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
    pub(crate) fn from_pending_swap(swap: PaymentSwapData, payment_type: PaymentType) -> Payment {
        let amount_sat = match payment_type {
            PaymentType::Receive => swap.receiver_amount_sat,
            PaymentType::Send => swap.payer_amount_sat,
        };

        Payment {
            destination: swap.bolt11.clone(),
            tx_id: None,
            timestamp: swap.created_at,
            amount_sat,
            fees_sat: swap.payer_amount_sat - swap.receiver_amount_sat,
            payment_type,
            status: swap.status,
            details: PaymentDetails::Lightning {
                swap_id: swap.swap_id,
                preimage: swap.preimage,
                bolt11: swap.bolt11,
                bolt12_offer: swap.bolt12_offer,
                payment_hash: swap.payment_hash,
                description: swap.description,
                refund_tx_id: swap.refund_tx_id,
                refund_tx_amount_sat: swap.refund_tx_amount_sat,
            },
        }
    }

    pub(crate) fn from_tx_data(
        tx: PaymentTxData,
        swap: Option<PaymentSwapData>,
        details: PaymentDetails,
    ) -> Payment {
        Payment {
            tx_id: Some(tx.tx_id),
            // When the swap is present and of type send and receive, we retrieve the destination from the invoice.
            // If it's a chain swap instead, we use the `claim_address` field from the swap data (either pure Bitcoin or Liquid address).
            // Otherwise, we specify the Liquid address (BIP21 or pure), set in `payment_details.address`.
            destination: match &swap {
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Receive,
                    bolt11,
                    ..
                }) => bolt11.clone(),
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Send,
                    bolt11,
                    bolt12_offer,
                    ..
                }) => bolt11.clone().or(bolt12_offer.clone()),
                Some(PaymentSwapData {
                    swap_type: PaymentSwapType::Chain,
                    claim_address,
                    ..
                }) => claim_address.clone(),
                _ => match &details {
                    PaymentDetails::Liquid { destination, .. } => Some(destination.clone()),
                    _ => None,
                },
            },
            timestamp: tx
                .timestamp
                .or(swap.as_ref().map(|s| s.created_at))
                .unwrap_or(utils::now()),
            amount_sat: tx.amount_sat,
            fees_sat: match swap.as_ref() {
                Some(s) => s.payer_amount_sat - s.receiver_amount_sat,
                None => match tx.payment_type {
                    PaymentType::Receive => 0,
                    PaymentType::Send => tx.fees_sat,
                },
            },
            payment_type: tx.payment_type,
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
    /// The [LnUrlPayRequestData] returned by [crate::input_parser::parse]
    pub data: LnUrlPayRequestData,
    /// The amount in millisatoshis for this payment
    pub amount_msat: u64,
    /// An optional comment for this payment
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
            Transaction::Bitcoin(tx) => tx.txid().to_hex(),
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

#[macro_export]
macro_rules! get_invoice_description {
    ($invoice:expr) => {
        match $invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .expect("Expecting valid invoice")
            .description()
        {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        }
    };
}
