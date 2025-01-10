use anyhow::Result;
use boltz_client::swaps::boltz::CreateReverseResponse;
use rusqlite::{named_params, params, Connection, Row, TransactionBehavior};
use sdk_common::bitcoin::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::{get_where_clause_state_in, Persister};
use crate::sync::model::data::ReceiveSyncData;
use crate::sync::model::RecordType;

impl Persister {
    pub(crate) fn insert_or_update_receive_swap_inner(
        con: &Connection,
        receive_swap: &ReceiveSwap,
    ) -> Result<()> {
        let id_hash = sha256::Hash::hash(receive_swap.id.as_bytes()).to_hex();
        con.execute(
            "
            INSERT INTO receive_swaps (
                id,
                id_hash,
                preimage,
                create_response_json,
                claim_private_key,
                invoice,
                timeout_block_height,
                payment_hash,
                destination_pubkey,
                payer_amount_sat,
                receiver_amount_sat,
                created_at,
                claim_fees_sat,
                mrh_address,
                state,
                pair_fees_json
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT DO NOTHING
            ",
            (
                &receive_swap.id,
                id_hash,
                &receive_swap.preimage,
                &receive_swap.create_response_json,
                &receive_swap.claim_private_key,
                &receive_swap.invoice,
                &receive_swap.timeout_block_height,
                &receive_swap.payment_hash,
                &receive_swap.destination_pubkey,
                &receive_swap.payer_amount_sat,
                &receive_swap.receiver_amount_sat,
                &receive_swap.created_at,
                &receive_swap.claim_fees_sat,
                &receive_swap.mrh_address,
                &receive_swap.state,
                &receive_swap.pair_fees_json,
            ),
        )?;

        con.execute(
            "UPDATE receive_swaps
            SET
                description = :description,
                claim_tx_id = :claim_tx_id,
                lockup_tx_id = :lockup_tx_id,
                mrh_tx_id = :mrh_tx_id,
                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": &receive_swap.id,
                ":description": &receive_swap.description,
                ":claim_tx_id": &receive_swap.claim_tx_id,
                ":lockup_tx_id": &receive_swap.lockup_tx_id,
                ":mrh_tx_id": &receive_swap.mrh_tx_id,
                ":state": &receive_swap.state,
            },
        )?;

        if receive_swap.mrh_tx_id.is_some() {
            Self::delete_reserved_address_inner(con, &receive_swap.mrh_address)?;
        }

        Ok(())
    }

    pub(crate) fn insert_or_update_receive_swap(&self, receive_swap: &ReceiveSwap) -> Result<()> {
        let maybe_swap = self.fetch_receive_swap_by_id(&receive_swap.id)?;
        let updated_fields = ReceiveSyncData::updated_fields(maybe_swap, receive_swap);

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        Self::insert_or_update_receive_swap_inner(&tx, receive_swap)?;

        // Trigger a sync if:
        // - updated_fields is None (swap is inserted, not updated)
        // - updated_fields in a non empty list of updated fields
        let trigger_sync = updated_fields.as_ref().map_or(true, |u| !u.is_empty());
        match trigger_sync {
            true => {
                self.commit_outgoing(&tx, &receive_swap.id, RecordType::Receive, updated_fields)?;
                tx.commit()?;
                self.sync_trigger.try_send(())?;
            }
            false => {
                tx.commit()?;
            }
        };

        Ok(())
    }

