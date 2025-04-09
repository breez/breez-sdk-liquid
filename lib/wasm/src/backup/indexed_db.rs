#![cfg(feature = "browser")]

use crate::backup::BackupStorage;
use anyhow::Result;
use indexed_db_futures::{
    database::Database, query_source::QuerySource, transaction::TransactionMode, Build,
};

const IDB_STORE_NAME: &str = "BREEZ_SDK_LIQUID_DB_BACKUP_STORE";

pub(crate) struct IndexedDbBackupStorage {
    db_name: String,
}

impl IndexedDbBackupStorage {
    pub fn new(backup_dir_path: &str) -> Self {
        let db_name = format!("{}-db-backup", backup_dir_path);
        Self { db_name }
    }
}

#[sdk_macros::async_trait]
impl BackupStorage for IndexedDbBackupStorage {
    async fn backup(&self, bytes: &[u8]) -> Result<()> {
        let idb = open_indexed_db(&self.db_name).await?;
        let tx = idb
            .transaction([IDB_STORE_NAME])
            .with_mode(TransactionMode::Readwrite)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build transaction: {}", e))?;

        let store = tx
            .object_store(IDB_STORE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to open object store: {}", e))?;

        store
            .put(bytes)
            .with_key(1)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to put key in db: {}", e))?;

        tx.commit()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to commit transaction: {}", e))?;

        Ok(())
    }

    async fn load(&self) -> Result<Option<Vec<u8>>> {
        let idb = open_indexed_db(&self.db_name).await?;

        let tx = idb
            .transaction([IDB_STORE_NAME])
            .with_mode(TransactionMode::Readonly)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build transaction: {}", e))?;

        let store = tx
            .object_store(IDB_STORE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to open object store: {}", e))?;

        store
            .get(1)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get data: {}", e))
    }
}

pub(crate) async fn open_indexed_db(name: &str) -> Result<Database> {
    let db = Database::open(name)
        .with_version(1u32)
        .with_on_upgrade_needed(|event, db| {
            if let (0.0, Some(1.0)) = (event.old_version(), event.new_version()) {
                db.create_object_store(IDB_STORE_NAME).build()?;
            }

            Ok(())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to open IndexedDB: {}", e))?;
    Ok(db)
}
