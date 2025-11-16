use aes::cipher::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore as _, Aes256Gcm, KeyInit as _, Nonce,
};
use anyhow::{bail, Result};

use std::sync::{Arc, Weak};

use crate::persist::Persister;

#[derive(Clone)]
pub struct PluginStorage {
    plugin_id: String,
    persister: Weak<Persister>,
    cipher: Aes256Gcm,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginStorageError {
    #[error("Could not write to storage: value has changed since last read.")]
    DataTooOld,

    #[error("Could not encrypt storage data: {err}")]
    Encryption { err: String },

    #[error("Plugin storage operation failed: {err}")]
    Generic { err: String },
}

impl From<aes_gcm::Error> for PluginStorageError {
    fn from(value: aes_gcm::Error) -> Self {
        Self::Encryption {
            err: value.to_string(),
        }
    }
}

impl From<rusqlite::Error> for PluginStorageError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Generic {
            err: value.to_string(),
        }
    }
}

impl From<anyhow::Error> for PluginStorageError {
    fn from(value: anyhow::Error) -> Self {
        Self::Generic {
            err: value.to_string(),
        }
    }
}

impl PluginStorage {
    pub(crate) fn new(
        persister: Weak<Persister>,
        passphrase: &[u8],
        plugin_id: String,
    ) -> Result<Self> {
        if plugin_id.is_empty() {
            log::error!("Plugin ID cannot be an empty string!");
            bail!("Plugin ID cannot be an empty string!");
        }
        let passphrase = GenericArray::clone_from_slice(passphrase);
        let cipher = Aes256Gcm::new(&passphrase);

        Ok(Self {
            cipher,
            persister,
            plugin_id,
        })
    }

    fn get_persister(&self) -> Result<Arc<Persister>, PluginStorageError> {
        self.persister.upgrade().ok_or(PluginStorageError::Generic {
            err: "SDK is not running.".to_string(),
        })
    }

    fn encrypt(&self, data: String) -> Result<String, PluginStorageError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = self.cipher.encrypt(&nonce, data.as_bytes())?;
        let mut payload = nonce.to_vec();
        payload.extend_from_slice(&encrypted);
        Ok(hex::encode(payload))
    }

    fn decrypt(&self, data: String) -> Result<String, PluginStorageError> {
        let decoded = hex::decode(data).map_err(|err| PluginStorageError::Encryption {
            err: err.to_string(),
        })?;
        let (nonce, data) = decoded.split_at(12);
        let nonce = Nonce::from_slice(nonce);
        let decrypted = self.cipher.decrypt(nonce, data)?;
        let result =
            String::from_utf8(decrypted).map_err(|err| PluginStorageError::Encryption {
                err: err.to_string(),
            })?;
        Ok(result)
    }

    pub(crate) fn scoped_key(&self, key: &str) -> String {
        format!("{}-{}", self.plugin_id, key)
    }

    /// Writes/updates a value in the database
    ///
    /// # Arguments
    ///   - key: The name of the database key to write into
    ///   - value: The value to write
    ///   - old_value (optional): The previous value of that field (if any). When provided, it
    ///     will ensure that the value that's being written has not been modified, throwing a
    ///     [PluginStorageError::DataTooOld] error otherwise
    pub fn set_item(
        &self,
        key: &str,
        value: String,
        old_value: Option<String>,
    ) -> Result<(), PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        let persister = self.get_persister()?;
        let mut con = persister.get_connection()?;
        let tx = con.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        if let Some(old_value) = old_value {
            if let Some(current_value) = Persister::get_cached_item_inner(&tx, &scoped_key)? {
                let current_value = self.decrypt(current_value)?;
                if old_value != current_value {
                    return Err(PluginStorageError::DataTooOld);
                }
            }
        }
        Persister::update_cached_item_inner(&tx, &scoped_key, self.encrypt(value)?)?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_item(&self, key: &str) -> Result<Option<String>, PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        let value = self
            .get_persister()?
            .get_cached_item(&scoped_key)
            .map_err(Into::<PluginStorageError>::into)?;
        if let Some(value) = value {
            return Ok(Some(self.decrypt(value)?));
        }
        Ok(None)
    }

    pub fn remove_item(&self, key: &str) -> Result<(), PluginStorageError> {
        let scoped_key = self.scoped_key(key);
        self.get_persister()?
            .delete_cached_item(&scoped_key)
            .map_err(Into::into)
    }
}
