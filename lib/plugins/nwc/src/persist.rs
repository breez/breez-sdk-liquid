use std::collections::HashMap;
use std::sync::Arc;

use breez_sdk_liquid::plugin::PluginStorage;

use crate::error::{NwcError, NwcResult};

const KEY_NWC_URIS: &str = "nwc_uris";
const KEY_NWC_SECKEY: &str = "nwc_seckey";

pub(crate) struct Persister {
    storage: Arc<PluginStorage>,
}

impl Persister {
    pub(crate) fn new(storage: Arc<PluginStorage>) -> Self {
        Self { storage }
    }

    pub(crate) fn set_nwc_seckey(&self, key: String) -> NwcResult<()> {
        self.storage
            .set_item(KEY_NWC_SECKEY, key)
            .map_err(Into::into)
    }

    pub(crate) fn get_nwc_seckey(&self) -> NwcResult<Option<String>> {
        self.storage.get_item(KEY_NWC_SECKEY).map_err(Into::into)
    }

    pub(crate) fn set_nwc_uri(&self, name: String, uri: String) -> NwcResult<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        nwc_uris.insert(name, uri);
        self.storage
            .set_item(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)?;
        Ok(())
    }

    pub(crate) fn list_nwc_uris(&self) -> NwcResult<HashMap<String, String>> {
        let raw_uris = self
            .storage
            .get_item(KEY_NWC_URIS)?
            .unwrap_or("{}".to_string());
        let uris = serde_json::from_str(&raw_uris)?;
        Ok(uris)
    }

    pub(crate) fn remove_nwc_uri(&self, name: String) -> NwcResult<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        if nwc_uris.remove(&name).is_none() {
            NwcError::generic("Connection string not found.");
        }
        self.storage
            .set_item(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)?;
        Ok(())
    }
}
