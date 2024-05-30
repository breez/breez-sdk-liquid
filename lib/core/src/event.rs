use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::sync::{broadcast, RwLock};

use crate::model::{EventListener, LiquidSdkEvent};

pub(crate) struct EventManager {
    listeners: RwLock<HashMap<String, Box<dyn EventListener>>>,
    notifier: broadcast::Sender<LiquidSdkEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        let (notifier, _) = broadcast::channel::<LiquidSdkEvent>(100);

        Self {
            listeners: Default::default(),
            notifier,
        }
    }

    pub async fn add(&self, listener: Box<dyn EventListener>) -> Result<String> {
        let id = format!(
            "{:X}",
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        );
        (*self.listeners.write().await).insert(id.clone(), listener);
        Ok(id)
    }

    pub async fn remove(&self, id: String) {
        (*self.listeners.write().await).remove(&id);
    }

    pub async fn notify(&self, e: LiquidSdkEvent) {
        let _ = self.notifier.send(e.clone());

        for listener in (*self.listeners.read().await).values() {
            listener.on_event(e.clone());
        }
    }

    pub(crate) fn subscribe(&self) -> broadcast::Receiver<LiquidSdkEvent> {
        self.notifier.subscribe()
    }
}
