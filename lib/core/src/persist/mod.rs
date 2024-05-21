mod backup;
mod migrations;
pub(crate) mod swap_in;
pub(crate) mod swap_out;

use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, str::FromStr};

use anyhow::Result;
use migrations::current_migrations;
use rusqlite::{params, Connection};
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
        let ongoing_swap_ins: Vec<Swap> = self
            .list_ongoing_send_swaps(&con)?
            .into_iter()
            .map(Swap::Send)
            .collect();
        let ongoing_swap_outs: Vec<Swap> = self
            .list_ongoing_receive_swaps(&con)?
            .into_iter()
            .map(Swap::Receive)
            .collect();
        Ok([ongoing_swap_ins, ongoing_swap_outs].concat())
    }

    pub fn get_payments(&self) -> Result<HashMap<String, Payment>> {
        let con = self.get_connection()?;

        // TODO For refund txs, do not create a new Payment
        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let mut stmt = con.prepare(
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
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.state
            FROM payment_tx_data AS ptx
            LEFT JOIN receive_swaps AS rs
                ON ptx.tx_id = rs.claim_tx_id
            LEFT JOIN send_swaps AS ss
                ON ptx.tx_id = ss.lockup_tx_id
        ",
        )?;

        let data = stmt
            .query_map(params![], |row| {
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
                let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(12)?;
                let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(13)?;
                let maybe_send_swap_state: Option<PaymentState> = row.get(14)?;

                let swap = match maybe_receive_swap_id {
                    Some(receive_swap_id) => Some(PaymentSwapData {
                        swap_id: receive_swap_id,
                        created_at: maybe_receive_swap_created_at.unwrap_or(utils::now()),
                        payer_amount_sat: maybe_receive_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),
                        status: maybe_receive_swap_receiver_state.unwrap_or(PaymentState::Created),
                    }),
                    None => maybe_send_swap_id.map(|send_swap_id| PaymentSwapData {
                        swap_id: send_swap_id,
                        created_at: maybe_send_swap_created_at.unwrap_or(utils::now()),
                        payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_send_swap_receiver_amount_sat.unwrap_or(0),
                        status: maybe_send_swap_state.unwrap_or(PaymentState::Created),
                    }),
                };

                Ok((tx.tx_id.clone(), Payment::from(tx, swap)))
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(data)
    }
}
