use crate::plugin::{Plugin, PluginSdk, PluginServices, PluginStorage};
use crate::sdk::{_Plugin, _PluginSdk, _PluginStorage};
use std::sync::Arc;

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

impl PluginServices {
    pub(crate) fn as_plugins(&self) -> Vec<Arc<dyn _Plugin>> {
        let mut plugins = vec![];
        if let Some(nwc_service) = self.nwc.clone() {
            let plugin = Box::new(nwc_service) as Box<dyn Plugin>;
            plugins.push(Arc::new(PluginWrapper { plugin }) as Arc<dyn _Plugin>);
        }
        plugins
    }
}
