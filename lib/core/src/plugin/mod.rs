use std::sync::Weak;

use crate::sdk::LiquidSdk;

mod storage;

pub use storage::*;

#[sdk_macros::async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    async fn on_start(&self, sdk: Weak<LiquidSdk>, storage: PluginStorage);
    async fn on_stop(&self);
}
