use std::collections::HashMap;

use anyhow::Result;
use rusqlite::{named_params, Connection, OptionalExtension, Row, Statement, TransactionBehavior};

use super::Persister;
use crate::{
    sync::model::{sync::Record, RecordType, SyncOutgoingChanges, SyncSettings, SyncState},
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

    pub(crate) fn set_sync_settings(&self, map: HashMap<&'static str, String>) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        for (key, value) in map {
            tx.execute(
                "INSERT OR REPLACE INTO sync_settings(key, value) VALUES(:key, :value)",
                named_params! {
                    ":key": key,
                    ":value": value,
                },
            )?;
        }

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
        con: &Connection,
        data_id: &str,
        record_type: RecordType,
        updated_fields: Option<Vec<String>>,
    ) -> Result<()> {
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
        con.execute(&format!("
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

    pub(crate) fn commit_incoming_receive_swap(
        &self,
        data: &ReceiveSyncData,
        sync_state: SyncState,
        is_update: bool,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(&tx, &sync_state.record_id, last_commit_time)?;
        }

        let params = named_params! {
            ":id": &data.swap_id,
            ":invoice": &data.invoice,
            ":preimage": &data.preimage,
            ":create_response_json": &data.create_response_json,
            ":claim_fees_sat": &data.claim_fees_sat,
            ":claim_private_key": &data.claim_private_key,
            ":payer_amount_sat": &data.payer_amount_sat,
            ":receiver_amount_sat": &data.receiver_amount_sat,
            ":mrh_address": &data.mrh_address,
            ":created_at": &data.created_at,
            ":payment_hash": &data.payment_hash,
            ":description": &data.description,
        };
        match is_update {
            true => {
                tx.execute(
                    "
                    UPDATE receive_swaps
                    SET
                        invoice = :invoice,
                        preimage = :preimage,
                        create_response_json = :create_response_json,
                        claim_fees_sat = :claim_fees_sat,
                        claim_private_key = :claim_private_key,
                        payer_amount_sat = :payer_amount_sat,
                        receiver_amount_sat = :receiver_amount_sat,
                        mrh_address = :mrh_address,
                        created_at = :created_at,
                        payment_hash = :payment_hash,
                        description = :description
                    WHERE id = :id",
                    params,
                )?;
            }
            false => {
                tx.execute(
                    "
                    INSERT INTO receive_swaps(
                        id,
                        invoice,
                        preimage,
                        create_response_json,
                        claim_fees_sat,
                        claim_private_key,
                        payer_amount_sat,
                        receiver_amount_sat,
                        mrh_address,
                        created_at,
                        payment_hash,
                        description,
                        state
                    )
                    VALUES(
                        :id,
                        :invoice,
                        :preimage,
                        :create_response_json,
                        :claim_fees_sat,
                        :claim_private_key,
                        :payer_amount_sat,
                        :receiver_amount_sat,
                        :mrh_address,
                        :created_at,
                        :payment_hash,
                        :description,
                        :state
                    )",
                    [params, &[(":state", &PaymentState::Created)]]
                        .concat()
                        .as_slice(),
                )?;
            }
        }

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn commit_incoming_send_swap(
        &self,
        data: &SendSyncData,
        sync_state: SyncState,
        is_update: bool,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(&tx, &sync_state.record_id, last_commit_time)?;
        }

        let params = named_params! {
            ":id": &data.swap_id,
            ":invoice": &data.invoice,
            ":create_response_json": &data.create_response_json,
            ":refund_private_key": &data.refund_private_key,
            ":payer_amount_sat": &data.payer_amount_sat,
            ":receiver_amount_sat": &data.receiver_amount_sat,
            ":created_at": &data.created_at,
            ":preimage": &data.preimage,
            ":payment_hash": &data.payment_hash,
            ":description": &data.description,
        };
        match is_update {
            true => {
                tx.execute(
                    "
                    UPDATE send_swaps
                    SET
                        invoice = :invoice,
                        create_response_json = :create_response_json,
                        refund_private_key = :refund_private_key,
                        payer_amount_sat = :payer_amount_sat,
                        receiver_amount_sat = :receiver_amount_sat,
                        created_at = :created_at,
                        preimage = :preimage,
                        payment_hash = :payment_hash,
                        description = :description
                    WHERE id = :id",
                    params,
                )?;
            }
            false => {
                tx.execute(
                    "
                    INSERT INTO send_swaps(
                        id,
                        invoice,
                        create_response_json,
                        refund_private_key,
                        payer_amount_sat,
                        receiver_amount_sat,
                        created_at,
                        preimage,
                        payment_hash,
                        description,
                        state
                    )
                    VALUES(
                        :id,
                        :invoice,
                        :create_response_json,
                        :refund_private_key,
                        :payer_amount_sat,
                        :receiver_amount_sat,
                        :created_at,
                        :preimage,
                        :payment_hash,
                        :description,
                        :state
                    )",
                    [params, &[(":state", &PaymentState::Created)]]
                        .concat()
                        .as_slice(),
                )?;
            }
        }

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn commit_incoming_chain_swap(
        &self,
        data: &ChainSyncData,
        sync_state: SyncState,
        is_update: bool,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        if let Some(last_commit_time) = last_commit_time {
            Self::check_commit_update(&tx, &sync_state.record_id, last_commit_time)?;
        }

        let params = named_params! {
            ":id": &data.swap_id,
            ":preimage": &data.preimage,
            ":create_response_json": &data.create_response_json,
            ":direction": &data.direction,
            ":lockup_address": &data.lockup_address,
            ":claim_fees_sat": &data.claim_fees_sat,
            ":claim_private_key": &data.claim_private_key,
            ":refund_private_key": &data.refund_private_key,
            ":timeout_block_height": &data.timeout_block_height,
            ":payer_amount_sat": &data.payer_amount_sat,
            ":receiver_amount_sat": &data.receiver_amount_sat,
            ":accept_zero_conf": &data.accept_zero_conf,
            ":created_at": &data.created_at,
            ":description": &data.description,
        };
        match is_update {
            true => {
                tx.execute(
                    "
                    UPDATE chain_swaps
                    SET
                        preimage = :preimage,
                        create_response_json = :create_response_json,
                        direction = :direction,
                        lockup_address = :lockup_address,
                        claim_fees_sat = :claim_fees_sat,
                        claim_private_key = :claim_private_key,
                        refund_private_key = :refund_private_key,
                        timeout_block_height = :timeout_block_height,
                        payer_amount_sat = :payer_amount_sat,
                        receiver_amount_sat = :receiver_amount_sat,
                        accept_zero_conf = :accept_zero_conf,
                        created_at = :created_at,
                        description = :description,
                        server_lockup_tx_id = :server_lockup_tx_id
                    WHERE id = :id",
                    params,
                )?;
            }
            false => {
                tx.execute(
                    "
                    INSERT INTO chain_swaps( 
                        id,
                        preimage,
                        create_response_json,
                        direction,
                        lockup_address,
                        claim_fees_sat,
                        claim_private_key,
                        refund_private_key,
                        timeout_block_height,
                        payer_amount_sat,
                        receiver_amount_sat,
                        accept_zero_conf,
                        created_at,
                        description,
                        server_lockup_tx_id,
                        state
                    )
                    VALUES(
                        :id,
                        :preimage,
                        :create_response_json,
                        :direction,
                        :lockup_address,
                        :claim_fees_sat,
                        :claim_private_key,
                        :refund_private_key,
                        :timeout_block_height,
                        :payer_amount_sat,
                        :receiver_amount_sat,
                        :accept_zero_conf,
                        :created_at,
                        :description,
                        :server_lockup_tx_id,
                        :state
                    )",
                    [params, &[(":state", &PaymentState::Created)]]
                        .concat()
                        .as_slice(),
                )?;
            }
        }

        Self::set_sync_state_stmt(&tx)?.execute(named_params! {
            ":data_id": &sync_state.data_id,
            ":record_id": &sync_state.record_id,
            ":record_revision": &sync_state.record_revision,
            ":is_local": &sync_state.is_local,
        })?;

        tx.commit()?;

        Ok(())
    }
}
