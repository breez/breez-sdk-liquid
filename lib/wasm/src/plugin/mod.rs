use std::sync::{Arc, Weak};

use breez_sdk_liquid::sdk::LiquidSdk;
use log::warn;
use wasm_bindgen::prelude::*;

use crate::{plugin::storage::PluginStorage, BindingLiquidSdk};

pub mod nwc;
pub mod storage;

pub struct WasmPlugin {
    pub plugin: Plugin,
}

// This assumes that we'll always be running in a single thread (true for Wasm environments)
unsafe impl Send for WasmPlugin {}
unsafe impl Sync for WasmPlugin {}

impl From<Plugin> for Arc<dyn breez_sdk_liquid::plugin::Plugin> {
    fn from(val: Plugin) -> Self {
        Arc::new(WasmPlugin { plugin: val })
    }
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid::prelude::Plugin for WasmPlugin {
    fn id(&self) -> String {
        self.plugin.id()
    }

    async fn on_start(
        &self,
        sdk: Weak<LiquidSdk>,
        storage: breez_sdk_liquid::plugin::PluginStorage,
    ) {
        let Some(sdk) = sdk.upgrade() else {
            warn!(
                "Tried to start plugin {} while SDK was unavailable",
                self.id()
            );
            return;
        };

        self.plugin
            .on_start(BindingLiquidSdk { sdk }, PluginStorage::new(storage));
    }

    async fn on_stop(&self) {
        self.plugin.on_stop();
    }
}

#[wasm_bindgen(typescript_custom_section)]
const PLUGIN_INTERFACE: &'static str = r#"export interface Plugin {
    id: () => string;
    onStart: (sdk: BindingLiquidSdk, storage: PluginStorage) => void;
    onStop: () => void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Plugin")]
    pub type Plugin;

    #[wasm_bindgen(structural, method, js_name = id)]
    fn id(this: &Plugin) -> String;

    #[wasm_bindgen(structural, method, js_name = onStart)]
    fn on_start(this: &Plugin, sdk: BindingLiquidSdk, storage: PluginStorage);

    #[wasm_bindgen(structural, method, js_name = onStop)]
    fn on_stop(this: &Plugin);
}
