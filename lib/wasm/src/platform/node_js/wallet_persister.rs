use super::fs::{
    ensure_dir_exists, exists_sync, read_file_vec, readdir_sync, remove_dir_all_sync,
    write_file_vec,
};
use crate::platform::wallet_persister_common::maybe_merge_updates;
use breez_sdk_liquid::wallet::persister::lwk_wollet::{
    ElementsNetwork, PersistError, Update, WolletDescriptor,
};
use breez_sdk_liquid::wallet::persister::{lwk_wollet, LwkPersister, WalletCachePersister};
use js_sys::Array;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub(crate) struct NodeFsWalletCachePersister {
    cache_dir: String,
    lwk_persister: Arc<NodeFsLwkPersister>,
}

impl NodeFsWalletCachePersister {
    pub fn new<P: AsRef<Path>>(
        path: P,
        network: ElementsNetwork,
        fingerprint: &str,
        desc: WolletDescriptor,
    ) -> anyhow::Result<Self> {
        let mut cache_dir_path = path.as_ref().to_path_buf();
        cache_dir_path.push(network.as_str());
        cache_dir_path.push("enc_cache");
        cache_dir_path.push(fingerprint);
        let cache_dir = cache_dir_path.to_string_lossy().to_string();

        ensure_dir_exists(&cache_dir)?;

        Ok(Self {
            cache_dir,
            lwk_persister: Arc::new(NodeFsLwkPersister::new(cache_dir_path, desc)),
        })
    }
}

#[sdk_macros::async_trait]
impl WalletCachePersister for NodeFsWalletCachePersister {
    fn get_lwk_persister(&self) -> anyhow::Result<LwkPersister> {
        let persister = Arc::clone(&self.lwk_persister);
        Ok(persister as LwkPersister)
    }

    async fn clear_cache(&self) -> anyhow::Result<()> {
        log::debug!("Clearing lwk wallet cache directory: {}", self.cache_dir);
        remove_dir_all_sync(&self.cache_dir)?;
        log::info!(
            "Successfully cleared lwk wallet cache directory: {}",
            self.cache_dir
        );
        ensure_dir_exists(&self.cache_dir)?;
        *self.lwk_persister.next_index.lock().unwrap() = 0;
        Ok(())
    }
}

struct NodeFsLwkPersister {
    cache_dir: PathBuf,
    next_index: Mutex<usize>,
    desc: WolletDescriptor,
}

impl NodeFsLwkPersister {
    fn new(cache_dir: PathBuf, desc: WolletDescriptor) -> Self {
        let initial_index = {
            let entries =
                readdir_sync(&cache_dir.to_string_lossy()).unwrap_or_else(|_| Array::new());
            let mut max_index: Option<usize> = None;

            for entry in entries.iter() {
                if let Some(name) = entry.as_string() {
                    if let Ok(index) = name.parse::<usize>() {
                        max_index = Some(max_index.map_or(index, |max| max.max(index)));
                    }
                }
            }
            max_index.map_or(0, |max| max + 1)
        };

        Self {
            cache_dir,
            next_index: Mutex::new(initial_index),
            desc,
        }
    }

    fn get_update_file_path(&self, index: usize) -> String {
        self.cache_dir
            .join(index.to_string())
            .to_string_lossy()
            .to_string()
    }
}

impl lwk_wollet::Persister for NodeFsLwkPersister {
    fn get(&self, index: usize) -> Result<Option<Update>, PersistError> {
        let file_path = self.get_update_file_path(index);
        if !exists_sync(&file_path) {
            log::trace!("Update file not found: {}", file_path);
            return Ok(None);
        }

        log::debug!("Reading update file: {}", file_path);
        let bytes = read_file_vec(&file_path).map_err(to_persist_error)?;

        log::debug!("Deserializing update from file: {}", file_path);
        let update = Update::deserialize_decrypted(&bytes, &self.desc).map_err(to_persist_error)?;
        Ok(Some(update))
    }

    fn push(&self, update: Update) -> Result<(), PersistError> {
        let mut next_index_guard = self.next_index.lock().unwrap();
        let next_index = *next_index_guard;

        let prev_update = if next_index == 0 {
            None
        } else {
            self.get(next_index - 1).unwrap_or(None)
        };
        let (update, write_index) = maybe_merge_updates(update, prev_update.as_ref(), next_index);

        let file_path = self.get_update_file_path(write_index);
        log::debug!("Serializing and writing update to: {}", file_path);
        let bytes = update
            .serialize_encrypted(&self.desc)
            .map_err(to_persist_error)?;

        write_file_vec(&file_path, &bytes).map_err(to_persist_error)?;

        if write_index == *next_index_guard {
            *next_index_guard += 1;
            log::info!(
                "Successfully pushed wallet cache update to index {}",
                write_index
            );
        } else {
            log::info!(
                "Successfully overwrote tip-only wallet cache update at index {}",
                write_index
            );
        }

        Ok(())
    }
}

fn to_persist_error<E: std::fmt::Debug>(e: E) -> PersistError {
    PersistError::Other(format!("{:?}", e))
}
