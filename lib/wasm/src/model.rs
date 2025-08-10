use crate::error::WasmError;

pub type WasmResult<T> = Result<T, WasmError>;

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Network)]
pub enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ExternalInputParser)]
pub struct ExternalInputParser {
    pub provider_id: String,
    pub input_regex: String,
    pub parser_url: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LNInvoice)]
pub struct LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RouteHint)]
pub struct RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RouteHintHop)]
pub struct RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: String,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Amount)]
pub enum Amount {
    Bitcoin {
        amount_msat: u64,
    },
    Currency {
        iso4217_code: String,
        fractional_amount: u64,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnOfferBlindedPath)]
pub struct LnOfferBlindedPath {
    pub blinded_hops: Vec<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LNOffer)]
pub struct LNOffer {
    pub offer: String,
    pub chains: Vec<String>,
    pub min_amount: Option<Amount>,
    pub description: Option<String>,
    pub absolute_expiry: Option<u64>,
    pub issuer: Option<String>,
    pub signing_pubkey: Option<String>,
    pub paths: Vec<LnOfferBlindedPath>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::InputType)]
pub enum InputType {
    BitcoinAddress {
        address: BitcoinAddressData,
    },
    LiquidAddress {
        address: LiquidAddressData,
    },
    Bolt11 {
        invoice: LNInvoice,
    },
    Bolt12Offer {
        offer: LNOffer,
        bip353_address: Option<String>,
    },
    NodeId {
        node_id: String,
    },
    Url {
        url: String,
    },
    LnUrlPay {
        data: LnUrlPayRequestData,
        bip353_address: Option<String>,
    },
    LnUrlWithdraw {
        data: LnUrlWithdrawRequestData,
    },
    LnUrlAuth {
        data: LnUrlAuthRequestData,
    },
    LnUrlError {
        data: LnUrlErrorData,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BitcoinAddressData)]
pub struct BitcoinAddressData {
    pub address: String,
    pub network: breez_sdk_liquid::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LiquidAddressData)]
pub struct LiquidAddressData {
    pub address: String,
    pub network: Network,
    pub asset_id: Option<String>,
    pub amount: Option<f64>,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlPayRequestData)]
pub struct LnUrlPayRequestData {
    pub callback: String,
    pub min_sendable: u64,
    pub max_sendable: u64,
    pub metadata_str: String,
    pub comment_allowed: u16,
    pub domain: String,
    pub allows_nostr: bool,
    pub nostr_pubkey: Option<String>,
    pub ln_address: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SuccessAction)]
pub enum SuccessAction {
    Aes { data: AesSuccessActionData },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SuccessActionProcessed)]
pub enum SuccessActionProcessed {
    Aes { result: AesSuccessActionDataResult },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AesSuccessActionData)]
pub struct AesSuccessActionData {
    pub description: String,
    pub ciphertext: String,
    pub iv: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AesSuccessActionDataResult)]
pub enum AesSuccessActionDataResult {
    Decrypted { data: AesSuccessActionDataDecrypted },
    ErrorStatus { reason: String },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AesSuccessActionDataDecrypted)]
pub struct AesSuccessActionDataDecrypted {
    pub description: String,
    pub plaintext: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::MessageSuccessActionData)]
pub struct MessageSuccessActionData {
    pub message: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::UrlSuccessActionData)]
pub struct UrlSuccessActionData {
    pub description: String,
    pub url: String,
    pub matches_callback_domain: bool,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlPayErrorData)]
pub struct LnUrlPayErrorData {
    pub payment_hash: String,
    pub reason: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlWithdrawRequestData)]
pub struct LnUrlWithdrawRequestData {
    pub callback: String,
    pub k1: String,
    pub default_description: String,
    pub min_withdrawable: u64,
    pub max_withdrawable: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlCallbackStatus)]
pub enum LnUrlCallbackStatus {
    Ok,
    ErrorStatus {
        #[serde(flatten)]
        data: LnUrlErrorData,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlAuthRequestData)]
