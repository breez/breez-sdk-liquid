use anyhow::Result;
use openssl::sha::sha256;
use std::sync::Arc;

use crate::{
    prelude::{Signer, SignerError},
    utils,
};

use super::{sync::ListChangesRequest, MESSAGE_PREFIX};

fn sign_message(msg: &[u8], signer: Arc<Box<dyn Signer>>) -> Result<String, SignerError> {
    let msg = [MESSAGE_PREFIX, msg].concat();
    let digest = sha256(&sha256(&msg));
    signer
        .sign_ecdsa_recoverable(digest.into())
        .map(|bytes| zbase32::encode_full_bytes(&bytes))
}

impl ListChangesRequest {
    pub(crate) fn new(since_revision: u64, signer: Arc<Box<dyn Signer>>) -> Result<Self> {
        let request_time = utils::now();
        let msg = format!("{}-{}", since_revision, request_time);
        let signature = sign_message(msg.as_bytes(), signer)?;
        Ok(Self {
            since_revision,
            request_time,
            signature,
        })
    }
}
