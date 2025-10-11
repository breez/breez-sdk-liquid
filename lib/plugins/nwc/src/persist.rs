use std::collections::HashMap;

use crate::error::{NwcError, NwcResult};
use aes::cipher::KeyInit;
use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore as _, Aes256Gcm, Nonce,
};
use breez_sdk_liquid::plugin::PluginStorage;
use sha2::Digest;

const KEY_NWC_URIS: &str = "nwc_uris";
const KEY_NWC_SECKEY: &str = "nwc_seckey";

pub(crate) struct Persister {
    storage: PluginStorage,
    cipher: Option<Aes256Gcm>,
}

impl Persister {
    pub(crate) fn new(storage: PluginStorage, passphrase: Option<String>) -> Self {
        let cipher = passphrase.map(|pass| {
            let digest = sha2::Sha256::digest(&pass);
            Aes256Gcm::new(&digest)
        });
        Self { storage, cipher }
    }

    fn encrypt(&self, data: String) -> NwcResult<String> {
        let Some(cipher) = self.cipher.as_ref() else {
            return Ok(data);
        };
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(NwcError::persist)?;
        let mut payload = nonce.to_vec();
        payload.extend_from_slice(&encrypted);
        Ok(hex::encode(payload))
    }

    fn set_item_encrypted(&self, key: &str, value: String) -> NwcResult<()> {
        self.storage
            .set_item(key, self.encrypt(value)?)
            .map_err(Into::into)
    }

    fn decrypt(&self, data: String) -> NwcResult<String> {
        let Some(cipher) = self.cipher.as_ref() else {
            return Ok(data);
        };

        let decoded = hex::decode(data).map_err(NwcError::persist)?;
        let (nonce, data) = decoded.split_at(12);
        let nonce = Nonce::from_slice(nonce);
        let decrypted = cipher.decrypt(nonce, data).map_err(NwcError::persist)?;
        let result = String::from_utf8(decrypted).map_err(NwcError::persist)?;
        Ok(result)
    }

    fn get_item_decrypted(&self, key: &str) -> NwcResult<Option<String>> {
        Ok(match self.storage.get_item(key)? {
            Some(data) => Some(self.decrypt(data)?),
            None => None,
        })
    }

    pub(crate) fn set_nwc_seckey(&self, key: String) -> NwcResult<()> {
        self.set_item_encrypted(KEY_NWC_SECKEY, key)
    }

    pub(crate) fn get_nwc_seckey(&self) -> NwcResult<Option<String>> {
        self.get_item_decrypted(KEY_NWC_SECKEY)
    }

    pub(crate) fn set_nwc_uri(&self, name: String, uri: String) -> NwcResult<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        nwc_uris.insert(name, uri);
        self.set_item_encrypted(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)
    }

    pub(crate) fn list_nwc_uris(&self) -> NwcResult<HashMap<String, String>> {
        let raw_uris = self
            .get_item_decrypted(KEY_NWC_URIS)?
            .unwrap_or("{}".to_string());
        let uris = serde_json::from_str(&raw_uris)?;
        Ok(uris)
    }

    pub(crate) fn remove_nwc_uri(&self, name: String) -> NwcResult<()> {
        let mut nwc_uris = self.list_nwc_uris()?;
        if nwc_uris.remove(&name).is_none() {
            NwcError::generic("Connection string not found.");
        }
        self.set_item_encrypted(KEY_NWC_URIS, serde_json::to_string(&nwc_uris)?)
    }
}
