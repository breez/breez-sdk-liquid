use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::{debug, info};
use tokio::sync::{broadcast, RwLock};

use crate::model::{EventListener, SdkEvent};

pub(crate) struct EventManager {
    listeners: RwLock<HashMap<String, Box<dyn EventListener>>>,
    notifier: broadcast::Sender<SdkEvent>,
    is_paused: AtomicBool,
}

impl EventManager {
    pub fn new() -> Self {
        let (notifier, _) = broadcast::channel::<SdkEvent>(100);

        Self {
            listeners: Default::default(),
            notifier,
            is_paused: AtomicBool::new(false),
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

    pub async fn notify(&self, e: SdkEvent) {
        match self.is_paused.load(Ordering::SeqCst) {
            true => info!("Event notifications are paused, not emitting event {e:?}"),
            false => {
                debug!("Emitting event: {e:?}");
                let _ = self.notifier.send(e.clone());

                for listener in (*self.listeners.read().await).values() {
                    listener.on_event(e.clone());
                }
            }
        }
    }

    pub(crate) fn subscribe(&self) -> broadcast::Receiver<SdkEvent> {
        self.notifier.subscribe()
    }

    pub(crate) fn pause_notifications(&self) {
        info!("Pausing event notifications");
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub(crate) fn resume_notifications(&self) {
        info!("Resuming event notifications");
        self.is_paused.store(false, Ordering::SeqCst);
    }
}
