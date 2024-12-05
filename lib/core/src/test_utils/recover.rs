use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::{model::Signer, recover::recoverer::Recoverer};

use super::chain::{MockBitcoinChainService, MockLiquidChainService};

pub(crate) fn new_recoverer(signer: Arc<Box<dyn Signer>>) -> Result<Recoverer> {
    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    Recoverer::new(
        signer.slip77_master_blinding_key()?,
        liquid_chain_service,
        bitcoin_chain_service,
    )
}
