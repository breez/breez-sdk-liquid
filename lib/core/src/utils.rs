use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{PaymentError, SdkResult};
use crate::prelude::{
    Config, LiquidNetwork, SendSwap, LOWBALL_FEE_RATE_SAT_PER_VBYTE,
    STANDARD_FEE_RATE_SAT_PER_VBYTE,
};
use crate::wallet::OnchainWallet;
use anyhow::{anyhow, Result};
use boltz_client::boltz::{
    BoltzApiClientV2, Cooperative, BOLTZ_MAINNET_URL_V2, BOLTZ_TESTNET_URL_V2,
};
use boltz_client::network::electrum::ElectrumConfig;
use boltz_client::Amount;
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

pub(crate) async fn derive_fee_rate_msat_per_vb(
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

    // Multiply sats/vb value by 1000 i.e. 1.0 sat/byte = 1000.0 sat/kvb = 1000.0 millisat/vb
    // We calculate using f64 and convert to f32 in the last step, so we keep the maximum precision possible
    let result_sat_per_vb =
        STANDARD_FEE_RATE_SAT_PER_VBYTE * absolute_fees_sat as f64 / standard_fees_sat;
    let result_msat_per_vb = result_sat_per_vb * 1000.0;
    let result_msat_per_vb_f32 = result_msat_per_vb as f32;
    debug!("derive_fee_rate_msat_per_vb: result_msat_per_vb_f32 {} from inputs: absolute_fees_sat {}, result_msat_per_vb: {}",
        result_msat_per_vb_f32, absolute_fees_sat, result_msat_per_vb);
    Ok(result_msat_per_vb_f32)
}

pub(crate) fn estimate_refund_fees(
    swap: &SendSwap,
    config: &Config,
    output_address: &str,
    is_cooperative: bool,
) -> Result<u64, PaymentError> {
    let swap_script = swap.get_swap_script()?;
    let electrum_config = ElectrumConfig::new(
        config.network.into(),
        &config.liquid_electrum_url,
        true,
        true,
        100,
    );
    let swap_tx = boltz_client::LBtcSwapTx::new_refund(
        swap_script,
        &output_address.to_string(),
        &electrum_config,
        config.liquid_electrum_url.clone(),
        swap.id.clone(),
    )?;
    let dummy_fees = Amount::from_sat(100);

    let boltz_api = &BoltzApiClientV2::new(match config.network {
        LiquidNetwork::Mainnet => BOLTZ_MAINNET_URL_V2,
        LiquidNetwork::Testnet => BOLTZ_TESTNET_URL_V2,
    });

    let (fee_rate, cooperative) = match (config.network, is_cooperative) {
        (LiquidNetwork::Mainnet, true) => (
            LOWBALL_FEE_RATE_SAT_PER_VBYTE,
            Some(Cooperative {
                boltz_api,
                swap_id: swap.id.clone(),
                pub_nonce: None,
                partial_sig: None,
            }),
        ),
        (LiquidNetwork::Testnet, true) => (
            STANDARD_FEE_RATE_SAT_PER_VBYTE,
            Some(Cooperative {
                boltz_api,
                swap_id: swap.id.clone(),
                pub_nonce: None,
                partial_sig: None,
            }),
        ),
        (_, false) => (STANDARD_FEE_RATE_SAT_PER_VBYTE, None),
    };
    let dummy_tx = swap_tx.sign_refund(&swap.get_refund_keypair()?, dummy_fees, cooperative)?;

    Ok((dummy_tx.vsize() as f64 * fee_rate).ceil() as u64)
}
