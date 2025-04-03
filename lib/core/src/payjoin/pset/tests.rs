#![cfg(test)]
use anyhow::Result;
use bip39::rand::{self, RngCore};
use lwk_wollet::bitcoin;
use lwk_wollet::elements::address::AddressParams;
use lwk_wollet::elements::confidential::{Asset, AssetBlindingFactor, Value, ValueBlindingFactor};
use lwk_wollet::elements::secp256k1_zkp::SecretKey;
use lwk_wollet::elements::{secp256k1_zkp, Address, AssetId, Script, TxOutSecrets, Txid};
use std::str::FromStr;

use crate::payjoin::pset::{construct_pset, ConstructPsetRequest, PsetInput, PsetOutput};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_secret_key() -> SecretKey {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 32];
    rng.fill_bytes(&mut buf);
    SecretKey::from_slice(&buf).expect("Expected valid secret key")
}

fn create_test_input(asset_id: AssetId, sk: &SecretKey) -> PsetInput {
    // Create a dummy txid
    let txid =
        Txid::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    // Create a dummy script pubkey
    let script_pub_key =
        Script::from_str("76a914000000000000000000000000000000000000000088ac").unwrap();

    // Create dummy asset and value commitments
    let secp = secp256k1_zkp::Secp256k1::new();

    let asset_bf = AssetBlindingFactor::from_slice(&sk.secret_bytes()).unwrap();
    let asset_gen =
        secp256k1_zkp::Generator::new_blinded(&secp, asset_id.into_tag(), asset_bf.into_inner());
    let asset_commitment = Asset::Confidential(asset_gen);

    // Create a Pedersen commitment for the value
    let value_bf = ValueBlindingFactor::from_slice(&sk.secret_bytes()).unwrap();
    let value_commit =
        secp256k1_zkp::PedersenCommitment::new(&secp, 10000, value_bf.into_inner(), asset_gen);
    let value_commitment = Value::Confidential(value_commit);

    // Create dummy txout secrets
    let tx_out_sec = TxOutSecrets {
        asset: asset_id,
        value: 10000,
        asset_bf,
        value_bf,
    };

    PsetInput {
        txid,
        vout: 0,
        script_pub_key,
        asset_commitment,
        value_commitment,
        tx_out_sec,
    }
}

fn create_test_output(asset_id: AssetId, sk: &SecretKey) -> PsetOutput {
    // Create a dummy blinded address
    let secp = secp256k1_zkp::Secp256k1::new();
    let blinding_key =
        bitcoin::PublicKey::new(secp256k1_zkp::PublicKey::from_secret_key(&secp, sk));
    let address_pk = bitcoin::PublicKey::new(secp256k1_zkp::PublicKey::from_secret_key(&secp, sk));

    let address = Address::p2pkh(
        &address_pk,
        Some(blinding_key.inner),
        &AddressParams::LIQUID,
    );

    PsetOutput {
        address,
        asset_id,
        amount: 5000,
    }
}

#[sdk_macros::test_all]
fn test_construct_pset_basic() -> Result<()> {
    // Create test data
    let asset_id = AssetId::from_slice(&[2; 32]).unwrap();
    let secret_key = create_test_secret_key();

    let policy_asset = AssetId::from_slice(&[8; 32]).unwrap();
    let inputs = vec![
        create_test_input(asset_id, &secret_key),
        create_test_input(asset_id, &secret_key),
    ];
    let outputs = vec![create_test_output(asset_id, &secret_key)];
    let network_fee = 1000;

    let request = ConstructPsetRequest {
        policy_asset,
        inputs,
        outputs,
        network_fee,
    };

    // Call the function
    let pset = construct_pset(request)?;

    // Validate the result
    assert_eq!(pset.inputs().len(), 2);
    assert_eq!(pset.outputs().len(), 2); // 1 regular output + 1 fee output

    Ok(())
}

#[sdk_macros::test_all]
fn test_construct_pset_multiple_outputs() -> Result<()> {
    // Create test data
    let asset_id = AssetId::from_slice(&[3; 32]).unwrap();
    let secret_key = create_test_secret_key();

    let policy_asset = AssetId::from_slice(&[8; 32]).unwrap();
    let inputs = vec![create_test_input(asset_id, &secret_key)];
    let outputs = vec![
        create_test_output(asset_id, &secret_key),
        create_test_output(asset_id, &secret_key),
        create_test_output(asset_id, &secret_key),
    ];
    let network_fee = 1000;

    let request = ConstructPsetRequest {
        policy_asset,
        inputs,
        outputs,
        network_fee,
    };

    // Call the function
    let pset = construct_pset(request)?;

    // Validate the result
    assert_eq!(pset.inputs().len(), 1);
    assert_eq!(pset.outputs().len(), 4); // 3 regular outputs + 1 fee output

    Ok(())
}

#[sdk_macros::test_all]
fn test_construct_pset_empty_inputs() {
    // Create test data
    let asset_id = AssetId::from_slice(&[4; 32]).unwrap();
    let secret_key = create_test_secret_key();

    let policy_asset = AssetId::from_slice(&[8; 32]).unwrap();
    let inputs = vec![];
    let outputs = vec![create_test_output(asset_id, &secret_key)];
    let network_fee = 1000;

    let request = ConstructPsetRequest {
        policy_asset,
        inputs,
        outputs,
        network_fee,
    };

    // Blinding should fail with empty inputs
    let result = construct_pset(request);
    assert!(result.is_err());
}

#[sdk_macros::test_all]
fn test_construct_pset_empty_outputs() {
    // Create test data
    let asset_id = AssetId::from_slice(&[5; 32]).unwrap();
    let secret_key = create_test_secret_key();

    let policy_asset = AssetId::from_slice(&[8; 32]).unwrap();
    let inputs = vec![create_test_input(asset_id, &secret_key)];
    let outputs = vec![];
    let network_fee = 1000;

    let request = ConstructPsetRequest {
        policy_asset,
        inputs,
        outputs,
        network_fee,
    };

    // Call the function
    let result = construct_pset(request);
    assert!(result.is_err());
}
