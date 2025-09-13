use std::{collections::HashMap, sync::Arc};

use breez_sdk_liquid::plugin::Plugin as _;
pub use breez_sdk_liquid_nwc::{
    error::{NwcError, NwcResult},
    NwcConfig, NwcService, SdkNwcService,
};

use crate::{rt, BindingLiquidSdk, Plugin, PluginEventEmitter, PluginStorage};

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
}

#[sdk_macros::async_trait]
impl Plugin for BindingNwcService {
    fn id(&self) -> String {
        self.inner.id()
    }

    fn on_start(
        &self,
        sdk: Arc<BindingLiquidSdk>,
        storage: Arc<PluginStorage>,
        event_emitter: Arc<PluginEventEmitter>,
    ) {
        let cloned = self.inner.clone();
        rt().spawn(async move {
            cloned
                .on_start(
                    Arc::downgrade(&sdk.sdk),
                    storage.storage.clone(),
                    event_emitter.event_emitter.clone(),
                )
                .await;
        });
    }

    fn on_stop(&self) {
        rt().block_on(async {
            self.inner.on_stop().await;
        })
    }
}
