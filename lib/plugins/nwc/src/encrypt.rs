use nostr_sdk::{nips::nip44::Version, Event, PublicKey, SecretKey, TagKind};

use crate::error::NwcError;

pub(crate) struct EncryptionHandler<'a> {
    our_secret: &'a SecretKey,
    client_pubkey: &'a PublicKey,
}

impl<'a> EncryptionHandler<'a> {
    pub(crate) fn new(our_secret: &'a SecretKey, client_pubkey: &'a PublicKey) -> Self {
        Self {
            our_secret,
            client_pubkey,
        }
    }

    fn nip44_decrypt(&self, msg: &str) -> Result<String, NwcError> {
        nostr_sdk::nips::nip44::decrypt(self.our_secret, self.client_pubkey, msg).map_err(|err| {
            NwcError::Encryption {
                err: err.to_string(),
            }
        })
    }

    fn nip04_decrypt(&self, msg: &str) -> Result<String, NwcError> {
        nostr_sdk::nips::nip04::decrypt(self.our_secret, self.client_pubkey, msg).map_err(|err| {
            NwcError::Encryption {
                err: err.to_string(),
            }
        })
    }

    pub(crate) fn decrypt(&self, e: &Event) -> Result<String, NwcError> {
        match e
            .tags
            .find(TagKind::Custom("encryption".into()))
            .is_some_and(|enc_type| enc_type.content() == Some("nip44_v2"))
        {
            true => self.nip44_decrypt(&e.content),
            false => self.nip04_decrypt(&e.content),
        }
    }

    pub(crate) fn nip44_encrypt(&self, msg: &str) -> Result<String, NwcError> {
        nostr_sdk::nips::nip44::encrypt(self.our_secret, self.client_pubkey, msg, Version::V2)
            .map_err(|err| NwcError::Encryption {
                err: err.to_string(),
            })
    }

    pub(crate) fn nip04_encrypt(&self, msg: &str) -> Result<String, NwcError> {
        nostr_sdk::nips::nip04::encrypt(self.our_secret, self.client_pubkey, msg).map_err(|err| {
            NwcError::Encryption {
                err: err.to_string(),
            }
        })
    }

    pub(crate) fn encrypt(&self, e: &Event, msg: &str) -> Result<String, NwcError> {
        match e
            .tags
            .find(TagKind::Custom("encryption".into()))
            .is_some_and(|enc_type| enc_type.content() == Some("nip44_v2"))
        {
            true => self.nip44_encrypt(msg),
            false => self.nip04_encrypt(msg),
        }
    }
}
