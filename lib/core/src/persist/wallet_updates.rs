use super::Persister;

use anyhow::Result;
use rusqlite::OptionalExtension;

impl Persister {
    /// Get a value from the wallet cache key-value store.
    pub(crate) fn get_wallet_cache(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let conn = self.get_connection()?;
        let data: Option<Vec<u8>> = conn
            .query_row(
                "SELECT value FROM wallet_cache_kv WHERE key = ?",
                [key],
                |row| row.get(0),
            )
            .optional()?;

        Ok(data)
    }

    /// Insert or update a value in the wallet cache key-value store.
    pub(crate) fn set_wallet_cache(&self, key: &str, value: &[u8]) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT INTO wallet_cache_kv (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    /// Remove a value from the wallet cache key-value store.
    pub(crate) fn remove_wallet_cache(&self, key: &str) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM wallet_cache_kv WHERE key = ?", [key])?;
        Ok(())
    }

    pub(crate) fn clear_wallet_cache(&self) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM wallet_cache_kv", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::persist::create_persister;
    use anyhow::Result;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_wallet_cache_basic_operations() -> Result<()> {
        create_persister!(storage);

        // Test getting non-existent key
        assert_eq!(storage.get_wallet_cache("key1")?, None);

        // Test inserting a value
        let value1 = b"test value 1";
        storage.set_wallet_cache("key1", value1)?;
        assert_eq!(storage.get_wallet_cache("key1")?, Some(value1.to_vec()));

        // Test updating an existing value
        let value2 = b"test value 2";
        storage.set_wallet_cache("key1", value2)?;
        assert_eq!(storage.get_wallet_cache("key1")?, Some(value2.to_vec()));

        // Test a second key
        storage.set_wallet_cache("key2", value1)?;
        assert_eq!(storage.get_wallet_cache("key2")?, Some(value1.to_vec()));

        // Test removing a key
        storage.remove_wallet_cache("key1")?;
        assert_eq!(storage.get_wallet_cache("key1")?, None);
        assert_eq!(storage.get_wallet_cache("key2")?, Some(value1.to_vec()));

        // Test clearing the cache
        storage.clear_wallet_cache()?;
        assert_eq!(storage.get_wallet_cache("key2")?, None);

        Ok(())
    }
}
