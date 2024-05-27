use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

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

    pub fn add(&self, listener: Box<dyn EventListener>) -> Result<String> {
        let id = format!(
            "{:X}",
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        );
        self.listeners.write().unwrap().insert(id.clone(), listener);
        Ok(id)
    }

    pub fn remove(&self, id: String) {
        self.listeners.write().unwrap().remove(&id);
    }

    pub fn notify(&self, e: LiquidSdkEvent) {
        for listener in self.listeners.read().unwrap().values() {
            listener.on_event(e.clone());
        }
    }
}
