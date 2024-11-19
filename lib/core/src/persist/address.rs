use anyhow::Result;
use log::debug;
use rusqlite::{Row, Transaction, TransactionBehavior};

use crate::error::PaymentError;

use super::{Persister, ReservedAddress};

impl Persister {
    pub(crate) fn next_expired_reserved_address(
        &self,
        tip: u32,
    ) -> Result<Option<ReservedAddress>> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        // Get the next expired reserved address
        let query = Self::get_reserved_address_query(vec!["expiry_block_height < ?1".to_string()]);
        let res = match tx.query_row(&query, [tip], Self::sql_row_to_reserved_address) {
            Ok(reserved_address) => {
                // Delete the reserved address
                Self::delete_reserved_address_inner(&tx, &reserved_address.address)?;
                Some(reserved_address)
            }
            Err(_) => None,
        };
        tx.commit()?;

        Ok(res)
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
            LIMIT 1
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
        )?;
        debug!(
            "Reserved address {} until block height {}",
            address, expiry_block_height
        );

        Ok(())
    }

    pub(crate) fn delete_reserved_address(&self, address: &str) -> Result<(), PaymentError> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        Self::delete_reserved_address_inner(&tx, address)?;
        tx.commit()?;

        Ok(())
    }

    fn delete_reserved_address_inner(tx: &Transaction, address: &str) -> Result<(), PaymentError> {
        tx.execute(
            "DELETE FROM reserved_addresses WHERE address = ?",
            [address],
        )?;

        Ok(())
    }

    fn sql_row_to_reserved_address(row: &Row) -> rusqlite::Result<ReservedAddress> {
        Ok(ReservedAddress {
            address: row.get(0)?,
            expiry_block_height: row.get(1)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::test_utils::persist::new_persister;

    #[test]
    fn test_next_expired_reserved_address() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let address = "tlq1pq2amlulhea6ltq7x3eu9atsc2nnrer7yt7xve363zxedqwu2mk6ctcyv9awl8xf28cythreqklt5q0qqwsxzlm6wu4z6d574adl9zh2zmr0h85gt534n";

        storage.insert_or_update_reserved_address(address, 100)?;

        let maybe_reserved_address = storage.next_expired_reserved_address(99)?;
        // Under the expiry, not popped
        assert!(maybe_reserved_address.is_none());

        let maybe_reserved_address = storage.next_expired_reserved_address(100)?;
        // Equal to expiry, not popped
        assert!(maybe_reserved_address.is_none());

        let maybe_reserved_address = storage.next_expired_reserved_address(101)?;
        // Address expired, popped
        assert!(maybe_reserved_address.is_some());

        let maybe_reserved_address = storage.next_expired_reserved_address(102)?;
        // Address already popped
        assert!(maybe_reserved_address.is_none());

        Ok(())
    }

    #[test]
    fn test_delete_reserved_address() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let address = "tlq1pq2amlulhea6ltq7x3eu9atsc2nnrer7yt7xve363zxedqwu2mk6ctcyv9awl8xf28cythreqklt5q0qqwsxzlm6wu4z6d574adl9zh2zmr0h85gt534n";

        storage.insert_or_update_reserved_address(address, 100)?;

        let maybe_reserved_address = storage.next_expired_reserved_address(99)?;
        // Under the expiry, not popped
        assert!(maybe_reserved_address.is_none());

        storage.delete_reserved_address(address)?;

        let maybe_reserved_address = storage.next_expired_reserved_address(101)?;
        // Over the expired, but already deleted
        assert!(maybe_reserved_address.is_none());

        Ok(())
    }
}
