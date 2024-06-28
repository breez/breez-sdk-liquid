#![cfg(test)]

use bip39::rand::{self, distributions::Alphanumeric, Rng};

pub(crate) mod chain;
pub(crate) mod chain_swap;
pub(crate) mod persist;
pub(crate) mod receive_swap;
pub(crate) mod sdk;
pub(crate) mod send_swap;
pub(crate) mod status_stream;
pub(crate) mod swapper;
pub(crate) mod wallet;

pub(crate) const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
pub(crate) const TEST_TX_TXID: &str =
    "59dd7a0bce4f3310272ff352402291bc555f141149812d8f573f62e7fdc19cc4";

pub(crate) fn generate_random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
