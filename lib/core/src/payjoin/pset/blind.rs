use anyhow::{anyhow, Result};
use bip39::rand;
use lwk_wollet::bitcoin::secp256k1::SecretKey;
use lwk_wollet::elements::pset::Input;
use lwk_wollet::elements::secp256k1_zkp::Generator;
use lwk_wollet::elements::{self, bitcoin, confidential, secp256k1_zkp};
use lwk_wollet::elements::{
    confidential::{AssetBlindingFactor, ValueBlindingFactor},
    pset::{
        raw::{ProprietaryKey, ProprietaryType},
        PartiallySignedTransaction,
    },
    TxOutSecrets,
};

const PSET_IN_EXPLICIT_VALUE: ProprietaryType = 0x11; // 8 bytes
const PSET_IN_VALUE_PROOF: ProprietaryType = 0x12; // 73 bytes
const PSET_IN_EXPLICIT_ASSET: ProprietaryType = 0x13; // 2 bytes
const PSET_IN_ASSET_PROOF: ProprietaryType = 0x14; // 67 bytes

pub fn remove_explicit_values(pset: &mut PartiallySignedTransaction) {
    for input in pset.inputs_mut() {
        for subtype in [
            PSET_IN_EXPLICIT_VALUE,
            PSET_IN_EXPLICIT_ASSET,
            PSET_IN_VALUE_PROOF,
            PSET_IN_ASSET_PROOF,
        ] {
            input
                .proprietary
                .remove(&ProprietaryKey::from_pset_pair(subtype, Vec::new()));
        }
    }
}

fn add_input_explicit_proofs(input: &mut Input, secret: &TxOutSecrets) -> Result<()> {
    if secret.asset_bf == AssetBlindingFactor::zero()
        && secret.value_bf == ValueBlindingFactor::zero()
    {
        return Ok(());
    }
    let secp = secp256k1_zkp::global::SECP256K1;
    let mut rng = rand::thread_rng();
    let asset_gen_unblinded = Generator::new_unblinded(secp, secret.asset.into_tag());
    let asset_gen_blinded = input
        .witness_utxo
        .as_ref()
        .ok_or(anyhow!("No witness utxo"))?
        .asset
        .into_asset_gen(secp)
        .ok_or(anyhow!("No asset gen"))?;

    let blind_asset_proof = secp256k1_zkp::SurjectionProof::new(
        secp,
        &mut rng,
        secret.asset.into_tag(),
        secret.asset_bf.into_inner(),
        &[(
            asset_gen_unblinded,
            secret.asset.into_tag(),
            secp256k1_zkp::ZERO_TWEAK,
        )],
    )?;

    let blind_value_proof = secp256k1_zkp::RangeProof::new(
        secp,
        secret.value,
        input
            .witness_utxo
            .as_ref()
            .ok_or(anyhow!("No witness utxo"))?
            .value
            .commitment()
            .ok_or(anyhow!("Invalid commitment"))?,
        secret.value,
        secret.value_bf.into_inner(),
        &[],
        &[],
        secp256k1_zkp::SecretKey::new(&mut rng),
        -1,
        0,
        asset_gen_blinded,
    )?;

    input.proprietary.insert(
        ProprietaryKey::from_pset_pair(PSET_IN_EXPLICIT_VALUE, Vec::new()),
        elements::encode::serialize(&secret.value),
    );

    input.proprietary.insert(
        ProprietaryKey::from_pset_pair(PSET_IN_EXPLICIT_ASSET, Vec::new()),
        elements::encode::serialize(&secret.asset),
    );

    let mut blind_value_proof = elements::encode::serialize(&blind_value_proof);
    blind_value_proof.remove(0);
    let mut blind_asset_proof = elements::encode::serialize(&blind_asset_proof);
    blind_asset_proof.remove(0);

    input.proprietary.insert(
        ProprietaryKey::from_pset_pair(PSET_IN_VALUE_PROOF, Vec::new()),
        blind_value_proof,
    );

    input.proprietary.insert(
        ProprietaryKey::from_pset_pair(PSET_IN_ASSET_PROOF, Vec::new()),
        blind_asset_proof,
    );

    Ok(())
}

