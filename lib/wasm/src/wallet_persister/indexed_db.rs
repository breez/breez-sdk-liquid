use anyhow::{anyhow, Context};
use breez_sdk_liquid::wallet::persister::lwk_wollet;
use breez_sdk_liquid::wallet::persister::{LwkPersister, WalletCachePersister};
use indexed_db_futures::database::Database;
use indexed_db_futures::query_source::QuerySource;
use indexed_db_futures::transaction::TransactionMode;
use indexed_db_futures::Build;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

const IDB_STORE_NAME: &str = "BREEZ_SDK_LIQUID_WALLET_CACHE_STORE";

#[derive(Clone)]
pub(crate) struct IndexedDbWalletCachePersister {
    db_name: String,
    lwk_persister: Arc<IndexedDbPersister>,
}

impl IndexedDbWalletCachePersister {
    pub async fn new(db_name: String) -> anyhow::Result<Self> {
        let lwk_persister = Arc::new(IndexedDbPersister::new(db_name.clone()).await?);
        Ok(Self {
            db_name,
            lwk_persister,
        })
    }
}

#[sdk_macros::async_trait]
impl WalletCachePersister for IndexedDbWalletCachePersister {
    fn get_lwk_persister(&self) -> LwkPersister {
        let persister = std::sync::Arc::clone(&self.lwk_persister);
        persister as LwkPersister
    }

    async fn clear_cache(&self) -> anyhow::Result<()> {
        let idb = open_indexed_db(&self.db_name).await?;

        let tx = idb
            .transaction([IDB_STORE_NAME])
            .with_mode(TransactionMode::Readwrite)
            .build()
            .map_err(|e| anyhow!("Failed to build transaction: {}", e))?;

        let store = tx
            .object_store(IDB_STORE_NAME)
            .map_err(|e| anyhow!("Failed to open object store: {}", e))?;

        store
            .clear()
            .map_err(|e| anyhow!("Failed to clear object store: {}", e))?;

        tx.commit().await.map_err(|e| {
            lwk_wollet::PersistError::Other(format!("Failed to commit transaction: {}", e))
        })?;

        self.lwk_persister.updates.lock().unwrap().clear();

        Ok(())
    }
}

struct IndexedDbPersister {
    updates: Mutex<Vec<lwk_wollet::Update>>,
    sender: Sender<(lwk_wollet::Update, /*index*/ u32)>,
}

impl IndexedDbPersister {
    pub async fn new(db_name: String) -> anyhow::Result<Self> {
        let updates = Self::load_updates(&db_name).await?;
        info!(
            "Loaded {} wallet cache updates from IndexedDB",
            updates.len()
        );

        let (sender, receiver) = tokio::sync::mpsc::channel(20);

        Self::start_persist_task(db_name, receiver);

        Ok(Self {
            updates: Mutex::new(updates),
            sender,
        })
    }

    async fn load_updates(db_name: &str) -> anyhow::Result<Vec<lwk_wollet::Update>> {
        let idb = open_indexed_db(db_name).await?;

        let tx = idb
            .transaction([IDB_STORE_NAME])
            .with_mode(TransactionMode::Readonly)
            .build()
            .map_err(|e| anyhow!("Failed to build transaction: {}", e))?;

        let store = tx
            .object_store(IDB_STORE_NAME)
            .map_err(|e| anyhow!("Failed to open object store: {}", e))?;

        let updates_count = store
            .count()
            .await
            .map_err(|e| anyhow!("Failed to get next index: {}", e))?;

        let mut updates = Vec::new();
        for i in 0..updates_count {
            let update_bytes: Vec<u8> = store
                .get(i)
                .await
                .map_err(|e| anyhow!("Failed to get update bytes: {}", e))?
                .ok_or(anyhow!("Missing update on index {i}"))?;
            updates.push(
                lwk_wollet::Update::deserialize(&update_bytes)
                    .context("Failed to deserialize update")?,
            );
        }

        Ok(updates)
    }

    fn start_persist_task(
        db_name: String,
        mut receiver: Receiver<(lwk_wollet::Update, /*index*/ u32)>,
    ) {
        wasm_bindgen_futures::spawn_local(async move {
            // Persist updates to IndexedDb and break on any error (giving up on cache persistence for the rest of the session)
            // A failed update followed by a successful one may leave the cache in an inconsistent state
            while let Some((update, index)) = receiver.recv().await {
                info!("Starting to persist wallet cache update");
                let update_bytes = match update.serialize() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!("Failed to serialize update: {}", e);
                        break;
                    }
                };

                let idb = match open_indexed_db(&db_name).await {
                    Ok(db) => db,
                    Err(e) => {
                        error!("Failed to open IndexedDB: {}", e);
                        break;
                    }
                };

                let tx = match idb
                    .transaction([IDB_STORE_NAME])
                    .with_mode(TransactionMode::Readwrite)
                    .build()
                {
                    Ok(tx) => tx,
                    Err(e) => {
                        error!("Failed to build transaction: {}", e);
                        break;
                    }
                };

                let store = match tx.object_store(IDB_STORE_NAME) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to open object store: {}", e);
                        break;
                    }
                };

                if let Err(e) = store.put(update_bytes).with_key(index).await {
                    error!("Failed to put update in store: {}", e);
                    break;
                }
                if let Err(e) = tx.commit().await {
                    error!("Failed to commit transaction: {}", e);
                    break;
                }
            }
        });
    }
}

impl lwk_wollet::Persister for IndexedDbPersister {
    fn get(&self, index: usize) -> Result<Option<lwk_wollet::Update>, lwk_wollet::PersistError> {
        Ok(self.updates.lock().unwrap().get(index).cloned())
    }

    fn push(&self, mut update: lwk_wollet::Update) -> Result<(), lwk_wollet::PersistError> {
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

pub(crate) async fn open_indexed_db(name: &str) -> Result<Database, lwk_wollet::PersistError> {
    let db = Database::open(name)
        .with_version(1u32)
        .with_on_upgrade_needed(|event, db| {
            if let (0.0, Some(1.0)) = (event.old_version(), event.new_version()) {
                db.create_object_store(IDB_STORE_NAME).build()?;
            }

            Ok(())
        })
        .await
        .map_err(|e| lwk_wollet::PersistError::Other(format!("Failed to open IndexedDB: {}", e)))?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use crate::wallet_persister::indexed_db::IndexedDbWalletCachePersister;
    use breez_sdk_liquid::elements::hashes::Hash;
    use breez_sdk_liquid::elements::{BlockHash, BlockHeader, TxMerkleNode, Txid};
    use breez_sdk_liquid::wallet::persister::{lwk_wollet, WalletCachePersister};
    use std::time::Duration;
    use tokio_with_wasm::alias as tokio;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_wasm]
    async fn test_indexed_db_wallet_cache() -> anyhow::Result<()> {
        let persister = IndexedDbWalletCachePersister::new("test".to_string()).await?;
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
        let persister = IndexedDbWalletCachePersister::new("test".to_string()).await?;
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
