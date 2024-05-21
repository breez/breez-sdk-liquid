use anyhow::Result;
use boltz_client::swaps::boltzv2::CreateSubmarineResponse;
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::Persister;

impl Persister {
    pub(crate) fn set_lockup_tx_id_for_swap_in(
        &self,
        swap_in_id: &str,
        lockup_tx_id: &str,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE send_swaps SET lockup_tx_id=:lockup_tx_id WHERE id=:id",
            named_params! {
             ":id": swap_in_id,
             ":lockup_tx_id": lockup_tx_id,
            },
        )?;

        Ok(())
    }

    pub(crate) fn set_claim_tx_seen_for_swap_in(&self, swap_in_id: &str) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE send_swaps SET is_claim_tx_seen=:is_claim_tx_seen WHERE id=:id",
            named_params! {
             ":id": swap_in_id,
             ":is_claim_tx_seen": true,
            },
        )?;

        Ok(())
    }

    pub(crate) fn insert_or_update_swap_in(&self, swap_in: SwapIn) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT OR REPLACE INTO send_swaps (
                id,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                lockup_tx_id,
                is_claim_tx_seen,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_in.id,
            swap_in.invoice,
            swap_in.payer_amount_sat,
            swap_in.receiver_amount_sat,
            swap_in.create_response_json,
            swap_in.lockup_tx_id,
            swap_in.is_claim_tx_seen,
            swap_in.created_at,
        ))?;

        Ok(())
    }

    fn list_swap_in_query(where_clauses: Vec<&str>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                lockup_tx_id,
                is_claim_tx_seen,
                created_at
            FROM send_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_swap_in(con: &Connection, id: &str) -> rusqlite::Result<Option<SwapIn>> {
        let query = Self::list_swap_in_query(vec!["id = ?1"]);
        con.query_row(&query, [id], Self::sql_row_to_swap_in)
            .optional()
    }

    fn sql_row_to_swap_in(row: &Row) -> rusqlite::Result<SwapIn> {
        Ok(SwapIn {
            id: row.get(0)?,
            invoice: row.get(1)?,
            payer_amount_sat: row.get(2)?,
            receiver_amount_sat: row.get(3)?,
            create_response_json: row.get(4)?,
            lockup_tx_id: row.get(5)?,
            is_claim_tx_seen: row.get(6)?,
            created_at: row.get(7)?,
        })
    }

    pub(crate) fn list_send_swaps(
        &self,
        con: &Connection,
        where_clauses: Vec<&str>,
    ) -> rusqlite::Result<Vec<SwapIn>> {
        let query = Self::list_swap_in_query(where_clauses);
        let ongoing_send = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_swap_in)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_send)
    }

    pub(crate) fn list_ongoing_send_swaps(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<Vec<SwapIn>> {
        let swap_ins = self.list_send_swaps(con, vec![])?;

        let filtered: Vec<SwapIn> = swap_ins
            .into_iter()
            .filter(|swap| swap.calculate_status() != SwapInStatus::Completed)
            .collect();

        Ok(filtered)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateSubmarineResponse {
    pub(crate) accept_zero_conf: bool,
    pub(crate) address: String,
    pub(crate) bip21: String,
    pub(crate) claim_public_key: String,
    pub(crate) expected_amount: u64,
    pub(crate) swap_tree: InternalSwapTree,
    pub(crate) blinding_key: Option<String>,
}
impl InternalCreateSubmarineResponse {
    pub(crate) fn try_convert_from_boltz(
        boltz_create_response: &CreateSubmarineResponse,
        expected_swap_id: &str,
    ) -> Result<InternalCreateSubmarineResponse, PaymentError> {
        // Do not store the CreateResponse fields that are already stored separately
        // Before skipping them, ensure they match the separately stored ones
        ensure_sdk!(
            boltz_create_response.id == expected_swap_id,
            PaymentError::PersistError
        );

        let res = InternalCreateSubmarineResponse {
            accept_zero_conf: boltz_create_response.accept_zero_conf,
            address: boltz_create_response.address.clone(),
            bip21: boltz_create_response.bip21.clone(),
            claim_public_key: boltz_create_response.claim_public_key.to_string(),
            expected_amount: boltz_create_response.expected_amount,
            swap_tree: boltz_create_response.swap_tree.clone().into(),
            blinding_key: boltz_create_response.blinding_key.clone(),
        };
        Ok(res)
    }
}
