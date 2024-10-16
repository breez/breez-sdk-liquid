use anyhow::Result;
use log::debug;
use rusqlite::{Connection, Row};

use crate::error::PaymentError;

use super::{Persister, ReservedAddress};

impl Persister {
    pub(crate) fn get_expired_reserved_address(&self, tip: u32) -> Result<Option<ReservedAddress>> {
        let con: Connection = self.get_connection()?;
        let query = Self::get_reserved_address_query(vec!["expiry_block_height < ?1".to_string()]);
        let res = con.query_row(&query, [tip], Self::sql_row_to_reserved_address);

        Ok(res.ok())
    }

    fn get_reserved_address_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                address,
                expiry_block_height
            FROM reserved_addresses
            {where_clause_str}
            ORDER BY expiry_block_height ASC
        "
        )
    }

    pub(crate) fn insert_or_update_reserved_address(
        &self,
        address: &str,
        expiry_block_height: u32,
    ) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "INSERT OR REPLACE INTO reserved_addresses (
           address,
           expiry_block_height
        )
        VALUES (?, ?)
        ",
            (&address, expiry_block_height),
        )
        .map_err(|_| PaymentError::PersistError)?;
        debug!(
            "Reserved address {} until block height {}",
            address, expiry_block_height
        );

        Ok(())
    }

    pub(crate) fn delete_reserved_address(&self, address: &str) -> Result<(), PaymentError> {
        let con = self.get_connection()?;
        con.execute(
            "DELETE FROM reserved_addresses WHERE address = ?",
            [address],
        )
        .map_err(|_| PaymentError::PersistError)?;

        Ok(())
    }

    fn sql_row_to_reserved_address(row: &Row) -> rusqlite::Result<ReservedAddress> {
        Ok(ReservedAddress {
            address: row.get(0)?,
            expiry_block_height: row.get(1)?,
        })
    }
}
