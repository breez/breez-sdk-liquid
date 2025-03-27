use sdk_common::{
    bitcoin::util::bip32::{ChildNumber, DerivationPath},
    prelude::{LnUrlResult, LnurlAuthSigner},
    utils::Arc,
};

use crate::model::Signer;

pub(crate) struct SdkLnurlAuthSigner {
    signer: Arc<Box<dyn Signer>>,
}

impl SdkLnurlAuthSigner {
    pub fn new(signer: Arc<Box<dyn Signer>>) -> Self {
        Self { signer }
    }
}

#[sdk_macros::async_trait]
impl LnurlAuthSigner for SdkLnurlAuthSigner {
    async fn derive_bip32_pub_key(&self, derivation_path: &[ChildNumber]) -> LnUrlResult<Vec<u8>> {
        let derivation: DerivationPath = derivation_path.to_vec().into();
        self.signer
            .derive_xpub(derivation.to_string())
            .map_err(|e| sdk_common::prelude::LnUrlError::Generic(e.to_string()))
            .map(|xpub| xpub.to_vec())
    }

    async fn sign_ecdsa(
        &self,
        msg: &[u8],
        derivation_path: &[ChildNumber],
    ) -> LnUrlResult<Vec<u8>> {
        let derivation: DerivationPath = derivation_path.to_vec().into();
        self.signer
            .sign_ecdsa(msg.to_vec(), derivation.to_string())
            .map_err(|e| sdk_common::prelude::LnUrlError::Generic(e.to_string()))
            .map(|s: Vec<u8>| s.to_vec())
    }

    async fn hmac_sha256(
        &self,
        key_derivation_path: &[ChildNumber],
        input: &[u8],
    ) -> LnUrlResult<Vec<u8>> {
        let derivation: DerivationPath = key_derivation_path.to_vec().into();
        self.signer
            .hmac_sha256(input.to_vec(), derivation.to_string())
            .map_err(|e| sdk_common::prelude::LnUrlError::Generic(e.to_string()))
    }
}
