use std::{collections::HashMap, sync::Arc};

use crate::frb_generated::StreamSink;
use crate::plugin::{Plugin, PluginSdk, PluginStorage};
use breez_sdk_liquid::model::Payment;
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
        pub listen_to_events: Option<bool>,
    }

    #[frb(mirror(PeriodicBudget))]
    pub struct _PeriodicBudget {
        pub used_budget_sat: u64,
        pub max_budget_sat: u64,
        pub renews_at: Option<u32>,
        pub updated_at: u32,
    }

    #[frb(mirror(PeriodicBudgetRequest))]
    pub struct _PeriodicBudgetRequest {
        pub max_budget_sat: u64,
        pub renewal_time_mins: Option<u32>,
    }

    #[frb(mirror(NwcConnection))]
    pub struct _NwcConnection {
        pub connection_string: String,
        pub created_at: u32,
        pub receive_only: bool,
        pub paid_amount_sat: u64,
        pub expires_at: Option<u32>,
        pub periodic_budget: Option<PeriodicBudget>,
    }

    #[frb(mirror(AddConnectionRequest))]
    pub struct _AddConnectionRequest {
        pub name: String,
        pub receive_only: Option<bool>,
        pub expiry_time_mins: Option<u32>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    }

    #[frb(mirror(AddConnectionResponse))]
    pub struct _AddConnectionResponse {
        pub connection: NwcConnection,
    }

    #[frb(mirror(EditConnectionRequest))]
    pub struct _EditConnectionRequest {
        pub name: String,
        pub receive_only: Option<bool>,
        pub expiry_time_mins: Option<u32>,
        pub remove_expiry: Option<bool>,
        pub periodic_budget_req: Option<PeriodicBudgetRequest>,
        pub remove_periodic_budget: Option<bool>,
    }

    #[frb(mirror(EditConnectionResponse))]
    pub struct _EditConnectionResponse {
        pub connection: NwcConnection,
    }

    #[frb(mirror(NostrServiceInfo))]
    pub struct _NostrServiceInfo {
        pub wallet_pubkey: String,
        pub connected_relays: Vec<String>,
    }
}

pub mod event {
    pub use breez_sdk_liquid_nwc::event::{
        NwcEvent, NwcEventDetails, NwcEventListener as _NwcEventListener,
    };
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
        MakeInvoice,
        ListTransactions,
        GetBalance,
        GetInfo,
        ConnectionExpired,
        ConnectionRefreshed,
        ZapReceived {
            invoice: String,
        },
    }

    #[frb(mirror(NwcEvent))]
    pub struct _NwcEvent {
        pub event_id: Option<String>,
        pub connection_name: Option<String>,
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
    impl _NwcEventListener for BreezNwcEventListener {
        async fn on_event(&self, e: NwcEvent) {
            NwcEventListener::on_event(self, e);
        }
    }
}
use event::BreezNwcEventListener;

#[derive(Clone)]
pub struct BreezNwcService {
    pub(crate) service: Arc<SdkNwcService>,
}

impl BreezNwcService {
    #[frb(sync)]
    pub fn new(config: NwcConfig) -> BreezNwcService {
        Self {
            service: Arc::new(SdkNwcService::new(config)),
        }
    }

    pub async fn add_connection(
        &self,
        req: AddConnectionRequest,
    ) -> Result<AddConnectionResponse, NwcError> {
        self.service.add_connection(req).await
    }

    pub async fn edit_connection(
        &self,
        req: EditConnectionRequest,
    ) -> Result<EditConnectionResponse, NwcError> {
        self.service.edit_connection(req).await
    }

    pub async fn list_connections(&self) -> Result<HashMap<String, NwcConnection>, NwcError> {
        self.service.list_connections().await
    }

    pub async fn handle_event(&self, event_id: String) -> Result<(), NwcError> {
        self.service.handle_event(event_id).await
    }

    pub async fn list_connection_payments(&self, name: String) -> Result<Vec<Payment>, NwcError> {
        self.service.list_connection_payments(name).await
    }

    pub async fn remove_connection(&self, name: String) -> Result<(), NwcError> {
        self.service.remove_connection(name).await
    }

    pub async fn add_event_listener(&self, listener: StreamSink<NwcEvent>) -> String {
        self.service
            .add_event_listener(Box::new(BreezNwcEventListener { stream: listener }))
            .await
    }

    pub async fn get_info(&self) -> Option<NostrServiceInfo> {
        self.service.get_info().await
    }

    pub async fn track_zap(&self, invoice: String, zap_request: String) -> Result<(), NwcError> {
        self.service.track_zap(invoice, zap_request).await
    }
}

impl Plugin for BreezNwcService {
    fn id(&self) -> String {
        self.service.id()
    }

    fn on_start(&self, plugin_sdk: PluginSdk, storage: PluginStorage) {
        futures::executor::block_on(
            self.service
                .on_start(plugin_sdk.plugin_sdk.clone(), storage.storage.clone()),
        );
    }

    fn on_stop(&self) {
        futures::executor::block_on(self.service.on_stop());
    }
}
