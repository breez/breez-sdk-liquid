use anyhow::Result;
use breez_sdk_liquid::model::{EventListener, SdkEvent};
use breez_sdk_liquid::persist::Persister;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) struct ForwardingEventListener {
    sender: Sender<SdkEvent>,
}

impl ForwardingEventListener {
    pub fn new(sender: Sender<SdkEvent>) -> Self {
        Self { sender }
    }
}

#[sdk_macros::async_trait]
impl EventListener for ForwardingEventListener {
    async fn on_event(&self, e: SdkEvent) {
        if let Err(e) = self.sender.try_send(e) {
            log::error!("Failed to forward event: {e:?}");
        }
    }
}

#[sdk_macros::async_trait]
pub(crate) trait BackupStorage {
    async fn backup(&self, bytes: &[u8]) -> Result<()>;

    async fn load(&self) -> Result<Option<Vec<u8>>>;
}

pub(crate) struct BackupPersister {
    storage: Rc<dyn BackupStorage>,
}

impl BackupPersister {
    pub fn new(storage: Rc<dyn BackupStorage>) -> Self {
        Self { storage }
    }

    pub(crate) fn start_backup_task(
        &self,
        persister: Arc<Persister>,
        mut receiver: Receiver<SdkEvent>,
    ) {
        let storage = self.storage.clone();
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(e) = receiver.recv().await {
                let start = web_time::Instant::now();

                let bytes = match persister.serialize() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        log::error!("Failed to serialize persister: {e:?}");
                        continue;
                    }
                };

                let res = match e {
                    SdkEvent::Synced => storage.backup(&bytes).await,
                    SdkEvent::DataSynced {
                        did_pull_new_records,
                    } if did_pull_new_records => storage.backup(&bytes).await,
                    _ => continue,
                };
                if let Err(e) = res {
                    log::error!("Failed to backup to IndexedDB: {e:?}");
                };

                let backup_duration_ms = start.elapsed().as_millis();
                log::info!("Backup completed successfully ({backup_duration_ms} ms)");
            }
        });
    }

    pub(crate) async fn load_backup(&self) -> Result<Option<Vec<u8>>> {
        self.storage.load().await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::platform::create_db_backup_persister;
    use breez_sdk_liquid::model::PaymentState;
    use breez_sdk_liquid::persist::Persister;
    use breez_sdk_liquid::prelude::LiquidNetwork;
    use breez_sdk_liquid::test_utils::persist::{
        create_persister, new_receive_swap, new_send_swap,
    };

    #[cfg(feature = "browser")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_wasm]
    async fn test_backup_and_restore() -> anyhow::Result<()> {
        create_persister!(local);

        let backup_dir_path = PathBuf::from_str(&format!("/tmp/{}", uuid::Uuid::new_v4()))?;
        let backup_persister = create_db_backup_persister(&backup_dir_path)?;

        local.test_insert_or_update_send_swap(&new_send_swap(Some(PaymentState::Pending), None))?;
        local.test_insert_or_update_receive_swap(&new_receive_swap(
            Some(PaymentState::Pending),
            None,
        ))?;
        assert_eq!(local.test_list_ongoing_swaps()?.len(), 2);

        backup_persister.storage.backup(&local.serialize()?).await?;

        let backup_bytes = backup_persister.load_backup().await?;
        let remote =
            Persister::new_in_memory("remote", LiquidNetwork::Testnet, false, None, backup_bytes)?;
        assert_eq!(remote.test_list_ongoing_swaps()?.len(), 2);

        // Try again to verify that a new backup overwrites an old one
        local.test_insert_or_update_send_swap(&new_send_swap(Some(PaymentState::Pending), None))?;
        local.test_insert_or_update_receive_swap(&new_receive_swap(
            Some(PaymentState::Pending),
            None,
        ))?;
        assert_eq!(local.test_list_ongoing_swaps()?.len(), 4);

        backup_persister.storage.backup(&local.serialize()?).await?;

        let backup_bytes = backup_persister.load_backup().await?;
        let remote =
            Persister::new_in_memory("remote", LiquidNetwork::Testnet, false, None, backup_bytes)?;
        assert_eq!(remote.test_list_ongoing_swaps()?.len(), 4);

        Ok(())
    }
}