    fn list_receive_swaps_query(where_clauses: Vec<String>) -> String {
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
                rs.claim_private_key,
                rs.invoice,
                rs.payment_hash,
                rs.destination_pubkey,
                rs.timeout_block_height,
                rs.description,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.claim_fees_sat,
                rs.claim_tx_id,
                rs.lockup_tx_id,
                rs.mrh_address,
                rs.mrh_tx_id,
                rs.created_at,
                rs.state,
                rs.pair_fees_json
            FROM receive_swaps AS rs
            {where_clause_str}
            ORDER BY rs.created_at
        "
        )
    }

    pub(crate) fn fetch_receive_swap_by_id(&self, id: &str) -> Result<Option<ReceiveSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_receive_swaps_query(vec!["id = ?1 or id_hash = ?1".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_receive_swap);

        Ok(res.ok())
    }

    pub(crate) fn fetch_receive_swap_by_invoice(
        &self,
        invoice: &str,
    ) -> Result<Option<ReceiveSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_receive_swaps_query(vec!["invoice= ?1".to_string()]);
        let res = con.query_row(&query, [invoice], Self::sql_row_to_receive_swap);

        Ok(res.ok())
    }

    fn sql_row_to_receive_swap(row: &Row) -> rusqlite::Result<ReceiveSwap> {
        Ok(ReceiveSwap {
            id: row.get(0)?,
            preimage: row.get(1)?,
            create_response_json: row.get(2)?,
            claim_private_key: row.get(3)?,
            invoice: row.get(4)?,
            payment_hash: row.get(5)?,
            destination_pubkey: row.get(6)?,
            timeout_block_height: row.get(7)?,
            description: row.get(8)?,
            payer_amount_sat: row.get(9)?,
            receiver_amount_sat: row.get(10)?,
            claim_fees_sat: row.get(11)?,
            claim_tx_id: row.get(12)?,
            lockup_tx_id: row.get(13)?,
            mrh_address: row.get(14)?,
            mrh_tx_id: row.get(15)?,
            created_at: row.get(16)?,
            state: row.get(17)?,
            pair_fees_json: row.get(18)?,
        })
    }

    pub(crate) fn list_receive_swaps_where(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> Result<Vec<ReceiveSwap>> {
        let query = Self::list_receive_swaps_query(where_clauses);
        let ongoing_receive = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_receive_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_receive)
    }

    pub(crate) fn list_ongoing_receive_swaps(&self) -> Result<Vec<ReceiveSwap>> {
        let con = self.get_connection()?;
        let where_clause = vec![get_where_clause_state_in(&[
            PaymentState::Created,
            PaymentState::Pending,
        ])];

        self.list_receive_swaps_where(&con, where_clause)
    }

    pub(crate) fn list_recoverable_receive_swaps(&self) -> Result<Vec<ReceiveSwap>> {
        let con = self.get_connection()?;
        let where_clause = vec![get_where_clause_state_in(&[
            PaymentState::Created,
            PaymentState::Pending,
        ])];

        self.list_receive_swaps_where(&con, where_clause)
    }

    // Only set the Receive Swap claim_tx_id if not set, otherwise return an error
    pub(crate) fn set_receive_swap_claim_tx_id(
        &self,
        swap_id: &str,
        claim_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        let row_count = con
            .execute(
                "UPDATE receive_swaps 
            SET claim_tx_id = :claim_tx_id
            WHERE id = :id AND claim_tx_id IS NULL",
                named_params! {
                            ":id": swap_id,
                            ":claim_tx_id": claim_tx_id,
                },
            )
            .map_err(|_| PaymentError::PersistError)?;
        match row_count {
            1 => Ok(()),
            _ => Err(PaymentError::AlreadyClaimed),
        }
    }

    // Only unset the Receive Swap claim_tx_id if set with the same tx id
    pub(crate) fn unset_receive_swap_claim_tx_id(
        &self,
        swap_id: &str,
        claim_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE receive_swaps 
            SET claim_tx_id = NULL
            WHERE id = :id AND claim_tx_id = :claim_tx_id",
            named_params! {
                        ":id": swap_id,
                        ":claim_tx_id": claim_tx_id,
            },
        )
        .map_err(|_| PaymentError::PersistError)?;
        Ok(())
    }

    pub(crate) fn try_handle_receive_swap_update(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
        lockup_tx_id: Option<&str>,
        mrh_tx_id: Option<&str>,
        mrh_amount_sat: Option<u64>,
    ) -> Result<(), PaymentError> {
        // Do not overwrite claim_tx_id or lockup_tx_id
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE receive_swaps
            SET
                claim_tx_id =
                    CASE
                        WHEN claim_tx_id IS NULL THEN :claim_tx_id
                        ELSE claim_tx_id
                    END,
                lockup_tx_id = 
                    CASE
                        WHEN lockup_tx_id IS NULL THEN :lockup_tx_id
                        ELSE lockup_tx_id
                    END,
                mrh_tx_id = 
                    CASE
                        WHEN mrh_tx_id IS NULL THEN :mrh_tx_id
                        ELSE mrh_tx_id
                    END,
                payer_amount_sat = COALESCE(:mrh_amount_sat, payer_amount_sat),
                receiver_amount_sat = COALESCE(:mrh_amount_sat, receiver_amount_sat),
                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":lockup_tx_id": lockup_tx_id,
                ":claim_tx_id": claim_tx_id,
                ":mrh_tx_id": mrh_tx_id,
                ":mrh_amount_sat": mrh_amount_sat,
                ":state": to_state,
            },
        )?;

        // NOTE: Receive currently does not update any fields, bypassing the commit logic for now
        // let updated_fields = None;
        // Self::commit_outgoing(&tx, swap_id, RecordType::Receive, updated_fields)?;
        // self.sync_trigger
        //     .try_send(())
        //     .map_err(|err| PaymentError::Generic {
        //         err: format!("Could not trigger manual sync: {err:?}"),
        //     })?;

        tx.commit()?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateReverseResponse {
    pub swap_tree: InternalSwapTree,
    pub lockup_address: String,
    pub refund_public_key: String,
    pub timeout_block_height: u32,
    pub onchain_amount: u64,
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

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::test_utils::persist::{create_persister, new_receive_swap};

    use super::PaymentState;

    #[test]
    fn test_fetch_receive_swap() -> Result<()> {
        create_persister!(storage);

        let receive_swap = new_receive_swap(None);

        storage.insert_or_update_receive_swap(&receive_swap)?;
        // Fetch swap by id
        assert!(storage.fetch_receive_swap_by_id(&receive_swap.id).is_ok());
        // Fetch swap by invoice
        assert!(storage
            .fetch_receive_swap_by_invoice(&receive_swap.invoice)
            .is_ok());

        Ok(())
    }

    #[test]
    fn test_list_receive_swap() -> Result<()> {
        create_persister!(storage);

        // List general receive swaps
        let range = 0..3;
        for _ in range.clone() {
            storage.insert_or_update_receive_swap(&new_receive_swap(None))?;
        }

        let con = storage.get_connection()?;
        let swaps = storage.list_receive_swaps_where(&con, vec![])?;
        assert_eq!(swaps.len(), range.len());

        // List ongoing receive swaps
        storage.insert_or_update_receive_swap(&new_receive_swap(Some(PaymentState::Pending)))?;
        let ongoing_swaps = storage.list_ongoing_receive_swaps()?;
        assert_eq!(ongoing_swaps.len(), 4);

        Ok(())
    }

    #[test]
    fn test_update_receive_swap() -> Result<()> {
        create_persister!(storage);

        let receive_swap = new_receive_swap(None);
        storage.insert_or_update_receive_swap(&receive_swap)?;

        // Update metadata
        let new_state = PaymentState::Pending;
        let claim_tx_id = Some("claim_tx_id");

        storage.try_handle_receive_swap_update(
            &receive_swap.id,
            new_state,
            claim_tx_id,
            None,
            None,
            None,
        )?;

        let updated_receive_swap = storage
            .fetch_receive_swap_by_id(&receive_swap.id)?
            .ok_or(anyhow!("Could not find Receive swap in database"))?;

        assert_eq!(new_state, updated_receive_swap.state);
        assert_eq!(claim_tx_id, updated_receive_swap.claim_tx_id.as_deref());

        Ok(())
    }
}
