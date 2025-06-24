use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz::CreateSubmarineResponse;
use rusqlite::{named_params, params, Connection, Row};
use sdk_common::bitcoin::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};

use crate::error::PaymentError;
use crate::model::*;
use crate::persist::{get_where_clause_state_in, Persister};
use crate::sync::model::data::SendSyncData;
use crate::sync::model::RecordType;
use crate::{ensure_sdk, get_updated_fields};

use super::where_clauses_to_string;

impl Persister {
    pub(crate) fn insert_or_update_send_swap_inner(
        con: &Connection,
        send_swap: &SendSwap,
    ) -> Result<()> {
        let id_hash = sha256::Hash::hash(send_swap.id.as_bytes()).to_hex();
        con.execute(
            "
            INSERT INTO send_swaps (
                id,
                id_hash,
                invoice,
                bolt12_offer,
                payment_hash,
                destination_pubkey,
                timeout_block_height,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                refund_private_key,
                created_at,
                state,
                pair_fees_json
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT DO NOTHING
            ",
            (
                &send_swap.id,
                &id_hash,
                &send_swap.invoice,
                &send_swap.bolt12_offer,
                &send_swap.payment_hash,
                &send_swap.destination_pubkey,
                &send_swap.timeout_block_height,
                &send_swap.payer_amount_sat,
                &send_swap.receiver_amount_sat,
                &send_swap.create_response_json,
                &send_swap.refund_private_key,
                &send_swap.created_at,
                &send_swap.state,
                &send_swap.pair_fees_json,
            ),
        )?;

        let rows_affected = con.execute(
            "UPDATE send_swaps 
            SET
                description = :description,
                payer_note = :payer_note,
                preimage = :preimage,
                lockup_tx_id = :lockup_tx_id,
                refund_address = :refund_address,
                refund_tx_id = :refund_tx_id,
                state = :state
            WHERE
                id = :id AND
                version = :version",
            named_params! {
                ":id": &send_swap.id,
                ":description": &send_swap.description,
                ":payer_note": &send_swap.payer_note,
                ":preimage": &send_swap.preimage,
                ":lockup_tx_id": &send_swap.lockup_tx_id,
                ":refund_address": &send_swap.refund_address,
                ":refund_tx_id": &send_swap.refund_tx_id,
                ":state": &send_swap.state,
                ":version": &send_swap.metadata.version,
            },
        )?;
        ensure_sdk!(
            rows_affected > 0,
            anyhow!("Version mismatch for send swap {}", send_swap.id)
        );

        Ok(())
    }

    pub(crate) fn insert_or_update_send_swap(&self, send_swap: &SendSwap) -> Result<()> {
        let maybe_swap = self.fetch_send_swap_by_id(&send_swap.id)?;
        let updated_fields = SendSyncData::updated_fields(maybe_swap, send_swap);

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        Self::insert_or_update_send_swap_inner(&tx, send_swap)?;

        // Trigger a sync if:
        // - updated_fields is None (swap is inserted, not updated)
        // - updated_fields in a non empty list of updated fields
        let trigger_sync = updated_fields.as_ref().is_none_or(|u| !u.is_empty());
        match trigger_sync {
            true => {
                self.commit_outgoing(&tx, &send_swap.id, RecordType::Send, updated_fields)?;
                tx.commit()?;
                self.trigger_sync();
            }
            false => {
                tx.commit()?;
            }
        };

        Ok(())
    }

    pub(crate) fn update_send_swaps_by_state(
        &self,
        from_state: PaymentState,
        to_state: PaymentState,
        is_local: Option<bool>,
    ) -> Result<()> {
        let con = self.get_connection()?;
        let mut where_clauses = vec!["state = :from_state".to_string()];
        if let Some(is_local) = is_local {
            let mut where_is_local = format!("sync_state.is_local = {}", is_local as u8);
            if is_local {
                where_is_local = format!("({} OR sync_state.is_local IS NULL)", where_is_local);
            }
            where_clauses.push(where_is_local);
        }

        let where_clause_str = where_clauses_to_string(where_clauses);
        let query = format!(
            "
            UPDATE send_swaps
            SET state = :to_state
            WHERE id IN (
                SELECT id
                FROM send_swaps AS ss
                LEFT JOIN sync_state ON ss.id = sync_state.data_id
                {where_clause_str}
                ORDER BY created_at
            )
            "
        );

        con.execute(
            &query,
            named_params! {
                ":from_state": from_state,
                ":to_state": to_state,
            },
        )?;

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
                bolt12_offer,
                payment_hash,
                destination_pubkey,
                timeout_block_height,
                description,
                payer_note,
                preimage,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                refund_private_key,
                lockup_tx_id,
                refund_address,
                refund_tx_id,
                created_at,
                state,
                pair_fees_json,
                version,
                last_updated_at,

                sync_state.is_local
            FROM send_swaps AS ss
            LEFT JOIN sync_state ON ss.id = sync_state.data_id
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_send_swap_by_id(&self, id: &str) -> Result<Option<SendSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_send_swaps_query(vec!["id = ?1 or id_hash = ?1".to_string()]);
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
            bolt12_offer: row.get(2)?,
            payment_hash: row.get(3)?,
            destination_pubkey: row.get(4)?,
            timeout_block_height: row.get(5)?,
            description: row.get(6)?,
            payer_note: row.get(7)?,
            preimage: row.get(8)?,
            payer_amount_sat: row.get(9)?,
            receiver_amount_sat: row.get(10)?,
            create_response_json: row.get(11)?,
            refund_private_key: row.get(12)?,
            lockup_tx_id: row.get(13)?,
            refund_address: row.get(14)?,
            refund_tx_id: row.get(15)?,
            created_at: row.get(16)?,
            state: row.get(17)?,
            pair_fees_json: row.get(18)?,
            metadata: SwapMetadata {
                version: row.get(19)?,
                last_updated_at: row.get(20)?,
                is_local: row.get::<usize, Option<bool>>(21)?.unwrap_or(true),
            },
        })
    }

    pub(crate) fn list_send_swaps_where(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> Result<Vec<SendSwap>> {
        let query = Self::list_send_swaps_query(where_clauses);
        let ongoing_send = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_send_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_send)
    }

    pub(crate) fn list_send_swaps_by_state(
        &self,
        states: Vec<PaymentState>,
    ) -> Result<Vec<SendSwap>> {
        let con = self.get_connection()?;
        let where_clause = vec![get_where_clause_state_in(&states)];
        self.list_send_swaps_where(&con, where_clause)
    }

    pub(crate) fn list_ongoing_send_swaps(&self) -> Result<Vec<SendSwap>> {
        self.list_send_swaps_by_state(vec![PaymentState::Created, PaymentState::Pending])
    }

    pub(crate) fn list_pending_send_swaps(&self) -> Result<Vec<SendSwap>> {
        self.list_send_swaps_by_state(vec![PaymentState::Pending, PaymentState::RefundPending])
    }

    pub(crate) fn list_recoverable_send_swaps(&self) -> Result<Vec<SendSwap>> {
        self.list_send_swaps_by_state(vec![
            PaymentState::Created,
            PaymentState::Pending,
            PaymentState::RefundPending,
        ])
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
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE send_swaps
            SET
                preimage = COALESCE(preimage, :preimage),
                lockup_tx_id = COALESCE(lockup_tx_id, :lockup_tx_id),
                refund_tx_id = COALESCE(refund_tx_id, :refund_tx_id),
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
        )?;

        let updated_fields = get_updated_fields!(preimage);
        self.commit_outgoing(&tx, swap_id, RecordType::Send, updated_fields)?;
        tx.commit()?;

        self.trigger_sync();

        Ok(())
    }

    pub(crate) fn set_send_swap_refund_address(
        &self,
        swap_id: &str,
        refund_address: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE send_swaps
            SET refund_address = :refund_address
            WHERE id = :id",
            named_params! {
                        ":id": swap_id,
                        ":refund_address": refund_address,
            },
        )?;
        Ok(())
    }

    pub(crate) fn set_send_swap_lockup_tx_id(
        &self,
        swap_id: &str,
        lockup_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;

        let row_count = con
            .execute(
                "UPDATE send_swaps
                SET lockup_tx_id = :lockup_tx_id
                WHERE id = :id AND lockup_tx_id IS NULL",
                named_params! {
                    ":id": swap_id,
                    ":lockup_tx_id": lockup_tx_id,
                },
            )
            .map_err(|_| PaymentError::PersistError)?;
        match row_count {
            1 => Ok(()),
            _ => Err(PaymentError::PaymentInProgress),
        }
    }

    pub(crate) fn unset_send_swap_lockup_tx_id(
        &self,
        swap_id: &str,
        lockup_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE send_swaps
            SET lockup_tx_id = NULL
            WHERE id = :id AND lockup_tx_id = :lockup_tx_id",
            named_params! {
                ":id": swap_id,
                ":lockup_tx_id": lockup_tx_id,
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
    pub(crate) referral_id: Option<String>,
    pub(crate) swap_tree: InternalSwapTree,
    #[serde(default)]
    pub(crate) timeout_block_height: u64,
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
            referral_id: boltz_create_response.referral_id.clone(),
            swap_tree: boltz_create_response.swap_tree.clone().into(),
            timeout_block_height: boltz_create_response.timeout_block_height,
            blinding_key: boltz_create_response.blinding_key.clone(),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::persist::{create_persister, new_send_swap};
    use anyhow::{anyhow, Result};

    use super::PaymentState;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_fetch_send_swap() -> Result<()> {
        create_persister!(storage);
        let send_swap = new_send_swap(None, None);

        storage.insert_or_update_send_swap(&send_swap)?;
        // Fetch swap by id
        assert!(storage.fetch_send_swap_by_id(&send_swap.id).is_ok());
        // Fetch swap by invoice
        assert!(storage
            .fetch_send_swap_by_invoice(&send_swap.invoice)
            .is_ok());

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_list_send_swap() -> Result<()> {
        create_persister!(storage);

        // List general send swaps
        let range = 0..3;
        for _ in range.clone() {
            storage.insert_or_update_send_swap(&new_send_swap(None, None))?;
        }

        let con = storage.get_connection()?;
        let swaps = storage.list_send_swaps_where(&con, vec![])?;
        assert_eq!(swaps.len(), range.len());

        // List ongoing send swaps
        storage.insert_or_update_send_swap(&new_send_swap(Some(PaymentState::Pending), None))?;
        let ongoing_swaps = storage.list_ongoing_send_swaps()?;
        assert_eq!(ongoing_swaps.len(), 4);

        // List pending send swaps
        let ongoing_swaps = storage.list_pending_send_swaps()?;
        assert_eq!(ongoing_swaps.len(), 1);

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_update_send_swap() -> Result<()> {
        create_persister!(storage);

        let mut send_swap = new_send_swap(None, None);
        storage.insert_or_update_send_swap(&send_swap)?;

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
        storage.update_send_swaps_by_state(send_swap.state, PaymentState::Complete, None)?;
        let updated_send_swap = storage
            .fetch_send_swap_by_id(&send_swap.id)?
            .ok_or(anyhow!("Could not find Send swap in database"))?;
        assert_eq!(new_state, updated_send_swap.state);

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_writing_stale_swap() -> Result<()> {
        create_persister!(storage);

        let send_swap = new_send_swap(None, None);
        storage.insert_or_update_send_swap(&send_swap)?;

        // read - update - write works if there are no updates in between
        let mut send_swap = storage.fetch_send_swap_by_id(&send_swap.id)?.unwrap();
        send_swap.refund_tx_id = Some("tx_id".to_string());
        storage.insert_or_update_send_swap(&send_swap)?;

        // read - update - write works if there are no updates in between even if no field changes
        let send_swap = storage.fetch_send_swap_by_id(&send_swap.id)?.unwrap();
        storage.insert_or_update_send_swap(&send_swap)?;

        // read - update - write fails if there are any updates in between
        let mut send_swap = storage.fetch_send_swap_by_id(&send_swap.id)?.unwrap();
        send_swap.refund_tx_id = Some("tx_id_2".to_string());
        // Concurrent update
        storage.set_send_swap_lockup_tx_id(&send_swap.id, "tx_id")?;
        assert!(storage.insert_or_update_send_swap(&send_swap).is_err());

        Ok(())
    }
}
