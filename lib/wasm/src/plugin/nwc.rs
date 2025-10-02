use breez_sdk_liquid::plugin::Plugin as _;
use breez_sdk_liquid_nwc::{NwcService as _, SdkNwcService};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    error::WasmError, model::WasmResult, plugin::storage::PluginStorage, BindingLiquidSdk,
};

mod model {
    use wasm_bindgen::prelude::*;

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::event::NwcEventDetails)]
    pub enum NwcEventDetails {
        Connected,
        Disconnected,
        PayInvoice {
            success: bool,
            preimage: Option<String>,
            fees_sat: Option<u64>,
            error: Option<String>,
        },
        ListTransactions,
        GetBalance,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::event::NwcEvent)]
    pub struct NwcEvent {
        pub event_id: Option<String>,
        pub details: NwcEventDetails,
    }

    pub struct WasmNwcEventListener {
        pub listener: NwcEventListener,
    }

    #[sdk_macros::async_trait]
    impl breez_sdk_liquid_nwc::event::NwcEventListener for WasmNwcEventListener {
        async fn on_event(&self, e: breez_sdk_liquid_nwc::event::NwcEvent) {
            self.listener.on_event(e.into()).await;
        }
    }

    #[wasm_bindgen(typescript_custom_section)]
    const NWC_EVENT_INTERFACE: &'static str = r#"export interface NwcEventListener {
        onEvent: (e: NwcEvent) => void;
    }"#;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "NwcEventListener")]
        pub type NwcEventListener;

        #[wasm_bindgen(structural, method, js_name = onEvent)]
        pub async fn on_event(this: &NwcEventListener, e: NwcEvent);
    }
}
pub use model::*;

#[wasm_bindgen]
pub struct BindingNwcService {
    inner: Rc<SdkNwcService>,
}

#[wasm_bindgen]
impl BindingNwcService {
    // NWC
    #[wasm_bindgen(js_name = "addConnectionString")]
    pub async fn add_connection_string(&self, name: String) -> WasmResult<String> {
        self.inner
            .add_connection_string(name)
            .await
            .map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "listConnectionStrings")]
    pub async fn list_connection_strings(&self) -> WasmResult<js_sys::Map> {
        let uris = self
            .inner
            .list_connection_strings()
            .await
            .map_err(Into::<WasmError>::into)?;
        let mut result = js_sys::Map::new();
        for (key, value) in uris.into_iter() {
            result = result.set(&JsValue::from_str(&key), &JsValue::from_str(&value));
        }
        Ok(result)
    }

    #[wasm_bindgen(js_name = "removeConnectionString")]
    pub async fn remove_connection_string(&self, name: String) -> WasmResult<()> {
        self.inner
            .remove_connection_string(name)
            .await
            .map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub async fn add_event_listener(&self, listener: model::NwcEventListener) -> String {
        let listener: Box<dyn breez_sdk_liquid_nwc::event::NwcEventListener> =
            Box::new(WasmNwcEventListener { listener });
        self.inner.add_event_listener(listener).await
    }

    #[wasm_bindgen(js_name = "removeEventListener")]
    pub async fn remove_event_listener(&self, listener_id: String) {
        self.inner.remove_event_listener(&listener_id).await
    }

    /// Plugin
    #[wasm_bindgen(js_name = "id")]
    pub fn id(&self) -> String {
        self.inner.id()
    }

    #[wasm_bindgen(js_name = "onStart")]
    pub async fn on_start(&self, sdk: &BindingLiquidSdk, storage: PluginStorage) {
        self.inner
            .on_start(Rc::downgrade(&sdk.sdk), storage.storage())
            .await;
    }

    #[wasm_bindgen(js_name = "onStop")]
    pub async fn on_stop(&self) {
        self.inner.on_stop().await;
    }
}
