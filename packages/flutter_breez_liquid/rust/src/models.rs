use flutter_rust_bridge::frb;

// === FRB mirroring
//
// This section contains frb "mirroring" structs and enums.
// These are needed by the flutter bridge in order to use structs defined in an external crate.
// See <https://cjycode.com/flutter_rust_bridge/v1/feature/lang_external.html#types-in-other-crates>

// === sdk_common structs and enums

pub use sdk_common::prelude::{
    AesSuccessActionData, AesSuccessActionDataDecrypted, AesSuccessActionDataResult, Amount,
    BitcoinAddressData, CurrencyInfo, ExternalInputParser, FiatCurrency, InputType, LNInvoice,
    LNOffer, LiquidAddressData, LnOfferBlindedPath, LnUrlAuthRequestData, LnUrlErrorData,
    LnUrlPayErrorData, LnUrlPayRequestData, LnUrlWithdrawRequest, LnUrlWithdrawRequestData,
    LocaleOverrides, LocalizedName, MessageSuccessActionData, Network, Rate, RouteHint,
    RouteHintHop, SuccessAction, SuccessActionProcessed, Symbol, UrlSuccessActionData,
};

#[frb(mirror(Network))]
pub enum _Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[frb(mirror(ExternalInputParser))]
pub struct _ExternalInputParser {
    pub provider_id: String,
    pub input_regex: String,
    pub parser_url: String,
}

#[frb(mirror(LNInvoice))]
pub struct _LNInvoice {
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

#[frb(mirror(RouteHint))]
pub struct _RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[frb(mirror(RouteHintHop))]
pub struct _RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: String,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}

#[frb(mirror(Amount))]
pub enum _Amount {
    Bitcoin {
        amount_msat: u64,
    },
    Currency {
        iso4217_code: String,
        fractional_amount: u64,
    },
}

#[frb(mirror(LnOfferBlindedPath))]
pub struct _LnOfferBlindedPath {
    pub blinded_hops: Vec<String>,
}

#[frb(mirror(LNOffer))]
pub struct _LNOffer {
    pub offer: String,
    pub chains: Vec<String>,
    pub min_amount: Option<Amount>,
    pub description: Option<String>,
    pub absolute_expiry: Option<u64>,
    pub issuer: Option<String>,
    pub signing_pubkey: Option<String>,
    pub paths: Vec<LnOfferBlindedPath>,
}

