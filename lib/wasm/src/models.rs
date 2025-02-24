use crate::error::WasmError;

pub type WasmResult<T> = Result<T, WasmError>;

#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid::prelude::Network)]
pub enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

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
