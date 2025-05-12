use anyhow::Result;
use log::warn;
use lwk_wollet::{PersistError, Update, WolletDescriptor};
use maybe_sync::{MaybeSend, MaybeSync};
use std::sync::{Arc, Mutex};

pub use lwk_wollet;

use crate::persist::Persister;

pub type LwkPersister = std::sync::Arc<dyn lwk_wollet::Persister + Send + Sync>;

#[sdk_macros::async_trait]
pub trait WalletCachePersister: MaybeSend + MaybeSync {
    fn get_lwk_persister(&self) -> Result<LwkPersister>;

    async fn clear_cache(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct SqliteWalletCachePersister {
    persister: std::sync::Arc<Persister>,
    descriptor: WolletDescriptor,
}

impl SqliteWalletCachePersister {
    pub fn new(persister: std::sync::Arc<Persister>, descriptor: WolletDescriptor) -> Result<Self> {
        Ok(Self {
            persister,
            descriptor,
        })
    }
}

#[sdk_macros::async_trait]
impl WalletCachePersister for SqliteWalletCachePersister {
    fn get_lwk_persister(&self) -> Result<LwkPersister> {
        SqliteLwkPersister::new(Arc::clone(&self.persister), self.descriptor.clone())
    }

    async fn clear_cache(&self) -> Result<()> {
        self.persister.clear_wallet_updates()
    }
}

pub(crate) struct SqliteLwkPersister {
    persister: std::sync::Arc<Persister>,
    descriptor: WolletDescriptor,
    // An empty next means that the persister is disabled
    next: Mutex<Option<u64>>,
}

impl SqliteLwkPersister {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(
        persister: std::sync::Arc<Persister>,
        descriptor: WolletDescriptor,
    ) -> Result<LwkPersister> {
        let next = persister.get_next_wallet_update_index()?;
        Ok(std::sync::Arc::new(Self {
            persister,
            descriptor,
            next: Mutex::new(Some(next)),
        }))
    }
}

impl lwk_wollet::Persister for SqliteLwkPersister {
    fn get(&self, index: usize) -> std::result::Result<Option<lwk_wollet::Update>, PersistError> {
        let maybe_update_bytes = self
            .persister
            .get_wallet_update(index as u64)
            .map_err(|e| PersistError::Other(e.to_string()))?;
        maybe_update_bytes
            .map(|update_bytes| {
                Update::deserialize_decrypted(&update_bytes, &self.descriptor)
                    .map_err(|e| PersistError::Other(e.to_string()))
            })
            .transpose()
    }

    fn push(
        &self,
        mut update: lwk_wollet::Update,
    ) -> std::result::Result<(), lwk_wollet::PersistError> {
        let mut next = self.next.lock().unwrap();
        if let Some(mut next_value) = *next {
            // If both updates are only tip updates, we can merge them.
            // See https://github.com/Blockstream/lwk/blob/0322a63310f8c8414c537adff68dcbbc7ff4662d/lwk_wollet/src/persister.rs#L174
            if update.only_tip() {
                if let Ok(Some(prev_update)) = self.get(next_value as usize - 1) {
                    if prev_update.only_tip() {
                        update.wollet_status = prev_update.wollet_status;
                        next_value -= 1;
                    }
                }
            }

            let ciphertext = update
                .serialize_encrypted(&self.descriptor)
                .map_err(|e| PersistError::Other(e.to_string()))?;
            if let Err(e) = self
                .persister
                .insert_or_update_wallet_update(next_value, &ciphertext)
            {
                warn!("Error persisting wollet update: {:?}", e);
                *next = None;
            } else {
                *next = Some(next_value + 1);
            }
        } else {
            // A past failure is likely to be caused by multiple competing instances of the SDK.
            // If we see a failure, we stop persisting and let the other instance(s) take over.
            warn!("Skipping wollet update - SqliteLwkPersister is disabled due to past failure");
        }
        Ok(())
    }
}
