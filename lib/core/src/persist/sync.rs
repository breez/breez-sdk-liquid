use std::collections::HashMap;

use anyhow::Result;
use rusqlite::{named_params, Connection, OptionalExtension, Row, Statement, TransactionBehavior};

use super::Persister;
use crate::sync::model::{sync::Record, SyncOutgoingDetails, SyncSettings, SyncState};

impl Persister {
    fn select_sync_state_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT 
                data_id,
                record_id,
                record_revision,
                is_local
            FROM sync_state
            {where_clause_str}
        "
        )
    }

    fn sql_row_to_sync_state(row: &Row) -> rusqlite::Result<SyncState> {
        Ok(SyncState {
            data_id: row.get(0)?,
            record_id: row.get(1)?,
            record_revision: row.get(2)?,
            is_local: row.get(3)?,
        })
    }

    pub(crate) fn get_sync_state_by_record_id(&self, record_id: &str) -> Result<Option<SyncState>> {
        let con = self.get_connection()?;
        let query = Self::select_sync_state_query(vec!["record_id = ?1".to_string()]);
        let sync_state = con
            .query_row(&query, [record_id], Self::sql_row_to_sync_state)
            .optional()?;
        Ok(sync_state)
    }

    pub(crate) fn get_sync_state_by_data_id(&self, data_id: &str) -> Result<Option<SyncState>> {
        let con = self.get_connection()?;
        let query = Self::select_sync_state_query(vec!["data_id = ?1".to_string()]);
        let sync_state = con
            .query_row(&query, [data_id], Self::sql_row_to_sync_state)
            .optional()?;
        Ok(sync_state)
    }

    fn set_sync_state_stmt(con: &Connection) -> rusqlite::Result<Statement> {
        con.prepare(
            "
            INSERT OR REPLACE INTO sync_state(data_id, record_id, record_revision, is_local)
            VALUES (:data_id, :record_id, :record_revision, :is_local)
        ",
        )
    }

    pub(crate) fn set_sync_state(&self, sync_state: SyncState) -> Result<()> {
        let con = self.get_connection()?;

        Self::set_sync_state_stmt(&con)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        Ok(())
    }

}
