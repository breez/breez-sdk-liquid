use crate::platform::db_backup_common::BackupPersister;
use anyhow::{bail, Result};
use breez_sdk_liquid::model::{Config, Signer};
use breez_sdk_liquid::persist::Persister;
use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
use breez_sdk_liquid::wallet::{LiquidOnchainWallet, OnchainWallet};
use std::path::Path;
use std::rc::Rc;

pub(crate) async fn create_onchain_wallet(
    _wallet_dir: &Path,
    config: Config,
    _descriptor: WolletDescriptor,
    _fingerprint: &str,
    persister: Rc<Persister>,
    signer: Rc<Box<dyn Signer>>,
) -> Result<Rc<dyn OnchainWallet>> {
    let onchain_wallet: Rc<dyn OnchainWallet> =
        Rc::new(LiquidOnchainWallet::new_in_memory(config, persister, signer).await?);
    Ok(onchain_wallet)
}

pub(crate) fn create_db_backup_persister(_backup_dir_path: &Path) -> Result<BackupPersister> {
    bail!("No backup persister available on this platform")
}
