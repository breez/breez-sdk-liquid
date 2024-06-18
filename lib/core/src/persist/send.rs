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
    pub(crate) fn insert_send_swap(&self, send_swap: &SendSwap) -> Result<()> {
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
            &send_swap.id,
            &send_swap.invoice,
            &send_swap.payer_amount_sat,
            &send_swap.receiver_amount_sat,
            &send_swap.create_response_json,
            &send_swap.refund_private_key,
            &send_swap.lockup_tx_id,
            &send_swap.refund_tx_id,
            &send_swap.created_at,
            &send_swap.state,
        ))?;

        Ok(())
    }

    pub(crate) fn update_send_swaps_by_state(
        &self,
        from_state: PaymentState,
        to_state: PaymentState,
    ) -> Result<()> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE send_swaps
            SET
                state = :to_state
            WHERE
                state = :from_state
            ",
            named_params! {
                ":from_state": from_state,
                ":to_state": to_state,
            },
        )
        .map_err(|_| PaymentError::PersistError)?;

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
                preimage,
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

    pub(crate) fn fetch_send_swap_by_id(&self, id: &str) -> Result<Option<SendSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_send_swaps_query(vec!["id = ?1".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_send_swap);

        Ok(res.ok())
    }

    pub(crate) fn fetch_send_swap_by_invoice(&self, invoice: &str) -> Result<Option<SendSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_send_swaps_query(vec!["invoice= ?1".to_string()]);
        let res = con.query_row(&query, [invoice], Self::sql_row_to_send_swap);

        Ok(res.ok())
    }

    fn sql_row_to_send_swap(row: &Row) -> rusqlite::Result<SendSwap> {
        Ok(SendSwap {
            id: row.get(0)?,
            invoice: row.get(1)?,
            preimage: row.get(2)?,
            payer_amount_sat: row.get(3)?,
            receiver_amount_sat: row.get(4)?,
            create_response_json: row.get(5)?,
            refund_private_key: row.get(6)?,
            lockup_tx_id: row.get(7)?,
            refund_tx_id: row.get(8)?,
            created_at: row.get(9)?,
            state: row.get(10)?,
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

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::test_utils::{new_send_swap, new_temp_persister};

    use super::PaymentState;

    #[test]
    fn test_fetch_send_swap() -> Result<()> {
        let storage = &new_temp_persister()?.persister;
        let send_swap = new_send_swap(None);

        storage.insert_send_swap(&send_swap)?;
        // Fetch swap by id
        assert!(storage.fetch_send_swap_by_id(&send_swap.id).is_ok());
        // Fetch swap by invoice
        assert!(storage
            .fetch_send_swap_by_invoice(&send_swap.invoice)
            .is_ok());

        Ok(())
    }

    #[test]
    fn test_list_send_swap() -> Result<()> {
        let storage = &new_temp_persister()?.persister;

        // List general send swaps
        let range = 0..3;
        for _ in range.clone() {
            storage.insert_send_swap(&new_send_swap(None))?;
        }

        let con = storage.get_connection()?;
        let swaps = storage.list_send_swaps(&con, vec![])?;
        assert_eq!(swaps.len(), range.len());

        // List ongoing send swaps
        storage.insert_send_swap(&new_send_swap(Some(PaymentState::Pending)))?;
        let ongoing_swaps = storage.list_ongoing_send_swaps(&con)?;
        assert_eq!(ongoing_swaps.len(), 4);

        // List pending send swaps
        let ongoing_swaps = storage.list_pending_send_swaps()?;
        assert_eq!(ongoing_swaps.len(), 1);

        Ok(())
    }

    #[test]
    fn test_update_send_swap() -> Result<()> {
        let storage = &new_temp_persister()?.persister;

        let mut send_swap = new_send_swap(None);
        storage.insert_send_swap(&send_swap)?;

        // Update metadata
        let new_state = PaymentState::Pending;
        let preimage = Some("preimage");
        let lockup_tx_id = Some("lockup_tx_id");
        let refund_tx_id = Some("refund_tx_id");

        storage.try_handle_send_swap_update(
            &send_swap.id,
            new_state,
            preimage,
            lockup_tx_id,
            refund_tx_id,
        )?;

        let updated_send_swap = storage
            .fetch_send_swap_by_id(&send_swap.id)?
            .ok_or(anyhow!("Could not find Send swap in database"))?;

        assert_eq!(new_state, updated_send_swap.state);
        assert_eq!(preimage, updated_send_swap.preimage.as_deref());
        assert_eq!(lockup_tx_id, updated_send_swap.lockup_tx_id.as_deref());
        assert_eq!(refund_tx_id, updated_send_swap.refund_tx_id.as_deref());

        send_swap.state = new_state;

        // Update state (global)
        let new_state = PaymentState::Complete;
        storage.update_send_swaps_by_state(send_swap.state, PaymentState::Complete)?;
        let updated_send_swap = storage
            .fetch_send_swap_by_id(&send_swap.id)?
            .ok_or(anyhow!("Could not find Send swap in database"))?;
        assert_eq!(new_state, updated_send_swap.state);

        Ok(())
    }
}
