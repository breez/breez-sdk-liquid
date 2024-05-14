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

    pub fn resolve_ongoing_swap(
        &self,
        id: &str,
        payment_data: Option<(String, PaymentData)>,
    ) -> Result<()> {
        let mut con = self.get_connection()?;

        let tx = con.transaction()?;
        tx.execute("DELETE FROM send_swaps WHERE id = ?", params![id])?;
        tx.execute(
            "DELETE FROM receive_swaps WHERE id = ?",
            params![id],
        )?;
        if let Some((txid, payment_data)) = payment_data {
            tx.execute(
                "INSERT INTO payment_data(id, payer_amount_sat, receiver_amount_sat)
              VALUES(?, ?, ?)",
                (
                    txid,
                    payment_data.payer_amount_sat,
                    payment_data.receiver_amount_sat,
                ),
            )?;
        }
        tx.commit()?;

        Ok(())
    }

    pub(crate) fn list_ongoing_swaps(&self) -> Result<Vec<OngoingSwap>> {
        let con = self.get_connection()?;
        let ongoing_swap_ins: Vec<OngoingSwap> = self
            .list_ongoing_send(&con, vec![])?
            .into_iter()
            .map(OngoingSwap::Send)
            .collect();
        let ongoing_swap_outs: Vec<OngoingSwap> = self
            .list_ongoing_receive(&con, vec![])?
            .into_iter()
            .map(OngoingSwap::Receive)
            .collect();
        Ok([ongoing_swap_ins, ongoing_swap_outs].concat())
    }

    pub fn get_payment_data(&self) -> Result<HashMap<String, PaymentData>> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            SELECT id, payer_amount_sat, receiver_amount_sat
            FROM payment_data
        ",
        )?;

        let data = stmt
            .query_map(params![], |row| {
                Ok((
                    row.get(0)?,
                    PaymentData {
                        payer_amount_sat: row.get(1)?,
                        receiver_amount_sat: row.get(2)?,
                    },
                ))
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(data)
    }
}
