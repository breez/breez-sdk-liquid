use std::{collections::HashMap, time::Duration};

use breez_sdk_liquid::plugin::PluginStorage;

use crate::{
    error::{NwcError, NwcResult},
    model::{NwcConnection, PeriodicBudget},
    MIN_REFRESH_INTERVAL_SEC,
};
use tokio_with_wasm::alias as tokio;

const REFRESH_INTERVAL_GRACE_PERIOD: u64 = 3;
const KEY_NWC_CONNECTIONS: &str = "nwc_connections";
const KEY_NWC_SECKEY: &str = "nwc_seckey";
const KEY_NWC_LOCK: &str = "nwc_lock";

pub(crate) struct Persister {
    pub(crate) storage: PluginStorage,
}

struct Lock<'a> {
    p: &'a Persister,
}

impl<'a> Drop for Lock<'a> {
    fn drop(&mut self) {
        let _ = self.p.storage.set_item(KEY_NWC_LOCK, 0.to_string());
    }
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

    async fn lock<'a>(&'a self) -> NwcResult<Lock<'a>> {
        let mut is_locked = true;
        for _ in 0..3 {
            is_locked = self
                .storage
                .get_item(KEY_NWC_LOCK)?
                .is_some_and(|lock| lock == "1");
            if !is_locked {
                break;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        if is_locked {
            return Err(NwcError::generic("Could not acquire database lock"));
        }
        self.storage.set_item(KEY_NWC_LOCK, 1.to_string())?;
        Ok(Lock { p: self })
    }

    pub(crate) async fn add_nwc_connection(
        &self,
        name: String,
        connection: NwcConnection,
    ) -> NwcResult<()> {
        let _ = self.lock().await?;
        let mut connections = self.list_nwc_connections_inner()?;
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

    // Helper method to update a connection's used budget directly
    pub(crate) async fn update_periodic_budget(
        &self,
        name: &str,
        periodic_budget: PeriodicBudget,
    ) -> NwcResult<()> {
        self.edit_nwc_connection(name, None, None, Some(periodic_budget))
            .await?;
        Ok(())
    }

    pub(crate) async fn edit_nwc_connection(
        &self,
        name: &str,
        expiry_time_sec: Option<u32>,
        receive_only: Option<bool>,
        periodic_budget: Option<PeriodicBudget>,
    ) -> NwcResult<NwcConnection> {
        let _ = self.lock().await?;
        let mut connections = self.list_nwc_connections_inner()?;
        let Some(connection) = connections.get_mut(name) else {
            return Err(NwcError::generic("Connection not found."));
        };
        if let Some(new_expiry_time) = expiry_time_sec {
            connection.expiry_time_sec = Some(new_expiry_time);
        }
        if let Some(new_receive_only) = receive_only {
            connection.receive_only = new_receive_only;
        }
        if let Some(new_periodic_budget) = periodic_budget {
            connection.periodic_budget = Some(new_periodic_budget);
        }
        let connection = connection.clone();
        self.storage
            .set_item(KEY_NWC_CONNECTIONS, serde_json::to_string(&connections)?)?;
        Ok(connection)
    }

    pub(crate) async fn get_min_interval(&self) -> Option<u64> {
        let get_min_connection_interval = async || -> NwcResult<Option<u64>> {
            let _ = self.lock().await?;
            let connections = self.list_nwc_connections_inner()?;
            if connections.is_empty() {
                return Ok(None);
            }
            let mut min_interval = Some(u32::MAX);
            for connection in connections.into_values() {
                min_interval = min_interval
                    .min(connection.expiry_time_sec)
                    .min(connection.periodic_budget.map(|b| b.reset_time_sec));
            }
            Ok(min_interval.map(u64::from))
        };
        match get_min_connection_interval().await {
            Ok(Some(mut interval)) => {
                // We set a minimum of MIN_REFRESH_INTERVAL_SEC to avoid breaking the service by
                // refreshing too frequently
                if interval < MIN_REFRESH_INTERVAL_SEC {
                    interval = MIN_REFRESH_INTERVAL_SEC;
                }
                // We add a grace period to avoid races between the interval tick and the connection
                // expiry/budget refresh
                Some(interval + REFRESH_INTERVAL_GRACE_PERIOD)
            }
            _ => None,
        }
    }

    pub(crate) fn set_connections_raw(
        &self,
        connections: HashMap<String, NwcConnection>,
    ) -> NwcResult<()> {
        let connections = serde_json::to_string(&connections)?;
        self.storage.set_item(KEY_NWC_CONNECTIONS, connections)?;
        Ok(())
    }

    fn list_nwc_connections_inner(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        let connections = self
            .storage
            .get_item(KEY_NWC_CONNECTIONS)?
            .unwrap_or("{}".to_string());
        let connections = serde_json::from_str(&connections)?;
        Ok(connections)
    }

    pub(crate) async fn list_nwc_connections(&self) -> NwcResult<HashMap<String, NwcConnection>> {
        let _ = self.lock().await?;
        self.list_nwc_connections_inner()
    }

    pub(crate) async fn remove_nwc_connection(&self, name: String) -> NwcResult<()> {
        let _ = self.lock().await?;
        let mut connections = self.list_nwc_connections_inner()?;
        if connections.remove(&name).is_none() {
            return Err(NwcError::generic("Connection not found."));
        }
        self.storage
            .set_item(KEY_NWC_CONNECTIONS, serde_json::to_string(&connections)?)?;
        Ok(())
    }
}
