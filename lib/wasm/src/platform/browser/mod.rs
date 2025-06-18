mod db_backup;

use crate::platform::browser::db_backup::IndexedDbBackupStorage;
use crate::platform::db_backup_common::BackupPersister;
use anyhow::Result;
use breez_sdk_liquid::model::{Config, LiquidNetwork, Signer};
use breez_sdk_liquid::persist::Persister;
use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
use breez_sdk_liquid::wallet::persister::WalletCachePersister;
use breez_sdk_liquid::wallet::{LiquidOnchainWallet, OnchainWallet};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) fn create_db_backup_persister(backup_dir_path: &Path) -> Result<BackupPersister> {
    let backup_storage = Rc::new(IndexedDbBackupStorage::new(
        &backup_dir_path.to_string_lossy(),
    ));
    Ok(BackupPersister::new(backup_storage))
}
