pub(crate) mod indexed_db;
pub(crate) mod node_fs;

use anyhow::Result;
use breez_sdk_liquid::wallet::persister::lwk_wollet::{PersistError, Update};
use breez_sdk_liquid::wallet::persister::{lwk_wollet, LwkPersister, WalletCachePersister};
use log::info;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

#[sdk_macros::async_trait]
pub(crate) trait WalletStorage: Send + Sync + Clone + 'static {
    // Load all existing updates from the storage backend.
    async fn load_updates(&self) -> Result<Vec<Update>>;

    // Persist a single update at a given index.
    async fn persist_update(&self, update: Update, index: u32) -> Result<()>;

    // Clear all persisted data.
    async fn clear(&self) -> Result<()>;
}

#[derive(Clone)]
pub(crate) struct WasmWalletCachePersister<S: WalletStorage> {
    lwk_persister: Arc<WasmLwkPersister<S>>,
}

impl<S: WalletStorage> WasmWalletCachePersister<S> {
    pub async fn new(storage: Arc<S>) -> Result<Self> {
        Ok(Self {
            lwk_persister: Arc::new(WasmLwkPersister::new(storage).await?),
        })
    }
}

#[sdk_macros::async_trait]
impl<S: WalletStorage> WalletCachePersister for WasmWalletCachePersister<S> {
    fn get_lwk_persister(&self) -> LwkPersister {
        let persister = std::sync::Arc::clone(&self.lwk_persister);
        persister as LwkPersister
    }

    async fn clear_cache(&self) -> Result<()> {
        self.lwk_persister.storage.clear().await?;
        self.lwk_persister.updates.lock().unwrap().clear();
        Ok(())
    }
}

struct WasmLwkPersister<S: WalletStorage> {
    updates: Mutex<Vec<Update>>,
    sender: Sender<(Update, /*index*/ u32)>,
    storage: Arc<S>,
}

impl<S: WalletStorage> WasmLwkPersister<S> {
    async fn new(storage: Arc<S>) -> Result<Self> {
        let updates = storage.load_updates().await?;

        let (sender, receiver) = tokio::sync::mpsc::channel(20);

        Self::start_persist_task(storage.clone(), receiver);

        Ok(Self {
            updates: Mutex::new(updates),
            sender,
            storage,
        })
    }

    fn start_persist_task(storage: Arc<S>, mut receiver: Receiver<(Update, /*index*/ u32)>) {
        wasm_bindgen_futures::spawn_local(async move {
            // Persist updates and break on any error (giving up on cache persistence for the rest of the session)
            // A failed update followed by a successful one may leave the cache in an inconsistent state
            while let Some((update, index)) = receiver.recv().await {
                info!("Starting to persist wallet cache update");
                if let Err(e) = storage.persist_update(update, index).await {
                    log::error!("Failed to persist wallet cache update: {:?} - giving up on persisting wallet updates...", e);
                    break;
                }
            }
        });
    }
}

impl<S: WalletStorage> lwk_wollet::Persister for WasmLwkPersister<S> {
    fn get(&self, index: usize) -> std::result::Result<Option<Update>, PersistError> {
        Ok(self.updates.lock().unwrap().get(index).cloned())
    }

    fn push(&self, mut update: Update) -> std::result::Result<(), PersistError> {
        let mut updates = self.updates.lock().unwrap();
        let mut next_index = updates.len();

        if update.only_tip() {
            if let Some(prev_update) = updates.last() {
                if prev_update.only_tip() {
                    // If both updates are only tip updates, we can merge them.
                    // See https://github.com/Blockstream/lwk/blob/0322a63310f8c8414c537adff68dcbbc7ff4662d/lwk_wollet/src/persister.rs#L174
                    update.wollet_status = prev_update.wollet_status;
                    next_index -= 1;
                }
            }
        }

        self.sender
            .try_send((update.clone(), next_index as u32))
            .map_err(|e| {
                lwk_wollet::PersistError::Other(format!(
                    "Failed to send update to persister task: {e}"
                ))
            })?;

        if next_index < updates.len() {
            updates[next_index] = update;
        } else {
            updates.push(update);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet_persister::indexed_db::IndexedDbWalletStorage;
    use crate::wallet_persister::WasmWalletCachePersister;
    use breez_sdk_liquid::elements::hashes::Hash;
    use breez_sdk_liquid::elements::{BlockHash, BlockHeader, TxMerkleNode, Txid};
    use breez_sdk_liquid::wallet::persister::{lwk_wollet, WalletCachePersister};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio_with_wasm::alias as tokio;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_wasm]
    async fn test_wallet_cache() -> anyhow::Result<()> {
        let wallet_storage = Arc::new(IndexedDbWalletStorage::new("test".to_string()));
        let persister = WasmWalletCachePersister::new(wallet_storage).await?;
        let lwk_persister = persister.get_lwk_persister();

        assert!(lwk_persister.get(0)?.is_none());

        lwk_persister.push(get_lwk_update(5, false))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert!(lwk_persister.get(1)?.is_none());

        lwk_persister.push(get_lwk_update(10, true))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 10);
        assert!(lwk_persister.get(2)?.is_none());

        lwk_persister.push(get_lwk_update(15, true))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 15);
        assert!(lwk_persister.get(2)?.is_none());

        // Allow persister task to persist updates
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Reload persister
        let wallet_storage = Arc::new(IndexedDbWalletStorage::new("test".to_string()));
        let persister = WasmWalletCachePersister::new(wallet_storage).await?;
        let lwk_persister = persister.get_lwk_persister();

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 15);
        assert!(lwk_persister.get(2)?.is_none());

        persister.clear_cache().await?;
        assert!(lwk_persister.get(0)?.is_none());
        assert!(lwk_persister.get(1)?.is_none());
        assert!(lwk_persister.get(2)?.is_none());

        Ok(())
    }

    fn get_lwk_update(height: u32, only_tip: bool) -> lwk_wollet::Update {
        let txid_height_new = match only_tip {
            true => Vec::new(),
            false => {
                vec![(Txid::all_zeros(), None)]
            }
        };
        lwk_wollet::Update {
            version: 1,
            wollet_status: 0,
            new_txs: Default::default(),
            txid_height_new,
            txid_height_delete: vec![],
            timestamps: vec![],
            scripts_with_blinding_pubkey: vec![],
            tip: BlockHeader {
                version: 0,
                prev_blockhash: BlockHash::all_zeros(),
                merkle_root: TxMerkleNode::all_zeros(),
                time: 0,
                height,
                ext: Default::default(),
            },
        }
    }
}
