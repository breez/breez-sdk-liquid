use std::collections::{BTreeMap, HashSet};

use breez_sdk_liquid::plugin::{PluginStorage, PluginStorageError};

use crate::{
    error::{NwcError, NwcResult},
    model::{EditConnectionRequest, NwcConnectionInner, PeriodicBudgetInner, RefreshResult},
    utils, MIN_REFRESH_INTERVAL_SEC,
};

const MAX_SAFE_WRITE_RETRIES: u64 = 3;
const REFRESH_INTERVAL_GRACE_PERIOD: u64 = 3;
const KEY_NWC_CONNECTIONS: &str = "nwc_connections";
const KEY_NWC_SECKEY: &str = "nwc_seckey";
const KEY_NWC_PAID_INVOICES: &str = "nwc_paid_invoices";

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
        F: Fn(&mut BTreeMap<String, NwcConnectionInner>) -> NwcResult<(bool, R)>,
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
                return Err(NwcError::ConnectionExists);
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
        self.set_connections_safe(|connections| {
            let Some(connection) = connections.get_mut(name) else {
                return Err(NwcError::ConnectionNotFound);
            };
            connection.periodic_budget = Some(periodic_budget.clone());
            Ok((true, ()))
        })
    }

    pub(crate) fn edit_nwc_connection(
        &self,
        req: EditConnectionRequest,
    ) -> NwcResult<NwcConnectionInner> {
        self.set_connections_safe(|connections| {
            let Some(connection) = connections.get_mut(&req.name) else {
                return Err(NwcError::ConnectionNotFound );
            };
            if let Some(new_receive_only) = req.receive_only {
                connection.receive_only = new_receive_only;
            }
            match (req.expiry_time_mins, req.remove_expiry) {
                (Some(_), Some(_)) => {
                    return Err(NwcError::generic(
                        "`expiry_time_mins` and `remove_expiry` cannot both be set at once",
                    ));
                }
                (Some(new_expiry_mins), None) => {
                    connection.expiry_time_sec = Some(utils::mins_to_seconds(new_expiry_mins))
                }
                (None, Some(true)) => connection.expiry_time_sec = None,
                _ => {}
            }
            match (req.periodic_budget_req.clone(), req.remove_periodic_budget) {
                (Some(_), Some(_)) => {
                    return Err(NwcError::generic(
                        "`periodic_budget_req` and `remove_periodic_budget` cannot both be set at once",
                    ));
                }
                (Some(periodic_budget_req), None) => {
                    connection.periodic_budget = Some(PeriodicBudgetInner::from_budget_request(periodic_budget_req, utils::now()));
                }
                (None, Some(true)) => connection.periodic_budget = None,
                _ => {}
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
                    connection.periodic_budget.and_then(|b| b.renewal_time_sec)
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
                    if let Some(renewal_time_sec) = budget.renewal_time_sec {
                        if now >= budget.updated_at + renewal_time_sec {
                            budget.used_budget_sat = 0;
                            budget.updated_at = now;
                            result.refreshed.push(name.clone());
                        }
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

    pub(crate) fn list_nwc_connections(&self) -> NwcResult<BTreeMap<String, NwcConnectionInner>> {
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
                return Err(NwcError::ConnectionNotFound);
            }
            Ok((true, ()))
        })?;
        self.set_paid_invoices_safe(|paid_invoices| {
            paid_invoices.remove(&name);
            Ok((true, ()))
        })?;
        Ok(())
    }

    fn set_paid_invoices_safe<F, R>(&self, f: F) -> NwcResult<R>
    where
        F: Fn(&mut BTreeMap<String, HashSet<String>>) -> NwcResult<(bool, R)>,
    {
        for _ in 0..MAX_SAFE_WRITE_RETRIES {
            let paid_invoices = self.list_paid_invoices()?;
            let mut new_paid_invoices = paid_invoices.clone();
            let (changed, result) = f(&mut new_paid_invoices)?;
            if changed {
                let set_result = self.storage.set_item(
                    KEY_NWC_PAID_INVOICES,
                    serde_json::to_string(&new_paid_invoices)?,
                    Some(serde_json::to_string(&paid_invoices)?),
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

    pub(crate) fn list_paid_invoices(&self) -> NwcResult<BTreeMap<String, HashSet<String>>> {
        let paid_invoices = self
            .storage
            .get_item(KEY_NWC_PAID_INVOICES)?
            .unwrap_or("{}".to_string());
        let paid_invoices = serde_json::from_str(&paid_invoices)?;
        Ok(paid_invoices)
    }

    pub(crate) fn add_paid_invoice(&self, connection: &str, invoice: String) -> NwcResult<()> {
        self.set_paid_invoices_safe(|paid_invoices| {
            let invoices = paid_invoices
                .entry(connection.to_string())
                .or_insert_with(HashSet::new);
            invoices.insert(invoice.clone());
            Ok((true, ()))
        })
    }
}
