use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    str::FromStr as _,
    sync::Arc,
};

use crate::{
    event::{EventManager, NwcEvent, NwcEventDetails},
    handler::{RelayMessageHandler, NWC_SUPPORTED_METHODS},
    model::ActiveConnection,
    persist::Persister,
    sdk_event::SdkEventListener,
};
use anyhow::Result;
use breez_sdk_liquid::plugin::PluginSdk;
use log::{info, warn};
use nostr_sdk::{
    nips::nip47::NostrWalletConnectURI, Alphabet, Client as NostrClient, EventBuilder, Filter,
    Keys, Kind, SingleLetterTag, Tag,
};
use tokio::sync::{mpsc, Mutex, MutexGuard, OnceCell};
use tokio::task::JoinHandle;
use tokio_with_wasm::alias as tokio;

pub(crate) struct RuntimeContext {
    pub sdk: PluginSdk,
    pub client: NostrClient,
    pub our_keys: Keys,
    pub persister: Persister,
    pub event_manager: Arc<EventManager>,
    pub handler: Box<dyn RelayMessageHandler>,
    pub resubscription_trigger: mpsc::Sender<()>,
    pub event_loop_handle: OnceCell<JoinHandle<()>>,
    pub sdk_listener_id: Mutex<Option<String>>,
    pub replied_event_ids: Mutex<HashSet<String>>,
}

impl RuntimeContext {
    pub async fn trigger_resubscription(&self) {
        let _ = self.resubscription_trigger.send(()).await;
    }

    pub async fn clear(&self) {
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
        self.remove_sdk_listener(&mut self.sdk_listener_id.lock().await)
            .await;
        self.client.disconnect().await;
        self.event_manager
            .notify(NwcEvent {
                event_id: None,
                connection_name: None,
                details: NwcEventDetails::Disconnected,
            })
            .await;
        self.event_manager.pause_notifications();
    }

    pub async fn new_sdk_listener(
        self: &Arc<Self>,
        active_connections: &HashMap<String, ActiveConnection>,
    ) -> Result<()> {
        let mut lock = self.sdk_listener_id.lock().await;
        self.remove_sdk_listener(&mut lock).await;
        let listener_id = self
            .sdk
            .add_event_listener(Box::new(SdkEventListener::new(
                self.clone(),
                active_connections
                    .values()
                    .map(|con| con.uri.clone())
                    .collect(),
            )))
            .await?;
        *lock = Some(listener_id);
        Ok(())
    }

    async fn remove_sdk_listener(&self, listener_lock: &mut MutexGuard<'_, Option<String>>) {
        if let Some(listener_id) = (**listener_lock).take() {
            if let Err(err) = self.sdk.remove_event_listener(listener_id).await {
                warn!("Could not remove payment event listener: {err:?}");
            }
        }
    }

    pub async fn list_active_connections(&self) -> Result<HashMap<String, ActiveConnection>> {
        Ok(self
            .persister
            .list_nwc_connections()?
            .into_iter()
            .filter_map(|(name, connection)| {
                NostrWalletConnectURI::from_str(&connection.connection_string)
                    .map(|uri| {
                        (
                            name,
                            ActiveConnection {
                                pubkey: Keys::new(uri.secret.clone()).public_key,
                                uri,
                                connection,
                            },
                        )
                    })
                    .ok()
            })
            .collect())
    }

    pub async fn resubscribe(
        &self,
        active_connections: &HashMap<String, ActiveConnection>,
    ) -> Result<()> {
        if active_connections.is_empty() {
            info!("No active connections, skipping subscription.");
            return Ok(());
        }
        self.client
            .subscribe(
                Filter {
                    generic_tags: BTreeMap::from([(
                        SingleLetterTag {
                            character: Alphabet::P,
                            uppercase: false,
                        },
                        BTreeSet::from([self.our_keys.public_key.to_string()]),
                    )]),
                    kinds: Some(BTreeSet::from([Kind::WalletConnectRequest])),
                    ..Default::default()
                },
                None,
            )
            .await?;
        info!("Successfully subscribed to events");
        Ok(())
    }

    pub async fn send_event(&self, event_builder: EventBuilder) -> Result<()> {
        let event = event_builder.sign_with_keys(&self.our_keys)?;
        self.client.send_event(&event).await?;
        Ok(())
    }

    pub async fn send_info_event(&self) {
        // Broadcast info event
        let content = NWC_SUPPORTED_METHODS.join(" ").to_string();
        if let Err(err) = self
            .send_event(
                EventBuilder::new(Kind::WalletConnectInfo, content).tag(Tag::custom(
                    "encryption".into(),
                    ["nip44_v2 nip04".to_string()],
                )),
            )
            .await
        {
            warn!("Could not send info event to relay pool: {err:?}");
        }
    }

    /// Returns true when we have replied to the event, and false otherwise (and inserts it)
    pub async fn check_replied_event(&self, event_id: String) -> bool {
        !self.replied_event_ids.lock().await.insert(event_id)
    }
}
