use std::sync::Arc;

use crate::{
    prelude::{Signer, SignerError},
    utils,
};
use anyhow::Result;
use log::trace;
use lwk_wollet::hashes::hex::DisplayHex as _;
use sdk_common::bitcoin::hashes::{sha256, Hash};

use super::{
    ListChangesRequest, ListenChangesRequest, Record, SetRecordRequest, CURRENT_SCHEMA_VERSION,
    MESSAGE_PREFIX,
};

fn sign_message(msg: &[u8], signer: Arc<Box<dyn Signer>>) -> Result<String, SignerError> {
    let msg = [MESSAGE_PREFIX, msg].concat();
    trace!("About to compute sha256 hash of msg: {msg:?}");
    let digest = sha256::Hash::hash(&sha256::Hash::hash(&msg));
    trace!("About to sign digest: {digest:?}");
    signer
        .sign_ecdsa_recoverable(digest.to_vec())
        .map(|bytes| zbase32::encode_full_bytes(&bytes))
}

impl ListChangesRequest {
    pub(crate) fn new(
        since_revision: u64,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, SignerError> {
        let request_time = utils::now();
        let msg = format!("{since_revision}-{request_time}");
        let signature = sign_message(msg.as_bytes(), signer)?;
        Ok(Self {
            since_revision,
            request_time,
            signature,
        })
    }
}
impl SetRecordRequest {
    pub(crate) fn new(
        record: Record,
        request_time: u32,
        signer: Arc<Box<dyn Signer>>,
        client_id: String,
    ) -> Result<Self, SignerError> {
        let msg = format!(
            "{}-{}-{}-{}-{}",
            record.id,
            record.data.to_lower_hex_string(),
            record.revision,
            *CURRENT_SCHEMA_VERSION,
            request_time,
        );
        trace!("About to sign message: {msg}");
        let signature = sign_message(msg.as_bytes(), signer)?;
        trace!("Got signature: {signature}");
        Ok(Self {
            record: Some(record),
            request_time,
            signature,
            client_id: Some(client_id),
        })
    }
}
impl ListenChangesRequest {
    pub(crate) fn new(signer: Arc<Box<dyn Signer>>) -> Result<Self, SignerError> {
        let request_time = utils::now();
        let msg = format!("{request_time}");
        let signature = sign_message(msg.as_bytes(), signer)?;
        Ok(Self {
            request_time,
            signature,
        })
    }
}
