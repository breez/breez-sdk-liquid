tonic::include_proto!("sync");

use self::data::SyncData;
use crate::prelude::{Signer, SignerError};
use anyhow::Result;
use lazy_static::lazy_static;
use log::trace;
use lwk_wollet::hashes::hex::DisplayHex;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};
use sdk_common::bitcoin::hashes::{sha256, Hash};
use sdk_common::utils::Arc;
use semver::Version;

pub(crate) mod client;
pub(crate) mod data;

const MESSAGE_PREFIX: &[u8; 13] = b"realtimesync:";
lazy_static! {
    static ref CURRENT_SCHEMA_VERSION: Version = Version::parse("0.8.0").unwrap();
}

#[derive(Copy, Clone)]
pub(crate) enum RecordType {
    Receive = 0,
    Send = 1,
    Chain = 2,
    LastDerivationIndex = 3,
    PaymentDetails = 4,
    Bolt12Offer = 5,
}

impl ToSql for RecordType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i8))
    }
}

impl FromSql for RecordType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(Self::Receive),
                1 => Ok(Self::Send),
                2 => Ok(Self::Chain),
                3 => Ok(Self::LastDerivationIndex),
                4 => Ok(Self::PaymentDetails),
                5 => Ok(Self::Bolt12Offer),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

pub(crate) struct SyncState {
    pub(crate) data_id: String,
    pub(crate) record_id: String,
    pub(crate) record_revision: u64,
    pub(crate) is_local: bool,
}

pub(crate) struct SyncSettings {
    pub(crate) remote_url: Option<String>,
    pub(crate) latest_revision: Option<u64>,
}

pub(crate) struct SyncOutgoingChanges {
    pub(crate) record_id: String,
    pub(crate) data_id: String,
    pub(crate) record_type: RecordType,
    pub(crate) commit_time: u32,
    pub(crate) updated_fields: Option<Vec<String>>,
}

pub(crate) struct DecryptedRecord {
    pub(crate) revision: u64,
    pub(crate) id: String,
    #[allow(dead_code)]
    pub(crate) schema_version: String,
    pub(crate) data: SyncData,
}

pub(crate) struct DecryptionInfo {
    pub(crate) new_sync_state: SyncState,
    pub(crate) record: DecryptedRecord,
    pub(crate) last_commit_time: Option<u32>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum PullError {
    #[error("Record is not applicable: schema_version too high")]
    SchemaNotApplicable,

    #[error("Remote record revision is lower or equal to the persisted one. Skipping update.")]
    AlreadyPersisted,

    #[error("Could not sign outgoing payload: {err}")]
    Signing { err: String },

    #[error("Could not decrypt incoming record: {err}")]
    Decryption { err: String },

    #[error("Could not deserialize record data: {err}")]
    Deserialization { err: String },

    #[error("Remote record version could not be parsed: {err}")]
    InvalidRecordVersion { err: String },

    #[error("Could not contact remote: {err}")]
    Network { err: String },

    #[error("Could not call the persister: {err}")]
    Persister { err: String },

    #[error("Could not merge record with updated fields: {err}")]
    Merge { err: String },

    #[error("Could not recover record data from onchain: {err}")]
    Recovery { err: String },
}

impl PullError {
    pub(crate) fn signing(err: SignerError) -> Self {
        Self::Signing {
            err: err.to_string(),
        }
    }

    pub(crate) fn decryption(err: SignerError) -> Self {
        Self::Decryption {
            err: err.to_string(),
        }
    }

    pub(crate) fn deserialization(err: serde_json::Error) -> Self {
        Self::Deserialization {
            err: err.to_string(),
        }
    }

    pub(crate) fn invalid_record_version(err: semver::Error) -> Self {
        Self::InvalidRecordVersion {
            err: err.to_string(),
        }
    }

    pub(crate) fn network(err: anyhow::Error) -> Self {
        Self::Network {
            err: err.to_string(),
        }
    }

    pub(crate) fn persister(err: anyhow::Error) -> Self {
        Self::Persister {
            err: err.to_string(),
        }
    }

