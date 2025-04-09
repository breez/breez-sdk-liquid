mod indexed_db;
mod node_fs;

use crate::utils::is_indexed_db_supported;
use anyhow::Result;
use breez_sdk_liquid::model::{EventListener, SdkEvent};
use breez_sdk_liquid::persist::Persister;
use indexed_db::{backup_to_indexed_db, load_indexed_db_backup};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) struct ForwardingEventListener {
    sender: Sender<SdkEvent>,
}

impl ForwardingEventListener {
    pub fn new(sender: Sender<SdkEvent>) -> Self {
        Self { sender }
    }
}

impl EventListener for ForwardingEventListener {
    fn on_event(&self, e: SdkEvent) {
        if let Err(e) = self.sender.try_send(e) {
            log::error!("Failed to forward event: {:?}", e);
        }
    }
}

pub(crate) fn start_backup_task(
    persister: Rc<Persister>,
    mut receiver: Receiver<SdkEvent>,
    backup_dir_path: PathBuf,
) {
    wasm_bindgen_futures::spawn_local(async move {
        while let Some(e) = receiver.recv().await {
            let res = match e {
                SdkEvent::Synced => backup(&persister, &backup_dir_path).await,
                SdkEvent::DataSynced {
                    did_pull_new_records,
                } if did_pull_new_records => backup(&persister, &backup_dir_path).await,
                _ => continue,
            };
            if let Err(e) = res {
                log::error!("Failed to backup to IndexedDB: {:?}", e);
            };
        }
    });
}

async fn backup(persister: &Rc<Persister>, backup_dir_path: &Path) -> Result<()> {
    let start = web_time::Instant::now();

    let db_bytes = persister.serialize()?;

    if is_indexed_db_supported() {
        backup_to_indexed_db(db_bytes, &get_backup_indexed_db_name(backup_dir_path)).await?;
    } else {
        #[cfg(not(feature = "node-js"))]
        return Err(anyhow::anyhow!("No backup mechanism available"));
        #[cfg(feature = "node-js")]
        node_fs::backup_to_file_system(db_bytes, backup_dir_path)?;
    }

    let backup_duration_ms = start.elapsed().as_millis();
    log::info!("Backup completed successfully ({backup_duration_ms} ms)");
    Ok(())
}

pub(crate) async fn load_backup(backup_dir_path: &Path) -> Result<Option<Vec<u8>>> {
    let maybe_data = if is_indexed_db_supported() {
        load_indexed_db_backup(&get_backup_indexed_db_name(backup_dir_path)).await?
    } else {
        #[cfg(not(feature = "node-js"))]
        return Err(anyhow::anyhow!("No backup restore mechanism available"));
        #[cfg(feature = "node-js")]
        node_fs::load_file_system_backup(backup_dir_path)?
    };
    Ok(maybe_data)
}

fn get_backup_indexed_db_name(backup_dir_path: &Path) -> String {
    format!("{}-db-backup", backup_dir_path.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use crate::backup::backup;
    use crate::backup::load_backup;
    use std::path::PathBuf;
    use std::str::FromStr;

    use breez_sdk_liquid::model::PaymentState;
    use breez_sdk_liquid::persist::Persister;
    use breez_sdk_liquid::prelude::LiquidNetwork;
    use breez_sdk_liquid::test_utils::persist::{
        create_persister, new_receive_swap, new_send_swap,
    };

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_wasm]
    async fn test_backup_and_restore() -> anyhow::Result<()> {
        create_persister!(local);

        local.test_insert_or_update_send_swap(&new_send_swap(Some(PaymentState::Pending), None))?;
        local.test_insert_or_update_receive_swap(&new_receive_swap(
            Some(PaymentState::Pending),
            None,
        ))?;
        assert_eq!(local.test_list_ongoing_swaps()?.len(), 2);

        let backup_dir_path = PathBuf::from_str(&format!("/tmp/{}", uuid::Uuid::new_v4()))?;
        backup(&local, &backup_dir_path).await?;

        let backup_bytes = load_backup(&backup_dir_path).await?;
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

        backup(&local, &backup_dir_path).await?;

        let backup_bytes = load_backup(&backup_dir_path).await?;
        let remote =
            Persister::new_in_memory("remote", LiquidNetwork::Testnet, false, None, backup_bytes)?;
        assert_eq!(remote.test_list_ongoing_swaps()?.len(), 4);

        Ok(())
    }
}
