use std::collections::HashMap;

use breez_sdk_liquid::plugin::PluginStorage;

use crate::{
    error::{NwcError, NwcResult},
    model::{EditConnectionRequest, NwcConnection, PeriodicBudget},
};

const KEY_NWC_CONNECTIONS: &str = "nwc_connections";
const KEY_NWC_SECKEY: &str = "nwc_seckey";

pub(crate) struct Persister {
    storage: PluginStorage,
}

impl Persister {
    pub(crate) fn new(storage: PluginStorage) -> Self {
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

    pub(crate) fn add_nwc_connection(
        &self,
        name: String,
        connection: NwcConnection,
    ) -> NwcResult<()> {
        let mut connections = self.list_nwc_connections()?;
        if connections.contains_key(&name) {
            return Err(NwcError::generic(format!(
                "Could not insert connection: `{name}` already exists"
            )));
        }
        connections.insert(name, connection);
        self.storage
            .set_item(KEY_NWC_CONNECTIONS, serde_json::to_string(&connections)?)?;
        Ok(())
    }

    pub(crate) fn edit_nwc_connection(
        &self,
        req: EditConnectionRequest,
    ) -> NwcResult<NwcConnection> {
        let mut connections = self.list_nwc_connections()?;
        let Some(connection) = connections.get_mut(&req.name) else {
            return Err(NwcError::generic("Connection not found."));
        };
        if let Some(new_periodic_balance) = req.periodic_budget_req {
            connection.periodic_budget = Some(PeriodicBudget::from_budget_request(
                new_periodic_balance,
                connection.created_at,
            ));
        }
        if let Some(new_expiry_time) = req.expiry_time_sec {
            connection.expiry_time_sec = Some(new_expiry_time);
        }
        let connection = connection.clone();
        self.storage
            .set_item(KEY_NWC_CONNECTIONS, serde_json::to_string(&connections)?)?;
        Ok(connection)
    }

    pub(crate) fn set_connections_raw(
        &self,
        connections: HashMap<String, NwcConnection>,
    ) -> NwcResult<()> {
        let connections = serde_json::to_string(&connections)?;
        self.storage.set_item(KEY_NWC_CONNECTIONS, connections)?;
        Ok(())
    }

    pub(crate) fn list_nwc_connections(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        let connections = self
            .storage
            .get_item(KEY_NWC_CONNECTIONS)?
            .unwrap_or("{}".to_string());
        let connections = serde_json::from_str(&connections)?;
        Ok(connections)
    }

    pub(crate) fn remove_nwc_connection(&self, name: String) -> NwcResult<()> {
        let mut connections = self.list_nwc_connections()?;
        if connections.remove(&name).is_none() {
            return Err(NwcError::generic("Connection not found."));
        }
        self.storage
            .set_item(KEY_NWC_CONNECTIONS, serde_json::to_string(&connections)?)?;
        Ok(())
    }
}
