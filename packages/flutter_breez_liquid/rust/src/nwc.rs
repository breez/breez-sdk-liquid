use std::{collections::HashMap, sync::Arc};

use crate::frb_generated::StreamSink;
use crate::plugin::{Plugin, PluginSdk, PluginStorage};
use breez_sdk_liquid::plugin::Plugin as _;
pub use breez_sdk_liquid_nwc::NwcService as _;
pub use breez_sdk_liquid_nwc::{
    error::{NwcError, NwcResult},
    event::{NwcEvent, NwcEventDetails},
    model::*,
    SdkNwcService,
};
use flutter_rust_bridge::frb;

pub mod model {
    pub use breez_sdk_liquid_nwc::model::*;
    use flutter_rust_bridge::frb;

    #[frb(mirror(NwcConfig))]
    pub struct _NwcConfig {
        pub relay_urls: Option<Vec<String>>,
        pub secret_key_hex: Option<String>,
    }

    #[frb(mirror(PeriodicBudget))]
    pub struct _PeriodicBudget {
        pub used_budget_sat: u64,
        pub max_budget_sat: u64,
        pub reset_time_sec: u32,
        pub updated_at: u32,
    }

    #[frb(mirror(PeriodicBudgetRequest))]
    pub struct _PeriodicBudgetRequest {
        pub max_budget_sat: u64,
        pub reset_time_sec: u32,
    }

    #[frb(mirror(NwcConnection))]
    pub struct _NwcConnection {
        pub connection_string: String,
        pub created_at: u32,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget: Option<PeriodicBudget>,
    }

    #[frb(mirror(AddConnectionRequest))]
    pub struct _AddConnectionRequest {
        pub name: String,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    }

    #[frb(mirror(AddConnectionResponse))]
    pub struct _AddConnectionResponse {
        pub connection: NwcConnection,
    }

    #[frb(mirror(EditConnectionRequest))]
    pub struct _EditConnectionRequest {
        pub name: String,
        pub expiry_time_sec: Option<u32>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    }

    #[frb(mirror(EditConnectionResponse))]
    pub struct _EditConnectionResponse {
        pub connection: NwcConnection,
    }
}

pub mod event {
    pub use breez_sdk_liquid_nwc::event::{NwcEvent, NwcEventDetails};
    use flutter_rust_bridge::frb;

    use crate::frb_generated::StreamSink;

    #[frb(mirror(NwcEventDetails))]
    pub enum _NwcEventDetails {
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

    #[frb(mirror(NwcEvent))]
    pub struct _NwcEvent {
        pub event_id: Option<String>,
        pub details: NwcEventDetails,
    }

    pub trait NwcEventListener: Send + Sync {
        fn on_event(&self, event: NwcEvent);
    }

    pub struct BreezNwcEventListener {
        pub stream: StreamSink<NwcEvent>,
    }

    impl NwcEventListener for BreezNwcEventListener {
        fn on_event(&self, e: NwcEvent) {
            let _ = self.stream.add(e);
        }
    }

    #[async_trait::async_trait]
    impl breez_sdk_liquid_nwc::event::NwcEventListener for BreezNwcEventListener {
        #[frb(ignore)]
        async fn on_event(&self, e: NwcEvent) {
            NwcEventListener::on_event(self, e);
        }
    }
}
use event::BreezNwcEventListener;

pub struct NwcService {
    inner: Arc<SdkNwcService>,
}

impl NwcService {
    #[frb(sync)]
    pub fn new(config: NwcConfig) -> Self {
        Self {
            inner: Arc::new(SdkNwcService::new(config)),
        }
    }
}

impl NwcService {
    pub async fn add_connection(
        &self,
        req: AddConnectionRequest,
    ) -> Result<AddConnectionResponse, NwcError> {
        self.inner.add_connection(req).await
    }

    pub async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> Result<EditConnectionResponse, NwcError> {
        self.inner.edit_connection(req).await
    }

    pub async fn list_connections(&self) -> Result<HashMap<String, NwcConnection>, NwcError> {
        self.inner.list_connections().await
    }

    pub async fn remove_connection(&self, name: String) -> Result<(), NwcError> {
        self.inner.remove_connection(name).await
    }

    pub async fn add_event_listener(&self, listener: StreamSink<NwcEvent>) -> String {
        self.inner
            .add_event_listener(Box::new(BreezNwcEventListener { stream: listener }))
            .await
    }
}

impl Plugin for NwcService {
    fn id(&self) -> String {
        self.inner.id()
    }

    fn on_start(&self, plugin_sdk: PluginSdk, storage: PluginStorage) {
        let _ = self
            .inner
            .on_start(plugin_sdk.plugin_sdk.clone(), storage.storage.clone());
    }

    fn on_stop(&self) {
        let _ = self.inner.on_stop();
    }
}
