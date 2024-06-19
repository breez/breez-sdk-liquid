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
        let src_con = self.get_connection()?;
        let mut dst_con = Connection::open(backup_path)?;

        let backup = Backup::new(&src_con, &mut dst_con)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;

        Ok(())
    }
}
