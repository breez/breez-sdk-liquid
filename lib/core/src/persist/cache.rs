use anyhow::Result;
use rusqlite::{Transaction, TransactionBehavior};
use std::str::FromStr;

use super::Persister;

const KEY_SWAPPER_PROXY_URL: &str = "swapper_proxy_url";
const KEY_IS_FIRST_SYNC_COMPLETE: &str = "is_first_sync_complete";
const KEY_WEBHOOK_URL: &str = "webhook_url";
// TODO: The `last_derivation_index` needs to be synced
const KEY_LAST_DERIVATION_INDEX: &str = "last_derivation_index";

impl Persister {
    fn get_cached_item_inner(tx: &Transaction, key: &str) -> Result<Option<String>> {
        let res = tx.query_row(
            "SELECT value FROM cached_items WHERE key = ?1",
            [key],
            |row| row.get(0),
        );
        Ok(res.ok())
    }

    fn update_cached_item_inner(tx: &Transaction, key: &str, value: String) -> Result<()> {
        tx.execute(
            "INSERT OR REPLACE INTO cached_items (key, value) VALUES (?1,?2)",
            (key, value),
        )?;
        Ok(())
    }

    pub fn delete_cached_item_inner(tx: &Transaction, key: &str) -> Result<()> {
        tx.execute("DELETE FROM cached_items WHERE key = ?1", [key])?;
        Ok(())
    }

    pub fn get_cached_item(&self, key: &str) -> Result<Option<String>> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        let res = Self::get_cached_item_inner(&tx, key);
        tx.commit()?;
        res
    }

    pub fn update_cached_item(&self, key: &str, value: String) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        let res = Self::update_cached_item_inner(&tx, key, value);
        tx.commit()?;
        res
    }

    pub fn delete_cached_item(&self, key: &str) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        let res = Self::delete_cached_item_inner(&tx, key);
        tx.commit()?;
        res
    }

    pub fn set_swapper_proxy_url(&self, swapper_proxy_url: String) -> Result<()> {
        self.update_cached_item(KEY_SWAPPER_PROXY_URL, swapper_proxy_url)
    }

    #[allow(dead_code)]
    pub fn remove_swapper_proxy_url(&self) -> Result<()> {
        self.delete_cached_item(KEY_SWAPPER_PROXY_URL)
    }

    pub fn get_swapper_proxy_url(&self) -> Result<Option<String>> {
        self.get_cached_item(KEY_SWAPPER_PROXY_URL)
    }

    pub fn set_is_first_sync_complete(&self, complete: bool) -> Result<()> {
        self.update_cached_item(KEY_IS_FIRST_SYNC_COMPLETE, complete.to_string())
    }

    pub fn get_is_first_sync_complete(&self) -> Result<Option<bool>> {
        self.get_cached_item(KEY_IS_FIRST_SYNC_COMPLETE)
            .map(|maybe_str| maybe_str.and_then(|val_str| bool::from_str(&val_str).ok()))
    }

    pub fn set_webhook_url(&self, webhook_url: String) -> Result<()> {
        self.update_cached_item(KEY_WEBHOOK_URL, webhook_url)
    }

    pub fn remove_webhook_url(&self) -> Result<()> {
        self.delete_cached_item(KEY_WEBHOOK_URL)
    }

    pub fn get_webhook_url(&self) -> Result<Option<String>> {
        self.get_cached_item(KEY_WEBHOOK_URL)
    }

    pub fn set_last_derivation_index(&self, index: u32) -> Result<()> {
        self.update_cached_item(KEY_LAST_DERIVATION_INDEX, index.to_string())
    }

    pub fn get_last_derivation_index(&self) -> Result<Option<u32>> {
        self.get_cached_item(KEY_LAST_DERIVATION_INDEX)
            .map(|maybe_str| maybe_str.and_then(|str| str.as_str().parse::<u32>().ok()))
    }

    pub fn next_derivation_index(&self) -> Result<Option<u32>> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let res = match Self::get_cached_item_inner(&tx, KEY_LAST_DERIVATION_INDEX)? {
            Some(last_index_str) => {
                let next_index = last_index_str
                    .as_str()
                    .parse::<u32>()
                    .map(|index| index + 1)?;
                Self::update_cached_item_inner(
                    &tx,
                    KEY_LAST_DERIVATION_INDEX,
                    next_index.to_string(),
                )?;
                Some(next_index)
            }
            None => None,
        };
        tx.commit()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::test_utils::persist::new_persister;

    #[test]
    fn test_cached_items() -> Result<()> {
        let (_temp_dir, persister) = new_persister()?;

        persister.update_cached_item("key1", "val1".to_string())?;
        let item_value = persister.get_cached_item("key1")?;
        assert_eq!(item_value, Some("val1".to_string()));

        persister.delete_cached_item("key1")?;
        let item_value = persister.get_cached_item("key1")?;
        assert_eq!(item_value, None);

        Ok(())
    }

    #[test]
    fn test_get_last_derivation_index() -> Result<()> {
        let (_temp_dir, persister) = new_persister()?;

        let maybe_last_index = persister.get_last_derivation_index()?;
        assert!(maybe_last_index.is_none());

        persister.set_last_derivation_index(50)?;

        let maybe_last_index = persister.get_last_derivation_index()?;
        assert!(maybe_last_index.is_some());
        assert_eq!(maybe_last_index, Some(50));

        persister.set_last_derivation_index(51)?;

        let maybe_last_index = persister.get_last_derivation_index()?;
        assert!(maybe_last_index.is_some());
        assert_eq!(maybe_last_index, Some(51));

        Ok(())
    }

    #[test]
    fn test_next_derivation_index() -> Result<()> {
        let (_temp_dir, persister) = new_persister()?;

        let maybe_next_index = persister.next_derivation_index()?;
        assert!(maybe_next_index.is_none());

        persister.set_last_derivation_index(50)?;

        let maybe_next_index = persister.next_derivation_index()?;
        assert!(maybe_next_index.is_some());
        assert_eq!(maybe_next_index, Some(51));

        let maybe_last_index = persister.get_last_derivation_index()?;
        assert!(maybe_last_index.is_some());
        assert_eq!(maybe_last_index, Some(51));

        persister.set_last_derivation_index(52)?;

        let maybe_next_index = persister.next_derivation_index()?;
        assert!(maybe_next_index.is_some());
        assert_eq!(maybe_next_index, Some(53));

        let maybe_next_index = persister.next_derivation_index()?;
        assert!(maybe_next_index.is_some());
        assert_eq!(maybe_next_index, Some(54));

        let maybe_last_index = persister.get_last_derivation_index()?;
        assert!(maybe_last_index.is_some());
        assert_eq!(maybe_last_index, Some(54));

        Ok(())
    }
}
