use breez_sdk_liquid::plugin::PluginStorageError;
use wasm_bindgen::prelude::*;

use crate::{error::WasmError, model::WasmResult};

#[wasm_bindgen]
pub struct PluginStorage {
    inner: breez_sdk_liquid::plugin::PluginStorage,
}

impl PluginStorage {
    pub(crate) fn new(inner: breez_sdk_liquid::plugin::PluginStorage) -> Self {
        Self { inner }
    }
}

impl From<PluginStorageError> for WasmError {
    fn from(value: PluginStorageError) -> Self {
        Self::new(value.to_string())
    }
}

#[wasm_bindgen]
impl PluginStorage {
    #[wasm_bindgen(js_name = "setItem")]
    pub fn set_item(&self, key: String, value: String) -> WasmResult<()> {
        self.inner.set_item(&key, value).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "getItem")]
    pub fn get_item(&self, key: String) -> WasmResult<Option<String>> {
        self.inner.get_item(&key).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "removeItem")]
    pub fn remove_item(&self, key: String) -> WasmResult<()> {
        self.inner.remove_item(&key).map_err(Into::into)
    }
}
