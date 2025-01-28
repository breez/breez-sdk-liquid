use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::{
    model::Signer, recover::recoverer::Recoverer, swapper::Swapper, wallet::OnchainWallet,
};

use super::chain::{MockBitcoinChainService, MockLiquidChainService};

pub(crate) fn new_recoverer(
    signer: Arc<Box<dyn Signer>>,
    swapper: Arc<dyn Swapper>,
    onchain_wallet: Arc<dyn OnchainWallet>,
) -> Result<Recoverer> {
    let liquid_chain_service = Arc::new(MockLiquidChainService::new());
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    Recoverer::new(
        signer.slip77_master_blinding_key()?,
        swapper,
        onchain_wallet,
        liquid_chain_service,
        bitcoin_chain_service,
    )
}
