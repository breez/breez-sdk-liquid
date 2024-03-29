use anyhow::Result;
use rusqlite::{backup::Backup, Connection};
use std::path::Path;

use super::Persister;
use crate::Network;

impl Persister {
    pub(crate) fn backup(&self) -> Result<()> {
        let con = self.get_connection()?;

        let backup_file = match self.network {
            Network::Liquid => "backup.sql",
            Network::LiquidTestnet => "backup-testnet.sql",
        };

        con.backup(
            rusqlite::DatabaseName::Main,
            self.main_db_dir.join(backup_file),
            None,
        )?;

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
