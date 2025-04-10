mod db_backup;
mod fs;
mod wallet_persister;

use crate::platform::db_backup_common::BackupPersister;
use crate::platform::node_js::db_backup::NodeFsBackupStorage;
use crate::platform::node_js::wallet_persister::NodeFsWalletCachePersister;
use anyhow::Result;
use breez_sdk_liquid::model::LiquidNetwork;
use breez_sdk_liquid::model::{Config, Signer};
use breez_sdk_liquid::persist::Persister;
use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
use breez_sdk_liquid::wallet::persister::WalletCachePersister;
use breez_sdk_liquid::wallet::{LiquidOnchainWallet, OnchainWallet};
use std::path::Path;
use std::rc::Rc;

pub(crate) async fn create_wallet_persister(
    wallet_dir: &Path,
    descriptor: WolletDescriptor,
    network: LiquidNetwork,
    fingerprint: &str,
) -> Result<Rc<dyn WalletCachePersister>> {
    let wallet_persister: Rc<dyn WalletCachePersister> = Rc::new(NodeFsWalletCachePersister::new(
        wallet_dir,
        network.into(),
        fingerprint,
        descriptor,
    )?);
    Ok(wallet_persister)
}

pub(crate) async fn create_onchain_wallet(
    wallet_dir: &Path,
    config: Config,
    descriptor: WolletDescriptor,
    fingerprint: &str,
    persister: Rc<Persister>,
    signer: Rc<Box<dyn Signer>>,
) -> Result<Rc<dyn OnchainWallet>> {
    let wallet_persister =
        create_wallet_persister(wallet_dir, descriptor, config.network, fingerprint).await?;
    let onchain_wallet: Rc<dyn OnchainWallet> = Rc::new(
        LiquidOnchainWallet::new_with_cache_persister(config, persister, signer, wallet_persister)
            .await?,
    );
    Ok(onchain_wallet)
}

pub(crate) fn create_db_backup_persister(backup_dir_path: &Path) -> Result<BackupPersister> {
    let backup_storage = Rc::new(NodeFsBackupStorage::new(backup_dir_path));
    Ok(BackupPersister::new(backup_storage))
}
