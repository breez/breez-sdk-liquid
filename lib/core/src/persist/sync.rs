use anyhow::{anyhow, Result};
use rusqlite::{params, Row};

use crate::sync::model::{sync::Record, DecryptedRecord, SyncData};

use super::Persister;

impl Persister {
    pub(crate) fn get_latest_record_id(&self) -> Result<i64> {
        let con = self.get_connection()?;

        let latest_record_id: i64 = con
            .query_row(
                "SELECT latestRecordId FROM settings WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

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

    fn sql_row_to_record(&self, row: &Row) -> rusqlite::Result<Record> {
        Ok(Record {
            id: row.get(0)?,
            version: row.get(1)?,
            data: row.get(2)?,
        })
    }

    pub(crate) fn get_records(&self) -> Result<Vec<Record>> {
        let con = self.get_connection()?;

        let records: Vec<Record> = con
            .prepare(
                "
            SELECT 
                id,
                version,
                data
            FROM pending_sync_records
        ",
            )?
            .query_map([], |row| self.sql_row_to_record(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(records)
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

    pub(crate) fn delete_record(&self, id: i64) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            DELETE FROM pending_sync_records
            WHERE id = ?
        ",
            params![id],
        )?;

        Ok(())
    }

    pub(crate) fn apply_record(&self, record: DecryptedRecord) -> Result<()> {
        match record.data {
            SyncData::Chain(chain_data) => self.insert_chain_swap(&chain_data.into()),
            SyncData::Send(send_data) => self.insert_send_swap(&send_data.into()),
            SyncData::Receive(receive_data) => self.insert_receive_swap(&receive_data.into()),
        }
    }
}
