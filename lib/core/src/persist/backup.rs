use std::path::{Path, PathBuf};

use anyhow::Result;
use rusqlite::{backup::Backup, Connection};

use super::Persister;
use crate::model::LiquidNetwork;

impl Persister {
    pub(crate) fn get_default_backup_path(&self) -> PathBuf {
        self.main_db_dir.join(match self.network {
            LiquidNetwork::Mainnet => "backup.sql",
            LiquidNetwork::Testnet => "backup-testnet.sql",
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
        let src_con = Connection::open(backup_path)?;
        let mut dst_con = self.get_connection()?;

        let backup = Backup::new(&src_con, &mut dst_con)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        model::PaymentState,
        test_utils::persist::{create_persister, new_receive_swap, new_send_swap},
    };

    #[test]
    fn test_backup_and_restore() -> Result<()> {
        create_persister!(local);

        local.insert_send_swap(&new_send_swap(Some(PaymentState::Pending)))?;
        local.insert_receive_swap(&new_receive_swap(Some(PaymentState::Pending)))?;
        assert_eq!(local.list_ongoing_swaps()?.len(), 2);

        let backup_path = local.get_default_backup_path();
        local.backup(backup_path.clone())?;
        assert!(backup_path.exists());

        create_persister!(remote);

        remote.restore_from_backup(backup_path)?;
        assert_eq!(remote.list_ongoing_swaps()?.len(), 2);

        Ok(())
    }
}
