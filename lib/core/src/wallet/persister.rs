use log::warn;
use lwk_wollet::{ElementsNetwork, FsPersister, NoPersist};
use maybe_sync::{MaybeSend, MaybeSync};
use std::path::PathBuf;
use std::str::FromStr;

pub use lwk_wollet;

pub type LwkPersister = std::sync::Arc<dyn lwk_wollet::Persister + Send + Sync>;

#[sdk_macros::async_trait]
pub trait WalletCachePersister: Clone + MaybeSend + MaybeSync {
    fn get_lwk_persister(&self) -> LwkPersister;

    async fn clear_cache(&self) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct FsWalletCachePersister {
    working_dir: String,
    persister: std::sync::Arc<FsPersister>,
    elements_network: ElementsNetwork,
}

impl FsWalletCachePersister {
    pub(crate) fn new(
        working_dir: String,
        persister: std::sync::Arc<FsPersister>,
        elements_network: ElementsNetwork,
    ) -> anyhow::Result<Self> {
        let working_dir_buf = PathBuf::from_str(&working_dir)?;
        if !working_dir_buf.exists() {
            std::fs::create_dir_all(&working_dir_buf)?;
        }

        Ok(Self {
            working_dir,
            persister,
            elements_network,
        })
    }
}

#[sdk_macros::async_trait]
impl WalletCachePersister for FsWalletCachePersister {
    fn get_lwk_persister(&self) -> LwkPersister {
        self.persister.clone()
    }

    async fn clear_cache(&self) -> anyhow::Result<()> {
        let mut path = std::path::PathBuf::from(&self.working_dir);
        path.push(self.elements_network.as_str());
        warn!("Wiping wallet in path: {:?}", path);
        std::fs::remove_dir_all(&path)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct NoWalletCachePersister {}

#[sdk_macros::async_trait]
impl WalletCachePersister for NoWalletCachePersister {
    fn get_lwk_persister(&self) -> LwkPersister {
        NoPersist::new()
    }

    async fn clear_cache(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
