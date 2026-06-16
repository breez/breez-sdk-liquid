use anyhow::Result;
use lwk_wollet::{BoxError, DynStore};
use std::fmt;
use std::sync::Arc;

pub use lwk_wollet;

use crate::persist::Persister;

pub type LwkPersister = Arc<dyn DynStore>;

#[sdk_macros::async_trait]
pub trait WalletCachePersister: Send + Sync {
    fn get_lwk_persister(&self) -> Result<LwkPersister>;

    async fn clear_cache(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct SqliteWalletCachePersister {
    persister: Arc<Persister>,
}

impl SqliteWalletCachePersister {
    pub fn new(persister: Arc<Persister>) -> Result<Self> {
        Ok(Self { persister })
    }
}

#[sdk_macros::async_trait]
impl WalletCachePersister for SqliteWalletCachePersister {
    fn get_lwk_persister(&self) -> Result<LwkPersister> {
        Ok(Arc::new(SqliteDynStore {
            persister: Arc::clone(&self.persister),
        }))
    }

    async fn clear_cache(&self) -> Result<()> {
        self.persister.clear_wallet_cache()
    }
}

/// A [`DynStore`] implementation backed by the SDK's SQLite [`Persister`].
///
/// LWK owns serialization, merging and (when persisted) encryption of the wallet
/// cache: this store only persists opaque key-value pairs.
struct SqliteDynStore {
    persister: Arc<Persister>,
}

impl fmt::Debug for SqliteDynStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SqliteDynStore").finish()
    }
}

impl DynStore for SqliteDynStore {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, BoxError> {
        self.persister
            .get_wallet_cache(key)
            .map_err(|e| e.to_string().into())
    }

    fn put(&self, key: &str, value: &[u8]) -> Result<(), BoxError> {
        self.persister
            .set_wallet_cache(key, value)
            .map_err(|e| e.to_string().into())
    }

    fn remove(&self, key: &str) -> Result<(), BoxError> {
        self.persister
            .remove_wallet_cache(key)
            .map_err(|e| e.to_string().into())
    }

    /// The cache is persisted across restarts, so LWK encrypts it with the
    /// descriptor-derived key.
    fn is_persisted(&self) -> bool {
        true
    }
}
