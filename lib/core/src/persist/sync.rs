use std::collections::HashMap;

use anyhow::Result;
use rusqlite::{
    named_params, Connection, OptionalExtension, Row, Statement, Transaction, TransactionBehavior,
};

use super::{cache::KEY_LAST_DERIVATION_INDEX, PaymentTxDetails, Persister, Swap};
use crate::{
    sync::model::{
        data::LAST_DERIVATION_INDEX_DATA_ID, Record, RecordType, SyncOutgoingChanges, SyncSettings,
        SyncState,
    },
    utils,
};

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

    pub(crate) fn get_sync_settings(&self) -> Result<SyncSettings> {
        let con = self.get_connection()?;

        let settings: HashMap<String, String> = con
            .prepare("SELECT key, value FROM sync_settings")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .map(|e| e.unwrap())
            .collect();

        let latest_revision = match settings.get("latest_revision") {
            Some(revision) => Some(revision.parse()?),
            None => None,
        };

        let sync_settings = SyncSettings {
            remote_url: settings.get("remote_url").cloned(),
            latest_revision,
        };

        Ok(sync_settings)
    }

    fn set_sync_setting_stmt(con: &Connection) -> rusqlite::Result<Statement> {
        con.prepare("INSERT OR REPLACE INTO sync_settings(key, value) VALUES(:key, :value)")
    }

    pub(crate) fn set_sync_settings(&self, map: HashMap<&'static str, String>) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        for (key, value) in map {
            Self::set_sync_setting_stmt(&tx)?.execute(named_params! {
                ":key": key,
                ":value": value,
            })?;
        }

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn set_new_remote(&self, remote_url: String) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute("DELETE FROM sync_state", [])?;
        tx.execute("DELETE FROM sync_incoming", [])?;
        tx.execute("DELETE FROM sync_outgoing", [])?;

        let swap_tables = HashMap::from([
            ("receive_swaps", RecordType::Receive),
            ("send_swaps", RecordType::Send),
            ("chain_swaps", RecordType::Chain),
        ]);
        for (table_name, record_type) in swap_tables {
            let mut stmt = tx.prepare(&format!("SELECT id FROM {table_name}"))?;
            let mut rows = stmt.query([])?;

            while let Some(row) = rows.next()? {
                let data_id: String = row.get(0)?;
                let record_id = Record::get_id_from_record_type(record_type, &data_id);

                tx.execute(
                    "
                    INSERT INTO sync_outgoing(record_id, data_id, record_type, commit_time)
                    VALUES(:record_id, :data_id, :record_type, :commit_time)
                ",
                    named_params! {
                        ":record_id": record_id,
                        ":data_id": data_id,
                        ":record_type": record_type,
                        ":commit_time": utils::now(),
                    },
                )?;
            }
        }

        Self::set_sync_setting_stmt(&tx)?.execute(named_params! {
            ":key": "remote_url",
            ":value": remote_url
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn get_incoming_records(&self) -> Result<Vec<Record>> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            SELECT 
                record_id,
                revision,
                schema_version,
                data
            FROM sync_incoming
        ",
        )?;
        let records = stmt
            .query_map([], |row| {
                Ok(Record {
                    id: row.get(0)?,
                    revision: row.get(1)?,
                    schema_version: row.get(2)?,
                    data: row.get(3)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(records)
    }

    pub(crate) fn set_incoming_records(&self, records: &[Record]) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        for record in records {
            tx.execute(
                "
                INSERT OR REPLACE INTO sync_incoming(record_id, revision, schema_version, data)
                VALUES(:record_id, :revision, :schema_version, :data)
            ",
                named_params! {
                    ":record_id": record.id,
                    ":revision": record.revision,
                    ":schema_version": record.schema_version,
                    ":data": record.data,
                },
            )?;
        }

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn remove_incoming_records(&self, record_ids: Vec<String>) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        for record_id in record_ids {
            tx.execute(
                "DELETE FROM sync_incoming WHERE record_id = :record_id",
                named_params! {
                    ":record_id": record_id
                },
            )?;
        }

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn commit_outgoing(
        &self,
        tx: &Transaction,
        data_id: &str,
        record_type: RecordType,
        updated_fields: Option<Vec<String>>,
    ) -> Result<()> {
        if self.sync_trigger.try_read().is_ok_and(|t| t.is_none()) {
            return Ok(());
        }

        let record_id = Record::get_id_from_record_type(record_type, data_id);
        let updated_fields = updated_fields
            .map(|fields| {
                let fields = fields
                    .iter()
                    .map(|field| format!("'$[#]', '{field}'"))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("json_insert(
                    COALESCE((SELECT updated_fields_json FROM sync_outgoing WHERE record_id = :record_id), '[]'),
                    {fields}
                )")
            })
            .unwrap_or("NULL".to_string());
        tx.execute(&format!("
            INSERT OR REPLACE INTO sync_outgoing(record_id, data_id, record_type, commit_time, updated_fields_json)
            VALUES(
                :record_id, 
                :data_id, 
                :record_type, 
                :commit_time, 
                {updated_fields}
            ) 
        "),
            named_params! {
                ":record_id": record_id,
                ":data_id": data_id,
                ":record_type": record_type,
                ":commit_time": utils::now(),
            },
        )?;

        Ok(())
    }

    fn select_sync_outgoing_changes_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT 
                record_id,
                data_id,
                record_type,
                commit_time,
                updated_fields_json
            FROM sync_outgoing
            {where_clause_str}
        "
        )
    }

    fn sql_row_to_sync_outgoing_changes(row: &Row) -> Result<SyncOutgoingChanges> {
        let record_id = row.get(0)?;
        let data_id = row.get(1)?;
        let record_type = row.get(2)?;
        let commit_time = row.get(3)?;
        let updated_fields = match row.get::<_, Option<String>>(4)? {
            Some(fields) => Some(serde_json::from_str(&fields)?),
            None => None,
        };

        Ok(SyncOutgoingChanges {
            record_id,
            data_id,
            record_type,
            commit_time,
            updated_fields,
        })
    }

    pub(crate) fn get_sync_outgoing_changes(&self) -> Result<Vec<SyncOutgoingChanges>> {
        let con = self.get_connection()?;

        let query = Self::select_sync_outgoing_changes_query(vec![]);
        let mut stmt = con.prepare(&query)?;
        let mut rows = stmt.query([])?;

        let mut outgoing_changes = vec![];
        while let Some(row) = rows.next()? {
            let detail = Self::sql_row_to_sync_outgoing_changes(row)?;
            outgoing_changes.push(detail);
        }

        Ok(outgoing_changes)
    }

    pub(crate) fn get_sync_outgoing_changes_by_id(
        &self,
        record_id: &str,
    ) -> Result<Option<SyncOutgoingChanges>> {
        let con = self.get_connection()?;
        let query =
            Self::select_sync_outgoing_changes_query(vec!["record_id = :record_id".to_string()]);
        let mut stmt = con.prepare(&query)?;
        let mut rows = stmt.query(named_params! {
            ":record_id": record_id,
        })?;

        if let Some(row) = rows.next()? {
            return Ok(Some(Self::sql_row_to_sync_outgoing_changes(row)?));
        }

        Ok(None)
    }

    pub(crate) fn remove_sync_outgoing_changes(&self, record_ids: Vec<String>) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        for record_id in record_ids {
            tx.execute(
                "DELETE FROM sync_outgoing WHERE record_id = :record_id",
                named_params! {
                    ":record_id": record_id
                },
            )?;
        }

        tx.commit()?;

        Ok(())
    }

    fn check_commit_update(con: &Connection, record_id: &str, last_commit_time: u32) -> Result<()> {
        let query =
            Self::select_sync_outgoing_changes_query(vec!["record_id = :record_id".to_string()]);
        let mut stmt = con.prepare(&query)?;
        let mut rows = stmt.query(named_params! {
            ":record_id": record_id,
        })?;

        if let Some(row) = rows.next()? {
            let sync_outgoing_changes = Self::sql_row_to_sync_outgoing_changes(row)?;

            if sync_outgoing_changes.commit_time > last_commit_time {
                return Err(anyhow::anyhow!("Record has been updated while pulling"));
            }
        }

        Ok(())
    }

    pub(crate) fn commit_incoming_swap(
        &self,
        swap: &Swap,
        sync_state: &SyncState,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(&tx, &sync_state.record_id, last_commit_time)?;
        }

        match swap {
            Swap::Receive(receive_swap) => {
                Self::insert_or_update_receive_swap_inner(&tx, receive_swap)
            }
            Swap::Send(send_swap) => Self::insert_or_update_send_swap_inner(&tx, send_swap),
            Swap::Chain(chain_swap) => Self::insert_or_update_chain_swap_inner(&tx, chain_swap),
        }?;

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn commit_incoming_address_index(
        &self,
        new_address_index: u32,
        sync_state: &SyncState,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(
                &tx,
                &Record::get_id_from_record_type(
                    RecordType::LastDerivationIndex,
                    LAST_DERIVATION_INDEX_DATA_ID,
                ),
                last_commit_time,
            )?;
        }

        Self::update_cached_item_inner(
            &tx,
            KEY_LAST_DERIVATION_INDEX,
            new_address_index.to_string(),
        )?;

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": sync_state.data_id,
            ":record_id": sync_state.record_id,
            ":record_revision": sync_state.record_revision,
            ":is_local": sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn commit_incoming_payment_details(
        &self,
        payment_tx_details: PaymentTxDetails,
        sync_state: &SyncState,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(&tx, &sync_state.record_id, last_commit_time)?;
        }

        Self::insert_or_update_payment_details_inner(&tx, &payment_tx_details, false)?;

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn trigger_sync(&self) -> Result<()> {
        if let Ok(lock) = self.sync_trigger.try_read() {
            if let Some(trigger) = lock.clone() {
                trigger.try_send(())?;
            }
        }
        Ok(())
    }
}
