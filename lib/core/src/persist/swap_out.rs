use std::collections::HashMap;

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use boltz_client::swaps::boltzv2::CreateReverseResponse;
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

impl Persister {
    pub(crate) fn insert_swap_out(&self, swap_out: SwapOut) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT INTO receive_swaps (
                id,
                preimage,
                create_response_json,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                created_at,
                claim_fees_sat,
                claim_tx_id,
                state
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_out.id,
            swap_out.preimage,
            swap_out.create_response_json,
            swap_out.invoice,
            swap_out.payer_amount_sat,
            swap_out.receiver_amount_sat,
            swap_out.created_at,
            swap_out.claim_fees_sat,
            swap_out.claim_tx_id,
            swap_out.state,
        ))?;

        Ok(())
    }

    fn list_swap_out_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                rs.id,
                rs.preimage,
                rs.create_response_json,
                rs.invoice,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.claim_fees_sat,
                rs.claim_tx_id,
                rs.created_at,
                rs.state
            FROM receive_swaps AS rs
            {where_clause_str}
            ORDER BY rs.created_at
        "
        )
    }

    pub(crate) fn fetch_swap_out(con: &Connection, id: &str) -> rusqlite::Result<Option<SwapOut>> {
        let query = Self::list_swap_out_query(vec!["id = ?1".to_string()]);
        con.query_row(&query, [id], Self::sql_row_to_swap_out)
            .optional()
    }

    fn sql_row_to_swap_out(row: &Row) -> rusqlite::Result<SwapOut> {
        Ok(SwapOut {
            id: row.get(0)?,
            preimage: row.get(1)?,
            create_response_json: row.get(2)?,
            invoice: row.get(3)?,
            payer_amount_sat: row.get(4)?,
            receiver_amount_sat: row.get(5)?,
            claim_fees_sat: row.get(6)?,
            claim_tx_id: row.get(7)?,
            created_at: row.get(8)?,
            state: row.get(9)?,
        })
    }

    pub(crate) fn list_receive_swaps(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> rusqlite::Result<Vec<SwapOut>> {
        let query = Self::list_swap_out_query(where_clauses);
        let ongoing_receive = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_swap_out)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_receive)
    }

    pub(crate) fn list_ongoing_receive_swaps(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<Vec<SwapOut>> {
        let mut where_clause: Vec<String> = Vec::new();
        where_clause.push(format!(
            "state in ({})",
            [PaymentState::Created, PaymentState::Pending]
                .iter()
                .map(|t| format!("'{}'", *t as i8))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        self.list_receive_swaps(con, where_clause)
    }

    pub(crate) fn list_pending_receive_swaps(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<Vec<SwapOut>> {
        let query = Self::list_swap_out_query(vec!["state = ?1".to_string()]);
        let res = con
            .prepare(&query)?
            .query_map(params![PaymentState::Pending], Self::sql_row_to_swap_out)?
            .map(|i| i.unwrap())
            .collect();
        Ok(res)
    }

    /// Pending swap outs, indexed by claim_tx_id
    pub(crate) fn list_pending_receive_swaps_by_claim_tx_id(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<HashMap<String, SwapOut>> {
        let res = self
            .list_pending_receive_swaps(con)?
            .iter()
            .filter_map(|pending_swap_out| {
                pending_swap_out
                    .claim_tx_id
                    .as_ref()
                    .map(|claim_tx_id| (claim_tx_id.clone(), pending_swap_out.clone()))
            })
            .collect();
        Ok(res)
    }

    pub(crate) fn try_handle_receive_swap_update(
        &self,
        con: &Connection,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        // Do not overwrite claim_tx_id
        con.execute(
            "UPDATE receive_swaps
            SET
                claim_tx_id =
                    CASE
                        WHEN claim_tx_id IS NULL THEN :claim_tx_id
                        ELSE claim_tx_id
                    END,

                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":claim_tx_id": claim_tx_id,
                ":state": to_state,
            },
        )
        .map_err(|_| PaymentError::PersistError)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateReverseResponse {
    pub swap_tree: InternalSwapTree,
    pub lockup_address: String,
    pub refund_public_key: String,
    pub timeout_block_height: u32,
    pub onchain_amount: u32,
    pub blinding_key: Option<String>,
}
impl InternalCreateReverseResponse {
    pub(crate) fn try_convert_from_boltz(
        boltz_create_response: &CreateReverseResponse,
        expected_swap_id: &str,
        expected_invoice: &str,
    ) -> Result<Self, PaymentError> {
        // Do not store the CreateResponse fields that are already stored separately
        // Before skipping them, ensure they match the separately stored ones
        ensure_sdk!(
            boltz_create_response.id == expected_swap_id,
            PaymentError::PersistError
        );
        ensure_sdk!(
            boltz_create_response.invoice == expected_invoice,
            PaymentError::PersistError
        );

        let res = InternalCreateReverseResponse {
            swap_tree: boltz_create_response.swap_tree.clone().into(),
            lockup_address: boltz_create_response.lockup_address.clone(),
            refund_public_key: boltz_create_response.refund_public_key.to_string(),
            timeout_block_height: boltz_create_response.timeout_block_height,
            onchain_amount: boltz_create_response.onchain_amount,
            blinding_key: boltz_create_response.blinding_key.clone(),
        };
        Ok(res)
    }
}
