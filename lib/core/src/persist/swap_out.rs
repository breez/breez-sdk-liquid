use crate::error::PaymentError;
use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use boltz_client::swaps::boltzv2::CreateReverseResponse;
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

impl Persister {
    pub(crate) fn set_claim_tx_id_for_swap_out(
        &self,
        swap_out_id: &str,
        claim_tx_id: &str,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE receive_swaps SET claim_tx_id=:claim_tx_id WHERE id=:id",
            named_params! {
             ":id": swap_out_id,
             ":claim_tx_id": claim_tx_id,
            },
        )?;

        Ok(())
    }

    pub(crate) fn insert_or_update_swap_out(&self, swap_out: SwapOut) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT OR REPLACE INTO receive_swaps (
                id,
                preimage,
                create_response_json,
                blinding_key,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                created_at,
                claim_fees_sat,
                claim_tx_id
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_out.id,
            swap_out.preimage,
            swap_out.create_response_json,
            swap_out.blinding_key,
            swap_out.invoice,
            swap_out.payer_amount_sat,
            swap_out.receiver_amount_sat,
            swap_out.created_at,
            swap_out.claim_fees_sat,
            swap_out.claim_tx_id,
        ))?;

        Ok(())
    }

    fn list_swap_out_query(where_clauses: Vec<&str>) -> String {
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
                rs.blinding_key,
                rs.invoice,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.claim_fees_sat,
                rs.claim_tx_id,
                rs.created_at,
                ptx.status
            FROM receive_swaps AS rs
            LEFT JOIN payment_tx_data AS ptx
                ON ptx.tx_id = rs.claim_tx_id
            {where_clause_str}
            ORDER BY rs.created_at
        "
        )
    }

    pub(crate) fn fetch_swap_out(con: &Connection, id: &str) -> rusqlite::Result<Option<SwapOut>> {
        let query = Self::list_swap_out_query(vec!["id = ?1"]);
        con.query_row(&query, [id], Self::sql_row_to_swap_out)
            .optional()
    }

    fn sql_row_to_swap_out(row: &Row) -> rusqlite::Result<SwapOut> {
        let maybe_payment_status: Option<PaymentStatus> = row.get(10)?;
        Ok(SwapOut {
            id: row.get(0)?,
            preimage: row.get(1)?,
            create_response_json: row.get(2)?,
            blinding_key: row.get(3)?,
            invoice: row.get(4)?,
            payer_amount_sat: row.get(5)?,
            receiver_amount_sat: row.get(6)?,
            claim_fees_sat: row.get(7)?,
            claim_tx_id: row.get(8)?,
            created_at: row.get(9)?,
            is_claim_tx_confirmed: match maybe_payment_status {
                Some(payment_status) => match payment_status {
                    PaymentStatus::Pending => false,
                    PaymentStatus::Complete => true,
                },
                None => false,
            },
        })
    }

    pub(crate) fn list_receive_swaps(
        &self,
        con: &Connection,
        where_clauses: Vec<&str>,
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
        let swap_outs = self.list_receive_swaps(con, vec![])?;

        let filtered: Vec<SwapOut> = swap_outs
            .into_iter()
            .filter(|swap| swap.calculate_status() != SwapOutStatus::Completed)
            .collect();

        Ok(filtered)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateReverseResponse {
    pub id: String,
    pub invoice: String,
    pub swap_tree: InternalSwapTree,
    pub lockup_address: String,
    pub refund_public_key: String,
    pub timeout_block_height: u32,
    pub onchain_amount: u32,
    pub blinding_key: Option<String>,
}
impl InternalCreateReverseResponse {
    pub(crate) fn convert_from_boltz(boltz_create_response: &CreateReverseResponse) -> Self {
        InternalCreateReverseResponse {
            id: boltz_create_response.id.clone(),
            invoice: boltz_create_response.invoice.clone(),
            swap_tree: boltz_create_response.swap_tree.clone().into(),
            lockup_address: boltz_create_response.lockup_address.clone(),
            refund_public_key: boltz_create_response.refund_public_key.to_string(),
            timeout_block_height: boltz_create_response.timeout_block_height,
            onchain_amount: boltz_create_response.onchain_amount,
            blinding_key: boltz_create_response.blinding_key.clone(),
        }
    }

    pub(crate) fn convert_to_boltz(&self) -> Result<CreateReverseResponse, PaymentError> {
        let res = CreateReverseResponse {
            id: self.id.clone(),
            invoice: self.invoice.clone(),
            swap_tree: self.swap_tree.clone().into(),
            lockup_address: self.lockup_address.clone(),
            refund_public_key: crate::utils::json_to_pubkey(&self.refund_public_key)?,
            timeout_block_height: self.timeout_block_height,
            onchain_amount: self.onchain_amount,
            blinding_key: self.blinding_key.clone(),
        };

        Ok(res)
    }
}
