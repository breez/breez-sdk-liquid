mod migrations;

use anyhow::Result;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

use crate::OngoingSwap;

use migrations::current_migrations;

pub(crate) struct Persister {
    main_db_file: String,
}

impl Persister {
    pub fn new(working_dir: &str) -> Self {
        let main_db_file = format!("{}/storage.sql", working_dir);
        Persister { main_db_file }
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        let con = Connection::open(self.main_db_file.clone())?;
        Ok(con)
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

    pub fn insert_ongoing_swap(&self, swaps: &[OngoingSwap]) -> Result<()> {
        let con = self.get_connection()?;

        for swap in swaps {
            match swap {
                OngoingSwap::Send {
                    id,
                    funding_address,
                    amount_sat,
                } => {
                    let mut stmt = con.prepare(
                        "
                            INSERT INTO ongoing_send_swaps (
                                id,
                                amount_sat,
                                funding_address
                            )
                            VALUES (?, ?, ?)
                        ",
                    )?;

                    _ = stmt.execute((&id, &amount_sat, &funding_address))?
                }
                OngoingSwap::Receive {
                    id,
                    preimage,
                    redeem_script,
                    blinding_key,
                    invoice_amount_sat,
                    onchain_amount_sat,
                } => {
                    let mut stmt = con.prepare(
                        "
                            INSERT INTO ongoing_receive_swaps (
                                id,
                                preimage,
                                redeem_script,
                                blinding_key,
                                invoice_amount_sat,
                                onchain_amount_sat
                            )
                            VALUES (?, ?, ?, ?, ?, ?)
                        ",
                    )?;

                    _ = stmt.execute((
                        &id,
                        &preimage,
                        &redeem_script,
                        &blinding_key,
                        &invoice_amount_sat,
                        &onchain_amount_sat,
                    ))?
                }
            }
        }

        Ok(())
    }

    pub fn resolve_ongoing_swap(&self, id: &str) -> Result<()> {
        let mut con = self.get_connection()?;

        let tx = con.transaction()?;
        tx.execute("DELETE FROM ongoing_send_swaps WHERE id = ?", params![id])?;
        tx.execute(
            "DELETE FROM ongoing_receive_swaps WHERE id = ?",
            params![id],
        )?;
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
               amount_sat,
               funding_address,
               created_at
           FROM ongoing_send_swaps
           ORDER BY created_at
       ",
        )?;

        let ongoing_send = stmt
            .query_map(params![], |row| {
                Ok(OngoingSwap::Send {
                    id: row.get(0)?,
                    amount_sat: row.get(1)?,
                    funding_address: row.get(2)?,
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
                invoice_amount_sat,
                onchain_amount_sat,
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
                    invoice_amount_sat: row.get(4)?,
                    onchain_amount_sat: row.get(5)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(ongoing_receive)
    }
}
