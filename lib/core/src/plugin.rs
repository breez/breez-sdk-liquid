use maybe_sync::{MaybeSend, MaybeSync};

use crate::sdk::LiquidSdk;
use sdk_common::utils::Arc;

pub trait Plugin: MaybeSend + MaybeSync {
    fn on_start(&self, sdk: Arc<LiquidSdk>);
    fn on_stop(&self);
}