pub struct LnUrlAuthRequestData {
    pub k1: String,
    pub action: Option<String>,
    pub domain: String,
    pub url: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlErrorData)]
pub struct LnUrlErrorData {
    pub reason: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlWithdrawRequest)]
pub struct LnUrlWithdrawRequest {
    pub data: LnUrlWithdrawRequestData,
    pub amount_msat: u64,
    pub description: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlWithdrawSuccessData)]
pub struct LnUrlWithdrawSuccessData {
    pub invoice: LNInvoice,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlWithdrawResult)]
pub enum LnUrlWithdrawResult {
    Ok { data: LnUrlWithdrawSuccessData },
    Timeout { data: LnUrlWithdrawSuccessData },
    ErrorStatus { data: LnUrlErrorData },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Rate)]
pub struct Rate {
    pub coin: String,
    pub value: f64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::FiatCurrency)]
pub struct FiatCurrency {
    pub id: String,
    pub info: CurrencyInfo,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::CurrencyInfo)]
pub struct CurrencyInfo {
    pub name: String,
    pub fraction_size: u32,
    pub spacing: Option<u32>,
    pub symbol: Option<Symbol>,
    pub uniq_symbol: Option<Symbol>,
    pub localized_name: Vec<LocalizedName>,
    pub locale_overrides: Vec<LocaleOverrides>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LocaleOverrides)]
pub struct LocaleOverrides {
    pub locale: String,
    pub spacing: Option<u32>,
    pub symbol: Symbol,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LocalizedName)]
pub struct LocalizedName {
    pub locale: String,
    pub name: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Symbol)]
pub struct Symbol {
    pub grapheme: Option<String>,
    pub template: Option<String>,
    pub rtl: Option<bool>,
    pub position: Option<u32>,
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BlockchainExplorer)]
pub enum BlockchainExplorer {
    Esplora { url: String, use_waterfalls: bool },
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Config)]
pub struct Config {
    pub liquid_explorer: BlockchainExplorer,
    pub bitcoin_explorer: BlockchainExplorer,
    pub working_dir: String,
    pub network: LiquidNetwork,
    pub payment_timeout_sec: u64,
    pub sync_service_url: Option<String>,
    pub zero_conf_max_amount_sat: Option<u64>,
    pub breez_api_key: Option<String>,
    pub external_input_parsers: Option<Vec<ExternalInputParser>>,
    pub use_default_external_input_parsers: bool,
    pub onchain_fee_rate_leeway_sat: Option<u64>,
    pub asset_metadata: Option<Vec<AssetMetadata>>,
    pub sideswap_api_key: Option<String>,
    pub use_magic_routing_hints: bool,
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LiquidNetwork)]
pub enum LiquidNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SdkEvent)]
pub enum SdkEvent {
    PaymentFailed { details: Payment },
    PaymentPending { details: Payment },
    PaymentRefundable { details: Payment },
    PaymentRefunded { details: Payment },
    PaymentRefundPending { details: Payment },
    PaymentSucceeded { details: Payment },
    PaymentWaitingConfirmation { details: Payment },
    PaymentWaitingFeeAcceptance { details: Payment },
    Synced,
    DataSynced { did_pull_new_records: bool },
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ConnectRequest)]
pub struct ConnectRequest {
    pub config: Config,
    pub mnemonic: Option<String>,
    pub passphrase: Option<String>,
    pub seed: Option<Vec<u8>>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ConnectWithSignerRequest)]
pub struct ConnectWithSignerRequest {
    pub config: Config,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PaymentMethod)]
pub enum PaymentMethod {
    Lightning,
    Bolt11Invoice,
    Bolt12Offer,
    BitcoinAddress,
    LiquidAddress,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ReceiveAmount)]
pub enum ReceiveAmount {
    Bitcoin {
        payer_amount_sat: u64,
    },
    Asset {
        asset_id: String,
        payer_amount: Option<f64>,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareReceiveRequest)]
