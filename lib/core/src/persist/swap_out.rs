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
                receiver_amount_sat,
                claim_fees_sat,
                claim_txid
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_out.id,
            swap_out.preimage,
            swap_out.redeem_script,
            swap_out.blinding_key,
            swap_out.invoice,
            swap_out.receiver_amount_sat,
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
                id,
                preimage,
                redeem_script,
                blinding_key,
                invoice,
                receiver_amount_sat,
                claim_fees_sat,
                claim_txid,
                created_at
            FROM receive_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_swap_out(con: &Connection, id: &str) -> rusqlite::Result<Option<SwapOut>> {
        let query = Self::list_swap_out_query(vec!["id = ?1"]);
        con.query_row(&query, [id], Self::sql_row_to_swap_out)
            .optional()
    }

    fn sql_row_to_swap_out(row: &Row) -> rusqlite::Result<SwapOut> {
        Ok(SwapOut {
            id: row.get(0)?,
            preimage: row.get(1)?,
            redeem_script: row.get(2)?,
            blinding_key: row.get(3)?,
            invoice: row.get(4)?,
            receiver_amount_sat: row.get(5)?,
            claim_fees_sat: row.get(6)?,
            claim_txid: row.get(7)?,
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
            .filter(|swap| swap.calculate_status() == SwapOutStatus::Pending)
            .collect();

        Ok(filtered)
    }
}
