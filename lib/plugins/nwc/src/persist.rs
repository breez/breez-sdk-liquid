use std::collections::HashMap;

use breez_sdk_liquid::plugin::{PluginStorage, PluginStorageError};

use crate::{
    error::{NwcError, NwcResult},
    model::{NwcConnectionInner, PeriodicBudgetInner, RefreshResult},
    utils, MIN_REFRESH_INTERVAL_SEC,
};

const MAX_SAFE_WRITE_RETRIES: u64 = 3;
const REFRESH_INTERVAL_GRACE_PERIOD: u64 = 3;
const KEY_NWC_CONNECTIONS: &str = "nwc_connections";
const KEY_NWC_SECKEY: &str = "nwc_seckey";

pub(crate) struct Persister {
    pub(crate) storage: PluginStorage,
}

impl Persister {
    pub(crate) fn new(storage: PluginStorage) -> Self {
        Self { storage }
    }

    pub(crate) fn set_nwc_seckey(&self, key: String) -> NwcResult<()> {
        self.storage
            .set_item(KEY_NWC_SECKEY, key, None)
            .map_err(Into::into)
    }

    pub(crate) fn get_nwc_seckey(&self) -> NwcResult<Option<String>> {
        self.storage.get_item(KEY_NWC_SECKEY).map_err(Into::into)
    }

    fn set_connections_safe<F, R>(&self, f: F) -> NwcResult<R>
    where
        F: Fn(&mut HashMap<String, NwcConnectionInner>) -> NwcResult<(bool, R)>,
    {
        for _ in 0..MAX_SAFE_WRITE_RETRIES {
            let connections = self.list_nwc_connections()?;
            let mut new_connections = connections.clone();
            let (changed, result) = f(&mut new_connections)?;
            if changed {
                let set_result = self.storage.set_item(
                    KEY_NWC_CONNECTIONS,
                    serde_json::to_string(&new_connections)?,
                    Some(serde_json::to_string(&connections)?),
                );
                match set_result {
                    Ok(_) => return Ok(result),
                    Err(PluginStorageError::DataTooOld) => continue,
                    Err(err) => return Err(err.into()),
                }
            }
            return Ok(result);
        }
        Err(NwcError::persist("Maximum write attempts reached"))
    }

    pub(crate) fn add_nwc_connection(
        &self,
        name: String,
        connection: NwcConnectionInner,
    ) -> NwcResult<()> {
        self.set_connections_safe(|connections| {
            if connections.contains_key(&name) {
                return Err(NwcError::generic(format!(
                    "Could not insert connection: `{name}` already exists"
                )));
            }
            connections.insert(name.clone(), connection.clone());
            Ok((true, ()))
        })
    }

    // Helper method to update a connection's used budget directly
    pub(crate) fn update_periodic_budget(
        &self,
        name: &str,
        periodic_budget: PeriodicBudgetInner,
    ) -> NwcResult<()> {
        self.edit_nwc_connection(name, None, None, Some(periodic_budget))?;
        Ok(())
    }

    pub(crate) fn edit_nwc_connection(
        &self,
        name: &str,
        expiry_time_sec: Option<u32>,
        receive_only: Option<bool>,
        periodic_budget: Option<PeriodicBudgetInner>,
    ) -> NwcResult<NwcConnectionInner> {
        self.set_connections_safe(|connections| {
            let Some(connection) = connections.get_mut(name) else {
                return Err(NwcError::generic("Connection not found."));
            };
            if let Some(new_expiry_time) = expiry_time_sec {
                connection.expiry_time_sec = Some(new_expiry_time);
            }
            if let Some(new_receive_only) = receive_only {
                connection.receive_only = new_receive_only;
            }
            if let Some(new_periodic_budget) = periodic_budget.clone() {
                connection.periodic_budget = Some(new_periodic_budget);
            }
            Ok((true, connection.clone()))
        })
    }

    pub(crate) fn get_min_interval(&self) -> Option<u64> {
        let get_min_connection_interval = || -> NwcResult<Option<u64>> {
            let connections = self.list_nwc_connections()?;
            if connections.is_empty() {
                return Ok(None);
            }
            let mut min_interval = None;
            for connection in connections.into_values() {
                if let Some(expiry_time_sec) = connection.expiry_time_sec {
                    if expiry_time_sec < min_interval.unwrap_or(u32::MAX) {
                        min_interval = Some(expiry_time_sec);
                    }
                }
                if let Some(renewal_time_sec) =
                    connection.periodic_budget.map(|b| b.renewal_time_sec)
                {
                    if renewal_time_sec < min_interval.unwrap_or(u32::MAX) {
                        min_interval = Some(renewal_time_sec);
                    }
                }
            }
            Ok(min_interval.map(u64::from))
        };
        match get_min_connection_interval() {
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

    /// Refreshes the active connections (expiry and budget) returning two arrays of the
    /// corresponding connection names
    pub(crate) fn refresh_connections(&self) -> NwcResult<RefreshResult> {
        self.set_connections_safe(|connections| {
            let now = utils::now();
            let mut result = RefreshResult::default();
            for (name, connection) in connections.iter_mut() {
                // If the connection has expired, mark it for deletion
                if let Some(expiry) = connection.expiry_time_sec {
                    if now >= connection.created_at + expiry {
                        result.deleted.push(name.clone());
                        continue;
                    }
                }
                // If the connection's periodic budget has to be updated
                if let Some(ref mut budget) = connection.periodic_budget {
                    if now >= budget.updated_at + budget.renewal_time_sec {
                        budget.used_budget_sat = 0;
                        budget.updated_at = now;
                        result.refreshed.push(name.clone());
                    }
                }
            }
            for name in &result.deleted {
                connections.remove(name);
            }
            Ok((
                !result.refreshed.is_empty() || !result.deleted.is_empty(),
                result,
            ))
        })
    }

    pub(crate) fn list_nwc_connections(&self) -> NwcResult<HashMap<String, NwcConnectionInner>> {
        let connections = self
            .storage
            .get_item(KEY_NWC_CONNECTIONS)?
            .unwrap_or("{}".to_string());
        let connections = serde_json::from_str(&connections)?;
        Ok(connections)
    }

    pub(crate) fn remove_nwc_connection(&self, name: String) -> NwcResult<()> {
        self.set_connections_safe(|connections| {
            if connections.remove(&name).is_none() {
                return Err(NwcError::generic("Connection not found."));
            }
            Ok((true, ()))
        })
    }
}
