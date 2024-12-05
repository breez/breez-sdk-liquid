use std::sync::Arc;

use crate::model::{Signer, SignerError};
use bip39::Mnemonic;
use boltz_client::PublicKey;
use lwk_common::Signer as LwkSigner;
use lwk_wollet::bitcoin::bip32::Xpriv;
use lwk_wollet::bitcoin::Network;
use lwk_wollet::elements_miniscript::{self, ToPublicKey as _};
use lwk_wollet::elements_miniscript::{
    bitcoin::{self, bip32::DerivationPath},
    elements::{
        bitcoin::bip32::{self, Fingerprint, Xpub},
        hashes::Hash,
        pset::PartiallySignedTransaction,
        secp256k1_zkp::{All, Secp256k1},
        sighash::SighashCache,
    },
    elementssig_to_rawsig,
    psbt::PsbtExt,
    slip77::MasterBlindingKey,
};
use lwk_wollet::hashes::{sha256, HashEngine, Hmac, HmacEngine};
use lwk_wollet::secp256k1::ecdsa::Signature;
use lwk_wollet::secp256k1::Message;

#[derive(thiserror::Error, Debug)]
pub enum SignError {
    #[error(transparent)]
    Pset(#[from] elements_miniscript::elements::pset::Error),

    #[error(transparent)]
    ElementsEncode(#[from] elements_miniscript::elements::encode::Error),

    #[error(transparent)]
    Sighash(#[from] elements_miniscript::psbt::SighashError),

    #[error(transparent)]
    PsetParse(#[from] elements_miniscript::elements::pset::ParseError),

    #[error(transparent)]
    Bip32(#[from] bip32::Error),

    #[error(transparent)]
    Generic(#[from] anyhow::Error),

    #[error(transparent)]
    UserSignerError(#[from] crate::model::SignerError),
}

/// Possible errors when creating a new software signer [`SwSigner`]
#[derive(thiserror::Error, Debug)]
pub enum NewError {
    #[error(transparent)]
    Bip39(#[from] bip39::Error),

    #[error(transparent)]
    Bip32(#[from] bip32::Error),
}

/// A software signer
pub struct SdkLwkSigner {
    sdk_signer: Arc<Box<dyn Signer>>,
}

impl SdkLwkSigner {
    /// Creates a new software signer from the given mnemonic.
    ///
    /// Takes also a flag if the network is mainnet so that generated extended keys are in the
    /// correct form xpub/tpub (there is no need to discriminate between regtest and testnet)
    pub fn new(sdk_signer: Arc<Box<dyn Signer>>) -> Result<Self, NewError> {
        Ok(Self { sdk_signer })
    }

    pub fn xpub(&self) -> Result<Xpub, SignError> {
        let xpub = self.sdk_signer.xpub()?;
        Ok(Xpub::decode(&xpub)?)
    }

    pub fn fingerprint(&self) -> Result<Fingerprint, SignError> {
        let f: Fingerprint = self.xpub()?.identifier()[0..4]
            .try_into()
            .map_err(|_| SignError::Generic(anyhow::anyhow!("Wrong fingerprint length")))?;
        Ok(f)
    }

    pub fn sign_ecdsa_recoverable(&self, msg: &Message) -> Result<Vec<u8>, SignError> {
        let sig_bytes = self
            .sdk_signer
            .sign_ecdsa_recoverable(msg.as_ref().to_vec())?;
        Ok(sig_bytes)
    }
}

impl LwkSigner for SdkLwkSigner {
    type Error = SignError;

    fn sign(&self, pset: &mut PartiallySignedTransaction) -> Result<u32, Self::Error> {
        let tx = pset.extract_tx()?;
        let mut sighash_cache = SighashCache::new(&tx);
        let mut signature_added = 0;

        // genesis hash is not used at all for sighash calculation
        let genesis_hash = elements_miniscript::elements::BlockHash::all_zeros();
        let mut messages = vec![];
        for i in 0..pset.inputs().len() {
            // computing all the messages to sign, it is not necessary if we are not going to sign
            // some input, but since the pset is borrowed, we can't do this action in a inputs_mut() for loop
            let msg = pset
                .sighash_msg(i, &mut sighash_cache, None, genesis_hash)?
                .to_secp_msg();
            messages.push(msg);
        }

        // Fixme: Take a parameter
        let hash_ty = elements_miniscript::elements::EcdsaSighashType::All;

        let signer_fingerprint = self.fingerprint()?;
        for (input, msg) in pset.inputs_mut().iter_mut().zip(messages) {
            for (want_public_key, (fingerprint, derivation_path)) in input.bip32_derivation.iter() {
                if &signer_fingerprint == fingerprint {
                    let xpub = self.derive_xpub(derivation_path)?;
                    let public_key: PublicKey = xpub.public_key.into();
                    if want_public_key == &public_key {
                        // fixme: for taproot use schnorr
                        let sig_bytes = self
                            .sdk_signer
                            .sign_ecdsa(msg.as_ref().to_vec(), derivation_path.to_string())?;
                        let sig = Signature::from_der(&sig_bytes).map_err(|_| {
                            SignError::Generic(anyhow::anyhow!("Invalid esda signature"))
                        })?;
                        let sig = elementssig_to_rawsig(&(sig, hash_ty));

                        let inserted = input.partial_sigs.insert(public_key, sig);
                        if inserted.is_none() {
                            signature_added += 1;
                        }
                    }
                }
            }
        }

        Ok(signature_added)
    }

    fn slip77_master_blinding_key(&self) -> Result<MasterBlindingKey, Self::Error> {
        let bytes: [u8; 32] = self
            .sdk_signer
            .slip77_master_blinding_key()?
            .try_into()
            .map_err(|_| {
                SignError::Generic(anyhow::anyhow!("Wrong slip77 master blinding key length"))
            })?;
        Ok(bytes.into())
    }

    fn derive_xpub(&self, path: &DerivationPath) -> Result<Xpub, Self::Error> {
        let pubkey_bytes = self.sdk_signer.derive_xpub(path.to_string())?;
        let xpub = Xpub::decode(pubkey_bytes.as_slice())?;
        Ok(xpub)
    }
}

pub struct SdkSigner {
    xprv: Xpriv,
    secp: Secp256k1<All>, // could be sign only, but it is likely the caller already has the All context.
    mnemonic: Mnemonic,
    network: Network,
}

impl SdkSigner {
    pub fn new(mnemonic: &str, is_mainnet: bool) -> Result<Self, NewError> {
        let secp = Secp256k1::new();
        let mnemonic: Mnemonic = mnemonic.parse()?;
        let seed = mnemonic.to_seed("");

        let network = if is_mainnet {
            bitcoin::Network::Bitcoin
        } else {
            bitcoin::Network::Testnet
        };

        let xprv = Xpriv::new_master(network, &seed)?;

        Ok(Self {
            xprv,
            secp,
            mnemonic,
            network,
        })
    }

    fn seed(&self) -> [u8; 64] {
        self.mnemonic.to_seed("")
    }
}

impl Signer for SdkSigner {
    fn xpub(&self) -> Result<Vec<u8>, SignerError> {
        Ok(Xpub::from_priv(&self.secp, &self.xprv).encode().to_vec())
    }

    fn derive_xpub(&self, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        let der: DerivationPath = derivation_path.parse()?;
        let derived = self.xprv.derive_priv(&self.secp, &der)?;
        Ok(Xpub::from_priv(&self.secp, &derived).encode().to_vec())
    }

    fn sign_ecdsa(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        let der: DerivationPath = derivation_path.parse()?;
        let ext_derived = self.xprv.derive_priv(&self.secp, &der)?;
        let sig = self.secp.sign_ecdsa_low_r(
            &Message::from_digest(
                msg.try_into()
                    .map_err(|_| anyhow::anyhow!("failed to sign"))?,
            ),
            &ext_derived.private_key,
        );
        Ok(sig.serialize_der().to_vec())
    }

    fn slip77_master_blinding_key(&self) -> Result<Vec<u8>, SignerError> {
        let seed = self.seed();
        let master_blinding_key = MasterBlindingKey::from_seed(&seed[..]);
        Ok(master_blinding_key.as_bytes().to_vec())
    }

    fn sign_ecdsa_recoverable(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        let seed = self.seed();
        let secp = Secp256k1::new();
        let keypair = Xpriv::new_master(self.network, &seed)
            .map_err(|e| anyhow::anyhow!("Could not get signer keypair: {e}"))?
            .to_keypair(&secp);
        let s = msg.as_slice();

        let msg: Message = Message::from_digest_slice(s)
            .map_err(|e| SignerError::Generic { err: e.to_string() })?;
        // Get message signature and encode to zbase32
        let recoverable_sig = secp.sign_ecdsa_recoverable(&msg, &keypair.secret_key());
        let (recovery_id, sig) = recoverable_sig.serialize_compact();
        let mut complete_signature = vec![31 + recovery_id.to_i32() as u8];
        complete_signature.extend_from_slice(&sig);
        Ok(complete_signature)
    }

    fn hmac_sha256(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        let der: DerivationPath = derivation_path.parse()?;
        let priv_key = self.xprv.derive_priv(&self.secp, &der)?;
        let mut engine = HmacEngine::<sha256::Hash>::new(priv_key.to_priv().to_bytes().as_slice());

        engine.input(msg.as_slice());
        Ok(Hmac::<sha256::Hash>::from_engine(engine)
            .as_byte_array()
            .to_vec())
    }

    fn ecies_encrypt(&self, msg: &[u8]) -> Result<Vec<u8>, SignerError> {
        let keypair = self.xprv.to_keypair(&self.secp);
        let rc_pub = keypair.public_key().to_public_key().to_bytes();
        Ok(
            ecies::encrypt(&rc_pub, msg).map_err(|err| SignerError::Generic {
                err: format!("Could not encrypt data: {err}"),
            })?,
        )
    }

    fn ecies_decrypt(&self, msg: &[u8]) -> Result<Vec<u8>, SignerError> {
        let rc_prv = self.xprv.to_priv().to_bytes();
        Ok(
            ecies::decrypt(&rc_prv, msg).map_err(|err| SignerError::Generic {
                err: format!("Could not decrypt data: {err}"),
            })?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip32::KeySource;
    use bitcoin::PublicKey;
    use elements::{
        pset::{Input, Output, PartiallySignedTransaction},
        AssetId, TxOut, Txid,
    };
    use lwk_common::{singlesig_desc, Singlesig};
    use lwk_signer::SwSigner;
    use lwk_wollet::{
        elements::{self, Script},
        ElementsNetwork, NoPersist, Wollet, WolletDescriptor,
    };
    use std::collections::BTreeMap;

    fn get_descriptor<S: LwkSigner>(
        signer: &S,
        is_mainnet: bool,
    ) -> Result<WolletDescriptor, anyhow::Error> {
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow::anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
    }

    fn create_signers(mnemonic: &str) -> (SwSigner, SdkLwkSigner) {
        let sw_signer = SwSigner::new(mnemonic, false).unwrap();
        let sdk_signer: Box<dyn Signer> = Box::new(SdkSigner::new(mnemonic, false).unwrap());
        let sdk_signer = SdkLwkSigner::new(Arc::new(sdk_signer)).unwrap();
        (sw_signer, sdk_signer)
    }

    fn create_pset<S: LwkSigner>(signer: &S) -> PartiallySignedTransaction {
        // Create a PartiallySignedTransaction
        let mut pset = PartiallySignedTransaction::new_v2();

        // Add a dummy input
        let prev_txid = Txid::from_slice(&[0; 32]).unwrap();
        let prev_vout = 0;

        let derivation_path: DerivationPath = "m/84'/0'/0'/0/0".parse().unwrap();
        let xpub = signer.derive_xpub(&derivation_path).unwrap();
        let mut bip32_derivation_map: BTreeMap<PublicKey, KeySource> = BTreeMap::new();
        bip32_derivation_map.insert(
            xpub.public_key.into(),
            (signer.fingerprint().unwrap(), derivation_path),
        );
        let input = Input {
            non_witness_utxo: None,
            witness_utxo: Some(TxOut::new_fee(
                100_000_000,
                AssetId::from_slice(&[1; 32]).unwrap(),
            )),
            previous_txid: prev_txid,
            previous_output_index: prev_vout,
            bip32_derivation: bip32_derivation_map,
            ..Default::default()
        };

        pset.add_input(input);

        // Add a dummy output using new_explicit
        let output_script = Script::new();
        let output_amount = 99_000_000;
        let output_asset = AssetId::from_slice(&[1; 32]).unwrap();
        let output = Output::new_explicit(
            output_script,
            output_amount,
            output_asset,
            None, // No blinding key for this example
        );
        pset.add_output(output);
        pset
    }

    #[test]
    fn test_sign() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let (sw_signer, sdk_signer) = create_signers(mnemonic);

        // Clone the PSET for each signer
        let mut pset_sw = create_pset(&sw_signer);
        let mut pset_sdk = create_pset(&sdk_signer);

        // Sign with SwSigner
        let sw_sig_count = sw_signer.sign(&mut pset_sw).unwrap();
        assert_eq!(sw_sig_count, 1);

        // Sign with SdkLwkSigner
        let sdk_sig_count = sdk_signer.sign(&mut pset_sdk).unwrap();
        assert_eq!(sdk_sig_count, 1);

        // Compare the sign results
        assert_eq!(pset_sw, pset_sdk);

        // Extract and compare the final transactions
        let tx_sw = pset_sw.extract_tx().unwrap();
        let tx_sdk = pset_sdk.extract_tx().unwrap();
        assert_eq!(tx_sw, tx_sdk);
    }

    #[test]
    fn test_slip77_master_blinding_key() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let (sw_signer, sdk_signer) = create_signers(mnemonic);

        let sw_key = sw_signer.slip77_master_blinding_key().unwrap();
        let sdk_key = sdk_signer.slip77_master_blinding_key().unwrap();

        assert_eq!(
            sw_key, sdk_key,
            "SLIP77 master blinding keys should be identical"
        );
    }

    #[test]
    fn test_derive_xpub() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let (sw_signer, sdk_signer) = create_signers(mnemonic);

        let path = "m/84'/0'/0'/0/0".parse().unwrap();
        let sw_xpub = sw_signer.derive_xpub(&path).unwrap();
        let sdk_xpub = sdk_signer.derive_xpub(&path).unwrap();

        assert_eq!(sw_xpub, sdk_xpub, "Derived xpubs should be identical");
    }

    #[test]
    fn test_identifier() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let (sw_signer, sdk_signer) = create_signers(mnemonic);

        let sw_identifier = sw_signer.xpub().identifier();
        let sdk_identifier = sdk_signer.xpub().unwrap().identifier();

        assert_eq!(
            sw_identifier, sdk_identifier,
            "Identifiers should be identical"
        );
    }

    #[test]
    fn test_fingerprint() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let (sw_signer, sdk_signer) = create_signers(mnemonic);

        let sw_fingerprint = sw_signer.fingerprint();
        let sdk_fingerprint = sdk_signer.fingerprint().unwrap();
        let manual_finger_print = sdk_signer.xpub().unwrap().identifier()[0..4]
            .try_into()
            .unwrap();
        assert_eq!(
            sw_fingerprint, sdk_fingerprint,
            "Fingerprints should be identical"
        );

        assert_eq!(
            sw_fingerprint, manual_finger_print,
            "Fingerprints should be identical"
        );
    }

    #[test]
    fn test_sdk_signer_vs_sw_signer() {
        // Use a test mnemonic (don't use this in production!)
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = ElementsNetwork::LiquidTestnet;

        // 1. Create a wallet using SwSigner
        let sw_signer = SwSigner::new(mnemonic, false).unwrap();
        let sw_wallet = Wollet::new(
            network,
            NoPersist::new(),
            get_descriptor(&sw_signer, false).unwrap(),
        )
        .unwrap();

        // 2. Create a wallet using SdkLwkSigner
        let sdk_signer: Box<dyn Signer> = Box::new(SdkSigner::new(mnemonic, false).unwrap());
        let sdk_signer = SdkLwkSigner::new(Arc::new(sdk_signer)).unwrap();
        let sdk_wallet = Wollet::new(
            network,
            NoPersist::new(),
            get_descriptor(&sdk_signer, false).unwrap(),
        )
        .unwrap();

        // Generate new addresses and compare
        let sw_address = sw_wallet.address(None).unwrap();
        let sdk_address = sdk_wallet.address(None).unwrap();

        assert_eq!(
            sw_address.address().to_string(),
            sdk_address.address().to_string(),
            "Addresses should be identical"
        );
    }
}