#[frb(mirror(InputType))]
pub enum _InputType {
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

#[frb(mirror(BitcoinAddressData))]
pub struct _BitcoinAddressData {
    pub address: String,
    pub network: sdk_common::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[frb(mirror(LiquidAddressData))]
pub struct _LiquidAddressData {
    pub address: String,
    pub network: Network,
    pub asset_id: Option<String>,
    pub amount: Option<f64>,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[frb(mirror(LnUrlPayRequestData))]
pub struct _LnUrlPayRequestData {
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

#[frb(mirror(SuccessAction))]
pub enum _SuccessAction {
    Aes { data: AesSuccessActionData },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[frb(mirror(SuccessActionProcessed))]
pub enum _SuccessActionProcessed {
    Aes { result: AesSuccessActionDataResult },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[frb(mirror(AesSuccessActionData))]
pub struct _AesSuccessActionData {
    pub description: String,
    pub ciphertext: String,
    pub iv: String,
}

#[frb(mirror(AesSuccessActionDataResult))]
pub enum _AesSuccessActionDataResult {
    Decrypted { data: AesSuccessActionDataDecrypted },
    ErrorStatus { reason: String },
}

#[frb(mirror(AesSuccessActionDataDecrypted))]
pub struct _AesSuccessActionDataDecrypted {
    pub description: String,
    pub plaintext: String,
}

#[frb(mirror(MessageSuccessActionData))]
pub struct _MessageSuccessActionData {
    pub message: String,
}

#[frb(mirror(UrlSuccessActionData))]
pub struct _UrlSuccessActionData {
    pub description: String,
    pub url: String,
    pub matches_callback_domain: bool,
}

#[frb(mirror(LnUrlPayErrorData))]
pub struct _LnUrlPayErrorData {
    pub payment_hash: String,
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawRequestData))]
pub struct _LnUrlWithdrawRequestData {
    pub callback: String,
    pub k1: String,
    pub default_description: String,
    pub min_withdrawable: u64,
    pub max_withdrawable: u64,
}

#[frb(mirror(LnUrlAuthRequestData))]
pub struct _LnUrlAuthRequestData {
    pub k1: String,
    pub action: Option<String>,
    pub domain: String,
    pub url: String,
}

#[frb(mirror(LnUrlErrorData))]
pub struct _LnUrlErrorData {
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawRequest))]
pub struct _LnUrlWithdrawRequest {
    pub data: LnUrlWithdrawRequestData,
    pub amount_msat: u64,
    pub description: Option<String>,
}

#[frb(mirror(Rate))]
pub struct _Rate {
    pub coin: String,
    pub value: f64,
}

#[frb(mirror(FiatCurrency))]
pub struct _FiatCurrency {
    pub id: String,
    pub info: CurrencyInfo,
}

#[frb(mirror(CurrencyInfo))]
pub struct _CurrencyInfo {
    pub name: String,
    pub fraction_size: u32,
    pub spacing: Option<u32>,
    pub symbol: Option<Symbol>,
    pub uniq_symbol: Option<Symbol>,
    pub localized_name: Vec<LocalizedName>,
    pub locale_overrides: Vec<LocaleOverrides>,
}

#[frb(mirror(LocaleOverrides))]
pub struct _LocaleOverrides {
    pub locale: String,
    pub spacing: Option<u32>,
    pub symbol: Symbol,
}

#[frb(mirror(LocalizedName))]
pub struct _LocalizedName {
    pub locale: String,
    pub name: String,
}

#[frb(mirror(Symbol))]
pub struct _Symbol {
    pub grapheme: Option<String>,
    pub template: Option<String>,
    pub rtl: Option<bool>,
    pub position: Option<u32>,
}

// === breez_sdk_liquid structs and enums

pub use breez_sdk_liquid::{
    model::{
        AcceptPaymentProposedFeesRequest, AssetBalance, AssetInfo, AssetMetadata, BackupRequest,
        BlockchainExplorer, BlockchainInfo, BuyBitcoinProvider, BuyBitcoinRequest,
        CheckMessageRequest, CheckMessageResponse, Config, ConnectRequest,
        CreateBolt12InvoiceRequest, CreateBolt12InvoiceResponse, FetchPaymentProposedFeesRequest,
        FetchPaymentProposedFeesResponse, GetInfoResponse, GetPaymentRequest,
        LightningPaymentLimitsResponse, Limits, LiquidNetwork, ListPaymentDetails,
        ListPaymentsRequest, LnUrlInfo, LnUrlPayRequest, LnUrlPayResult, LnUrlPaySuccessData,
        OnchainPaymentLimitsResponse, PayAmount, PayOnchainRequest, Payment, PaymentDetails,
        PaymentMethod, PaymentState, PaymentType, PrepareBuyBitcoinRequest,
        PrepareBuyBitcoinResponse, PrepareLnUrlPayRequest, PrepareLnUrlPayResponse,
        PreparePayOnchainRequest, PreparePayOnchainResponse, PrepareReceiveRequest,
        PrepareReceiveResponse, PrepareRefundRequest, PrepareRefundResponse, PrepareSendRequest,
        PrepareSendResponse, ReceiveAmount, ReceivePaymentRequest, ReceivePaymentResponse,
        RecommendedFees, RefundRequest, RefundResponse, RefundableSwap, RestoreRequest, SdkEvent,
        SendDestination, SendPaymentRequest, SendPaymentResponse, SignMessageRequest,
        SignMessageResponse, WalletInfo,
    },
    sdk::LiquidSdk,
};

#[frb(mirror(AcceptPaymentProposedFeesRequest))]
pub struct _AcceptPaymentProposedFeesRequest {
    pub response: FetchPaymentProposedFeesResponse,
}

#[frb(mirror(BackupRequest))]
pub struct _BackupRequest {
    pub backup_path: Option<String>,
}

#[frb(mirror(BuyBitcoinRequest))]
pub struct _BuyBitcoinRequest {
    pub prepare_response: PrepareBuyBitcoinResponse,
    pub redirect_url: Option<String>,
}

#[frb(mirror(CheckMessageRequest))]
pub struct _CheckMessageRequest {
    pub message: String,
    pub pubkey: String,
    pub signature: String,
}

#[frb(mirror(CheckMessageResponse))]
pub struct _CheckMessageResponse {
    pub is_valid: bool,
}

#[frb(mirror(Config))]
pub struct _Config {
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
    pub onchain_sync_period_sec: u32,
    pub onchain_sync_request_timeout_sec: u32,
}

#[frb(mirror(ConnectRequest))]
pub struct _ConnectRequest {
    pub config: Config,
    pub mnemonic: Option<String>,
    pub passphrase: Option<String>,
    pub seed: Option<Vec<u8>>,
}

#[frb(mirror(CreateBolt12InvoiceRequest))]
pub struct _CreateBolt12InvoiceRequest {
    pub offer: String,
    pub invoice_request: String,
}

#[frb(mirror(CreateBolt12InvoiceResponse))]
pub struct _CreateBolt12InvoiceResponse {
    pub invoice: String,
}

#[frb(mirror(FetchPaymentProposedFeesRequest))]
pub struct _FetchPaymentProposedFeesRequest {
    pub swap_id: String,
}

#[frb(mirror(FetchPaymentProposedFeesResponse))]
pub struct _FetchPaymentProposedFeesResponse {
    pub swap_id: String,
    pub fees_sat: u64,
    pub payer_amount_sat: u64,
    pub receiver_amount_sat: u64,
}

#[frb(mirror(GetInfoResponse))]
pub struct _GetInfoResponse {
    pub wallet_info: WalletInfo,
    pub blockchain_info: BlockchainInfo,
}

#[frb(mirror(GetPaymentRequest))]
pub enum _GetPaymentRequest {
    PaymentHash { payment_hash: String },
    SwapId { swap_id: String },
}

#[frb(mirror(LightningPaymentLimitsResponse))]
pub struct _LightningPaymentLimitsResponse {
    pub send: Limits,
    pub receive: Limits,
}

#[frb(mirror(LiquidNetwork))]
pub enum _LiquidNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

#[frb(mirror(ListPaymentsRequest))]
pub struct _ListPaymentsRequest {
    pub filters: Option<Vec<PaymentType>>,
    pub states: Option<Vec<PaymentState>>,
    pub from_timestamp: Option<i64>,
    pub to_timestamp: Option<i64>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub details: Option<ListPaymentDetails>,
    pub sort_ascending: Option<bool>,
}

#[frb(mirror(LnUrlPayRequest))]
pub struct _LnUrlPayRequest {
    pub prepare_response: PrepareLnUrlPayResponse,
}

#[frb(mirror(LnUrlPayResult))]
pub enum _LnUrlPayResult {
    EndpointSuccess { data: LnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[frb(mirror(OnchainPaymentLimitsResponse))]
pub struct _OnchainPaymentLimitsResponse {
    pub send: Limits,
    pub receive: Limits,
}

#[frb(mirror(PayOnchainRequest))]
pub struct _PayOnchainRequest {
    pub address: String,
    pub prepare_response: PreparePayOnchainResponse,
}

#[frb(mirror(Payment))]
pub struct _Payment {
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

#[frb(mirror(PrepareBuyBitcoinRequest))]
pub struct _PrepareBuyBitcoinRequest {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
}

#[frb(mirror(PrepareBuyBitcoinResponse))]
pub struct _PrepareBuyBitcoinResponse {
    pub provider: BuyBitcoinProvider,
    pub amount_sat: u64,
    pub fees_sat: u64,
}

#[frb(mirror(PrepareLnUrlPayRequest))]
pub struct _PrepareLnUrlPayRequest {
    pub data: LnUrlPayRequestData,
    pub amount: PayAmount,
    pub bip353_address: Option<String>,
    pub comment: Option<String>,
    pub validate_success_action_url: Option<bool>,
}

#[frb(mirror(PrepareLnUrlPayResponse))]
pub struct _PrepareLnUrlPayResponse {
    pub destination: SendDestination,
    pub fees_sat: u64,
    pub data: LnUrlPayRequestData,
    pub amount: PayAmount,
    pub comment: Option<String>,
    pub success_action: Option<SuccessAction>,
}

#[frb(mirror(PreparePayOnchainRequest))]
pub struct _PreparePayOnchainRequest {
    pub amount: PayAmount,
    pub fee_rate_sat_per_vbyte: Option<u32>,
}

#[frb(mirror(PreparePayOnchainResponse))]
pub struct _PreparePayOnchainResponse {
    pub receiver_amount_sat: u64,
    pub claim_fees_sat: u64,
    pub total_fees_sat: u64,
}

#[frb(mirror(PrepareReceiveRequest))]
pub struct _PrepareReceiveRequest {
    pub payment_method: PaymentMethod,
    pub amount: Option<ReceiveAmount>,
}

#[frb(mirror(PrepareReceiveResponse))]
pub struct _PrepareReceiveResponse {
    pub payment_method: PaymentMethod,
    pub fees_sat: u64,
    pub amount: Option<ReceiveAmount>,
    pub min_payer_amount_sat: Option<u64>,
    pub max_payer_amount_sat: Option<u64>,
    pub swapper_feerate: Option<f64>,
}

#[frb(mirror(PrepareRefundRequest))]
pub struct _PrepareRefundRequest {
    pub swap_address: String,
    pub refund_address: String,
    pub fee_rate_sat_per_vbyte: u32,
}

#[frb(mirror(PrepareRefundResponse))]
pub struct _PrepareRefundResponse {
    pub tx_vsize: u32,
    pub tx_fee_sat: u64,
    pub last_refund_tx_id: Option<String>,
}

#[frb(mirror(PrepareSendRequest))]
pub struct _PrepareSendRequest {
    pub destination: String,
    pub amount: Option<PayAmount>,
}

#[frb(mirror(PrepareSendResponse))]
pub struct _PrepareSendResponse {
    pub destination: SendDestination,
    pub amount: Option<PayAmount>,
    pub fees_sat: Option<u64>,
    pub estimated_asset_fees: Option<f64>,
    pub exchange_amount_sat: Option<u64>,
}

#[frb(mirror(ReceivePaymentRequest))]
pub struct _ReceivePaymentRequest {
    pub prepare_response: PrepareReceiveResponse,
    pub description: Option<String>,
    pub use_description_hash: Option<bool>,
    pub payer_note: Option<String>,
}

#[frb(mirror(ReceivePaymentResponse))]
pub struct _ReceivePaymentResponse {
    pub destination: String,
    pub liquid_expiration_blockheight: Option<u32>,
    pub bitcoin_expiration_blockheight: Option<u32>,
}

#[frb(mirror(RecommendedFees))]
pub struct _RecommendedFees {
    pub fastest_fee: u64,
    pub half_hour_fee: u64,
    pub hour_fee: u64,
    pub economy_fee: u64,
    pub minimum_fee: u64,
}

#[frb(mirror(RefundRequest))]
pub struct _RefundRequest {
    pub swap_address: String,
    pub refund_address: String,
    pub fee_rate_sat_per_vbyte: u32,
}

#[frb(mirror(RefundResponse))]
pub struct _RefundResponse {
    pub refund_tx_id: String,
}

#[frb(mirror(RefundableSwap))]
pub struct _RefundableSwap {
    pub swap_address: String,
    pub timestamp: u32,
    pub amount_sat: u64,
    pub last_refund_tx_id: Option<String>,
}

#[frb(mirror(RestoreRequest))]
pub struct _RestoreRequest {
    pub backup_path: Option<String>,
}

#[frb(mirror(SendPaymentRequest))]
pub struct _SendPaymentRequest {
    pub prepare_response: PrepareSendResponse,
    pub use_asset_fees: Option<bool>,
    pub payer_note: Option<String>,
}

#[frb(mirror(SendPaymentResponse))]
pub struct _SendPaymentResponse {
    pub payment: Payment,
}

#[frb(mirror(SignMessageRequest))]
pub struct _SignMessageRequest {
    pub message: String,
}

#[frb(mirror(SignMessageResponse))]
pub struct _SignMessageResponse {
    pub signature: String,
}

#[frb(mirror(AssetMetadata))]
pub struct _AssetMetadata {
    pub asset_id: String,
    pub name: String,
    pub ticker: String,
    pub precision: u8,
    pub fiat_id: Option<String>,
}

#[frb(mirror(BlockchainExplorer))]
pub enum _BlockchainExplorer {
    Electrum { url: String },
    Esplora { url: String, use_waterfalls: bool },
}

#[frb(mirror(BlockchainInfo))]
pub struct _BlockchainInfo {
    pub liquid_tip: u32,
    pub bitcoin_tip: u32,
}

#[frb(mirror(BuyBitcoinProvider))]
pub enum _BuyBitcoinProvider {
    Moonpay,
}

#[frb(mirror(Limits))]
pub struct _Limits {
    pub min_sat: u64,
    pub max_sat: u64,
    pub max_zero_conf_sat: u64,
}

#[frb(mirror(ListPaymentDetails))]
pub enum _ListPaymentDetails {
    Liquid {
        asset_id: Option<String>,
        destination: Option<String>,
    },
    Bitcoin {
        address: Option<String>,
    },
}

#[frb(mirror(PayAmount))]
pub enum _PayAmount {
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

#[frb(mirror(PaymentDetails))]
pub enum _PaymentDetails {
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

#[frb(mirror(PaymentMethod))]
pub enum _PaymentMethod {
    Lightning,
    Bolt11Invoice,
    Bolt12Offer,
    BitcoinAddress,
    LiquidAddress,
}

#[frb(mirror(PaymentState))]
pub enum _PaymentState {
    Created,
    Pending,
    Complete,
    Failed,
    TimedOut,
    Refundable,
    RefundPending,
    WaitingFeeAcceptance,
}

#[frb(mirror(PaymentType))]
pub enum _PaymentType {
    Receive,
    Send,
}

#[frb(mirror(ReceiveAmount))]
pub enum _ReceiveAmount {
    Bitcoin {
        payer_amount_sat: u64,
    },
    Asset {
        asset_id: String,
        payer_amount: Option<f64>,
    },
}

#[frb(mirror(SendDestination))]
pub enum _SendDestination {
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

#[frb(mirror(LnUrlPaySuccessData))]
pub struct _LnUrlPaySuccessData {
    pub payment: Payment,
    pub success_action: Option<SuccessActionProcessed>,
}

#[frb(mirror(WalletInfo))]
pub struct _WalletInfo {
    pub balance_sat: u64,
    pub pending_send_sat: u64,
    pub pending_receive_sat: u64,
    pub fingerprint: String,
    pub pubkey: String,
    pub asset_balances: Vec<AssetBalance>,
}

#[frb(mirror(AssetBalance))]
pub struct _AssetBalance {
    pub asset_id: String,
    pub balance_sat: u64,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub balance: Option<f64>,
}

#[frb(mirror(AssetInfo))]
pub struct _AssetInfo {
    pub name: String,
    pub ticker: String,
    pub amount: f64,
    pub fees: Option<f64>,
}

#[frb(mirror(LnUrlInfo))]
pub struct _LnUrlInfo {
    pub ln_address: Option<String>,
    pub lnurl_pay_comment: Option<String>,
    pub lnurl_pay_domain: Option<String>,
    pub lnurl_pay_metadata: Option<String>,
    pub lnurl_pay_success_action: Option<SuccessActionProcessed>,
    pub lnurl_pay_unprocessed_success_action: Option<SuccessAction>,
    pub lnurl_withdraw_endpoint: Option<String>,
}