pub fn blind_pset(
    pset: &mut PartiallySignedTransaction,
    inp_txout_sec: &[TxOutSecrets],
    blinding_factors: &[(AssetBlindingFactor, ValueBlindingFactor, SecretKey)],
) -> Result<()> {
    let secp = secp256k1_zkp::global::SECP256K1;
    let rng = &mut rand::thread_rng();

    for (input, secret) in pset.inputs_mut().iter_mut().zip(inp_txout_sec.iter()) {
        add_input_explicit_proofs(input, secret)?;
    }

    let mut last_blinded_index = None;
    let mut exp_out_secrets = Vec::new();

    for (index, out) in pset.outputs().iter().enumerate() {
        if out.blinding_key.is_none() {
            let value = out
                .amount
                .ok_or(anyhow!("Output {index} value must be set"))?;
            exp_out_secrets.push((
                value,
                AssetBlindingFactor::zero(),
                ValueBlindingFactor::zero(),
            ));
        } else {
            last_blinded_index = Some(index);
        }
    }

    let last_blinded_index = last_blinded_index.ok_or(anyhow!("No blinding output found"))?;

    let inputs = inp_txout_sec
        .iter()
        .map(|secret| {
            let tag = secret.asset.into_tag();
            let tweak = secret.asset_bf.into_inner();
            let gen = Generator::new_blinded(secp, tag, tweak);
            (gen, tag, tweak)
        })
        .collect::<Vec<_>>();

    for (index, output) in pset.outputs_mut().iter_mut().enumerate() {
        let asset_id = output
            .asset
            .ok_or(anyhow!("Output {index} asset must be set"))?;
        let value = output
            .amount
            .ok_or(anyhow!("Output {index} value must be set"))?;
        if let Some(receiver_blinding_pk) = output.blinding_key {
            let is_last = index == last_blinded_index;
            let blinding_factor = blinding_factors.get(index);

            let out_abf = if let Some(blinding_factor) = blinding_factor {
                blinding_factor.0
            } else {
                AssetBlindingFactor::new(rng)
            };

            let out_asset_commitment =
                Generator::new_blinded(secp, asset_id.into_tag(), out_abf.into_inner());

            let out_vbf = if is_last {
                let inp_secrets = inp_txout_sec
                    .iter()
                    .map(|o| (o.value, o.asset_bf, o.value_bf))
                    .collect::<Vec<_>>();

                ValueBlindingFactor::last(secp, value, out_abf, &inp_secrets, &exp_out_secrets)
            } else if let Some(blinding_factor) = blinding_factor {
                blinding_factor.1
            } else {
                ValueBlindingFactor::new(rng)
            };

            let value_commitment = secp256k1_zkp::PedersenCommitment::new(
                secp,
                value,
                out_vbf.into_inner(),
                out_asset_commitment,
            );

            let ephemeral_sk = if let Some(blinding_factor) = blinding_factor {
                blinding_factor.2
            } else {
                SecretKey::new(rng)
            };

            let (nonce, shared_secret) = confidential::Nonce::with_ephemeral_sk(
                secp,
                ephemeral_sk,
                &receiver_blinding_pk.inner,
            );

            let mut message = [0u8; 64];
            message[..32].copy_from_slice(asset_id.into_tag().as_ref());
            message[32..].copy_from_slice(out_abf.into_inner().as_ref());

            let rangeproof = secp256k1_zkp::RangeProof::new(
                secp,
                1,
                value_commitment,
                value,
                out_vbf.into_inner(),
                &message,
                output.script_pubkey.as_bytes(),
                shared_secret,
                0,
                52,
                out_asset_commitment,
            )?;

            let surjection_proof = secp256k1_zkp::SurjectionProof::new(
                secp,
                rng,
                asset_id.into_tag(),
                out_abf.into_inner(),
                &inputs,
            )?;

            output.value_rangeproof = Some(Box::new(rangeproof));
            output.asset_surjection_proof = Some(Box::new(surjection_proof));
            output.amount_comm = Some(value_commitment);
            output.asset_comm = Some(out_asset_commitment);
            output.ecdh_pubkey = nonce.commitment().map(|pk| bitcoin::PublicKey {
                inner: pk,
                compressed: true,
            });

            let gen = Generator::new_unblinded(secp, asset_id.into_tag());
            output.blind_asset_proof = Some(Box::new(secp256k1_zkp::SurjectionProof::new(
                secp,
                rng,
                asset_id.into_tag(),
                out_abf.into_inner(),
                &[(gen, asset_id.into_tag(), secp256k1_zkp::ZERO_TWEAK)],
            )?));

            output.blind_value_proof = Some(Box::new(secp256k1_zkp::RangeProof::new(
                secp,
                value,
                value_commitment,
                value,
                out_vbf.into_inner(),
                &[],
                &[],
                secp256k1_zkp::SecretKey::new(rng),
                -1,
                0,
                out_asset_commitment,
            )?));

            exp_out_secrets.push((value, out_abf, out_vbf));
        }
    }

    Ok(())
}
