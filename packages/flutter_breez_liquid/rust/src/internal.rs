#![allow(dead_code)]
use crate::plugin::{Plugin, PluginSdk, PluginStorage};
use crate::sdk::{_Plugin, _PluginSdk, _PluginStorage};

pub(crate) struct PluginWrapper {
    pub(crate) plugin: Box<dyn Plugin>,
}

#[async_trait::async_trait]
impl _Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.plugin.id()
    }

    async fn on_start(&self, plugin_sdk: _PluginSdk, storage: _PluginStorage) {
        self.plugin
            .on_start(PluginSdk { plugin_sdk }, PluginStorage { storage });
    }

    async fn on_stop(&self) {
        self.plugin.on_stop();
    }
}
