mod backup;
mod migrations;
mod swap_in;
mod swap_out;

use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, str::FromStr};

use anyhow::Result;
use migrations::current_migrations;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

use crate::model::{Network::*, *};

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
           txid,
           timestamp,
           amount_sat,
           payment_type,
           status
        )
        VALUES (?, ?, ?, ?, ?)
        ",
            (
                ptx.txid,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.payment_type,
                ptx.status,
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

        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let mut stmt = con.prepare(
            "
            SELECT
                ptx.txid,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.payment_type,
                ptx.status,
                rs.created_at,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                ss.created_at,
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.is_claim_tx_seen
            FROM payment_tx_data AS ptx
            LEFT JOIN receive_swaps AS rs
                ON ptx.txid = rs.claim_txid
            LEFT JOIN send_swaps AS ss
                ON ptx.txid = ss.lockup_txid
        ",
        )?;

        let data = stmt
            .query_map(params![], |row| {
                let tx = PaymentTxData {
                    txid: row.get(0)?,
                    timestamp: row.get(1)?,
                    amount_sat: row.get(2)?,
                    payment_type: row.get(3)?,
                    status: row.get(4)?,
                };

                let maybe_receive_swap_created_at: Option<u32> = row.get(5)?;
                let maybe_receive_swap_payer_amount_sat: Option<u64> = row.get(6)?;
                let maybe_receive_swap_receiver_amount_sat: Option<u64> = row.get(7)?;
                let maybe_send_swap_created_at: Option<u32> = row.get(8)?;
                let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(9)?;
                let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(10)?;
                let maybe_send_swap_is_claim_tx_seen: Option<bool> = row.get(11)?;

                let swap = match maybe_receive_swap_created_at {
                    Some(receive_swap_created_at) => Some(PaymentSwapData {
                        created_at: receive_swap_created_at,
                        payer_amount_sat: maybe_receive_swap_payer_amount_sat.unwrap_or(0),
                        receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),

                        // Receive: payment changes to
                        // - Pending when the claim tx is broadcast
                        // - Complete when the claim tx is confirmed
                        status: tx.status,
                    }),
                    None => {
                        maybe_send_swap_created_at.map(|send_swap_created_at| PaymentSwapData {
                            created_at: send_swap_created_at,
                            payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                            receiver_amount_sat: maybe_send_swap_receiver_amount_sat.unwrap_or(0),

                            // Send: payment changes to
                            // - Pending when we broadcast the lockup tx
                            // - Complete when we see the claim tx in the mempool
                            status: match maybe_send_swap_is_claim_tx_seen {
                                Some(send_swap_is_claim_tx_seen) => {
                                    match send_swap_is_claim_tx_seen {
                                        true => PaymentStatus::Complete,
                                        false => PaymentStatus::Pending,
                                    }
                                }
                                // Pending is the default in this situation, as this tx is the lockup tx
                                None => PaymentStatus::Pending,
                            },
                        })
                    }
                };

                Ok((
                    tx.txid.clone(),
                    Payment {
                        tx: tx.clone(),
                        swap,
                        timestamp: match swap {
                            Some(swap) => Some(swap.created_at),
                            None => tx.timestamp,
                        },
                        status: match swap {
                            Some(swap) => swap.status,
                            None => tx.status,
                        },
                    },
                ))
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(data)
    }
}
