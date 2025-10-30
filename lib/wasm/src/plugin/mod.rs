use std::sync::Arc;

use crate::{
    event::{EventListener, WasmEventListener},
    model::*,
};
use wasm_bindgen::prelude::*;

use crate::plugin::storage::PluginStorage;

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
impl breez_sdk_liquid::plugin::Plugin for WasmPlugin {
    fn id(&self) -> String {
        self.plugin.id()
    }

    async fn on_start(
        &self,
        plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
        storage: breez_sdk_liquid::plugin::PluginStorage,
    ) {
        self.plugin
            .on_start(PluginSdk { plugin_sdk }, PluginStorage::new(storage));
    }

    async fn on_stop(&self) {
        self.plugin.on_stop();
    }
}

#[wasm_bindgen(typescript_custom_section)]
const PLUGIN_INTERFACE: &'static str = r#"export interface Plugin {
    id: () => string;
    onStart: (plugin_sdk: PluginSdk, storage: PluginStorage) => void;
    onStop: () => void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Plugin")]
    pub type Plugin;

    #[wasm_bindgen(structural, method, js_name = id)]
    fn id(this: &Plugin) -> String;

    #[wasm_bindgen(structural, method, js_name = onStart)]
    fn on_start(this: &Plugin, plugin_sdk: PluginSdk, storage: PluginStorage);

    #[wasm_bindgen(structural, method, js_name = onStop)]
    fn on_stop(this: &Plugin);
}

#[wasm_bindgen]
pub struct PluginSdk {
    plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
}

impl PluginSdk {
    pub(crate) fn sdk(&self) -> breez_sdk_liquid::plugin::PluginSdk {
        self.plugin_sdk.clone()
    }
}

#[wasm_bindgen]
impl PluginSdk {
    #[wasm_bindgen(js_name = "getInfo")]
    pub async fn get_info(&self) -> WasmResult<GetInfoResponse> {
        Ok(self.plugin_sdk.get_info().await?.into())
    }

    #[wasm_bindgen(js_name = "prepareSendPayment")]
    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> WasmResult<PrepareSendResponse> {
        Ok(self
            .plugin_sdk
            .prepare_send_payment(&req.into())
            .await?
            .into())
    }

    #[wasm_bindgen(js_name = "sendPayment")]
    pub async fn send_payment(&self, req: SendPaymentRequest) -> WasmResult<SendPaymentResponse> {
        Ok(self.plugin_sdk.send_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "prepareReceivePayment")]
    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> WasmResult<PrepareReceiveResponse> {
        Ok(self
            .plugin_sdk
            .prepare_receive_payment(&req.into())
            .await?
            .into())
    }

    #[wasm_bindgen(js_name = "receivePayment")]
    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> WasmResult<ReceivePaymentResponse> {
        Ok(self.plugin_sdk.receive_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub async fn add_event_listener(&self, listener: EventListener) -> WasmResult<String> {
        Ok(self
            .plugin_sdk
            .add_event_listener(Box::new(WasmEventListener { listener }))
            .await?)
    }

    #[wasm_bindgen(js_name = "removeEventListener")]
    pub async fn remove_event_listener(&self, id: String) -> WasmResult<()> {
        self.plugin_sdk.remove_event_listener(id).await?;
        Ok(())
    }
}
