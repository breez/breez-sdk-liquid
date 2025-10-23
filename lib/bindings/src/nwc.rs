use std::{collections::HashMap, sync::Arc};

use breez_sdk_liquid::plugin::Plugin as _;
pub use breez_sdk_liquid_nwc::{
    error::{NwcError, NwcResult},
    event::{NwcEvent, NwcEventDetails},
    NwcConfig, NwcService, SdkNwcService,
};

use crate::{rt, BindingLiquidSdk, Plugin, PluginStorage};

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
    pub fn add_connection_string(&self, name: String) -> NwcResult<String> {
        rt().block_on(async { self.inner.add_connection_string(name).await })
    }

    pub fn list_connection_strings(&self) -> NwcResult<HashMap<String, String>> {
        rt().block_on(async { self.inner.list_connection_strings().await })
    }

    pub fn remove_connection_string(&self, name: String) -> NwcResult<()> {
        rt().block_on(async { self.inner.remove_connection_string(name).await })
    }

    pub fn add_event_listener(&self, listener: Box<dyn NwcEventListener>) -> String {
        let listener: Box<dyn breez_sdk_liquid_nwc::event::NwcEventListener> =
            Box::new(NwcEventListenerWrapper::new(listener));
        rt().block_on(async { self.inner.add_event_listener(listener).await })
    }

    pub fn remove_event_listener(&self, listener_id: String) {
        rt().block_on(async { self.inner.remove_event_listener(&listener_id).await })
    }
}

#[sdk_macros::async_trait]
impl Plugin for BindingNwcService {
    fn id(&self) -> String {
        self.inner.id()
    }

    fn on_start(&self, sdk: Arc<BindingLiquidSdk>, storage: Arc<PluginStorage>) {
        let cloned = self.inner.clone();
        rt().spawn(async move {
            cloned
                .on_start(sdk.sdk.clone(), storage.storage.clone())
                .await;
        });
    }

    fn on_stop(&self) {
        rt().block_on(async {
            self.inner.on_stop().await;
        })
    }
}
