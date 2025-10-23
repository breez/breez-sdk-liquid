use std::sync::{Arc, Weak};

use breez_sdk_liquid::prelude::*;
use sdk_macros;

use crate::BindingLiquidSdk;

pub use breez_sdk_liquid::plugin::PluginStorageError;

pub struct PluginStorage {
    pub(crate) storage: breez_sdk_liquid::plugin::PluginStorage,
}

impl PluginStorage {
    pub fn set_item(&self, key: String, value: String) -> Result<(), PluginStorageError> {
        self.storage.set_item(&key, value)
    }

    pub fn get_item(&self, key: String) -> Result<Option<String>, PluginStorageError> {
        self.storage.get_item(&key)
    }

    pub fn remove_item(&self, key: String) -> Result<(), PluginStorageError> {
        self.storage.remove_item(&key)
    }
}

pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    fn on_start(&self, sdk: Arc<BindingLiquidSdk>, storage: Arc<PluginStorage>);
    fn on_stop(&self);
}

pub(crate) struct PluginWrapper {
    pub(crate) inner: Box<dyn Plugin>,
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid::plugin::Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.inner.id()
    }

    async fn on_start(
        &self,
        sdk: Weak<LiquidSdk>,
        storage: breez_sdk_liquid::plugin::PluginStorage,
    ) {
        self.inner.on_start(
            BindingLiquidSdk { sdk }.into(),
            PluginStorage { storage }.into(),
        );
    }

    async fn on_stop(&self) {
        self.inner.on_stop();
    }
}
