use anyhow::{bail, Result};
use maybe_sync::{MaybeSend, MaybeSync};

use crate::{error::SdkResult, persist::Persister, sdk::LiquidSdk};
use sdk_common::utils::Arc;

pub struct PluginStorage {
    plugin_id: String,
    persister: std::sync::Arc<Persister>,
}

impl PluginStorage {
    pub(crate) fn new(persister: std::sync::Arc<Persister>, plugin_id: String) -> Result<Self> {
        if plugin_id.is_empty() {
            log::error!("Plugin ID cannot be an empty string!");
            bail!("Plugin ID cannot be an empty string!");
        }

        Ok(Self {
            persister,
            plugin_id,
        })
    }

    pub(crate) fn scoped_key(&self, key: String) -> String {
        format!("{}-{}", self.plugin_id, key)
    }

    pub fn set_item(&self, key: String, value: String) -> SdkResult<()> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .update_cached_item(&scoped_key, value)
            .map_err(Into::into)
    }

    pub fn get_item(&self, key: String) -> SdkResult<Option<String>> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .get_cached_item(&scoped_key)
            .map_err(Into::into)
    }

    pub fn remove_item(&self, key: String) -> SdkResult<()> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .delete_cached_item(&scoped_key)
            .map_err(Into::into)
    }
}

#[sdk_macros::async_trait]
pub trait Plugin: MaybeSend + MaybeSync {
    fn id(&self) -> String;
    async fn on_start(&self, sdk: Arc<LiquidSdk>, storage: PluginStorage);
    async fn on_stop(&self);
}
