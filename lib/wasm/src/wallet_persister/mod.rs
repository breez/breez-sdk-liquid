pub(crate) mod indexed_db;
pub(crate) mod node_fs;

use anyhow::Result;
use breez_sdk_liquid::wallet::persister::lwk_wollet::{PersistError, Update};
use breez_sdk_liquid::wallet::persister::{lwk_wollet, LwkPersister, WalletCachePersister};
use log::info;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

#[sdk_macros::async_trait]
pub(crate) trait AsyncWalletStorage: Send + Sync + Clone + 'static {
    // Load all existing updates from the storage backend.
    async fn load_updates(&self) -> Result<Vec<Update>>;

    // Persist a single update at a given index.
    async fn persist_update(&self, update: Update, index: u32) -> Result<()>;

    // Clear all persisted data.
    async fn clear(&self) -> Result<()>;
}

#[derive(Clone)]
pub(crate) struct AsyncWalletCachePersister<S: AsyncWalletStorage> {
    lwk_persister: Arc<AsyncLwkPersister<S>>,
}

impl<S: AsyncWalletStorage> AsyncWalletCachePersister<S> {
    pub async fn new(storage: Arc<S>) -> Result<Self> {
        Ok(Self {
            lwk_persister: Arc::new(AsyncLwkPersister::new(storage).await?),
        })
    }
}

#[sdk_macros::async_trait]
impl<S: AsyncWalletStorage> WalletCachePersister for AsyncWalletCachePersister<S> {
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

struct AsyncLwkPersister<S: AsyncWalletStorage> {
    updates: Mutex<Vec<Update>>,
    sender: Sender<(Update, /*index*/ u32)>,
    storage: Arc<S>,
}

impl<S: AsyncWalletStorage> AsyncLwkPersister<S> {
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
                info!("Starting to persist wallet cache update at index {}", index);
                if let Err(e) = storage.persist_update(update, index).await {
                    log::error!("Failed to persist wallet cache update: {:?} - giving up on persisting wallet updates...", e);
                    break;
                }
            }
        });
    }
}

impl<S: AsyncWalletStorage> lwk_wollet::Persister for AsyncLwkPersister<S> {
    fn get(&self, index: usize) -> std::result::Result<Option<Update>, PersistError> {
        Ok(self.updates.lock().unwrap().get(index).cloned())
    }

    fn push(&self, update: Update) -> std::result::Result<(), PersistError> {
        let mut updates = self.updates.lock().unwrap();

        let (update, write_index) = maybe_merge_updates(update, updates.last(), updates.len());

        self.sender
            .try_send((update.clone(), write_index as u32))
            .map_err(|e| {
                lwk_wollet::PersistError::Other(format!(
                    "Failed to send update to persister task: {e}"
                ))
            })?;

        if write_index < updates.len() {
            updates[write_index] = update;
        } else {
            updates.push(update);
        }

        Ok(())
    }
}

// If both updates are only tip updates, we can merge them.
// See https://github.com/Blockstream/lwk/blob/0322a63310f8c8414c537adff68dcbbc7ff4662d/lwk_wollet/src/persister.rs#L174
fn maybe_merge_updates(
    mut new_update: Update,
    prev_update: Option<&Update>,
    mut next_index: usize,
) -> (Update, /*index*/ usize) {
    if new_update.only_tip() {
        if let Some(prev_update) = prev_update {
            if prev_update.only_tip() {
                new_update.wollet_status = prev_update.wollet_status;
                next_index -= 1;
            }
        }
    }
    (new_update, next_index)
}

#[cfg(test)]
mod tests {
    use crate::utils::is_indexed_db_supported;
    use crate::wallet_persister::indexed_db::IndexedDbWalletStorage;
    use crate::wallet_persister::AsyncWalletCachePersister;
    use breez_sdk_liquid::elements::hashes::Hash;
    use breez_sdk_liquid::elements::{BlockHash, BlockHeader, TxMerkleNode, Txid};
    use breez_sdk_liquid::model::{LiquidNetwork, Signer};
    use breez_sdk_liquid::signer::{SdkLwkSigner, SdkSigner};
    use breez_sdk_liquid::wallet::get_descriptor;
    use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
    use breez_sdk_liquid::wallet::persister::{lwk_wollet, WalletCachePersister};
    use std::future::Future;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio_with_wasm::alias as tokio;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn get_wollet_descriptor() -> anyhow::Result<WolletDescriptor> {
        let signer: Rc<Box<dyn Signer>> = Rc::new(Box::new(SdkSigner::new("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", "", false)?));
        let sdk_lwk_signer = SdkLwkSigner::new(signer)?;
        Ok(get_descriptor(&sdk_lwk_signer, LiquidNetwork::Testnet)?)
    }

    #[sdk_macros::async_test_wasm]
    async fn test_wallet_cache_indexed_db() -> anyhow::Result<()> {
        let desc = get_wollet_descriptor()?;
        if is_indexed_db_supported() {
            let build_persister = || async {
                let wallet_storage = Arc::new(IndexedDbWalletStorage::new(
                    &PathBuf::from("test"),
                    desc.clone(),
                ));
                AsyncWalletCachePersister::new(wallet_storage)
                    .await
                    .unwrap()
            };

            test_wallet_cache(build_persister).await?;
        }

        Ok(())
    }

    #[cfg(feature = "node-js")]
    #[sdk_macros::async_test_wasm]
    async fn test_wallet_cache_node_fs() -> anyhow::Result<()> {
        let desc = get_wollet_descriptor()?;
        let working_dir = format!("/tmp/{}", uuid::Uuid::new_v4());
        let build_persister = || async {
            crate::wallet_persister::node_fs::NodeFsWalletCachePersister::new(
                working_dir.clone(),
                lwk_wollet::ElementsNetwork::Liquid,
                "test",
                desc.clone(),
            )
            .unwrap()
        };

        test_wallet_cache(build_persister).await?;

        Ok(())
    }

    async fn test_wallet_cache<F, Fut, WP>(build_persister: F) -> anyhow::Result<()>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = WP>,
        WP: WalletCachePersister,
    {
        let persister = build_persister().await;
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

        // Allow persister task to persist updates when persister is async
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Reload persister
        let persister = build_persister().await;
        let lwk_persister = persister.get_lwk_persister();

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 15);
        assert!(lwk_persister.get(2)?.is_none());

        persister.clear_cache().await?;
        assert!(lwk_persister.get(0)?.is_none());
        assert!(lwk_persister.get(1)?.is_none());
        assert!(lwk_persister.get(2)?.is_none());

        lwk_persister.push(get_lwk_update(20, false))?;
        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 20);
        assert!(lwk_persister.get(1)?.is_none());

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
