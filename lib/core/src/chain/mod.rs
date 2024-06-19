pub(crate) mod bitcoin;

use lwk_wollet::{BlockchainBackend, ElectrumClient};

pub(crate) trait ChainService: Send + Sync + BlockchainBackend {}
impl ChainService for ElectrumClient {}
