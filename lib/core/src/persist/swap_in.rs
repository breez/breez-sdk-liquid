use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};

impl Persister {
    pub(crate) fn set_lockup_txid_for_swap_in(
        &self,
        swap_in_id: &str,
        lockup_txid: &str,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE send_swaps SET lockup_txid=:lockup_txid WHERE id=:id",
            named_params! {
             ":id": swap_in_id,
             ":lockup_txid": lockup_txid,
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
                lockup_txid,
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
            swap_in.lockup_txid,
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
                lockup_txid,
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
            lockup_txid: row.get(5)?,
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
