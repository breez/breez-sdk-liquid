use breez_sdk_liquid::prelude::SignerError;
use wasm_bindgen::prelude::*;

pub struct WasmSigner {
    pub signer: Signer,
}

impl breez_sdk_liquid::prelude::Signer for WasmSigner {
    fn xpub(&self) -> Result<Vec<u8>, SignerError> {
        self.signer.xpub().map_err(|e| SignerError::Generic {
            err: e.to_string().into(),
        })
    }

    fn derive_xpub(&self, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        self.signer
            .derive_xpub(derivation_path)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn sign_ecdsa(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        self.signer
            .sign_ecdsa(msg, derivation_path)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn sign_ecdsa_recoverable(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        self.signer
            .sign_ecdsa_recoverable(msg)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn slip77_master_blinding_key(&self) -> Result<Vec<u8>, SignerError> {
        self.signer
            .slip77_master_blinding_key()
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn hmac_sha256(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError> {
        self.signer
            .hmac_sha256(msg, derivation_path)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn ecies_encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        self.signer
            .ecies_encrypt(msg)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }

    fn ecies_decrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        self.signer
            .ecies_decrypt(msg)
            .map_err(|e| SignerError::Generic {
                err: e.to_string().into(),
            })
    }
}

#[wasm_bindgen(typescript_custom_section)]
const SIGNER_INTERFACE: &'static str = r#"export interface Signer {
    xpub: () => number[];
    deriveXpub: (derivationPath: string) => number[];
    signEcdsa: (msg: number[], derivationPath: string) => number[];
    signEcdsaRecoverable: (msg: number[]) => number[];
    slip77MasterBlindingKey: () => number[];
    hmacSha256: (msg: number[], derivationPath: string) => number[];
    eciesEncrypt: (msg: number[]) => number[];
    eciesDecrypt: (msg: number[]) => number[];
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Signer")]
    pub type Signer;

    #[wasm_bindgen(structural, catch, method, js_name = xpub)]
    pub fn xpub(this: &Signer) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = deriveXpub)]
    fn derive_xpub(this: &Signer, derivation_path: String) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = signEcdsa)]
    fn sign_ecdsa(
        this: &Signer,
        msg: Vec<u8>,
        derivation_path: String,
    ) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = signEcdsaRecoverable)]
    fn sign_ecdsa_recoverable(this: &Signer, msg: Vec<u8>) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = slip77MasterBlindingKey)]
    fn slip77_master_blinding_key(this: &Signer) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = hmacSha256)]
    fn hmac_sha256(
        this: &Signer,
        msg: Vec<u8>,
        derivation_path: String,
    ) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = eciesEncrypt)]
    fn ecies_encrypt(this: &Signer, msg: Vec<u8>) -> Result<Vec<u8>, js_sys::Error>;

    #[wasm_bindgen(structural, catch, method, js_name = eciesDecrypt)]
    fn ecies_decrypt(this: &Signer, msg: Vec<u8>) -> Result<Vec<u8>, js_sys::Error>;
}
