mod db_backup;
mod fs;

use crate::platform::db_backup_common::BackupPersister;
use crate::platform::node_js::db_backup::NodeFsBackupStorage;
use anyhow::Result;
use breez_sdk_liquid::model::LiquidNetwork;
use breez_sdk_liquid::model::{Config, Signer};
use breez_sdk_liquid::persist::Persister;
use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
use breez_sdk_liquid::wallet::persister::WalletCachePersister;
use std::path::Path;
use std::rc::Rc;

pub(crate) fn create_db_backup_persister(backup_dir_path: &Path) -> Result<BackupPersister> {
    let backup_storage = Rc::new(NodeFsBackupStorage::new(backup_dir_path));
    Ok(BackupPersister::new(backup_storage))
}
