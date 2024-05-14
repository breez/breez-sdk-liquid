use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};

impl Persister {
    pub(crate) fn insert_or_update_ongoing_swap_out(&self, swap_out: OngoingSwapOut) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT OR REPLACE INTO receive_swaps (
                id,
                preimage,
                redeem_script,
                blinding_key,
                invoice,
                receiver_amount_sat,
                claim_fees_sat
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_out.id,
            swap_out.preimage,
            swap_out.redeem_script,
            swap_out.blinding_key,
            swap_out.invoice,
            swap_out.receiver_amount_sat,
            swap_out.claim_fees_sat,
        ))?;

        Ok(())
    }

    fn list_ongoing_swap_out_query(where_clauses: Vec<&str>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                preimage,
                redeem_script,
                blinding_key,
                invoice,
                receiver_amount_sat,
                claim_fees_sat,
                created_at
            FROM receive_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_ongoing_swap_out(
        con: &Connection,
        id: &str,
    ) -> rusqlite::Result<Option<OngoingSwapOut>> {
        let query = Self::list_ongoing_swap_out_query(vec!["id = ?1"]);
        con.query_row(&query, [id], Self::sql_row_to_ongoing_swap_out)
            .optional()
    }

    fn sql_row_to_ongoing_swap_out(row: &Row) -> rusqlite::Result<OngoingSwapOut> {
        Ok(OngoingSwapOut {
            id: row.get(0)?,
            preimage: row.get(1)?,
            redeem_script: row.get(2)?,
            blinding_key: row.get(3)?,
            invoice: row.get(4)?,
            receiver_amount_sat: row.get(5)?,
            claim_fees_sat: row.get(6)?,
        })
    }

    pub(crate) fn list_ongoing_receive(
        &self,
        con: &Connection,
        where_clauses: Vec<&str>,
    ) -> rusqlite::Result<Vec<OngoingSwapOut>> {
        let query = Self::list_ongoing_swap_out_query(where_clauses);
        let ongoing_receive = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_ongoing_swap_out)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_receive)
    }
}
