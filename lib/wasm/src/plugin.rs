use std::rc::Rc;

use breez_sdk_liquid::sdk::LiquidSdk;
use wasm_bindgen::prelude::*;

use crate::BindingLiquidSdk;

pub struct WasmPlugin {
    pub plugin: Plugin,
}

impl From<Plugin> for Rc<dyn breez_sdk_liquid::plugin::Plugin> {
    fn from(val: Plugin) -> Self {
        Rc::new(WasmPlugin { plugin: val })
    }
}

impl breez_sdk_liquid::prelude::Plugin for WasmPlugin {
    fn on_start(&self, sdk: Rc<LiquidSdk>) {
        self.plugin.on_start(BindingLiquidSdk { sdk });
    }

    fn on_stop(&self) {
        self.plugin.on_stop();
    }
}

#[wasm_bindgen(typescript_custom_section)]
const PLUGIN_INTERFACE: &'static str = r#"export interface Plugin {
    on_start: (sdk: BindingLiquidSdk) => void;
    on_stop: () => void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Plugin")]
    pub type Plugin;

    #[wasm_bindgen(structural, method, js_name = on_start)]
    fn on_start(this: &Plugin, sdk: BindingLiquidSdk);

    #[wasm_bindgen(structural, method, js_name = on_stop)]
    fn on_stop(this: &Plugin);
}
