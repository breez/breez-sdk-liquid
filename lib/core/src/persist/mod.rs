mod backup;
mod migrations;
pub(crate) mod receive;
pub(crate) mod send;

use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, str::FromStr};

use anyhow::Result;
use migrations::current_migrations;
use rusqlite::{params, Connection, OptionalExtension, Row};
use rusqlite_migration::{Migrations, M};

use crate::model::{Network::*, *};
use crate::utils;

pub(crate) struct Persister {
    main_db_dir: PathBuf,
    network: Network,
}

impl Persister {
    pub fn new(working_dir: &str, network: Network) -> Result<Self> {
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
            Liquid => "storage.sql",
            LiquidTestnet => "storage-testnet.sql",
        };
        Ok(Connection::open(self.main_db_dir.join(db_file))?)
    }

    pub fn init(&self) -> Result<()> {
        self.migrate_main_db()?;
        Ok(())
    }

    fn migrate_main_db(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations.to_latest(&mut conn)?;
        Ok(())
    }

    pub(crate) fn insert_or_update_payment(&self, ptx: PaymentTxData) -> Result<()> {
        let mut con = self.get_connection()?;

        let tx = con.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO payment_tx_data (
           tx_id,
           timestamp,
           amount_sat,
           payment_type,
           is_confirmed
        )
        VALUES (?, ?, ?, ?, ?)
        ",
            (
                ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
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
        Ok([ongoing_send_swaps, ongoing_receive_swaps].concat())
    }

    fn select_payment_query(&self, where_clause: Option<&str>) -> String {
        format!(
            "
            SELECT
                ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.payment_type,
                ptx.is_confirmed,
                rs.id,
                rs.created_at,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.state,
                ss.id,
                ss.created_at,
                ss.preimage,
                ss.refund_tx_id,
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.state,
                rtx.amount_sat
            FROM payment_tx_data AS ptx          -- Payment tx (each tx results in a Payment)
            LEFT JOIN receive_swaps AS rs        -- Receive Swap data
                ON ptx.tx_id = rs.claim_tx_id
            LEFT JOIN send_swaps AS ss           -- Send Swap data
                ON ptx.tx_id = ss.lockup_tx_id
            LEFT JOIN payment_tx_data AS rtx     -- Refund tx data
                ON rtx.tx_id = ss.refund_tx_id
            WHERE                                -- Filter out refund txs from Payment tx list
                ptx.tx_id NOT IN (SELECT refund_tx_id FROM send_swaps)
                AND {}
            ",
            where_clause.unwrap_or("true")
        )
    }

    fn sql_row_to_payment(&self, row: &Row) -> Result<Payment, rusqlite::Error> {
        let tx = PaymentTxData {
            tx_id: row.get(0)?,
            timestamp: row.get(1)?,
            amount_sat: row.get(2)?,
            payment_type: row.get(3)?,
            is_confirmed: row.get(4)?,
        };

                let maybe_receive_swap_id: Option<String> = row.get(5)?;
                let maybe_receive_swap_created_at: Option<u32> = row.get(6)?;
                let maybe_receive_swap_payer_amount_sat: Option<u64> = row.get(7)?;
                let maybe_receive_swap_receiver_amount_sat: Option<u64> = row.get(8)?;
                let maybe_receive_swap_receiver_state: Option<PaymentState> = row.get(9)?;

                let maybe_send_swap_id: Option<String> = row.get(10)?;
                let maybe_send_swap_created_at: Option<u32> = row.get(11)?;
                let maybe_send_swap_preimage: Option<String> = row.get(12)?;
                let maybe_send_swap_refund_tx_id: Option<String> = row.get(13)?;
                let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(14)?;
                let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(15)?;
                let maybe_send_swap_state: Option<PaymentState> = row.get(16)?;
                let maybe_send_swap_refund_tx_amount_sat: Option<u64> = row.get(17)?;

                let swap = match maybe_receive_swap_id {
                    Some(receive_swap_id) => Some(PaymentSwapData {
                        swap_id: receive_swap_id,
                        created_at: maybe_receive_swap_created_at.unwrap_or(utils::now()),
                        preimage: None,
                        payer_amount_sat: maybe_receive_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),
                        refund_tx_id: None,
                        refund_tx_amount_sat: None,
                        status: maybe_receive_swap_receiver_state.unwrap_or(PaymentState::Created),
                    }),
                    None => maybe_send_swap_id.map(|send_swap_id| PaymentSwapData {
                        swap_id: send_swap_id,
                        created_at: maybe_send_swap_created_at.unwrap_or(utils::now()),
                        preimage: maybe_send_swap_preimage,
                        payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_send_swap_receiver_amount_sat.unwrap_or(0),
                        refund_tx_id: maybe_send_swap_refund_tx_id,
                        refund_tx_amount_sat: maybe_send_swap_refund_tx_amount_sat,
                        status: maybe_send_swap_state.unwrap_or(PaymentState::Created),
                    }),
                };

        Ok(Payment::from(tx, swap))
    }

    pub fn get_payment(&self, id: String) -> Result<Option<Payment>> {
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(Some(&format!("ptx.tx_id = ?1"))),
                params![id],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payments(&self) -> Result<HashMap<String, Payment>> {
        let con = self.get_connection()?;

        // TODO For refund txs, do not create a new Payment
        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let mut stmt = con.prepare(&self.select_payment_query(None))?;

        let data = stmt
            .query_map(params![], |row| {
                self.sql_row_to_payment(row).map(|p| (p.tx_id.clone(), p))
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(data)
    }
}
