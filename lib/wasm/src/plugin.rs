use std::rc::Rc;

use breez_sdk_liquid::sdk::LiquidSdk;
use wasm_bindgen::prelude::*;

use crate::{model::WasmResult, BindingLiquidSdk};

pub struct WasmPlugin {
    pub plugin: Plugin,
}

impl From<Plugin> for Rc<dyn breez_sdk_liquid::plugin::Plugin> {
    fn from(val: Plugin) -> Self {
        Rc::new(WasmPlugin { plugin: val })
    }
}

#[wasm_bindgen]
pub struct PluginStorage {
    inner: breez_sdk_liquid::plugin::PluginStorage,
}

impl PluginStorage {
    fn new(inner: breez_sdk_liquid::plugin::PluginStorage) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl PluginStorage {
    #[wasm_bindgen(js_name = "setItem")]
    pub fn set_item(&self, key: String, value: String) -> WasmResult<()> {
        self.inner.set_item(key, value).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "getItem")]
    pub fn get_item(&self, key: String) -> WasmResult<Option<String>> {
        self.inner.get_item(key).map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "removeItem")]
    pub fn remove_item(&self, key: String) -> WasmResult<()> {
        self.inner.remove_item(key).map_err(Into::into)
    }
}

impl breez_sdk_liquid::prelude::Plugin for WasmPlugin {
    fn on_start(&self, sdk: Rc<LiquidSdk>, storage: breez_sdk_liquid::plugin::PluginStorage) {
        self.plugin
            .on_start(BindingLiquidSdk { sdk }, PluginStorage::new(storage));
    }

    fn on_stop(&self) {
        self.plugin.on_stop();
    }
}

#[wasm_bindgen(typescript_custom_section)]
const PLUGIN_INTERFACE: &'static str = r#"export interface Plugin {
    on_start: (sdk: BindingLiquidSdk, storage: PluginStorage) => void;
    on_stop: () => void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Plugin")]
    pub type Plugin;

    #[wasm_bindgen(structural, method, js_name = on_start)]
    fn on_start(this: &Plugin, sdk: BindingLiquidSdk, storage: PluginStorage);

    #[wasm_bindgen(structural, method, js_name = on_stop)]
    fn on_stop(this: &Plugin);
}