pub struct PrepareReceiveRequest {
    pub payment_method: PaymentMethod,
    pub amount: Option<ReceiveAmount>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareReceiveResponse)]
pub struct PrepareReceiveResponse {
    pub payment_method: PaymentMethod,
    pub fees_sat: u64,
    pub amount: Option<ReceiveAmount>,
    pub min_payer_amount_sat: Option<u64>,
    pub max_payer_amount_sat: Option<u64>,
    pub swapper_feerate: Option<f64>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ReceivePaymentRequest)]
pub struct ReceivePaymentRequest {
    pub prepare_response: PrepareReceiveResponse,
    pub description: Option<String>,
    pub use_description_hash: Option<bool>,
    pub payer_note: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ReceivePaymentResponse)]
pub struct ReceivePaymentResponse {
    pub destination: String,
    pub liquid_expiration_blockheight: Option<u32>,
    pub bitcoin_expiration_blockheight: Option<u32>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::CreateBolt12InvoiceRequest)]
pub struct CreateBolt12InvoiceRequest {
    pub offer: String,
    pub invoice_request: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::CreateBolt12InvoiceResponse)]
pub struct CreateBolt12InvoiceResponse {
    pub invoice: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Limits)]
pub struct Limits {
    pub min_sat: u64,
    pub max_sat: u64,
    pub max_zero_conf_sat: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LightningPaymentLimitsResponse)]
pub struct LightningPaymentLimitsResponse {
    pub send: Limits,
    pub receive: Limits,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::OnchainPaymentLimitsResponse)]
pub struct OnchainPaymentLimitsResponse {
    pub send: Limits,
    pub receive: Limits,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareSendRequest)]
pub struct PrepareSendRequest {
    pub destination: String,
    pub amount: Option<PayAmount>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SendDestination)]
pub enum SendDestination {
    LiquidAddress {
        address_data: LiquidAddressData,
        bip353_address: Option<String>,
    },
    Bolt11 {
        invoice: LNInvoice,
        bip353_address: Option<String>,
    },
    Bolt12 {
        offer: LNOffer,
        receiver_amount_sat: u64,
        bip353_address: Option<String>,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareSendResponse)]
pub struct PrepareSendResponse {
    pub destination: SendDestination,
    pub amount: Option<PayAmount>,
    pub fees_sat: Option<u64>,
    pub estimated_asset_fees: Option<f64>,
    pub exchange_amount_sat: Option<u64>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SendPaymentRequest)]
pub struct SendPaymentRequest {
    pub prepare_response: PrepareSendResponse,
    pub use_asset_fees: Option<bool>,
    pub payer_note: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SendPaymentResponse)]
pub struct SendPaymentResponse {
    pub payment: Payment,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PayAmount)]
pub enum PayAmount {
    Bitcoin {
        receiver_amount_sat: u64,
    },
    Asset {
        to_asset: String,
        receiver_amount: f64,
        estimate_asset_fees: Option<bool>,
        from_asset: Option<String>,
    },
    Drain,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PreparePayOnchainRequest)]
pub struct PreparePayOnchainRequest {
    pub amount: PayAmount,
    pub fee_rate_sat_per_vbyte: Option<u32>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PreparePayOnchainResponse)]
pub struct PreparePayOnchainResponse {
    pub receiver_amount_sat: u64,
    pub claim_fees_sat: u64,
    pub total_fees_sat: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PayOnchainRequest)]
pub struct PayOnchainRequest {
    pub address: String,
    pub prepare_response: PreparePayOnchainResponse,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareRefundRequest)]
pub struct PrepareRefundRequest {
    pub swap_address: String,
    pub refund_address: String,
    pub fee_rate_sat_per_vbyte: u32,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareRefundResponse)]
pub struct PrepareRefundResponse {
    pub tx_vsize: u32,
    pub tx_fee_sat: u64,
    pub last_refund_tx_id: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RefundRequest)]
