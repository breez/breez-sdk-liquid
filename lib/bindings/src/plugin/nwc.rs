use std::{collections::HashMap, sync::Arc};

use breez_sdk_liquid::plugin::Plugin as _;
pub use breez_sdk_liquid_nwc::{
    error::{NwcError, NwcResult},
    event::{NwcEvent, NwcEventDetails},
    model::*,
    NwcService, SdkNwcService,
};

use crate::{plugin::PluginSdk, rt, Plugin, PluginStorage};

pub trait NwcEventListener: Send + Sync {
    fn on_event(&self, event: NwcEvent);
}

struct NwcEventListenerWrapper {
    inner: Box<dyn NwcEventListener>,
}

impl NwcEventListenerWrapper {
    pub(crate) fn new(inner: Box<dyn NwcEventListener>) -> Self {
        Self { inner }
    }
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid_nwc::event::NwcEventListener for NwcEventListenerWrapper {
    async fn on_event(&self, e: NwcEvent) {
        self.inner.on_event(e);
    }
}

#[derive(Clone)]
pub struct BindingNwcService {
    inner: Arc<SdkNwcService>,
}

impl BindingNwcService {
    pub fn new(config: NwcConfig) -> Self {
        Self {
            inner: Arc::new(SdkNwcService::new(config)),
        }
    }
}

impl BindingNwcService {
    pub fn add_connection(&self, req: AddConnectionRequest) -> NwcResult<AddConnectionResponse> {
        rt().block_on(self.inner.add_connection(req))
    }

    pub fn edit_connection(&self, req: EditConnectionRequest) -> NwcResult<EditConnectionResponse> {
        rt().block_on(self.inner.edit_connection(req))
    }

    pub fn list_connections(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        rt().block_on(self.inner.list_connections())
    }

    pub fn remove_connection(&self, name: String) -> NwcResult<()> {
        rt().block_on(self.inner.remove_connection(name))
    }

    pub fn add_event_listener(&self, listener: Box<dyn NwcEventListener>) -> String {
        let listener: Box<dyn breez_sdk_liquid_nwc::event::NwcEventListener> =
            Box::new(NwcEventListenerWrapper::new(listener));
        rt().block_on(self.inner.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, listener_id: String) {
        rt().block_on(self.inner.remove_event_listener(&listener_id))
    }
}

#[sdk_macros::async_trait]
impl Plugin for BindingNwcService {
    fn id(&self) -> String {
        self.inner.id()
    }

    fn on_start(&self, plugin_sdk: Arc<PluginSdk>, storage: Arc<PluginStorage>) {
        let cloned = self.inner.clone();
        rt().spawn(async move {
            cloned
                .on_start(plugin_sdk.plugin_sdk.clone(), storage.storage.clone())
                .await;
        });
    }

    fn on_stop(&self) {
        rt().block_on(async {
            self.inner.on_stop().await;
        })
    }
}
