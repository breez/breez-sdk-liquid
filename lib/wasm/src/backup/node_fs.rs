#![cfg(feature = "node-js")]

use crate::utils::node::{ensure_dir_exists, exists_sync, read_file_vec, write_file_vec};

use anyhow::Result;
use std::path::{Path, PathBuf};

fn get_backup_file_path(backup_dir_path: &Path) -> PathBuf {
    backup_dir_path.join("backup.sql")
}

pub(crate) fn backup_to_file_system(db_bytes: Vec<u8>, backup_dir_path: &Path) -> Result<()> {
    ensure_dir_exists(&backup_dir_path.to_string_lossy())
        .map_err(|e| anyhow::anyhow!("Failed to create backup directory: {:?}", e))?;

    let backup_file_path = get_backup_file_path(backup_dir_path);

    write_file_vec(&backup_file_path.to_string_lossy(), &db_bytes)?;
    Ok(())
}

pub(crate) fn load_file_system_backup(backup_dir_path: &Path) -> Result<Option<Vec<u8>>> {
    let backup_file_path = get_backup_file_path(backup_dir_path);
    let backup_file_path_string = backup_file_path.to_string_lossy();
    if !exists_sync(&backup_file_path_string) {
        log::debug!("Backup file '{backup_file_path_string:?}' not found.");
        return Ok(None);
    }
    log::debug!("Backup file '{backup_file_path_string:?}' found, attempting to read.",);

    Ok(Some(read_file_vec(&backup_file_path_string)?))
}
