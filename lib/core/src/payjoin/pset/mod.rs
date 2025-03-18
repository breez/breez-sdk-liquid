pub(crate) mod blind;
mod tests;

use anyhow::{anyhow, ensure, Result};
use bip39::rand;
use lwk_wollet::bitcoin;
use lwk_wollet::elements::confidential::{Asset, Value};
use lwk_wollet::elements::pset::{Input, Output, PartiallySignedTransaction};
use lwk_wollet::elements::script::Script;
use lwk_wollet::elements::{self, AssetId, OutPoint, TxOut, TxOutSecrets, Txid};
use rand::seq::SliceRandom;

pub struct PsetInput {
    pub txid: Txid,
    pub vout: u32,
    pub script_pub_key: Script,
    pub asset_commitment: Asset,
    pub value_commitment: Value,
    pub tx_out_sec: TxOutSecrets,
}

pub struct PsetOutput {
    pub address: elements::Address,
    pub asset_id: AssetId,
    pub amount: u64,
}

pub struct ConstructPsetRequest {
    pub policy_asset: AssetId,
    pub inputs: Vec<PsetInput>,
    pub outputs: Vec<PsetOutput>,
    pub network_fee: u64,
}

fn pset_input(input: PsetInput) -> Input {
    let PsetInput {
        txid,
        vout,
        script_pub_key,
        asset_commitment,
        value_commitment,
        tx_out_sec: _,
    } = input;

    let mut pset_input = Input::from_prevout(OutPoint { txid, vout });

    pset_input.witness_utxo = Some(TxOut {
        asset: asset_commitment,
        value: value_commitment,
        nonce: elements::confidential::Nonce::Null,
        script_pubkey: script_pub_key,
        witness: elements::TxOutWitness::default(),
    });

    pset_input
}

fn pset_output(output: PsetOutput) -> Result<Output> {
    let PsetOutput {
        address,
        asset_id,
        amount,
    } = output;

    let blinding_pubkey = address
        .blinding_pubkey
        .ok_or_else(|| anyhow!("only blinded addresses allowed"))?;
    ensure!(amount > 0);

    let txout = TxOut {
        asset: Asset::Explicit(asset_id),
        value: Value::Explicit(amount),
        nonce: elements::confidential::Nonce::Confidential(blinding_pubkey),
        script_pubkey: address.script_pubkey(),
        witness: elements::TxOutWitness::default(),
    };

    let mut output = Output::from_txout(txout);

    output.blinding_key = Some(bitcoin::PublicKey::new(blinding_pubkey));
    output.blinder_index = Some(0);

    Ok(output)
}

fn pset_network_fee(asset: AssetId, amount: u64) -> Output {
    let network_fee_output = TxOut::new_fee(amount, asset);
    Output::from_txout(network_fee_output)
}

pub fn construct_pset(req: ConstructPsetRequest) -> Result<PartiallySignedTransaction> {
    let ConstructPsetRequest {
        policy_asset,
        mut inputs,
        mut outputs,
        network_fee,
    } = req;

    let mut pset = PartiallySignedTransaction::new_v2();
    let mut input_secrets = Vec::new();
    let blinding_factors = Vec::new();

    let mut rng = rand::thread_rng();
    inputs.shuffle(&mut rng);
    outputs.shuffle(&mut rng);

    for input in inputs.into_iter() {
        input_secrets.push(input.tx_out_sec);

        pset.add_input(pset_input(input));
    }

    for output in outputs {
        pset.add_output(pset_output(output)?);
    }

    pset.add_output(pset_network_fee(policy_asset, network_fee));

    blind::blind_pset(&mut pset, &input_secrets, &blinding_factors)?;
    Ok(pset)
}