    pub(crate) fn merge(err: anyhow::Error) -> Self {
        Self::Merge {
            err: err.to_string(),
        }
    }

    pub(crate) fn recovery(err: anyhow::Error) -> Self {
        Self::Recovery {
            err: err.to_string(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum PushError {
    #[error("Received conflict status from remote")]
    RecordConflict,

    #[error("Could not sign outgoing payload: {err}")]
    Signing { err: String },

    #[error("Could not encrypt outgoing record: {err}")]
    Encryption { err: String },

    #[error("Could not serialize record data: {err}")]
    Serialization { err: String },

    #[error("Could not contact remote: {err}")]
    Network { err: String },

    #[error("Could not call the persister: {err}")]
    Persister { err: String },

    #[error("Push completed with too many failed records: succeded {succeded} total {total} recoverable {recoverable}")]
    ExcessiveRecordConflicts {
        succeded: usize,
        total: usize,
        recoverable: usize,
    },
}

impl PushError {
    pub(crate) fn signing(err: SignerError) -> Self {
        Self::Signing {
            err: err.to_string(),
        }
    }

    pub(crate) fn encryption(err: SignerError) -> Self {
        Self::Encryption {
            err: err.to_string(),
        }
    }

    pub(crate) fn serialization(err: serde_json::Error) -> Self {
        Self::Serialization {
            err: err.to_string(),
        }
    }

    pub(crate) fn network(err: anyhow::Error) -> Self {
        Self::Network {
            err: err.to_string(),
        }
    }

    pub(crate) fn persister(err: anyhow::Error) -> Self {
        Self::Persister {
            err: err.to_string(),
        }
    }
}

impl Record {
    pub(crate) fn new(
        data: SyncData,
        revision: u64,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, PushError> {
        let id = Self::get_id_from_sync_data(&data);
        let data = data.to_bytes().map_err(PushError::serialization)?;
        trace!("About to encrypt sync data: {data:?}");
        let data = signer.ecies_encrypt(data).map_err(PushError::encryption)?;
        trace!("Got encrypted sync data: {data:?}");
        let schema_version = CURRENT_SCHEMA_VERSION.to_string();
        Ok(Self {
            id,
            revision,
            schema_version,
            data,
        })
    }

    fn id(prefix: String, data_id: &str) -> String {
        sha256::Hash::hash((prefix + ":" + data_id).as_bytes()).to_lower_hex_string()
    }

    pub(crate) fn get_id_from_sync_data(data: &SyncData) -> String {
        let prefix = match data {
            SyncData::Chain(_) => "chain-swap",
            SyncData::Send(_) => "send-swap",
            SyncData::Receive(_) => "receive-swap",
            SyncData::LastDerivationIndex(_) => "derivation-index",
            SyncData::PaymentDetails(_) => "payment-details",
            SyncData::Bolt12Offer(_) => "bolt12-offer",
        }
        .to_string();
        Self::id(prefix, data.id())
    }

    pub(crate) fn get_id_from_record_type(record_type: RecordType, data_id: &str) -> String {
        let prefix = match record_type {
            RecordType::Chain => "chain-swap",
            RecordType::Send => "send-swap",
            RecordType::Receive => "receive-swap",
            RecordType::LastDerivationIndex => "derivation-index",
            RecordType::PaymentDetails => "payment-details",
            RecordType::Bolt12Offer => "bolt12-offer",
        }
        .to_string();
        Self::id(prefix, data_id)
    }

    pub(crate) fn is_applicable(&self) -> Result<bool, semver::Error> {
        let record_version = Version::parse(&self.schema_version)?;
        Ok(CURRENT_SCHEMA_VERSION.major >= record_version.major)
    }

    pub(crate) fn decrypt(
        self,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<DecryptedRecord, PullError> {
        let dec_data = signer
            .ecies_decrypt(self.data)
            .map_err(PullError::decryption)?;
        let data = serde_json::from_slice(&dec_data).map_err(PullError::deserialization)?;
        Ok(DecryptedRecord {
            id: self.id,
            revision: self.revision,
            schema_version: self.schema_version,
            data,
        })
    }
}
