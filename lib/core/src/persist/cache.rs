use anyhow::Result;
use rusqlite::{OptionalExtension, Transaction, TransactionBehavior};
use std::str::FromStr;

use crate::model::GetInfoResponse;
use crate::sync::model::{data::LAST_DERIVATION_INDEX_DATA_ID, RecordType};

use super::{BlockchainInfo, Persister, WalletInfo};

const KEY_WALLET_INFO: &str = "wallet_info";
const KEY_BLOCKCHAIN_INFO: &str = "blockchain_info";
const KEY_SWAPPER_PROXY_URL: &str = "swapper_proxy_url";
const KEY_IS_FIRST_SYNC_COMPLETE: &str = "is_first_sync_complete";
const KEY_WEBHOOK_URL: &str = "webhook_url";
pub(crate) const KEY_LAST_DERIVATION_INDEX: &str = "last_derivation_index";
const KEY_LAST_SCANNED_DERIVATION_INDEX: &str = "last_scanned_derivation_index";

impl Persister {
    pub(crate) fn get_cached_item_inner(tx: &Transaction, key: &str) -> Result<Option<String>> {
        let res = tx.query_row(
            "SELECT value FROM cached_items WHERE key = ?1",
            [key],
            |row| row.get(0),
        );
        Ok(res.ok())
    }

    pub(crate) fn update_cached_item_inner(
        tx: &Transaction,
        key: &str,
        value: String,
    ) -> Result<()> {
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

    pub fn set_wallet_info(&self, info: &WalletInfo) -> Result<()> {
        let serialized_info = serde_json::to_string(info)?;
        self.update_cached_item(KEY_WALLET_INFO, serialized_info)
    }

    pub fn update_blockchain_info(&self, liquid_tip: u32, bitcoin_tip: Option<u32>) -> Result<()> {
        let info = match bitcoin_tip {
            Some(bitcoin_tip) => BlockchainInfo {
                liquid_tip,
                bitcoin_tip,
            },
            None => {
                let current_tip = self
                    .get_cached_item(KEY_BLOCKCHAIN_INFO)?
                    .and_then(|info| serde_json::from_str::<BlockchainInfo>(&info).ok())
                    .map(|info| info.bitcoin_tip)
                    .unwrap_or(0);
                BlockchainInfo {
                    liquid_tip,
                    bitcoin_tip: current_tip,
                }
            }
        };

        let serialized_info = serde_json::to_string(&info)?;
        self.update_cached_item(KEY_BLOCKCHAIN_INFO, serialized_info)
    }

    pub fn get_info(&self) -> Result<Option<GetInfoResponse>> {
        let con = self.get_connection()?;

        let info: Option<(Option<String>, Option<String>)> = con
            .query_row(
                &format!(
                    "
            SELECT
                c1.value AS wallet_info,
                COALESCE(c2.value, NULL) AS blockchain_info
            FROM (SELECT value FROM cached_items WHERE key = '{KEY_WALLET_INFO}') c1
            LEFT JOIN (SELECT value FROM cached_items WHERE key = '{KEY_BLOCKCHAIN_INFO}') c2
        "
                ),
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        match info {
            Some((Some(wallet_info), blockchain_info)) => {
                let wallet_info = serde_json::from_str(&wallet_info)?;
                let blockchain_info = blockchain_info
                    .and_then(|info| serde_json::from_str(&info).ok())
                    .unwrap_or_default();
                Ok(Some(GetInfoResponse {
                    wallet_info,
                    blockchain_info,
                }))
            }
            _ => Ok(None),
        }
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

    pub fn set_last_derivation_index_inner(&self, tx: &Transaction, index: u32) -> Result<()> {
        Self::update_cached_item_inner(tx, KEY_LAST_DERIVATION_INDEX, index.to_string())?;
        self.commit_outgoing(
            tx,
            LAST_DERIVATION_INDEX_DATA_ID,
            RecordType::LastDerivationIndex,
            // insert a mock updated field so that merging with incoming data works as expected
            Some(vec![LAST_DERIVATION_INDEX_DATA_ID.to_string()]),
        )
    }

    pub fn set_last_derivation_index(&self, index: u32) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        self.set_last_derivation_index_inner(&tx, index)?;
        tx.commit()?;
        self.trigger_sync();
        Ok(())
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
                self.set_last_derivation_index_inner(&tx, next_index)?;
                Some(next_index)
            }
            None => None,
        };
        tx.commit()?;
        self.trigger_sync();
        Ok(res)
    }

    pub fn set_last_scanned_derivation_index(&self, index: u32) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        Self::update_cached_item_inner(&tx, KEY_LAST_SCANNED_DERIVATION_INDEX, index.to_string())?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_last_scanned_derivation_index(&self) -> Result<Option<u32>> {
        self.get_cached_item(KEY_LAST_SCANNED_DERIVATION_INDEX)
            .map(|maybe_str| maybe_str.and_then(|str| str.as_str().parse::<u32>().ok()))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::test_utils::persist::create_persister;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_cached_items() -> Result<()> {
        create_persister!(persister);

        persister.update_cached_item("key1", "val1".to_string())?;
        let item_value = persister.get_cached_item("key1")?;
        assert_eq!(item_value, Some("val1".to_string()));

        persister.delete_cached_item("key1")?;
        let item_value = persister.get_cached_item("key1")?;
        assert_eq!(item_value, None);

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_get_last_derivation_index() -> Result<()> {
        create_persister!(persister);

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

    #[sdk_macros::test_all]
    fn test_next_derivation_index() -> Result<()> {
        create_persister!(persister);

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
