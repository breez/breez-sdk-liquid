use breez_sdk_liquid::plugin::Plugin as _;
use breez_sdk_liquid_nwc::{NwcService as _, SdkNwcService};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    error::WasmError,
    model::WasmResult,
    plugin::{storage::PluginStorage, PluginSdk},
};

mod model {
    use wasm_bindgen::prelude::*;

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::PeriodicBudget)]
    pub struct PeriodicBudget {
        pub used_budget_sat: u64,
        pub max_budget_sat: u64,
        pub reset_time_sec: u32,
        pub updated_at: u32,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::PeriodicBudgetRequest)]
    pub struct PeriodicBudgetRequest {
        pub max_budget_sat: u64,
        pub reset_time_sec: u32,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::NwcConnection)]
    pub struct NwcConnection {
        pub connection_string: String,
        pub created_at: u32,
        pub receive_only: bool,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget: Option<PeriodicBudget>,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::AddConnectionRequest)]
    pub struct AddConnectionRequest {
        pub name: String,
        pub receive_only: Option<bool>,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::AddConnectionResponse)]
    pub struct AddConnectionResponse {
        pub connection: NwcConnection,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::EditConnectionRequest)]
    pub struct EditConnectionRequest {
        pub name: String,
        pub receive_only: Option<bool>,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    }

    #[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::EditConnectionResponse)]
    pub struct EditConnectionResponse {
        pub connection: NwcConnection,
    }

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

    unsafe impl Send for WasmNwcEventListener {}
    unsafe impl Sync for WasmNwcEventListener {}

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

#[derive(Clone)]
#[sdk_macros::extern_wasm_bindgen(breez_sdk_liquid_nwc::model::NwcConfig)]
pub struct NwcConfig {
    pub relay_urls: Option<Vec<String>>,
    pub secret_key_hex: Option<String>,
}

#[wasm_bindgen]
pub struct BindingNwcService {
    inner: Rc<SdkNwcService>,
}

#[wasm_bindgen]
impl BindingNwcService {
    #[wasm_bindgen(constructor)]
    pub fn new(config: NwcConfig) -> Self {
        let inner = Rc::new(SdkNwcService::new(config.into()));
        Self { inner }
    }

    // NWC
    #[wasm_bindgen(js_name = "addConnection")]
    pub async fn add_connection(
        &self,
        req: AddConnectionRequest,
    ) -> WasmResult<AddConnectionResponse> {
        self.inner
            .add_connection(req.into())
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "editConnection")]
    pub async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> WasmResult<EditConnectionResponse> {
        self.inner
            .edit_connection(req.into())
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    #[wasm_bindgen(js_name = "listConnections")]
    pub async fn list_connections(&self) -> WasmResult<js_sys::Map> {
        let connections = self
            .inner
            .list_connections()
            .await
            .map_err(Into::<WasmError>::into)?;
        let mut result = js_sys::Map::new();
        for (name, con) in connections.into_iter() {
            let con: NwcConnection = con.into();
            result = result.set(&JsValue::from_str(&name), &JsValue::from(con));
        }
        Ok(result)
    }

    #[wasm_bindgen(js_name = "removeConnection")]
    pub async fn remove_connection(&self, name: String) -> WasmResult<()> {
        self.inner.remove_connection(name).await.map_err(Into::into)
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
    pub async fn on_start(&self, plugin_sdk: PluginSdk, storage: PluginStorage) {
        self.inner
            .on_start(plugin_sdk.sdk(), storage.storage())
            .await;
    }

    #[wasm_bindgen(js_name = "onStop")]
    pub async fn on_stop(&self) {
        self.inner.on_stop().await;
    }
}
