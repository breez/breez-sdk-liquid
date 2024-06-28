#![cfg(test)]

use crate::{model::Config, wallet::LiquidOnchainWallet};
use anyhow::Result;

use super::TEST_MNEMONIC;

pub(crate) fn new_onchain_wallet(config: &Config) -> Result<LiquidOnchainWallet> {
    LiquidOnchainWallet::new(TEST_MNEMONIC.to_string(), config.clone())
}
