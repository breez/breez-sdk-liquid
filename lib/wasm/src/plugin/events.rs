use wasm_bindgen::prelude::*;

use crate::model::SdkEvent;

#[wasm_bindgen]
pub struct PluginEventEmitter {
    inner: breez_sdk_liquid::plugin::PluginEventEmitter,
}

impl PluginEventEmitter {
    pub(crate) fn new(inner: breez_sdk_liquid::plugin::PluginEventEmitter) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl PluginEventEmitter {
    #[wasm_bindgen(js_name = "broadcast")]
    pub async fn broadcast(&self, e: SdkEvent) {
        self.inner.broadcast(e.into()).await
    }
}
