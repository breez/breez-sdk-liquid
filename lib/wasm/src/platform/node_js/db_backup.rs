use super::fs::{ensure_dir_exists, exists_sync, read_file_vec, write_file_vec};
use crate::platform::db_backup_common::BackupStorage;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) struct NodeFsBackupStorage {
    backup_dir_path: PathBuf,
}

impl NodeFsBackupStorage {
    pub fn new<P: AsRef<Path>>(backup_dir_path: P) -> Self {
        Self {
            backup_dir_path: backup_dir_path.as_ref().to_path_buf(),
        }
    }
}

#[sdk_macros::async_trait]
impl BackupStorage for NodeFsBackupStorage {
    async fn backup(&self, bytes: &[u8]) -> Result<()> {
        ensure_dir_exists(&self.backup_dir_path.to_string_lossy())
            .map_err(|e| anyhow::anyhow!("Failed to create backup directory: {:?}", e))?;

        let backup_file_path = get_backup_file_path(&self.backup_dir_path);

        write_file_vec(&backup_file_path.to_string_lossy(), bytes)?;
        Ok(())
    }

    async fn load(&self) -> Result<Option<Vec<u8>>> {
        let backup_file_path = get_backup_file_path(&self.backup_dir_path);
        let backup_file_path_string = backup_file_path.to_string_lossy();
        if !exists_sync(&backup_file_path_string) {
            log::debug!("Backup file '{backup_file_path_string:?}' not found.");
            return Ok(None);
        }
        log::debug!("Backup file '{backup_file_path_string:?}' found, attempting to read.",);

        Ok(Some(read_file_vec(&backup_file_path_string)?))
    }
}

fn get_backup_file_path(backup_dir_path: &Path) -> PathBuf {
    backup_dir_path.join("backup.sql")
}
