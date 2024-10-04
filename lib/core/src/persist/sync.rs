use anyhow::{anyhow, Result};
use rusqlite::params;

use crate::{
    sync::model::{sync::Record, DecryptedRecord, SyncData},
    utils,
};

use super::Persister;

impl Persister {
    pub(crate) fn get_latest_record_id(&self) -> Result<i64> {
        let con = self.get_connection()?;

        let latest_record_id: i64 = con.query_row(
            "SELECT latestRecordId FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )?;

        Ok(latest_record_id)
    }

    pub(crate) fn set_latest_record_id(&self, new_latest_id: i64) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "INSERT OR REPLACE INTO settings(id, latestRecordId) VALUES(1, ?)",
            params![new_latest_id],
        )
        .map_err(|err| anyhow!("Could not write latest record id to database: {err}"))?;

        Ok(())
    }

    pub(crate) fn insert_record(&self, record: &Record) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            INSERT INTO pending_sync_records(
                id,
                version,
                data
            )
            VALUES (?, ?, ?)
        ",
            (record.id, record.version, &record.data),
        )?;

        Ok(())
    }

    pub(crate) fn delete_record(&self, record: &Record) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            DELETE FROM pending_sync_records
            WHERE id = ?
        ",
            params![record.id],
        )?;

        Ok(())
    }

    pub(crate) fn apply_record(&self, record: &DecryptedRecord) -> Result<()> {
        let con = self.get_connection()?;

        match &record.data {
            SyncData::Chain(chain_data) => {}
            SyncData::Send(send_data) => {}
            SyncData::Receive(receive_data) => {}
        }
        Ok(())
    }
}
