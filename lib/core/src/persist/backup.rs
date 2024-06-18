use std::path::{Path, PathBuf};

use anyhow::Result;
use rusqlite::{backup::Backup, Connection};

use super::Persister;
use crate::model::Network;

impl Persister {
    pub(crate) fn get_default_backup_path(&self) -> PathBuf {
        self.main_db_dir.join(match self.network {
            Network::Mainnet => "backup.sql",
            Network::Testnet => "backup-testnet.sql",
        })
    }

    pub(crate) fn backup(&self, backup_path: PathBuf) -> Result<()> {
        let con = self.get_connection()?;
        con.backup(rusqlite::DatabaseName::Main, backup_path, None)?;
        Ok(())
    }

    pub(crate) fn restore_from_backup<P>(&self, backup_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let src_con = self.get_connection()?;
        let mut dst_con = Connection::open(backup_path)?;

        let backup = Backup::new(&src_con, &mut dst_con)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        persist::PaymentState,
        test_utils::{new_send_swap, new_temp_persister},
    };

    #[test]
    fn test_backup_and_restore() -> Result<()> {
        let local = &new_temp_persister()?.persister;
        local.insert_send_swap(&new_send_swap(Some(PaymentState::Pending)))?;

        let backup_path = local.get_default_backup_path();
        local.backup(backup_path.clone())?;
        assert!(backup_path.exists());

        let remote = &new_temp_persister()?.persister;

        remote.restore_from_backup(backup_path)?;
        assert_eq!(remote.list_ongoing_swaps()?.len(), 1);

        Ok(())
    }
}
