use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{PaymentError, SdkResult};
use crate::prelude::STANDARD_FEE_RATE_SAT_PER_VBYTE;
use crate::wallet::OnchainWallet;
use anyhow::{anyhow, Result};
use log::debug;
use lwk_wollet::elements::encode::deserialize;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::elements::{
    LockTime::{self, *},
    Transaction,
};

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

pub(crate) async fn derive_fee_rate_sats_per_kvb(
    wallet: Arc<dyn OnchainWallet>,
    amount_sat: u64,
    recipient_address: &str,
    absolute_fees_sat: u64,
) -> Result<f32> {
    let standard_fees_sat = wallet
        .build_tx(None, recipient_address, amount_sat)
        .await?
        .all_fees()
        .values()
        .sum::<u64>() as f64;

    // Multiply sats/vb value by 1000 i.e. 1.0 sat/byte = 1000.0 sat/kvb
    // We calculate using f64 and convert to f32 in the last step, so we keep the maximum precision possible
    let result_sat_per_vb =
        STANDARD_FEE_RATE_SAT_PER_VBYTE * absolute_fees_sat as f64 / standard_fees_sat;
    let result_sat_per_kvb = result_sat_per_vb * 1000.0;
    let result_sat_per_kvb_f32 = result_sat_per_kvb as f32;
    debug!("derive_fee_rate_sats_per_kvb: result_sat_per_kvb_f32 {} from inputs: absolute_fees_sat {}, result_sat_per_kvb: {}",
        result_sat_per_kvb_f32, absolute_fees_sat, result_sat_per_kvb);
    Ok(result_sat_per_kvb_f32)
}
