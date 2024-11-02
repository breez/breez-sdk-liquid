use anyhow::Result;
use rusqlite::{named_params, params, Row};

use crate::sync::model::{sync::Record, SyncDetails};

use super::Persister;

impl Persister {
    pub(crate) fn get_latest_revision(&self) -> Result<i64> {
        let con = self.get_connection()?;

        let latest_revision: i64 = con
            .query_row(
                "
                SELECT revision 
                FROM sync_details 
                ORDER BY revision DESC 
                LIMIT 1
            ",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(latest_revision)
    }

    fn sql_row_to_record(&self, row: &Row) -> rusqlite::Result<Record> {
        Ok(Record {
            id: row.get(0)?,
            schema_version: row.get(1)?,
            data: row.get(2)?,
            revision: row.get(3)?,
        })
    }

    fn get_records_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
                SELECT 
                    id,
                    schema_version,
                    data,
                    revision
                FROM pending_sync_records
                {where_clause_str}
                ORDER BY revision
            "
        )
    }

    pub(crate) fn get_pending_records(&self) -> Result<Vec<Record>> {
        let con = self.get_connection()?;

        let where_clauses = vec![];
        let query = Self::get_records_query(where_clauses);

        let records: Vec<Record> = con
            .prepare(&query)?
            .query_map([], |row| self.sql_row_to_record(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(records)
    }

    pub(crate) fn insert_pending_record(&self, record: &Record) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            INSERT INTO pending_sync_records(
                id,
                schema_version,
                data,
                revision
            )
            VALUES (?, ?, ?, ?)
        ",
            (
                &record.id,
                &record.schema_version,
                &record.data,
                &record.revision,
            ),
        )?;

        Ok(())
    }

    pub(crate) fn delete_pending_record(&self, id: String) -> Result<()> {
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

    pub(crate) fn insert_or_update_sync_details(
        &self,
        id: &str,
        sync_details: &SyncDetails,
    ) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "INSERT OR REPLACE INTO sync_details(
                data_identifier,
                is_local,
                revision,
                record_id
            )
            VALUES (:data_identifier, :is_local, :revision, :record_id)",
            named_params! {
                ":data_identifier": id,
                ":is_local": sync_details.is_local,
                ":revision": sync_details.revision,
                ":record_id": sync_details.record_id
            },
        )?;

        Ok(())
    }

    pub(crate) fn get_sync_details(&self, data_identifier: &str) -> Result<SyncDetails> {
        let con = self.get_connection()?;

        let sync_details = con.query_row(
            "
                SELECT
                    is_local,
                    revision,
                    record_id
                FROM sync_details
                WHERE data_identifier = :data_identifier
            ",
            named_params! {
                ":data_identifier": data_identifier,
            },
            |row| {
                Ok(SyncDetails {
                    is_local: row.get(0)?,
                    revision: row.get(1)?,
                    record_id: row.get(2)?,
                })
            },
        )?;

        Ok(sync_details)
    }
}
