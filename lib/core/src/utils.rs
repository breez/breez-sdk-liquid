use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{PaymentError, SdkResult};
use anyhow::{anyhow, ensure, Result};
use lightning::offers::invoice::Bolt12Invoice;
use lwk_wollet::elements::encode::deserialize;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::elements::{
    LockTime::{self, *},
    Transaction,
};
use sdk_common::bitcoin::bech32;
use sdk_common::bitcoin::bech32::FromBase32;

pub(crate) fn now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

pub(crate) fn json_to_pubkey(json: &str) -> Result<boltz_client::PublicKey, PaymentError> {
    boltz_client::PublicKey::from_str(json).map_err(|e| PaymentError::Generic {
        err: format!("Failed to deserialize PublicKey: {e:?}"),
    })
}

pub(crate) fn generate_keypair() -> boltz_client::Keypair {
    let secp = boltz_client::Secp256k1::new();
    let mut rng = lwk_wollet::secp256k1::rand::thread_rng();
    let secret_key = lwk_wollet::secp256k1::SecretKey::new(&mut rng);
    boltz_client::Keypair::from_secret_key(&secp, &secret_key)
}

pub(crate) fn decode_keypair(secret_key: &str) -> SdkResult<boltz_client::Keypair> {
    let secp = boltz_client::Secp256k1::new();
    let secret_key = lwk_wollet::secp256k1::SecretKey::from_str(secret_key)?;
    Ok(boltz_client::Keypair::from_secret_key(&secp, &secret_key))
}

pub(crate) fn is_locktime_expired(current_locktime: LockTime, expiry_locktime: LockTime) -> bool {
    match (current_locktime, expiry_locktime) {
        (Blocks(n), Blocks(lock_time)) => n >= lock_time,
        (Seconds(n), Seconds(lock_time)) => n >= lock_time,
        _ => false, // Not using the same units
    }
}

pub(crate) fn deserialize_tx_hex(tx_hex: &str) -> Result<Transaction> {
    Ok(deserialize(&Vec::<u8>::from_hex(tx_hex).map_err(
        |err| anyhow!("Could not deserialize transaction: {err:?}"),
    )?)?)
}

/// Parsing logic that decodes a string into a [Bolt12Invoice].
///
/// It matches the encoding logic on Boltz side.
pub(crate) fn parse_bolt12_invoice(invoice: &str) -> Result<Bolt12Invoice> {
    let (hrp, data) = bech32::decode_without_checksum(invoice)?;
    ensure!(hrp.as_str() == "lni", "Invalid HRP");

    let data = Vec::<u8>::from_base32(&data)?;

    lightning::offers::invoice::Bolt12Invoice::try_from(data)
        .map_err(|e| anyhow!("Failed to parse BOLT12: {e:?}"))
}
