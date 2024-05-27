use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::model::{EventListener, LiquidSdkEvent};

pub(crate) struct EventManager {
    listeners: RwLock<HashMap<String, Box<dyn EventListener>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            listeners: Default::default(),
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
        for listener in (*self.listeners.read().await).values() {
            listener.on_event(e.clone());
        }
    }
}
