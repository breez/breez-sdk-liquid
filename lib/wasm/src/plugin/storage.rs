use wasm_bindgen::prelude::*;

use crate::model::WasmResult;

#[wasm_bindgen]
pub struct PluginStorage {
    storage: breez_sdk_liquid::plugin::PluginStorage,
}

impl PluginStorage {
    pub(crate) fn new(storage: breez_sdk_liquid::plugin::PluginStorage) -> Self {
        Self { storage }
    }

    pub(crate) fn storage(&self) -> breez_sdk_liquid::plugin::PluginStorage {
        self.storage.clone()
    }
}

#[wasm_bindgen]
impl PluginStorage {
    #[wasm_bindgen(js_name = "setItem")]
    pub fn set_item(&self, key: String, value: String) -> WasmResult<()> {
        self.storage.set_item(&key, value).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "getItem")]
    pub fn get_item(&self, key: String) -> WasmResult<Option<String>> {
        self.storage.get_item(&key).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "removeItem")]
    pub fn remove_item(&self, key: String) -> WasmResult<()> {
        self.storage.remove_item(&key).map_err(Into::into)
    }
}
