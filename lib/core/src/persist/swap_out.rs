use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};

impl Persister {
    pub(crate) fn set_claim_txid_for_swap_out(
        &self,
        swap_out_id: &str,
        claim_txid: &str,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE receive_swaps SET claim_txid=:claim_txid WHERE id=:id",
            named_params! {
             ":id": swap_out_id,
             ":claim_txid": claim_txid,
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
                redeem_script,
                blinding_key,
                invoice,
                payer_amount_sat,
                receiver_amount_sat,
                created_at,
                claim_fees_sat,
                claim_txid
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_out.id,
            swap_out.preimage,
            swap_out.redeem_script,
            swap_out.blinding_key,
            swap_out.invoice,
            swap_out.payer_amount_sat,
            swap_out.receiver_amount_sat,
            swap_out.created_at,
            swap_out.claim_fees_sat,
            swap_out.claim_txid,
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
                rs.redeem_script,
                rs.blinding_key,
                rs.invoice,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.claim_fees_sat,
                rs.claim_txid,
                rs.created_at,
                ptx.status
            FROM receive_swaps AS rs
            LEFT JOIN payment_tx_data AS ptx
                ON ptx.txid = rs.claim_txid
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
            redeem_script: row.get(2)?,
            blinding_key: row.get(3)?,
            invoice: row.get(4)?,
            payer_amount_sat: row.get(5)?,
            receiver_amount_sat: row.get(6)?,
            claim_fees_sat: row.get(7)?,
            claim_txid: row.get(8)?,
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
