mod backup;
mod cache;
pub(crate) mod chain;
mod migrations;
pub(crate) mod receive;
pub(crate) mod send;

use std::{fs::create_dir_all, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use migrations::current_migrations;
use rusqlite::{params, Connection, OptionalExtension, Row};
use rusqlite_migration::{Migrations, M};

use crate::model::{LiquidNetwork::*, *};
use crate::utils;

pub(crate) struct Persister {
    main_db_dir: PathBuf,
    network: LiquidNetwork,
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
        let db_file = match self.network {
            Mainnet => "storage.sql",
            Testnet => "storage-testnet.sql",
        };
        Ok(Connection::open(self.main_db_dir.join(db_file))?)
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
            _ => match self.fetch_receive_swap(id) {
                Ok(Some(receive_swap)) => Ok(Swap::Receive(receive_swap)),
                _ => match self.fetch_chain_swap_by_id(id) {
                    Ok(Some(chain_swap)) => Ok(Swap::Chain(chain_swap)),
                    _ => Err(anyhow!("Could not find Swap {id}")),
                },
            },
        }
    }

    pub(crate) fn insert_or_update_payment(&self, ptx: PaymentTxData) -> Result<()> {
        let mut con = self.get_connection()?;

        let tx = con.transaction()?;
        tx.execute(
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
                ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.fees_sat,
                ptx.payment_type,
                ptx.is_confirmed,
            ),
        )?;
        tx.commit()?;

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

    fn select_payment_query(&self, where_clause: Option<&str>) -> String {
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
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.state,
                ss.id,
                ss.created_at,
                ss.invoice,
                ss.preimage,
                ss.refund_tx_id,
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.state,
                cs.id,
                cs.created_at,
                cs.direction,
                cs.preimage,
                cs.refund_tx_id,
                cs.payer_amount_sat,
                cs.receiver_amount_sat,
                cs.state,
                rtx.amount_sat
            FROM payment_tx_data AS ptx          -- Payment tx (each tx results in a Payment)
            FULL JOIN (
                SELECT * FROM receive_swaps
                WHERE claim_tx_id IS NOT NULL OR lockup_tx_id IS NOT NULL
            ) rs                                 -- Receive Swap data (by claim)
                ON ptx.tx_id = rs.claim_tx_id
            LEFT JOIN send_swaps AS ss           -- Send Swap data
                ON ptx.tx_id = ss.lockup_tx_id
            LEFT JOIN chain_swaps AS cs          -- Chain Swap data
                ON ptx.tx_id in (cs.user_lockup_tx_id, cs.claim_tx_id)
            LEFT JOIN payment_tx_data AS rtx     -- Refund tx data
                ON rtx.tx_id in (ss.refund_tx_id, cs.refund_tx_id)
            WHERE                                -- Filter out refund txs from Payment tx list
                ptx.tx_id NOT IN (SELECT refund_tx_id FROM send_swaps WHERE refund_tx_id NOT NULL)
            AND {}
            ",
            where_clause.unwrap_or("true")
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
        let maybe_receive_swap_payer_amount_sat: Option<u64> = row.get(9)?;
        let maybe_receive_swap_receiver_amount_sat: Option<u64> = row.get(10)?;
        let maybe_receive_swap_receiver_state: Option<PaymentState> = row.get(11)?;

        let maybe_send_swap_id: Option<String> = row.get(12)?;
        let maybe_send_swap_created_at: Option<u32> = row.get(13)?;
        let maybe_send_swap_invoice: Option<String> = row.get(14)?;
        let maybe_send_swap_preimage: Option<String> = row.get(15)?;
        let maybe_send_swap_refund_tx_id: Option<String> = row.get(16)?;
        let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(17)?;
        let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(18)?;
        let maybe_send_swap_state: Option<PaymentState> = row.get(19)?;

        let maybe_chain_swap_id: Option<String> = row.get(20)?;
        let maybe_chain_swap_created_at: Option<u32> = row.get(21)?;
        let maybe_chain_swap_direction: Option<Direction> = row.get(22)?;
        let maybe_chain_swap_preimage: Option<String> = row.get(23)?;
        let maybe_chain_swap_refund_tx_id: Option<String> = row.get(24)?;
        let maybe_chain_swap_payer_amount_sat: Option<u64> = row.get(25)?;
        let maybe_chain_swap_receiver_amount_sat: Option<u64> = row.get(26)?;
        let maybe_chain_swap_state: Option<PaymentState> = row.get(27)?;

        let maybe_swap_refund_tx_amount_sat: Option<u64> = row.get(28)?;

        let (swap, payment_type) = match maybe_receive_swap_id {
            Some(receive_swap_id) => (
                Some(PaymentSwapData {
                    swap_id: receive_swap_id,
                    created_at: maybe_receive_swap_created_at.unwrap_or(utils::now()),
                    preimage: None,
                    bolt11: maybe_receive_swap_invoice,
                    payer_amount_sat: maybe_receive_swap_payer_amount_sat.unwrap_or(0),
                    receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),
                    refund_tx_id: None,
                    refund_tx_amount_sat: None,
                    status: maybe_receive_swap_receiver_state.unwrap_or(PaymentState::Created),
                }),
                PaymentType::Receive,
            ),
            None => match maybe_send_swap_id {
                Some(send_swap_id) => (
                    Some(PaymentSwapData {
                        swap_id: send_swap_id,
                        created_at: maybe_send_swap_created_at.unwrap_or(utils::now()),
                        preimage: maybe_send_swap_preimage,
                        bolt11: maybe_send_swap_invoice,
                        payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_send_swap_receiver_amount_sat.unwrap_or(0),
                        refund_tx_id: maybe_send_swap_refund_tx_id,
                        refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
                        status: maybe_send_swap_state.unwrap_or(PaymentState::Created),
                    }),
                    PaymentType::Send,
                ),
                None => match maybe_chain_swap_id {
                    Some(chain_swap_id) => (
                        Some(PaymentSwapData {
                            swap_id: chain_swap_id,
                            created_at: maybe_chain_swap_created_at.unwrap_or(utils::now()),
                            preimage: maybe_chain_swap_preimage,
                            bolt11: None,
                            payer_amount_sat: maybe_chain_swap_payer_amount_sat.unwrap_or(0),
                            receiver_amount_sat: maybe_chain_swap_receiver_amount_sat.unwrap_or(0),
                            refund_tx_id: maybe_chain_swap_refund_tx_id,
                            refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
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

        match (tx, swap.clone()) {
            (None, None) => Err(maybe_tx_tx_id.err().unwrap()),
            (None, Some(swap)) => Ok(Payment::from_pending_swap(swap, payment_type)),
            (Some(tx), None) => Ok(Payment::from_tx_data(tx, None)),
            (Some(tx), Some(swap)) => Ok(Payment::from_tx_data(tx, Some(swap))),
        }
    }

    pub fn get_payment(&self, id: String) -> Result<Option<Payment>> {
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(Some("ptx.tx_id = ?1")),
                params![id],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payments(&self) -> Result<Vec<Payment>> {
        let con = self.get_connection()?;

        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let mut stmt = con.prepare(&self.select_payment_query(None))?;
        let payments: Vec<Payment> = stmt
            .query_map(params![], |row| self.sql_row_to_payment(row))?
            .map(|i| i.unwrap())
            .collect();
        Ok(payments)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::test_utils::persist::{
        new_payment_tx_data, new_persister, new_receive_swap, new_send_swap,
    };

    use super::{PaymentState, PaymentType};

    #[test]
    fn test_get_payments() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;

        let payment_tx_data = new_payment_tx_data(PaymentType::Send);
        storage.insert_or_update_payment(payment_tx_data.clone())?;

        assert!(storage.get_payments()?.first().is_some());
        assert!(storage.get_payment(payment_tx_data.tx_id)?.is_some());

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
