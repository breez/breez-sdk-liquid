#![cfg(test)]

use std::sync::Arc;

use bip39::rand::{self, distributions::Alphanumeric, Rng};
use lwk_wollet::elements::Transaction;

use crate::wallet::OnchainWallet;

pub(crate) mod chain;
pub(crate) mod chain_swap;
pub(crate) mod persist;
pub(crate) mod receive_swap;
pub(crate) mod sdk;
pub(crate) mod send_swap;
pub(crate) mod status_stream;
pub(crate) mod swapper;
pub(crate) mod wallet;

pub(crate) fn generate_random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub(crate) async fn create_mock_liquid_tx(
    onchain_wallet: Arc<dyn OnchainWallet>,
    amount_sat: u64,
) -> anyhow::Result<Transaction> {
    let address = onchain_wallet.next_unused_address().await?.to_string();
    Ok(onchain_wallet.build_tx(None, &address, amount_sat).await?)
}
