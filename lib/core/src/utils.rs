use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::ensure_sdk;
use crate::error::{PaymentError, SdkResult};
use crate::prelude::LiquidNetwork;
use anyhow::{anyhow, ensure, Result};
use boltz_client::boltz::SubmarinePair;
use boltz_client::util::secrets::Preimage;
use boltz_client::ToHex;
use lazy_static::lazy_static;
use lwk_wollet::elements::encode::deserialize;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::elements::AssetId;
use lwk_wollet::elements::{
    LockTime::{self, *},
    Transaction,
};
use sdk_common::bitcoin::bech32;
use sdk_common::bitcoin::bech32::FromBase32;
use sdk_common::lightning_125::offers::invoice::Bolt12Invoice;
use sdk_common::lightning_invoice::Bolt11Invoice;

lazy_static! {
    static ref LBTC_TESTNET_ASSET_ID: AssetId =
        AssetId::from_str("144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49")
            .unwrap();
    static ref LBTC_REGTEST_ASSET_ID: AssetId =
        AssetId::from_str("5ac9f65c0efcc4775e0baec4ec03abdde22473cd3cf33c0419ca290e0751b225")
            .unwrap();
}

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

    sdk_common::lightning_125::offers::invoice::Bolt12Invoice::try_from(data)
        .map_err(|e| anyhow!("Failed to parse BOLT12: {e:?}"))
}

/// Parse and extract the destination pubkey from the invoice.
/// The payee pubkey for Bolt11 and signing pubkey for Bolt12.
pub(crate) fn get_invoice_destination_pubkey(invoice: &str, is_bolt12: bool) -> Result<String> {
    if is_bolt12 {
        parse_bolt12_invoice(invoice).map(|i| i.signing_pubkey().to_hex())
    } else {
        invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map(|i| sdk_common::prelude::invoice_pubkey(&i))
            .map_err(Into::into)
    }
}

/// Verifies a BOLT11/12 invoice against a preimage
pub(crate) fn verify_payment_hash(
    preimage: &str,
    invoice: &str,
) -> std::result::Result<(), PaymentError> {
    let preimage = Preimage::from_str(preimage)?;
    let preimage_hash = preimage.sha256.to_string();

    let invoice_payment_hash = match Bolt11Invoice::from_str(invoice) {
        Ok(invoice) => Ok(invoice.payment_hash().to_string()),
        Err(_) => match parse_bolt12_invoice(invoice) {
            Ok(invoice) => Ok(invoice.payment_hash().to_string()),
            Err(e) => Err(PaymentError::InvalidInvoice {
                err: format!("Could not parse invoice: {e:?}"),
            }),
        },
    }?;

    ensure_sdk!(
        invoice_payment_hash == preimage_hash,
        PaymentError::InvalidPreimage
    );

    Ok(())
}

pub(crate) fn lbtc_asset_id(network: LiquidNetwork) -> AssetId {
    match network {
        LiquidNetwork::Mainnet => AssetId::LIQUID_BTC,
        LiquidNetwork::Testnet => *LBTC_TESTNET_ASSET_ID,
        LiquidNetwork::Regtest => *LBTC_REGTEST_ASSET_ID,
    }
}

/// Increments the inversely calculated invoice amount up to the maximum drainable amount,
/// as calculating the inverse invoice amount in some cases has rounding down errors
pub(crate) fn increment_invoice_amount_up_to_drain_amount(
    invoice_amount_sat: u64,
    lbtc_pair: &SubmarinePair,
    drain_amount_sat: u64,
) -> u64 {
    let incremented_amount_sat = invoice_amount_sat + 1;
    let fees_sat = lbtc_pair.fees.total(incremented_amount_sat);
    if incremented_amount_sat + fees_sat <= drain_amount_sat {
        increment_invoice_amount_up_to_drain_amount(
            incremented_amount_sat,
            lbtc_pair,
            drain_amount_sat,
        )
    } else {
        invoice_amount_sat
    }
}

pub(crate) fn log_print_header(init_time_ms: Duration) {
    log::info!(
        "
            ↘↘↘
         ↘↘↘↘↘↘↘↘       
           ↘↘↘↘↘↘↘↘    
     ↘↘↘↘↘       ↘↘↘↘   
                 ↘↘↘      Breez SDK Nodeless - version {}
 ↘↘↘↘↘↘↘↘↘  ↘↘↘↘↘↘↘↘↘     Initialization time: {init_time_ms:?}
                  ↘↘↘↘    Github: https://github.com/breez/breez-sdk-liquid
   ↘↘↘↘↘↘↘↘↘↘↘↘    ↘↘↘↘   Docs: https://sdk-doc-liquid.breez.technology/
                  ↘↘↘↘↘ 
      ↘↘↘↘↘↘↘↘↘↘↘↘↘↘↘   
        ↘↘↘↘↘↘↘↘↘↘
            ↘↘↘
    ",
        env!("CARGO_PKG_VERSION"),
    );
}