pub struct RefundRequest {
    pub swap_address: String,
    pub refund_address: String,
    pub fee_rate_sat_per_vbyte: u32,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RefundResponse)]
pub struct RefundResponse {
    pub refund_tx_id: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AssetBalance)]
pub struct AssetBalance {
    pub asset_id: String,
    pub balance_sat: u64,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub balance: Option<f64>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BlockchainInfo)]
pub struct BlockchainInfo {
    pub liquid_tip: u32,
    pub bitcoin_tip: u32,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::WalletInfo)]
pub struct WalletInfo {
    pub balance_sat: u64,
    pub pending_send_sat: u64,
    pub pending_receive_sat: u64,
    pub fingerprint: String,
    pub pubkey: String,
    pub asset_balances: Vec<AssetBalance>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::GetInfoResponse)]
pub struct GetInfoResponse {
    pub wallet_info: WalletInfo,
    pub blockchain_info: BlockchainInfo,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SignMessageRequest)]
pub struct SignMessageRequest {
    pub message: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::SignMessageResponse)]
pub struct SignMessageResponse {
    pub signature: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::CheckMessageRequest)]
pub struct CheckMessageRequest {
    pub message: String,
    pub pubkey: String,
    pub signature: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::CheckMessageResponse)]
pub struct CheckMessageResponse {
    pub is_valid: bool,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BackupRequest)]
pub struct BackupRequest {
    pub backup_path: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RestoreRequest)]
pub struct RestoreRequest {
    pub backup_path: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ListPaymentsRequest)]
pub struct ListPaymentsRequest {
    pub filters: Option<Vec<PaymentType>>,
    pub states: Option<Vec<PaymentState>>,
    pub from_timestamp: Option<i64>,
    pub to_timestamp: Option<i64>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub details: Option<ListPaymentDetails>,
    pub sort_ascending: Option<bool>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::ListPaymentDetails)]
pub enum ListPaymentDetails {
    Liquid {
        asset_id: Option<String>,
        destination: Option<String>,
    },
    Bitcoin {
        address: Option<String>,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::GetPaymentRequest)]
pub enum GetPaymentRequest {
    PaymentHash { payment_hash: String },
    SwapId { swap_id: String },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RefundableSwap)]
pub struct RefundableSwap {
    pub swap_address: String,
    pub timestamp: u32,
    pub amount_sat: u64,
    pub last_refund_tx_id: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PaymentState)]
pub enum PaymentState {
    Created = 0,
    Pending = 1,
    Complete = 2,
    Failed = 3,
    TimedOut = 4,
    Refundable = 5,
    RefundPending = 6,
    WaitingFeeAcceptance = 7,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PaymentType)]
pub enum PaymentType {
    Receive = 0,
    Send = 1,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PaymentStatus)]
pub enum PaymentStatus {
    Pending = 0,
    Complete = 1,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LnUrlInfo)]
pub struct LnUrlInfo {
    pub ln_address: Option<String>,
    pub lnurl_pay_comment: Option<String>,
    pub lnurl_pay_domain: Option<String>,
    pub lnurl_pay_metadata: Option<String>,
    pub lnurl_pay_success_action: Option<SuccessActionProcessed>,
    pub lnurl_pay_unprocessed_success_action: Option<SuccessAction>,
    pub lnurl_withdraw_endpoint: Option<String>,
}

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AssetMetadata)]
pub struct AssetMetadata {
    pub asset_id: String,
    pub name: String,
    pub ticker: String,
    pub precision: u8,
    pub fiat_id: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AssetInfo)]
pub struct AssetInfo {
    pub name: String,
    pub ticker: String,
    pub amount: f64,
    pub fees: Option<f64>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PaymentDetails)]
