use std::collections::HashMap;

use log::{debug, info};
use maybe_sync::{MaybeSend, MaybeSync};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum NwcEventDetails {
    Connected,
    Disconnected,
    PayInvoice {
        success: bool,
        preimage: Option<String>,
        fees_sat: Option<u64>,
        error: Option<String>,
    },
    ListTransactions,
    GetBalance,
}

/// The event emitted when an NWC operation has been handled
#[derive(Clone, Debug, PartialEq)]
pub struct NwcEvent {
    pub event_id: Option<String>,
    pub details: NwcEventDetails,
}

#[sdk_macros::async_trait]
pub trait NwcEventListener: MaybeSend + MaybeSync {
    async fn on_event(&self, event: NwcEvent);
}

pub(crate) struct EventManager {
    listeners: RwLock<HashMap<String, Box<dyn NwcEventListener>>>,
    notifier: broadcast::Sender<NwcEvent>,
    is_paused: AtomicBool,
}

impl EventManager {
    pub fn new() -> Self {
        let (notifier, _) = broadcast::channel::<NwcEvent>(100);

        Self {
            listeners: Default::default(),
            notifier,
            is_paused: AtomicBool::new(false),
        }
    }

    pub async fn add(&self, listener: Box<dyn NwcEventListener>) -> String {
        let id = Uuid::new_v4().to_string();
        (*self.listeners.write().await).insert(id.clone(), listener);
        id
    }

    pub async fn remove(&self, id: &str) {
        (*self.listeners.write().await).remove(id);
    }

    pub async fn notify(&self, e: NwcEvent) {
        match self.is_paused.load(Ordering::SeqCst) {
            true => info!("Event notifications are paused, not emitting event {e:?}"),
            false => {
                debug!("Emitting event: {e:?}");
                let _ = self.notifier.send(e.clone());

                for listener in (*self.listeners.read().await).values() {
                    listener.on_event(e.clone()).await;
                }
            }
        }
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
