mod migrations;

use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, str::FromStr};

use anyhow::Result;
use migrations::current_migrations;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

use crate::{Network, Network::*, OngoingSwap, PaymentData};

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

    pub fn insert_or_update_ongoing_swap(&self, swaps: &[OngoingSwap]) -> Result<()> {
        let con = self.get_connection()?;

        for swap in swaps {
            match swap {
                OngoingSwap::Send {
                    id,
                    funding_address,
                    invoice,
                    receiver_amount_sat,
                    txid,
                } => {
                    let mut stmt = con.prepare(
                        "
                            INSERT OR REPLACE INTO ongoing_send_swaps (
                                id,
                                funding_address,
                                invoice,
                                receiver_amount_sat,
                                txid
                            )
                            VALUES (?, ?, ?, ?, ?)
                        ",
                    )?;
                    _ = stmt.execute((id, funding_address, invoice, receiver_amount_sat, txid))?
                }
                OngoingSwap::Receive {
                    id,
                    preimage,
                    redeem_script,
                    blinding_key,
                    invoice,
                    receiver_amount_sat,
                } => {
                    let mut stmt = con.prepare(
                        "
                            INSERT OR REPLACE INTO ongoing_receive_swaps (
                                id,
                                preimage,
                                redeem_script,
                                blinding_key,
                                invoice,
                                receiver_amount_sat
                            )
                            VALUES (?, ?, ?, ?, ?, ?)
                        ",
                    )?;

                    _ = stmt.execute((
                        id,
                        preimage,
                        redeem_script,
                        blinding_key,
                        invoice,
                        receiver_amount_sat,
                    ))?
                }
            }
        }

        Ok(())
    }

    pub fn resolve_ongoing_swap(
        &self,
        id: &str,
        payment_data: Option<(String, PaymentData)>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;

        let tx = con.transaction()?;
        tx.execute("DELETE FROM ongoing_send_swaps WHERE id = ?", params![id])?;
        tx.execute(
            "DELETE FROM ongoing_receive_swaps WHERE id = ?",
            params![id],
        )?;
        if let Some((txid, payment_data)) = payment_data {
            tx.execute(
                "INSERT INTO payment_data(id, payer_amount_sat)
              VALUES(?, ?)",
                (txid, payment_data.payer_amount_sat),
            )?;
        }
        tx.commit()?;

        Ok(())
    }

    pub fn list_ongoing_swaps(&self) -> Result<Vec<OngoingSwap>> {
        let con = self.get_connection()?;
        let mut ongoing_swaps = self.list_ongoing_send(&con)?;
        ongoing_swaps.append(&mut self.list_ongoing_receive(&con)?);
        Ok(ongoing_swaps)
    }

    fn list_ongoing_send(&self, con: &Connection) -> Result<Vec<OngoingSwap>, rusqlite::Error> {
        let mut stmt = con.prepare(
            "
           SELECT 
               id,
               funding_address,
               invoice,
               receiver_amount_sat,
               txid,
               created_at
           FROM ongoing_send_swaps
           ORDER BY created_at
       ",
        )?;

        let ongoing_send = stmt
            .query_map(params![], |row| {
                Ok(OngoingSwap::Send {
                    id: row.get(0)?,
                    funding_address: row.get(1)?,
                    invoice: row.get(2)?,
                    receiver_amount_sat: row.get(3)?,
                    txid: row.get(4)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(ongoing_send)
    }

    fn list_ongoing_receive(&self, con: &Connection) -> Result<Vec<OngoingSwap>, rusqlite::Error> {
        let mut stmt = con.prepare(
            "
            SELECT
                id,
                preimage,
                redeem_script,
                blinding_key,
                invoice,
                receiver_amount_sat,
                created_at
            FROM ongoing_receive_swaps
            ORDER BY created_at
       ",
        )?;

        let ongoing_receive = stmt
            .query_map(params![], |row| {
                Ok(OngoingSwap::Receive {
                    id: row.get(0)?,
                    preimage: row.get(1)?,
                    redeem_script: row.get(2)?,
                    blinding_key: row.get(3)?,
                    invoice: row.get(4)?,
                    receiver_amount_sat: row.get(5)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(ongoing_receive)
    }

    pub fn get_payment_data(&self) -> Result<HashMap<String, PaymentData>> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            SELECT id, payer_amount_sat 
            FROM payment_data
        ",
        )?;

        let data = stmt
            .query_map(params![], |row| {
                Ok((
                    row.get(0)?,
                    PaymentData {
                        payer_amount_sat: row.get(1)?,
                    },
                ))
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(data)
    }
}
