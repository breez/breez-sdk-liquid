mod address;
mod backup;
mod cache;
pub(crate) mod chain;
mod migrations;
pub(crate) mod receive;
pub(crate) mod send;

use std::collections::HashSet;
use std::{fs::create_dir_all, path::PathBuf, str::FromStr};

use crate::error::PaymentError;
use crate::lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
use crate::model::*;
use crate::{get_invoice_description, utils};
use anyhow::{anyhow, Result};
use migrations::current_migrations;
use rusqlite::{params, params_from_iter, Connection, OptionalExtension, Row, ToSql};
use rusqlite_migration::{Migrations, M};

const DEFAULT_DB_FILENAME: &str = "storage.sql";

pub(crate) struct Persister {
    main_db_dir: PathBuf,
    network: LiquidNetwork,
}

/// Builds a WHERE clause that checks if `state` is any of the given arguments
fn get_where_clause_state_in(allowed_states: &[PaymentState]) -> String {
    format!(
        "state in ({})",
        allowed_states
            .iter()
            .map(|t| format!("'{}'", *t as i8))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

impl Persister {
    pub fn new(working_dir: &str, network: LiquidNetwork) -> Result<Self> {
        let main_db_dir = PathBuf::from_str(working_dir)?;
        if !main_db_dir.exists() {
            create_dir_all(&main_db_dir)?;
        }
        Ok(Persister {
            main_db_dir,
            network,
        })
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        Ok(Connection::open(
            self.main_db_dir.join(DEFAULT_DB_FILENAME),
        )?)
    }

    pub fn init(&self) -> Result<()> {
        self.migrate_main_db()?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn get_database_dir(&self) -> &PathBuf {
        &self.main_db_dir
    }

    fn migrate_main_db(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations.to_latest(&mut conn)?;
        Ok(())
    }

    pub(crate) fn fetch_swap_by_id(&self, id: &str) -> Result<Swap> {
        match self.fetch_send_swap_by_id(id) {
            Ok(Some(send_swap)) => Ok(Swap::Send(send_swap)),
            _ => match self.fetch_receive_swap_by_id(id) {
                Ok(Some(receive_swap)) => Ok(Swap::Receive(receive_swap)),
                _ => match self.fetch_chain_swap_by_id(id) {
                    Ok(Some(chain_swap)) => Ok(Swap::Chain(chain_swap)),
                    _ => Err(anyhow!("Could not find Swap {id}")),
                },
            },
        }
    }

    pub(crate) fn insert_or_update_payment(
        &self,
        ptx: PaymentTxData,
        destination: Option<String>,
        description: Option<String>,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "INSERT OR REPLACE INTO payment_tx_data (
           tx_id,
           timestamp,
           amount_sat,
           fees_sat,
           payment_type,
           is_confirmed
        )
        VALUES (?, ?, ?, ?, ?, ?)
        ",
            (
                &ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.fees_sat,
                ptx.payment_type,
                ptx.is_confirmed,
            ),
        )?;

        if let Some(destination) = destination {
            con.execute(
                "INSERT OR REPLACE INTO payment_details (
                    tx_id,
                    destination,
                    description 
                )
                VALUES (?, ?, ?)
            ",
                (ptx.tx_id, destination, description),
            )?;
        }

        Ok(())
    }

    pub(crate) fn list_ongoing_swaps(&self) -> Result<Vec<Swap>> {
        let con = self.get_connection()?;
        let ongoing_send_swaps: Vec<Swap> = self
            .list_ongoing_send_swaps(&con)?
            .into_iter()
            .map(Swap::Send)
            .collect();
        let ongoing_receive_swaps: Vec<Swap> = self
            .list_ongoing_receive_swaps(&con)?
            .into_iter()
            .map(Swap::Receive)
            .collect();
        let ongoing_chain_swaps: Vec<Swap> = self
            .list_ongoing_chain_swaps(&con)?
            .into_iter()
            .map(Swap::Chain)
            .collect();
        Ok([
            ongoing_send_swaps,
            ongoing_receive_swaps,
            ongoing_chain_swaps,
        ]
        .concat())
    }

    fn select_payment_query(
        &self,
        where_clause: Option<&str>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> String {
        format!(
            "
            SELECT
                ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.fees_sat,
                ptx.payment_type,
                ptx.is_confirmed,
                rs.id,
                rs.created_at,
                rs.invoice,
                rs.payment_hash,
                rs.description,
                rs.preimage,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.state,
                ss.id,
                ss.created_at,
                ss.invoice,
                ss.bolt12_offer,
                ss.payment_hash,
                ss.description,
                ss.preimage,
                ss.refund_tx_id,
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.state,
                cs.id,
                cs.created_at,
                cs.direction,
                cs.preimage,
                cs.description,
                cs.refund_tx_id,
                cs.payer_amount_sat,
                cs.receiver_amount_sat,
                cs.claim_address,
                cs.state,
                rtx.amount_sat,
                pd.destination,
                pd.description
            FROM payment_tx_data AS ptx          -- Payment tx (each tx results in a Payment)
            FULL JOIN (
                SELECT * FROM receive_swaps
                WHERE COALESCE(claim_tx_id, lockup_tx_id, mrh_tx_id) IS NOT NULL
            ) rs                                 -- Receive Swap data (by claim)
                ON ptx.tx_id in (rs.claim_tx_id, rs.mrh_tx_id)
            LEFT JOIN send_swaps AS ss           -- Send Swap data
                ON ptx.tx_id = ss.lockup_tx_id
            LEFT JOIN chain_swaps AS cs          -- Chain Swap data
                ON ptx.tx_id in (cs.user_lockup_tx_id, cs.claim_tx_id)
            LEFT JOIN payment_tx_data AS rtx     -- Refund tx data
                ON rtx.tx_id in (ss.refund_tx_id, cs.refund_tx_id)
            LEFT JOIN payment_details AS pd      -- Payment details
                ON pd.tx_id = ptx.tx_id
            WHERE                                -- Filter out refund txs from Send Swaps
                ptx.tx_id NOT IN (SELECT refund_tx_id FROM send_swaps WHERE refund_tx_id NOT NULL)
            AND                                  -- Filter out refund txs from Chain Swaps
                ptx.tx_id NOT IN (SELECT refund_tx_id FROM chain_swaps WHERE refund_tx_id NOT NULL)
            AND {}
            ORDER BY                             -- Order by swap creation time or tx timestamp (in case of direct tx)
                COALESCE(rs.created_at, ss.created_at, cs.created_at, ptx.timestamp) DESC
            LIMIT {}
            OFFSET {}
            ",
            where_clause.unwrap_or("true"),
            limit.unwrap_or(u32::MAX),
            offset.unwrap_or(0),
        )
    }

    fn sql_row_to_payment(&self, row: &Row) -> Result<Payment, rusqlite::Error> {
        let maybe_tx_tx_id: Result<String, rusqlite::Error> = row.get(0);
        let tx = match maybe_tx_tx_id {
            Ok(ref tx_id) => Some(PaymentTxData {
                tx_id: tx_id.to_string(),
                timestamp: row.get(1)?,
                amount_sat: row.get(2)?,
                fees_sat: row.get(3)?,
                payment_type: row.get(4)?,
                is_confirmed: row.get(5)?,
            }),
            _ => None,
        };

        let maybe_receive_swap_id: Option<String> = row.get(6)?;
        let maybe_receive_swap_created_at: Option<u32> = row.get(7)?;
        let maybe_receive_swap_invoice: Option<String> = row.get(8)?;
        let maybe_receive_swap_payment_hash: Option<String> = row.get(9)?;
        let maybe_receive_swap_description: Option<String> = row.get(10)?;
        let maybe_receive_swap_preimage: Option<String> = row.get(11)?;
        let maybe_receive_swap_payer_amount_sat: Option<u64> = row.get(12)?;
        let maybe_receive_swap_receiver_amount_sat: Option<u64> = row.get(13)?;
        let maybe_receive_swap_receiver_state: Option<PaymentState> = row.get(14)?;

        let maybe_send_swap_id: Option<String> = row.get(15)?;
        let maybe_send_swap_created_at: Option<u32> = row.get(16)?;
        let maybe_send_swap_invoice: Option<String> = row.get(17)?;
        let maybe_send_swap_bolt12_offer: Option<String> = row.get(18)?;
        let maybe_send_swap_payment_hash: Option<String> = row.get(19)?;
        let maybe_send_swap_description: Option<String> = row.get(20)?;
        let maybe_send_swap_preimage: Option<String> = row.get(21)?;
        let maybe_send_swap_refund_tx_id: Option<String> = row.get(22)?;
        let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(23)?;
        let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(24)?;
        let maybe_send_swap_state: Option<PaymentState> = row.get(25)?;

        let maybe_chain_swap_id: Option<String> = row.get(26)?;
        let maybe_chain_swap_created_at: Option<u32> = row.get(27)?;
        let maybe_chain_swap_direction: Option<Direction> = row.get(28)?;
        let maybe_chain_swap_preimage: Option<String> = row.get(29)?;
        let maybe_chain_swap_description: Option<String> = row.get(30)?;
        let maybe_chain_swap_refund_tx_id: Option<String> = row.get(31)?;
        let maybe_chain_swap_payer_amount_sat: Option<u64> = row.get(32)?;
        let maybe_chain_swap_receiver_amount_sat: Option<u64> = row.get(33)?;
        let maybe_chain_swap_claim_address: Option<String> = row.get(34)?;
        let maybe_chain_swap_state: Option<PaymentState> = row.get(35)?;

        let maybe_swap_refund_tx_amount_sat: Option<u64> = row.get(36)?;

        let maybe_payment_details_destination: Option<String> = row.get(37)?;
        let maybe_payment_details_description: Option<String> = row.get(38)?;

        let (swap, payment_type) = match maybe_receive_swap_id {
            Some(receive_swap_id) => (
                Some(PaymentSwapData {
                    swap_id: receive_swap_id,
                    swap_type: PaymentSwapType::Receive,
                    created_at: maybe_receive_swap_created_at.unwrap_or(utils::now()),
                    preimage: maybe_receive_swap_preimage,
                    bolt11: maybe_receive_swap_invoice.clone(),
                    bolt12_offer: None, // Bolt12 not supported for Receive Swaps
                    payment_hash: maybe_receive_swap_payment_hash,
                    description: maybe_receive_swap_description.unwrap_or_else(|| {
                        maybe_receive_swap_invoice
                            .and_then(|bolt11| get_invoice_description!(bolt11))
                            .unwrap_or("Lightning payment".to_string())
                    }),
                    payer_amount_sat: maybe_receive_swap_payer_amount_sat.unwrap_or(0),
                    receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),
                    refund_tx_id: None,
                    refund_tx_amount_sat: None,
                    claim_address: None,
                    status: maybe_receive_swap_receiver_state.unwrap_or(PaymentState::Created),
                }),
                PaymentType::Receive,
            ),
            None => match maybe_send_swap_id {
                Some(send_swap_id) => (
                    Some(PaymentSwapData {
                        swap_id: send_swap_id,
                        swap_type: PaymentSwapType::Send,
                        created_at: maybe_send_swap_created_at.unwrap_or(utils::now()),
                        preimage: maybe_send_swap_preimage,
                        bolt11: match maybe_send_swap_bolt12_offer.is_some() {
                            true => None, // We don't expose the Bolt12 invoice
                            false => maybe_send_swap_invoice,
                        },
                        bolt12_offer: maybe_send_swap_bolt12_offer,
                        payment_hash: maybe_send_swap_payment_hash,
                        description: maybe_send_swap_description
                            .unwrap_or("Lightning payment".to_string()),
                        payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_send_swap_receiver_amount_sat.unwrap_or(0),
                        refund_tx_id: maybe_send_swap_refund_tx_id,
                        refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
                        claim_address: None,
                        status: maybe_send_swap_state.unwrap_or(PaymentState::Created),
                    }),
                    PaymentType::Send,
                ),
                None => match maybe_chain_swap_id {
                    Some(chain_swap_id) => (
                        Some(PaymentSwapData {
                            swap_id: chain_swap_id,
                            swap_type: PaymentSwapType::Chain,
                            created_at: maybe_chain_swap_created_at.unwrap_or(utils::now()),
                            preimage: maybe_chain_swap_preimage,
                            bolt11: None,
                            bolt12_offer: None, // Bolt12 not supported for Chain Swaps
                            payment_hash: None,
                            description: maybe_chain_swap_description
                                .unwrap_or("Bitcoin transfer".to_string()),
                            payer_amount_sat: maybe_chain_swap_payer_amount_sat.unwrap_or(0),
                            receiver_amount_sat: maybe_chain_swap_receiver_amount_sat.unwrap_or(0),
                            refund_tx_id: maybe_chain_swap_refund_tx_id,
                            refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
                            claim_address: maybe_chain_swap_claim_address,
                            status: maybe_chain_swap_state.unwrap_or(PaymentState::Created),
                        }),
                        maybe_chain_swap_direction
                            .unwrap_or(Direction::Outgoing)
                            .into(),
                    ),
                    None => (None, PaymentType::Send),
                },
            },
        };

        let description = swap.as_ref().map(|s| s.description.clone());
        let payment_details = match swap.clone() {
            Some(
                PaymentSwapData {
                    swap_type: PaymentSwapType::Receive,
                    swap_id,
                    bolt11,
                    bolt12_offer,
                    payment_hash,
                    refund_tx_id,
                    preimage,
                    refund_tx_amount_sat,
                    ..
                }
                | PaymentSwapData {
                    swap_type: PaymentSwapType::Send,
                    swap_id,
                    bolt11,
                    bolt12_offer,
                    payment_hash,
                    preimage,
                    refund_tx_id,
                    refund_tx_amount_sat,
                    ..
                },
            ) => PaymentDetails::Lightning {
                swap_id,
                preimage,
                bolt11,
                bolt12_offer,
                payment_hash,
                refund_tx_id,
                refund_tx_amount_sat,
                description: description.unwrap_or("Lightning transfer".to_string()),
            },
            Some(PaymentSwapData {
                swap_type: PaymentSwapType::Chain,
                swap_id,
                refund_tx_id,
                refund_tx_amount_sat,
                ..
            }) => PaymentDetails::Bitcoin {
                swap_id,
                refund_tx_id,
                refund_tx_amount_sat,
                description: description.unwrap_or("Bitcoin transfer".to_string()),
            },
            _ => PaymentDetails::Liquid {
                destination: maybe_payment_details_destination
                    .unwrap_or("Destination unknown".to_string()),
                description: maybe_payment_details_description
                    .unwrap_or("Liquid transfer".to_string()),
            },
        };

        match (tx, swap.clone()) {
            (None, None) => Err(maybe_tx_tx_id.err().unwrap()),
            (None, Some(swap)) => Ok(Payment::from_pending_swap(swap, payment_type)),
            (Some(tx), None) => Ok(Payment::from_tx_data(tx, None, payment_details)),
            (Some(tx), Some(swap)) => Ok(Payment::from_tx_data(tx, Some(swap), payment_details)),
        }
    }

    pub fn get_payment(&self, id: &str) -> Result<Option<Payment>> {
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(Some("ptx.tx_id = ?1"), None, None),
                params![id],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payment_by_request(&self, req: &GetPaymentRequest) -> Result<Option<Payment>> {
        let (where_clause, param) = match req {
            GetPaymentRequest::Lightning { payment_hash } => (
                "(rs.payment_hash = ?1 OR ss.payment_hash = ?1)",
                payment_hash,
            ),
        };
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(Some(where_clause), None, None),
                params![param],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payments(&self, req: &ListPaymentsRequest) -> Result<Vec<Payment>> {
        let (where_clause, where_params) = filter_to_where_clause(req);
        let maybe_where_clause = match where_clause.is_empty() {
            false => Some(where_clause.as_str()),
            true => None,
        };

        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let con = self.get_connection()?;
        let mut stmt =
            con.prepare(&self.select_payment_query(maybe_where_clause, req.offset, req.limit))?;
        let payments: Vec<Payment> = stmt
            .query_map(params_from_iter(where_params), |row| {
                self.sql_row_to_payment(row)
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(payments)
    }
}

fn filter_to_where_clause(req: &ListPaymentsRequest) -> (String, Vec<Box<dyn ToSql + '_>>) {
    let mut where_clause: Vec<String> = Vec::new();
    let mut where_params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(t) = req.from_timestamp {
        where_clause.push("coalesce(ptx.timestamp, rs.created_at) >= ?".to_string());
        where_params.push(Box::new(t));
    };
    if let Some(t) = req.to_timestamp {
        where_clause.push("coalesce(ptx.timestamp, rs.created_at) <= ?".to_string());
        where_params.push(Box::new(t));
    };

    if let Some(filters) = &req.filters {
        if !filters.is_empty() {
            let mut type_filter_clause: HashSet<i8> = HashSet::new();

            for type_filter in filters {
                type_filter_clause.insert(*type_filter as i8);
            }

            where_clause.push(format!(
                "ptx.payment_type in ({})",
                type_filter_clause
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if let Some(details) = &req.details {
        match details {
            ListPaymentDetails::Bitcoin { address } => {
                // Use the lockup address if it's incoming, else use the claim address
                where_clause.push(
                    "(cs.direction = 0 AND cs.lockup_address = ? OR cs.direction = 1 AND cs.claim_address = ?)"
                        .to_string(),
                );
                where_params.push(Box::new(address));
                where_params.push(Box::new(address));
            }
            ListPaymentDetails::Liquid { destination } => {
                where_clause.push("pd.destination = ?".to_string());
                where_params.push(Box::new(destination));
            }
        }
    }

    (where_clause.join(" and "), where_params)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        prelude::ListPaymentsRequest,
        test_utils::persist::{
            new_payment_tx_data, new_persister, new_receive_swap, new_send_swap,
        },
    };

    use super::{PaymentState, PaymentType};

    #[test]
    fn test_get_payments() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;

        let payment_tx_data = new_payment_tx_data(PaymentType::Send);
        storage.insert_or_update_payment(
            payment_tx_data.clone(),
            Some("mock-address".to_string()),
            None,
        )?;

        assert!(storage
            .get_payments(&ListPaymentsRequest {
                ..Default::default()
            })?
            .first()
            .is_some());
        assert!(storage.get_payment(&payment_tx_data.tx_id)?.is_some());

        Ok(())
    }

    #[test]
    fn test_list_ongoing_swaps() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;

        storage.insert_send_swap(&new_send_swap(None))?;
        storage.insert_receive_swap(&new_receive_swap(Some(PaymentState::Pending)))?;

        assert_eq!(storage.list_ongoing_swaps()?.len(), 2);

        Ok(())
    }
}
