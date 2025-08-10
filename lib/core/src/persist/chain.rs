use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz::{ChainSwapDetails, CreateChainResponse};
use rusqlite::{named_params, params, Connection, Row, TransactionBehavior};
use sdk_common::bitcoin::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::{get_where_clause_state_in, where_clauses_to_string, Persister};
use crate::sync::model::data::ChainSyncData;
use crate::sync::model::RecordType;

impl Persister {
    pub(crate) fn insert_or_update_chain_swap_inner(
        con: &Connection,
        chain_swap: &ChainSwap,
    ) -> Result<()> {
        // There is a limit of 16 param elements in a single tuple in rusqlite,
        // so we split up the insert into two statements.
        let id_hash = sha256::Hash::hash(chain_swap.id.as_bytes()).to_hex();
        con.execute(
            "
            INSERT INTO chain_swaps (
                id,
                id_hash,
                direction,
                lockup_address,
                timeout_block_height,
                claim_timeout_block_height,
                preimage,
                payer_amount_sat,
                receiver_amount_sat,
                accept_zero_conf,
                create_response_json,
                claim_private_key,
                refund_private_key,
                claim_fees_sat,
                created_at,
                state
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
		    ON CONFLICT DO NOTHING",
            (
                &chain_swap.id,
                &id_hash,
                &chain_swap.direction,
                &chain_swap.lockup_address,
                &chain_swap.timeout_block_height,
                &chain_swap.claim_timeout_block_height,
                &chain_swap.preimage,
                &chain_swap.payer_amount_sat,
                &chain_swap.receiver_amount_sat,
                &chain_swap.accept_zero_conf,
                &chain_swap.create_response_json,
                &chain_swap.claim_private_key,
                &chain_swap.refund_private_key,
                &chain_swap.claim_fees_sat,
                &chain_swap.created_at,
                &chain_swap.state,
            ),
        )?;

        let rows_affected = con.execute(
            "UPDATE chain_swaps
            SET
                description = :description,
                accept_zero_conf = :accept_zero_conf,
                server_lockup_tx_id = :server_lockup_tx_id,
                user_lockup_tx_id = :user_lockup_tx_id,
                claim_address = :claim_address,
                claim_tx_id = :claim_tx_id,
                refund_address = :refund_address,
                refund_tx_id = :refund_tx_id,
                pair_fees_json = :pair_fees_json,
                state = :state,
                actual_payer_amount_sat = :actual_payer_amount_sat,
                accepted_receiver_amount_sat = :accepted_receiver_amount_sat
            WHERE
                id = :id AND
                version = :version",
            named_params! {
                ":id": &chain_swap.id,
                ":description": &chain_swap.description,
                ":accept_zero_conf": &chain_swap.accept_zero_conf,
                ":server_lockup_tx_id": &chain_swap.server_lockup_tx_id,
                ":user_lockup_tx_id": &chain_swap.user_lockup_tx_id,
                ":claim_address": &chain_swap.claim_address,
                ":claim_tx_id": &chain_swap.claim_tx_id,
                ":refund_address": &chain_swap.refund_address,
                ":refund_tx_id": &chain_swap.refund_tx_id,
                ":pair_fees_json": &chain_swap.pair_fees_json,
                ":state": &chain_swap.state,
                ":actual_payer_amount_sat": &chain_swap.actual_payer_amount_sat,
                ":accepted_receiver_amount_sat": &chain_swap.accepted_receiver_amount_sat,
                ":version": &chain_swap.metadata.version,
            },
        )?;
        ensure_sdk!(
            rows_affected > 0,
            anyhow!("Version mismatch for chain swap {}", chain_swap.id)
        );

        Ok(())
    }

    pub(crate) fn insert_or_update_chain_swap(&self, chain_swap: &ChainSwap) -> Result<()> {
        let maybe_swap = self.fetch_chain_swap_by_id(&chain_swap.id)?;
        let updated_fields = ChainSyncData::updated_fields(maybe_swap, chain_swap);

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        Self::insert_or_update_chain_swap_inner(&tx, chain_swap)?;

        // Trigger a sync if:
        // - updated_fields is None (swap is inserted, not updated)
        // - updated_fields in a non empty list of updated fields
        let trigger_sync = updated_fields.as_ref().is_none_or(|u| !u.is_empty());
        match trigger_sync {
            true => {
                self.commit_outgoing(&tx, &chain_swap.id, RecordType::Chain, updated_fields)?;
                tx.commit()?;
                self.trigger_sync();
            }
            false => {
                tx.commit()?;
            }
        };

        Ok(())
    }

    fn list_chain_swaps_query(where_clauses: Vec<String>) -> String {
        let where_clause_str = where_clauses_to_string(where_clauses);

        format!(
            "
            SELECT
                id,
                direction,
                claim_address,
                lockup_address,
                refund_address,
                timeout_block_height,
                claim_timeout_block_height,
                preimage,
                description,
                payer_amount_sat,
                receiver_amount_sat,
                accept_zero_conf,
                create_response_json,
                claim_private_key,
                refund_private_key,
                server_lockup_tx_id,
                user_lockup_tx_id,
                claim_fees_sat,
                claim_tx_id,
                refund_tx_id,
                created_at,
                state,
                pair_fees_json,
                actual_payer_amount_sat,
                accepted_receiver_amount_sat,
                auto_accepted_fees,
                version,
                last_updated_at,

                sync_state.is_local
            FROM chain_swaps
            LEFT JOIN sync_state ON chain_swaps.id = sync_state.data_id
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_chain_swap_by_id(&self, id: &str) -> Result<Option<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_chain_swaps_query(vec!["id = ?1 or id_hash = ?1".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_chain_swap);

        Ok(res.ok())
    }

    pub(crate) fn fetch_chain_swap_by_lockup_address(
        &self,
        lockup_address: &str,
    ) -> Result<Option<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_chain_swaps_query(vec!["lockup_address = ?1".to_string()]);
        let res = con.query_row(&query, [lockup_address], Self::sql_row_to_chain_swap);

        Ok(res.ok())
    }

    fn sql_row_to_chain_swap(row: &Row) -> rusqlite::Result<ChainSwap> {
        Ok(ChainSwap {
            id: row.get(0)?,
            direction: row.get(1)?,
            claim_address: row.get(2)?,
            lockup_address: row.get(3)?,
            refund_address: row.get(4)?,
            timeout_block_height: row.get(5)?,
            claim_timeout_block_height: row.get(6)?,
            preimage: row.get(7)?,
            description: row.get(8)?,
            payer_amount_sat: row.get(9)?,
            receiver_amount_sat: row.get(10)?,
            accept_zero_conf: row.get(11)?,
            create_response_json: row.get(12)?,
            claim_private_key: row.get(13)?,
            refund_private_key: row.get(14)?,
            server_lockup_tx_id: row.get(15)?,
            user_lockup_tx_id: row.get(16)?,
            claim_fees_sat: row.get(17)?,
            claim_tx_id: row.get(18)?,
            refund_tx_id: row.get(19)?,
            created_at: row.get(20)?,
            state: row.get(21)?,
            pair_fees_json: row.get(22)?,
            actual_payer_amount_sat: row.get(23)?,
            accepted_receiver_amount_sat: row.get(24)?,
            auto_accepted_fees: row.get(25)?,
            metadata: SwapMetadata {
                version: row.get(26)?,
                last_updated_at: row.get(27)?,
                is_local: row.get::<usize, Option<bool>>(28)?.unwrap_or(true),
            },
        })
    }

    pub(crate) fn list_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        self.list_chain_swaps_where(&con, vec![])
    }

    pub(crate) fn list_chain_swaps_where(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> Result<Vec<ChainSwap>> {
        let query = Self::list_chain_swaps_query(where_clauses);
        let chain_swaps = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_chain_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(chain_swaps)
    }

    pub(crate) fn list_chain_swaps_by_state(
        &self,
        states: Vec<PaymentState>,
    ) -> Result<Vec<ChainSwap>> {
        let con = self.get_connection()?;
        let where_clause = vec![get_where_clause_state_in(&states)];
        self.list_chain_swaps_where(&con, where_clause)
    }

    pub(crate) fn list_ongoing_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        self.list_chain_swaps_by_state(vec![
            PaymentState::Created,
            PaymentState::Pending,
            PaymentState::WaitingFeeAcceptance,
        ])
    }

    pub(crate) fn list_pending_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        self.list_chain_swaps_by_state(vec![
            PaymentState::Pending,
            PaymentState::RefundPending,
            PaymentState::WaitingFeeAcceptance,
        ])
    }

