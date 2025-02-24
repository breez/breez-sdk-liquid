mod error;
pub mod models;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LiquidSdk {
    _inner: breez_sdk_liquid::sdk::LiquidSdk,
}
