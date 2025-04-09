use crate::backup::indexed_db::IndexedDbBackupStorage;
use crate::backup::BackupPersister;
use crate::wallet_persister::indexed_db::AsyncWalletCachePersister;
use crate::wallet_persister::indexed_db::IndexedDbWalletStorage;
use anyhow::Result;
use breez_sdk_liquid::model::{Config, LiquidNetwork, Signer};
use breez_sdk_liquid::persist::Persister;
use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
use breez_sdk_liquid::wallet::persister::WalletCachePersister;
use breez_sdk_liquid::wallet::{LiquidOnchainWallet, OnchainWallet};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) async fn create_wallet_persister(
    wallet_dir: &Path,
    descriptor: WolletDescriptor,
    _network: LiquidNetwork,
    _fingerprint: &str,
) -> Result<Rc<dyn WalletCachePersister>> {
    let wallet_storage = Arc::new(IndexedDbWalletStorage::new(wallet_dir, descriptor));
    let wallet_persister: Rc<dyn WalletCachePersister> =
        Rc::new(AsyncWalletCachePersister::new(wallet_storage).await?);
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
    let backup_storage = Rc::new(IndexedDbBackupStorage::new(
        &backup_dir_path.to_string_lossy(),
    ));
    Ok(BackupPersister::new(backup_storage))
}
