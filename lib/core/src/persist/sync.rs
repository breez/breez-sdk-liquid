use anyhow::{anyhow, Result};
use rusqlite::{params, Row};

use crate::sync::model::{sync::Record};

use super::{Direction, Persister};

impl Persister {
    pub(crate) fn get_latest_revision(&self) -> Result<i64> {
        let con = self.get_connection()?;

        let latest_revision: i64 = con
            .query_row(
                "SELECT syncLatestRevision FROM settings WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(latest_revision)
    }

    pub(crate) fn set_latest_revision(&self, new_latest_revision: i64) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "INSERT OR REPLACE INTO settings(id, syncLatestRevision) VALUES(1, ?)",
            params![new_latest_revision],
        )
        .map_err(|err| anyhow!("Could not write latest record id to database: {err}"))?;

        Ok(())
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
                FROM sync_records
                {where_clause_str}
                ORDER BY revision
            "
        )
    }

    pub(crate) fn get_records(&self, direction: Option<Direction>) -> Result<Vec<Record>> {
        let con = self.get_connection()?;

        let mut where_clauses = vec![];
        if let Some(direction) = direction {
            where_clauses.push("direction = ?1".to_string())
        }
        let query = Self::get_records_query(where_clauses);

        let records: Vec<Record> = con
            .prepare(&query)?
            .query_map([direction], |row| self.sql_row_to_record(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(records)
    }

    pub(crate) fn insert_record(&self, record: &Record, direction: Direction) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            INSERT INTO sync_records(
                id,
                schema_version,
                data,
                revision,
                direction
            )
            VALUES (?, ?, ?, ?, ?)
        ",
            (
                &record.id,
                &record.schema_version,
                &record.data,
                &record.revision,
                direction,
            ),
        )?;

        Ok(())
    }

    pub(crate) fn delete_record(&self, id: String) -> Result<()> {
        let con = self.get_connection()?;

        con.execute(
            "
            DELETE FROM sync_records
            WHERE id = ?
        ",
            params![id],
        )?;

        Ok(())
    }
}
