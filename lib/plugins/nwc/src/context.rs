use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr as _,
};

use crate::{handler::RelayMessageHandler, persist::Persister};
use anyhow::Result;
use breez_sdk_liquid::prelude::*;
use log::{info, warn};
use nostr_sdk::{
    nips::nip47::NostrWalletConnectURI, Alphabet, Client as NostrClient, EventBuilder, Filter,
    Keys, Kind, SingleLetterTag, Tag,
};
use sdk_common::utils::Weak;
use tokio::sync::{mpsc, Mutex, OnceCell};
use tokio::task::JoinHandle;
use tokio_with_wasm::alias as tokio;

pub(crate) struct RuntimeContext {
    pub sdk: Weak<LiquidSdk>,
    pub client: NostrClient,
    pub our_keys: Keys,
    pub persister: Persister,
    pub event_emitter: PluginEventEmitter,
    pub handler: Box<dyn RelayMessageHandler>,
    pub resubscription_trigger: mpsc::Sender<()>,
    pub event_loop_handle: OnceCell<JoinHandle<()>>,
    pub sdk_listener_id: Mutex<Option<String>>,
}

impl RuntimeContext {
    pub async fn trigger_resubscription(&self) {
        let _ = self.resubscription_trigger.send(()).await;
    }

    pub async fn clear(&self) {
        if let Some(handle) = self.event_loop_handle.get() {
            handle.abort();
        }
        if let Some(ref listener_id) = *self.sdk_listener_id.lock().await {
            if let Some(sdk) = self.sdk.upgrade() {
                let _ = sdk.remove_event_listener(listener_id.clone()).await;
            }
        }
        self.client.disconnect().await;
        self.event_emitter
            .broadcast(SdkEvent::NWC {
                details: NwcEvent::DisconnectedHandled,
                event_id: "".to_string(),
            })
            .await;
    }

    pub async fn list_clients(&self) -> Result<HashMap<String, NostrWalletConnectURI>> {
        Ok(self
            .persister
            .list_nwc_uris()?
            .into_iter()
            .filter_map(|(name, uri)| {
                NostrWalletConnectURI::from_str(&uri)
                    .map(|uri| (name, uri))
                    .ok()
            })
            .collect())
    }

    pub async fn resubscribe(
        &self,
        clients: &HashMap<String, NostrWalletConnectURI>,
    ) -> Result<()> {
        let pubkeys = clients
            .values()
            .map(|uri| uri.public_key.to_string())
            .collect();
        self.client
            .subscribe(
                Filter {
                    generic_tags: BTreeMap::from([(
                        SingleLetterTag {
                            character: Alphabet::P,
                            uppercase: false,
                        },
                        pubkeys,
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
        let content = "pay_invoice list_transactions get_balance notifications".to_string();
        if let Err(err) = self
            .send_event(
                EventBuilder::new(Kind::WalletConnectInfo, content)
                    .tag(Tag::custom("encryption".into(), ["nip44_v2".to_string()])),
            )
            .await
        {
            warn!("Could not send info event to relay pool: {err:?}");
        }
    }
}