#[allow(clippy::large_enum_variant)]
pub enum PaymentDetails {
    Lightning {
        swap_id: String,
        description: String,
        liquid_expiration_blockheight: u32,
        preimage: Option<String>,
        invoice: Option<String>,
        bolt12_offer: Option<String>,
        payment_hash: Option<String>,
        destination_pubkey: Option<String>,
        lnurl_info: Option<LnUrlInfo>,
        bip353_address: Option<String>,
        payer_note: Option<String>,
        claim_tx_id: Option<String>,
        refund_tx_id: Option<String>,
        refund_tx_amount_sat: Option<u64>,
    },
    Liquid {
        destination: String,
        description: String,
        asset_id: String,
        asset_info: Option<AssetInfo>,
        lnurl_info: Option<LnUrlInfo>,
        bip353_address: Option<String>,
        payer_note: Option<String>,
    },
    Bitcoin {
        swap_id: String,
        bitcoin_address: String,
        description: String,
        auto_accepted_fees: bool,
        liquid_expiration_blockheight: u32,
        bitcoin_expiration_blockheight: u32,
        lockup_tx_id: Option<String>,
        claim_tx_id: Option<String>,
        refund_tx_id: Option<String>,
        refund_tx_amount_sat: Option<u64>,
    },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Payment)]
pub struct Payment {
    pub destination: Option<String>,
    pub tx_id: Option<String>,
    pub unblinding_data: Option<String>,
    pub timestamp: u32,
    pub amount_sat: u64,
    pub fees_sat: u64,
    pub swapper_fees_sat: Option<u64>,
    pub payment_type: PaymentType,
    pub status: PaymentState,
    pub details: PaymentDetails,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::RecommendedFees)]
pub struct RecommendedFees {
    pub fastest_fee: u64,
    pub half_hour_fee: u64,
    pub hour_fee: u64,
    pub economy_fee: u64,
    pub minimum_fee: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BuyBitcoinProvider)]
pub enum BuyBitcoinProvider {
    Moonpay,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareBuyBitcoinRequest)]
pub struct PrepareBuyBitcoinRequest {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareBuyBitcoinResponse)]
pub struct PrepareBuyBitcoinResponse {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
    pub fees_sat: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::BuyBitcoinRequest)]
pub struct BuyBitcoinRequest {
    pub prepare_response: PrepareBuyBitcoinResponse,
    pub redirect_url: Option<String>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::LogEntry)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareLnUrlPayRequest)]
pub struct PrepareLnUrlPayRequest {
    pub data: LnUrlPayRequestData,
    pub amount: PayAmount,
    pub bip353_address: Option<String>,
    pub comment: Option<String>,
    pub validate_success_action_url: Option<bool>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::PrepareLnUrlPayResponse)]
pub struct PrepareLnUrlPayResponse {
    pub destination: SendDestination,
    pub fees_sat: u64,
    pub data: LnUrlPayRequestData,
    pub amount: PayAmount,
    pub comment: Option<String>,
    pub success_action: Option<SuccessAction>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::model::LnUrlPayRequest)]
pub struct LnUrlPayRequest {
    pub prepare_response: PrepareLnUrlPayResponse,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::model::LnUrlPayResult)]
#[allow(clippy::large_enum_variant)]
pub enum LnUrlPayResult {
    EndpointSuccess { data: LnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::model::LnUrlPaySuccessData)]
pub struct LnUrlPaySuccessData {
    pub payment: Payment,
    pub success_action: Option<SuccessActionProcessed>,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::FetchPaymentProposedFeesRequest)]
pub struct FetchPaymentProposedFeesRequest {
    pub swap_id: String,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::FetchPaymentProposedFeesResponse)]
pub struct FetchPaymentProposedFeesResponse {
    pub swap_id: String,
    pub fees_sat: u64,
    pub payer_amount_sat: u64,
    pub receiver_amount_sat: u64,
}

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::AcceptPaymentProposedFeesRequest)]
pub struct AcceptPaymentProposedFeesRequest {
    pub response: FetchPaymentProposedFeesResponse,
}
