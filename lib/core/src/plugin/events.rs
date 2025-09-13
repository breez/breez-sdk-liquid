use crate::{event::EventManager, model::SdkEvent};
use sdk_common::utils::Arc;

pub struct PluginEventEmitter {
    event_manager: Arc<EventManager>,
}

impl PluginEventEmitter {
    pub(crate) fn new(event_manager: Arc<EventManager>) -> Self {
        Self { event_manager }
    }

    pub async fn broadcast(&self, e: SdkEvent) {
        self.event_manager.notify(e).await
    }
}
