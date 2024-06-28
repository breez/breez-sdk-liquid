#![cfg(test)]

use bip39::rand::{self, distributions::Alphanumeric, Rng};

pub(crate) mod chain_swap;
pub(crate) mod persist;
pub(crate) mod receive_swap;
pub(crate) mod send_swap;
pub(crate) mod swapper;
pub(crate) mod wallet;

pub(crate) const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

pub(crate) fn generate_random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
