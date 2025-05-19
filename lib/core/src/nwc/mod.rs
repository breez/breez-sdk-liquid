use anyhow::Result;
use bip39::rand::{self, RngCore};
use hex;
use urlencoding::encode;

mod handler;

pub trait NWCService {
    /// Creates a connection string for Nostr Wallet Connect
    ///
    /// # Arguments
    /// * `relay` - The relay to use for the connection
    /// * `pubkey` - The public key of the wallet
    ///
    /// # Returns
    /// Connection string for Nostr Wallet Connect

    fn create_connection_string(&self, relay: &str, pubkey: &str) -> Result<String>;
}

pub struct BreezNWCService;

impl NWCService for BreezNWCService {
    fn create_connection_string(&self, relay: &str, pubkey: &str) -> Result<String> {
        let mut secret_key = [0u8; 32];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut secret_key)?;
        let secret_key_hex = hex::encode(secret_key);

        Ok(format!(
            "nostr+walletconnect://{}?relay={}&secret={}",
            pubkey,
            encode(relay),
            secret_key_hex
        ))
    }
}

