use anyhow::Result;
use log::debug;
use lwk_wollet::{PersistError, Update, WolletDescriptor};
use maybe_sync::{MaybeSend, MaybeSync};
use std::sync::{Arc, Mutex};

pub use lwk_wollet;

use crate::persist::Persister;

pub type LwkPersister = Arc<dyn lwk_wollet::Persister + Send + Sync>;

#[sdk_macros::async_trait]
pub trait WalletCachePersister: MaybeSend + MaybeSync {
    fn get_lwk_persister(&self) -> Result<LwkPersister>;

    async fn clear_cache(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct SqliteWalletCachePersister {
    persister: Arc<Persister>,
    descriptor: WolletDescriptor,
}

impl SqliteWalletCachePersister {
    pub fn new(persister: Arc<Persister>, descriptor: WolletDescriptor) -> Result<Self> {
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
    persister: Arc<Persister>,
    descriptor: WolletDescriptor,
    next: Mutex<u64>,
}

impl SqliteLwkPersister {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(
        persister: Arc<Persister>,
        descriptor: WolletDescriptor,
    ) -> Result<LwkPersister> {
        let next = persister.get_next_wallet_update_index()?;
        Ok(Arc::new(Self {
            persister,
            descriptor,
            next: Mutex::new(next),
        }))
    }
}

impl lwk_wollet::Persister for SqliteLwkPersister {
    fn get(&self, index: usize) -> std::result::Result<Option<Update>, PersistError> {
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

    fn push(&self, update: Update) -> std::result::Result<(), PersistError> {
        debug!(
            "LwkPersister starting push update with status {}",
            update.wollet_status
        );

        let mut next = self.next.lock().unwrap();

        let ciphertext = update
            .serialize_encrypted(&self.descriptor)
            .map_err(|e| PersistError::Other(e.to_string()))?;

        self.persister
            .insert_wallet_update(*next, &ciphertext)
            .map_err(|e| PersistError::Other(e.to_string()))?;

        *next += 1;

        Ok(())
    }
}
