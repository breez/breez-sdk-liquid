use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LiquidSdk {
    inner: breez_sdk_liquid::sdk::LiquidSdk,
}
