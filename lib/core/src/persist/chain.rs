use std::collections::HashMap;

use anyhow::Result;
use boltz_client::swaps::boltz::{ChainSwapDetails, CreateChainResponse};
use rusqlite::{named_params, params, Connection, Row};
use sdk_common::bitcoin::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::{get_where_clause_state_in, Persister};

impl Persister {
    pub(crate) fn insert_chain_swap(&self, chain_swap: &ChainSwap) -> Result<()> {
        let con = self.get_connection()?;

        // There is a limit of 16 param elements in a single tuple in rusqlite,
        // so we split up the insert into two statements.
        let mut stmt = con.prepare(
            "
            INSERT INTO chain_swaps (
                id,
                id_hash,
                direction,
                claim_address,
                lockup_address,
                timeout_block_height,
                preimage,
                payer_amount_sat,
                receiver_amount_sat,
                accept_zero_conf,
                create_response_json,
                claim_private_key,
                refund_private_key,
                claim_fees_sat,
                created_at,
                state,
                pair_fees_json
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        let id_hash = sha256::Hash::hash(chain_swap.id.as_bytes()).to_hex();
        _ = stmt.execute(params![
            &chain_swap.id,
            &id_hash,
            &chain_swap.direction,
            &chain_swap.claim_address,
            &chain_swap.lockup_address,
            &chain_swap.timeout_block_height,
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
            &chain_swap.pair_fees_json
        ])?;

        con.execute(
            "UPDATE chain_swaps
            SET
                description = :description,
                server_lockup_tx_id = :server_lockup_tx_id,
                user_lockup_tx_id = :user_lockup_tx_id,
                claim_tx_id = :claim_tx_id,
                refund_tx_id = :refund_tx_id
            WHERE
                id = :id",
            named_params! {
                ":id": &chain_swap.id,
                ":description": &chain_swap.description,
                ":server_lockup_tx_id": &chain_swap.server_lockup_tx_id,
                ":user_lockup_tx_id": &chain_swap.user_lockup_tx_id,
                ":claim_tx_id": &chain_swap.claim_tx_id,
                ":refund_tx_id": &chain_swap.refund_tx_id,
            },
        )?;

        Ok(())
    }

    fn list_chain_swaps_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                direction,
                claim_address,
                lockup_address,
                timeout_block_height,
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
                pair_fees_json
            FROM chain_swaps
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
            timeout_block_height: row.get(4)?,
            preimage: row.get(5)?,
            description: row.get(6)?,
            payer_amount_sat: row.get(7)?,
            receiver_amount_sat: row.get(8)?,
            accept_zero_conf: row.get(9)?,
            create_response_json: row.get(10)?,
            claim_private_key: row.get(11)?,
            refund_private_key: row.get(12)?,
            server_lockup_tx_id: row.get(13)?,
            user_lockup_tx_id: row.get(14)?,
            claim_fees_sat: row.get(15)?,
            claim_tx_id: row.get(16)?,
            refund_tx_id: row.get(17)?,
            created_at: row.get(18)?,
            state: row.get(19)?,
            pair_fees_json: row.get(20)?,
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
        con: &Connection,
        states: Vec<PaymentState>,
    ) -> Result<Vec<ChainSwap>> {
        let where_clause = vec![get_where_clause_state_in(&states)];
        self.list_chain_swaps_where(con, where_clause)
    }

    pub(crate) fn list_ongoing_chain_swaps(&self, con: &Connection) -> Result<Vec<ChainSwap>> {
        self.list_chain_swaps_by_state(con, vec![PaymentState::Created, PaymentState::Pending])
    }

    pub(crate) fn list_pending_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        self.list_chain_swaps_by_state(
            &con,
            vec![PaymentState::Pending, PaymentState::RefundPending],
        )
    }

    pub(crate) fn list_refundable_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        self.list_chain_swaps_by_state(&con, vec![PaymentState::Refundable])
    }

    /// Pending Chain swaps, indexed by refund tx id
    pub(crate) fn list_pending_chain_swaps_by_refund_tx_id(
        &self,
    ) -> Result<HashMap<String, ChainSwap>> {
        let res: HashMap<String, ChainSwap> = self
            .list_pending_chain_swaps()?
            .iter()
            .filter_map(|pending_chain_swap| {
                pending_chain_swap
                    .refund_tx_id
                    .as_ref()
                    .map(|refund_tx_id| (refund_tx_id.clone(), pending_chain_swap.clone()))
            })
            .collect();
        Ok(res)
    }

    /// This only returns the swaps that have a claim tx, skipping the pending ones that are being refunded.
    pub(crate) fn list_pending_chain_swaps_by_claim_tx_id(
        &self,
    ) -> Result<HashMap<String, ChainSwap>> {
        let con: Connection = self.get_connection()?;
        let res: HashMap<String, ChainSwap> = self
            .list_chain_swaps_by_state(&con, vec![PaymentState::Pending])?
            .iter()
            .filter_map(|pending_chain_swap| {
                pending_chain_swap
                    .claim_tx_id
                    .as_ref()
                    .map(|claim_tx_id| (claim_tx_id.clone(), pending_chain_swap.clone()))
            })
            .collect();
        Ok(res)
    }

    pub(crate) fn update_chain_swap_accept_zero_conf(
        &self,
        swap_id: &str,
        accept_zero_conf: bool,
    ) -> Result<(), PaymentError> {
        let con: Connection = self.get_connection()?;
        con.execute(
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
        Ok(())
    }

    /// Used for Zero-amount Receive Chain swaps, when we fetched the quote and we know how much
    /// the sender locked up
    pub(crate) fn update_zero_amount_swap_values(
        &self,
        swap_id: &str,
        payer_amount_sat: u64,
        receiver_amount_sat: u64,
    ) -> Result<(), PaymentError> {
        log::info!("Updating chain swap {swap_id}: payer_amount_sat = {payer_amount_sat}, receiver_amount_sat = {receiver_amount_sat}");
        let con: Connection = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps
            SET
                payer_amount_sat = :payer_amount_sat,
                receiver_amount_sat = :receiver_amount_sat
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":payer_amount_sat": payer_amount_sat,
                ":receiver_amount_sat": receiver_amount_sat,
            },
        )?;
        Ok(())
    }

    // Only set the Chain Swap claim_tx_id if not set, otherwise return an error
    pub(crate) fn set_chain_swap_claim_tx_id(
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
        swap_id: &str,
        to_state: PaymentState,
        server_lockup_tx_id: Option<&str>,
        user_lockup_tx_id: Option<&str>,
        claim_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        // Do not overwrite server_lockup_tx_id, user_lockup_tx_id, claim_tx_id, refund_tx_id
        let con: Connection = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps
            SET
                server_lockup_tx_id =
                    CASE
                        WHEN server_lockup_tx_id IS NULL THEN :server_lockup_tx_id
                        ELSE server_lockup_tx_id
                    END,

                user_lockup_tx_id =
                    CASE
                        WHEN user_lockup_tx_id IS NULL THEN :user_lockup_tx_id
                        ELSE user_lockup_tx_id
                    END,

                claim_tx_id =
                    CASE
                        WHEN claim_tx_id IS NULL THEN :claim_tx_id
                        ELSE claim_tx_id
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
                ":server_lockup_tx_id": server_lockup_tx_id,
                ":user_lockup_tx_id": user_lockup_tx_id,
                ":claim_tx_id": claim_tx_id,
                ":refund_tx_id": refund_tx_id,
                ":state": to_state,
            },
        )?;

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
