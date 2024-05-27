use std::collections::HashMap;

use anyhow::Result;
use boltz_client::swaps::boltzv2::CreateSubmarineResponse;
use rusqlite::{named_params, params, Connection, Row};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::Persister;

impl Persister {
    pub(crate) fn insert_send_swap(&self, send_swap: SendSwap) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT INTO send_swaps (
                id,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                refund_private_key,
                lockup_tx_id,
                refund_tx_id,
                created_at,
                state
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            send_swap.id,
            send_swap.invoice,
            send_swap.payer_amount_sat,
            send_swap.receiver_amount_sat,
            send_swap.create_response_json,
            send_swap.refund_private_key,
            send_swap.lockup_tx_id,
            send_swap.refund_tx_id,
            send_swap.created_at,
            send_swap.state,
        ))?;

        Ok(())
    }

    fn list_send_swaps_query(where_clauses: Vec<String>) -> String {
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
                refund_private_key,
                lockup_tx_id,
                refund_tx_id,
                created_at,
                state
            FROM send_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_send_swap(&self, id: &str) -> Result<Option<SendSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_send_swaps_query(vec!["id = ?1".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_send_swap);

        Ok(res.ok())
    }

    fn sql_row_to_send_swap(row: &Row) -> rusqlite::Result<SendSwap> {
        Ok(SendSwap {
            id: row.get(0)?,
            invoice: row.get(1)?,
            payer_amount_sat: row.get(2)?,
            receiver_amount_sat: row.get(3)?,
            create_response_json: row.get(4)?,
            refund_private_key: row.get(5)?,
            lockup_tx_id: row.get(6)?,
            refund_tx_id: row.get(7)?,
            created_at: row.get(8)?,
            state: row.get(9)?,
        })
    }

    pub(crate) fn list_send_swaps(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> rusqlite::Result<Vec<SendSwap>> {
        let query = Self::list_send_swaps_query(where_clauses);
        let ongoing_send = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_send_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_send)
    }

    pub(crate) fn list_ongoing_send_swaps(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<Vec<SendSwap>> {
        let mut where_clause: Vec<String> = Vec::new();
        where_clause.push(format!(
            "state in ({})",
            [PaymentState::Created, PaymentState::Pending]
                .iter()
                .map(|t| format!("'{}'", *t as i8))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        self.list_send_swaps(con, where_clause)
    }

    pub(crate) fn list_pending_send_swaps(&self) -> Result<Vec<SendSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_send_swaps_query(vec!["state = ?1".to_string()]);
        let res = con
            .prepare(&query)?
            .query_map(params![PaymentState::Pending], Self::sql_row_to_send_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(res)
    }

    /// Pending Send swaps, indexed by refund tx id
    pub(crate) fn list_pending_send_swaps_by_refund_tx_id(
        &self,
    ) -> Result<HashMap<String, SendSwap>> {
        let res: HashMap<String, SendSwap> = self
            .list_pending_send_swaps()?
            .iter()
            .filter_map(|pending_send_swap| {
                pending_send_swap
                    .refund_tx_id
                    .as_ref()
                    .map(|refund_tx_id| (refund_tx_id.clone(), pending_send_swap.clone()))
            })
            .collect();
        Ok(res)
    }

    pub(crate) fn try_handle_send_swap_update(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        preimage: Option<&str>,
        lockup_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        // Do not overwrite preimage, lockup_tx_id, refund_tx_id
        let con: Connection = self.get_connection()?;
        con.execute(
            "UPDATE send_swaps
            SET
                preimage =
                    CASE
                        WHEN preimage IS NULL THEN :preimage
                        ELSE preimage
                    END,

                lockup_tx_id =
                    CASE
                        WHEN lockup_tx_id IS NULL THEN :lockup_tx_id
                        ELSE lockup_tx_id
                    END,

                refund_tx_id =
                    CASE
                        WHEN refund_tx_id IS NULL THEN :refund_tx_id
                        ELSE refund_tx_id
                    END,

                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":preimage": preimage,
                ":lockup_tx_id": lockup_tx_id,
                ":refund_tx_id": refund_tx_id,
                ":state": to_state,
            },
        )
        .map_err(|_| PaymentError::PersistError)?;

        Ok(())
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
