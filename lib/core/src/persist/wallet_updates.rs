use crate::utils::{from_row_to_optional_u64, from_u64_to_row};

use super::Persister;

use anyhow::Result;
use rusqlite::{OptionalExtension, TransactionBehavior};

impl Persister {
    /// Inserts a new wallet update if the provided index matches the next index
    pub(crate) fn insert_wallet_update(&self, index: u64, update: &[u8]) -> Result<()> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

        let next_index = self.get_next_index(&tx)?;

        // Only allow inserting at next_index
        if index != next_index {
            return Err(anyhow::anyhow!(
                "Invalid index for insert: tried {} - must be {}",
                index,
                next_index
            ));
        }

        tx.execute(
            "INSERT INTO wallet_updates (id, data) VALUES (?, ?)",
            (from_u64_to_row(index)?, update),
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn get_next_wallet_update_index(&self) -> Result<u64> {
        let conn = self.get_connection()?;
        self.get_next_index(&conn)
    }

    pub(crate) fn get_wallet_update(&self, index: u64) -> Result<Option<Vec<u8>>> {
        let conn = self.get_connection()?;
        let data: Option<Vec<u8>> = conn
            .query_row(
                "SELECT data FROM wallet_updates WHERE id = ?",
                [from_u64_to_row(index)?],
                |row| row.get(0),
            )
            .optional()?;

        Ok(data)
    }

    pub(crate) fn clear_wallet_updates(&self) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM wallet_updates", [])?;
        Ok(())
    }

    fn get_next_index(&self, conn: &rusqlite::Connection) -> Result<u64> {
        let max_index: Option<u64> =
            conn.query_row("SELECT MAX(id) FROM wallet_updates", [], |row| {
                from_row_to_optional_u64(row, 0)
            })?;
        Ok(max_index.map_or(0, |max| max + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::persist::create_persister;
    use anyhow::Result;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_wallet_updates_basic_operations() -> Result<()> {
        create_persister!(storage);

        // Test initial state
        assert_eq!(storage.get_next_wallet_update_index()?, 0);

        // Test inserting first update
        let update1 = b"test update 1";
        storage.insert_wallet_update(0, update1)?;
        assert_eq!(storage.get_next_wallet_update_index()?, 1);
        assert_eq!(storage.get_wallet_update(0)?, Some(update1.to_vec()));

        // Test inserting second update
        let update2 = b"test update 2";
        storage.insert_wallet_update(1, update2)?;
        assert_eq!(storage.get_next_wallet_update_index()?, 2);
        assert_eq!(storage.get_wallet_update(1)?, Some(update2.to_vec()));

        // Test clearing updates
        storage.clear_wallet_updates()?;
        assert_eq!(storage.get_next_wallet_update_index()?, 0);

        // Verify we can insert again
        storage.insert_wallet_update(0, update1)?;

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_wallet_updates_invalid_index() -> Result<()> {
        create_persister!(storage);

        // Test inserting with invalid index
        let update = b"test update";
        assert!(storage.insert_wallet_update(1, update).is_err());

        // Insert first update
        storage.insert_wallet_update(0, update)?;

        // Test inserting with index too far ahead
        assert!(storage.insert_wallet_update(2, update).is_err());

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_wallet_updates_get_nonexistent() -> Result<()> {
        create_persister!(storage);

        // Test getting non-existent update
        assert_eq!(storage.get_wallet_update(0)?, None);

        Ok(())
    }
}