// This WASM sleep implementation is copied from https://github.com/Blockstream/lwk/blob/8d20554fd7b2774518f7467ed380ee95c0364091/lwk_wollet/src/clients/asyncr/esplora.rs#L668
// There are issues with using the `setTimeout()` API. See https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#timeouts_in_inactive_tabs
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub async fn async_sleep(millis: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    };
    let p = js_sys::Promise::new(&mut cb);
    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub async fn async_sleep(millis: i32) {
    tokio::time::sleep(Duration::from_millis(millis as u64)).await;
}

#[cfg(test)]
mod tests {
    use crate::error::PaymentError;
    use crate::utils::verify_payment_hash;

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_verify_payment_hash() -> anyhow::Result<()> {
        let bolt11_invoice = "lnbc10u1pnczjaupp55392fur38rc2y9vzmhdy0tclvfels0lvlmzgvmhpg6q2mndxzmrsdqqcqzzsxqyz5vqsp5ya6pvchlsvl3mzqh3zw4hg3tz5pww77q6rcwfr52qchyrp7s6krs9p4gqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqysgqgnp0sskk0ljjew8vkc3udhzgquzs79evf5wezfaex9q4gjk5qcn8m3luauyte93lgassd8skh5m90glhtt52ry2wtftzrjn4h076z7sqdjry3d";
        let bolt11_preimage = "c17a0a28d0523596ec909c2d439c0c2315b5bd996bf4ff48be50b2df08fb8ac1";
        let bolt12_invoice = "lni1qqg274t4yefgrj0pn3cwjz4vaacayyxvqwryaup9lh50kkranzgcdnn2fgvx390wgj5jd07rwr3vxeje0glc7quxqu8s2dggmw92army03kdxt825vqkawcz33kvrennr6g2fu6f7vpqx40040703nghrcj3pp9gax3y2my9v4kd5gtzv5k68k7vnxa3y7dlqqee9gty2hdrprsd8t04jz5nea79phgs9vyp9ruexzczwfhzg57c3yl345x3jy3kqpgr3e3u54fu0vewzjv2jq4lvj8gghjf5h8kgpw23c8tugua4qh432mylsdj3ac260fwvptzgcqpq486c0qlz6aqrj7q7k804w5mv92jqv85yszcyypft03wvpgapj7t0h58t6da6tdwx69admya2p0dl435la7wq4ljk79ql5qe5quxfmcztl0gldv8mxy3sm8x5jscdz27u39fy6luxu8zcdn9j73l3upa3vjg727ft7cwfkg4yqxyctm98hq6utue2k5k5at05azu2wgw57szq2qztaq2rnqjt6ugna4em3uj2el3cr7gj2glzuwkm346qpx93y9ruqz9fkumys35w9jdqxs45qzec44fhpy7lldwzt80y3q33sk09nkgf7h9r6etd45zp80snmz5x4uquqk7a0cusp0sluhku8md0eaxejqvkdd6mcp0gxqr6hsfwsxu4vx6lx08axqpj3fe87jeqfvdmetqxcaadn993vv3fe3qpny568lpz00dj3w6rag6gv3jyj9nnqmh6455l4h7ewe4zstwprmumemut8fexnnmgqmfzj0xwgr3mmwygw59jjqqv0h9vgc8vhkcx4g3s3av4kd48w4p4qs29zggh4vz924t23m7va0am4d7d4uur96uypayuchcgs2wxm0ktsaadewffys0jdlz245saetsd4f2m7ljp3tdxmt45qw64slkmlwaeak0h7508hftjdh6vyzr7skx2eucwwmgce0pydvgx5egmv4fnu0e7383ygyhwa0vwd4gy6zsez6kktvdezn79ejh2n8zmdtk998jvzuq7syv4gsuqqqq86qqqqqxgqfqqqqqqqqqqqp7sqqqqqqqq85ysqqqpfqyv7pxql4xqvq4rq9gyztzsk4ktppyn45peyvhfpl6lv8ewjr666gkzttspjkp8zn0g2n9f2srpapyptsrqgqqpvppq2lkfr5ytey6tnmyqh9gur47yww6st6c4dj0cxeg7u9d85hxq43yduzqrxguu82stp5egwzefmhvm9k63r0nxemf0pg54j3hdfzgt068te3dv5s089p54gcplnk778kcnfhkn6l8tggjqmgyc88vrgr6gc3gx7q";
        let bolt12_preimage = "443c900d61ed8e90a7bfbb7958f1485a7f57e74adacd3e216deba03f8326a392";

        // Test valid inputs
        verify_payment_hash(bolt11_preimage, bolt11_invoice)?;
        verify_payment_hash(bolt12_preimage, bolt12_invoice)?;

        // Test invalid preimages
        assert!(matches!(
            verify_payment_hash(bolt12_preimage, bolt11_invoice),
            Err(PaymentError::InvalidPreimage { .. })
        ));
        assert!(matches!(
            verify_payment_hash(bolt11_preimage, bolt12_invoice),
            Err(PaymentError::InvalidPreimage { .. })
        ));

        // Test invalid invoice
        assert!(matches!(
            verify_payment_hash(bolt11_preimage, "not an invoice"),
            Err(PaymentError::InvalidInvoice { .. })
        ));

        Ok(())
    }
}
