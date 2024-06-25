use anyhow::Result;

use super::Persister;

const KEY_SWAPPER_PROXY_URL: &str = "swapper_proxy_url";

impl Persister {
    pub fn get_cached_item(&self, key: &str) -> Result<Option<String>> {
        let res = self.get_connection()?.query_row(
            "SELECT value FROM cached_items WHERE key = ?1",
            [key],
            |row| row.get(0),
        );
        Ok(res.ok())
    }

    pub fn update_cached_item(&self, key: &str, value: String) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO cached_items (key, value) VALUES (?1,?2)",
            (key, value),
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_cached_item(&self, key: &str) -> Result<()> {
        self.get_connection()?
            .execute("DELETE FROM cached_items WHERE key = ?1", [key])?;
        Ok(())
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
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::test_utils::new_persister;

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
}
