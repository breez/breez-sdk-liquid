use std::collections::HashMap;

use crate::persist::Persister;
use anyhow::Result;

const KEY_NWC_URIS: &str = "nwc_uris";
const KEY_NWC_SECKEY: &str = "nwc_seckey";

impl Persister {
    pub(crate) fn set_nwc_seckey(&self, key: String) -> Result<()> {
        self.update_cached_item(KEY_NWC_SECKEY, key)
    }

    pub(crate) fn get_nwc_seckey(&self) -> Result<Option<String>> {
        self.get_cached_item(KEY_NWC_SECKEY)
    }

    pub(crate) fn set_nwc_uri(&self, name: String, uri: String) -> Result<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        nwc_uris.insert(name, uri);
        self.update_cached_item(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)?;
        Ok(())
    }

    pub(crate) fn list_nwc_uris(&self) -> Result<HashMap<String, String>> {
        let raw_uris = self
            .get_cached_item(KEY_NWC_URIS)?
            .unwrap_or("{}".to_string());
        let uris = serde_json::from_str(&raw_uris)?;
        Ok(uris)
    }

    pub(crate) fn remove_nwc_uri(&self, name: String) -> Result<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        if nwc_uris.remove(&name).is_none() {
            anyhow::bail!("Connection string not found.");
        }
        self.update_cached_item(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)?;
        Ok(())
    }
}
