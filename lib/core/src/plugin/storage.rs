use anyhow::{bail, Result};

use crate::persist::Persister;

pub struct PluginStorage {
    plugin_id: String,
    persister: std::sync::Arc<Persister>,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginStorageError {
    #[error("Could not write to plugin storage: {err}")]
    Generic { err: String },
}

impl From<anyhow::Error> for PluginStorageError {
    fn from(value: anyhow::Error) -> Self {
        Self::Generic {
            err: value.to_string(),
        }
    }
}

impl PluginStorage {
    pub(crate) fn new(persister: std::sync::Arc<Persister>, plugin_id: String) -> Result<Self> {
        if plugin_id.is_empty() {
            log::error!("Plugin ID cannot be an empty string!");
            bail!("Plugin ID cannot be an empty string!");
        }

        Ok(Self {
            persister,
            plugin_id,
        })
    }

    pub(crate) fn scoped_key(&self, key: &str) -> String {
        format!("{}-{}", self.plugin_id, key)
    }

    pub fn set_item(&self, key: &str, value: String) -> Result<(), PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .update_cached_item(&scoped_key, value)
            .map_err(Into::into)
    }

    pub fn get_item(&self, key: &str) -> Result<Option<String>, PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .get_cached_item(&scoped_key)
            .map_err(Into::into)
    }

    pub fn remove_item(&self, key: &str) -> Result<(), PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        self.persister
            .delete_cached_item(&scoped_key)
            .map_err(Into::into)
    }
}
