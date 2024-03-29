use anyhow::Result;
use std::path::Path;

use crate::{BackupHistoryItem, BACKUP_DB_FILE};

use super::Persister;

impl Persister {
    pub(crate) fn backup(&self) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;

        tx.backup(
            rusqlite::DatabaseName::Main,
            self.main_db_dir.join(BACKUP_DB_FILE),
            None,
        )?;
        tx.execute(
            "INSERT INTO backup_history(timestamp) VALUES(strftime('%s', 'now'))",
            [],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn get_backup_history(&self) -> Result<Vec<BackupHistoryItem>> {
        let con = self.get_connection()?;
        let history = con
            .prepare(
                "
            SELECT 
                timestamp
            FROM backup_history
            ORDER BY timestamp DESC
        ",
            )?
            .query_map([], |row| {
                Ok(BackupHistoryItem {
                    timestamp: row.get(0)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(history)
    }

    pub(crate) fn restore_from_backup<P>(&self, backup_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut con = self.get_connection()?;
        let backup_path = backup_path
            .as_ref()
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid path specified"))?;

        let tx = con.transaction()?;

        tx.execute("ATTACH DATABASE ? as backup", [backup_path])?;
        tx.execute(
            "
           INSERT OR IGNORE INTO ongoing_receive_swaps
           SELECT
               id,
               preimage,
               redeem_script,
               blinding_key,
               invoice,
               onchain_amount_sat,
               created_at
           FROM backup.ongoing_receive_swaps
        ",
            [],
        )?;
        tx.execute(
            "
           INSERT OR IGNORE INTO ongoing_send_swaps
           SELECT
               id,
               amount_sat,
               funding_address,
               created_at
           FROM backup.ongoing_send_swaps
        ",
            [],
        )?;

        tx.commit()?;

        con.execute("DETACH DATABASE backup", [])?;

        Ok(())
    }
}