    pub(crate) fn list_refundable_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        self.list_chain_swaps_by_state(vec![PaymentState::Refundable, PaymentState::RefundPending])
    }

    pub(crate) fn update_chain_swap_accept_zero_conf(
        &self,
        swap_id: &str,
        accept_zero_conf: bool,
    ) -> Result<(), PaymentError> {
        let mut con: Connection = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE chain_swaps
            SET
                accept_zero_conf = :accept_zero_conf
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":accept_zero_conf": accept_zero_conf,
            },
        )?;
        self.commit_outgoing(
            &tx,
            swap_id,
            RecordType::Chain,
            Some(vec!["accept_zero_conf".to_string()]),
        )?;
        tx.commit()?;
        self.trigger_sync();

        Ok(())
    }

    /// Used for receive chain swaps, when the sender over/underpays
    pub(crate) fn update_actual_payer_amount(
        &self,
        swap_id: &str,
        actual_payer_amount_sat: u64,
    ) -> Result<(), PaymentError> {
        log::info!(
            "Updating chain swap {swap_id}: actual_payer_amount_sat = {actual_payer_amount_sat}"
        );
        let con: Connection = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps 
            SET actual_payer_amount_sat = :actual_payer_amount_sat
            WHERE id = :id",
            named_params! {
                ":id": swap_id,
                ":actual_payer_amount_sat": actual_payer_amount_sat,
            },
        )?;

        Ok(())
    }

    /// Used for receive chain swaps, when fees are accepted and thus the agreed received amount is known
    ///
    /// Can also be used to erase a previously persisted accepted amount in case of failure to accept.
    pub(crate) fn update_accepted_receiver_amount(
        &self,
        swap_id: &str,
        accepted_receiver_amount_sat: Option<u64>,
    ) -> Result<(), PaymentError> {
        log::info!(
            "Updating chain swap {swap_id}: accepted_receiver_amount_sat = {accepted_receiver_amount_sat:?}"
        );
        let mut con: Connection = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE chain_swaps 
            SET accepted_receiver_amount_sat = :accepted_receiver_amount_sat
            WHERE id = :id",
            named_params! {
                ":id": swap_id,
                ":accepted_receiver_amount_sat": accepted_receiver_amount_sat,
            },
        )?;
        self.commit_outgoing(
            &tx,
            swap_id,
            RecordType::Chain,
            Some(vec!["accepted_receiver_amount_sat".to_string()]),
        )?;
        tx.commit()?;
        self.trigger_sync();

        Ok(())
    }

    pub(crate) fn set_chain_swap_refund_address(
        &self,
        swap_id: &str,
        refund_address: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps
            SET refund_address = :refund_address
            WHERE id = :id",
            named_params! {
                        ":id": swap_id,
                        ":refund_address": refund_address,
            },
        )?;
        Ok(())
    }

    pub(crate) fn set_chain_swap_auto_accepted_fees(
        &self,
        swap_id: &str,
    ) -> Result<(), PaymentError> {
        log::info!("Setting chain swap {swap_id}: auto_accepted_fees to TRUE");

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE chain_swaps
            SET auto_accepted_fees = 1
            WHERE id = :id",
            named_params! {
                ":id": swap_id,
            },
        )?;
        self.commit_outgoing(
            &tx,
            swap_id,
            RecordType::Chain,
            Some(vec!["auto_accepted_fees".to_string()]),
        )?;
        tx.commit()?;
        self.trigger_sync();
        Ok(())
    }

    // Only set the Chain Swap claim_tx_id if not set, otherwise return an error
    pub(crate) fn set_chain_swap_claim(
        &self,
        swap_id: &str,
        claim_address: Option<String>,
        claim_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        let row_count = con
            .execute(
                "UPDATE chain_swaps
            SET claim_address = :claim_address, claim_tx_id = :claim_tx_id
            WHERE id = :id AND claim_tx_id IS NULL",
                named_params! {
                            ":id": swap_id,
                            ":claim_address": claim_address,
                            ":claim_tx_id": claim_tx_id,
                },
            )
            .map_err(|_| PaymentError::PersistError)?;
        match row_count {
            1 => Ok(()),
            _ => Err(PaymentError::AlreadyClaimed),
        }
    }

    // Only unset the Chain Swap claim_tx_id if set with the same tx id
    pub(crate) fn unset_chain_swap_claim_tx_id(
        &self,
        swap_id: &str,
        claim_tx_id: &str,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps
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

    pub(crate) fn try_handle_chain_swap_update(
        &self,
        swap_update: &ChainSwapUpdate,
    ) -> Result<(), PaymentError> {
        // Do not overwrite server_lockup_tx_id, user_lockup_tx_id, claim_address, claim_tx_id
        // Overwrite refund_tx_id if provided (refund tx fee bump)
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute(
            "UPDATE chain_swaps
            SET
                server_lockup_tx_id = COALESCE(server_lockup_tx_id, :server_lockup_tx_id),
                user_lockup_tx_id = COALESCE(user_lockup_tx_id, :user_lockup_tx_id),
                claim_address = COALESCE(claim_address, :claim_address),
                claim_tx_id = COALESCE(claim_tx_id, :claim_tx_id),

                refund_tx_id = COALESCE(:refund_tx_id, refund_tx_id),
                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": swap_update.swap_id,
                ":server_lockup_tx_id": swap_update.server_lockup_tx_id,
                ":user_lockup_tx_id": swap_update.user_lockup_tx_id,
                ":claim_address": swap_update.claim_address,
                ":claim_tx_id": swap_update.claim_tx_id,
                ":refund_tx_id": swap_update.refund_tx_id,
                ":state": swap_update.to_state,
            },
        )?;

        tx.commit()?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateChainResponse {
    pub(crate) claim_details: ChainSwapDetails,
    pub(crate) lockup_details: ChainSwapDetails,
}
impl InternalCreateChainResponse {
    pub(crate) fn try_convert_from_boltz(
        boltz_create_response: &CreateChainResponse,
        expected_swap_id: &str,
    ) -> Result<InternalCreateChainResponse, PaymentError> {
        // Do not store the CreateResponse fields that are already stored separately
        // Before skipping them, ensure they match the separately stored ones
        ensure_sdk!(
            boltz_create_response.id == expected_swap_id,
            PaymentError::PersistError
        );

        let res = InternalCreateChainResponse {
            claim_details: boltz_create_response.claim_details.clone(),
            lockup_details: boltz_create_response.lockup_details.clone(),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Direction;
    use crate::test_utils::chain_swap::new_chain_swap;
    use crate::test_utils::persist::create_persister;
    use anyhow::Result;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_writing_stale_swap() -> Result<()> {
        create_persister!(storage);

        let chain_swap = new_chain_swap(Direction::Incoming, None, false, None, false, false, None);
        storage.insert_or_update_chain_swap(&chain_swap)?;

        // read - update - write works if there are no updates in between
        let mut chain_swap = storage.fetch_chain_swap_by_id(&chain_swap.id)?.unwrap();
        chain_swap.claim_tx_id = Some("tx_id".to_string());
        storage.insert_or_update_chain_swap(&chain_swap)?;

        // read - update - write works if there are no updates in between even if no field changes
        let chain_swap = storage.fetch_chain_swap_by_id(&chain_swap.id)?.unwrap();
        storage.insert_or_update_chain_swap(&chain_swap)?;

        // read - update - write fails if there are any updates in between
        let mut chain_swap = storage.fetch_chain_swap_by_id(&chain_swap.id)?.unwrap();
        chain_swap.claim_tx_id = Some("tx_id_2".to_string());
        // Concurrent update
        storage.update_chain_swap_accept_zero_conf(&chain_swap.id, true)?;
        assert!(storage.insert_or_update_chain_swap(&chain_swap).is_err());

        Ok(())
    }
}
