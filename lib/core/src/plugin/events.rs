use crate::{event::EventManager, model::SdkEvent};
use log::warn;
use sdk_common::utils::Weak;

#[derive(Clone)]
pub struct PluginEventEmitter {
    event_manager: Weak<EventManager>,
}

impl PluginEventEmitter {
    pub(crate) fn new(event_manager: Weak<EventManager>) -> Self {
        Self { event_manager }
    }

    pub async fn broadcast(&self, e: SdkEvent) {
        if let Some(event_manager) = self.event_manager.upgrade() {
            event_manager.notify(e).await;
        } else {
            warn!("Attempted to broadcast event while SDK was unavailable: {e:?}");
        }
    }
}
