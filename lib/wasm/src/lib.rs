mod error;
pub mod models;

use std::sync::Arc;

use breez_sdk_liquid::sdk::LiquidSdk;
use models::{Config, ConnectRequest, LNInvoice, LiquidNetwork, WasmResult};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

#[wasm_bindgen(js_name = "connect")]
pub async fn connect(req: ConnectRequest) -> WasmResult<BindingLiquidSdk> {
    let sdk = LiquidSdk::connect(req.into()).await?;
    Ok(BindingLiquidSdk { sdk })
}

#[wasm_bindgen(js_name = "defaultConfig")]
pub fn default_config(network: LiquidNetwork, breez_api_key: Option<String>) -> WasmResult<Config> {
    let config = LiquidSdk::default_config(network.into(), breez_api_key)?;
    Ok(config.into())
}

#[wasm_bindgen(js_name = "parseInvoice")]
pub fn parse_invoice(input: String) -> WasmResult<LNInvoice> {
    let invoice = LiquidSdk::parse_invoice(&input)?;
    Ok(invoice.into())
}
