use crate::wallet_persister::AsyncWalletStorage;
use anyhow::{anyhow, Context};
use breez_sdk_liquid::wallet::persister::lwk_wollet;
use breez_sdk_liquid::wallet::persister::lwk_wollet::{Update, WolletDescriptor};
use indexed_db_futures::database::Database;
use indexed_db_futures::query_source::QuerySource;
use indexed_db_futures::transaction::TransactionMode;
use indexed_db_futures::Build;
use std::path::Path;

const IDB_STORE_NAME: &str = "BREEZ_SDK_LIQUID_WALLET_CACHE_STORE";

#[derive(Clone)]
pub(crate) struct IndexedDbWalletStorage {
    db_name: String,
    desc: WolletDescriptor,
}

impl IndexedDbWalletStorage {
    pub fn new(working_dir: &Path, desc: WolletDescriptor) -> Self {
        let db_name = format!("{}-wallet-cache", working_dir.to_string_lossy());
        Self { db_name, desc }
    }
}

#[sdk_macros::async_trait]
impl AsyncWalletStorage for IndexedDbWalletStorage {
    async fn load_updates(&self) -> anyhow::Result<Vec<Update>> {
        let idb = open_indexed_db(&self.db_name).await?;

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
                Update::deserialize_decrypted(&update_bytes, &self.desc)
                    .context("Failed to deserialize update")?,
            );
        }

        Ok(updates)
    }

    async fn persist_update(&self, update: Update, index: u32) -> anyhow::Result<()> {
        let update_bytes = update
            .serialize_encrypted(&self.desc)
            .map_err(|e| anyhow!("Failed to serialize update: {e}"))?;

        let idb = open_indexed_db(&self.db_name)
            .await
            .map_err(|e| anyhow!("Failed to open IndexedDB: {e}"))?;

        let tx = idb
            .transaction([IDB_STORE_NAME])
            .with_mode(TransactionMode::Readwrite)
            .build()
            .map_err(|e| anyhow!("Failed to build transaction: {e}"))?;

        let store = tx
            .object_store(IDB_STORE_NAME)
            .map_err(|e| anyhow!("Failed to open object store: {e}"))?;

        store
            .put(update_bytes)
            .with_key(index)
            .await
            .map_err(|e| anyhow!("Failed to put update in store: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| anyhow!("Failed to commit transaction: {e}"))?;

        Ok(())
    }

    async fn clear(&self) -> anyhow::Result<()> {
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
