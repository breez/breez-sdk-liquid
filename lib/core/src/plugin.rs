use maybe_sync::{MaybeSend, MaybeSync};

use crate::{error::SdkResult, persist::Persister, sdk::LiquidSdk};
use sdk_common::utils::Arc;

pub struct PluginStorage {
    persister: std::sync::Arc<Persister>,
}

impl PluginStorage {
    pub(crate) fn new(persister: std::sync::Arc<Persister>) -> Self {
        Self { persister }
    }

    pub fn set_item(&self, key: String, value: String) -> SdkResult<()> {
        self.persister
            .update_cached_item(&key, value)
            .map_err(Into::into)
    }

    pub fn get_item(&self, key: String) -> SdkResult<Option<String>> {
        self.persister.get_cached_item(&key).map_err(Into::into)
    }

    pub fn remove_item(&self, key: String) -> SdkResult<()> {
        self.persister.delete_cached_item(&key).map_err(Into::into)
    }
}

#[sdk_macros::async_trait]
pub trait Plugin: MaybeSend + MaybeSync {
    async fn on_start(&self, sdk: Arc<LiquidSdk>, storage: PluginStorage);
    async fn on_stop(&self);
}
