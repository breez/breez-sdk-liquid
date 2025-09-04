use maybe_sync::{MaybeSend, MaybeSync};

use crate::sdk::LiquidSdk;
use sdk_common::utils::Weak;

mod events;
mod storage;

pub use events::*;
pub use storage::*;

#[sdk_macros::async_trait]
pub trait Plugin: MaybeSend + MaybeSync {
    fn id(&self) -> String;
    async fn on_start(
        &self,
        sdk: Weak<LiquidSdk>,
        storage: PluginStorage,
        events: PluginEventEmitter,
    );
    async fn on_stop(&self);
}
