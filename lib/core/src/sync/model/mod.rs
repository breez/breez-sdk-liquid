use std::sync::Arc;

use self::{data::SyncData, sync::Record};
use crate::prelude::Signer;
use anyhow::Result;
use lazy_static::lazy_static;
use lwk_wollet::hashes::hex::DisplayHex;
use openssl::sha::sha256;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};
use semver::Version;

pub(crate) mod client;
pub(crate) mod data;
pub(crate) mod sync;

const MESSAGE_PREFIX: &[u8; 13] = b"realtimesync:";
lazy_static! {
    static ref CURRENT_SCHEMA_VERSION: Version = Version::parse("0.0.1").unwrap();
}

#[derive(Copy, Clone)]
pub(crate) enum RecordType {
    Receive = 0,
    Send = 1,
    Chain = 2,
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

impl Record {
    pub(crate) fn new(
        data: SyncData,
        revision: u64,
        signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self, anyhow::Error> {
        let id = Self::get_id_from_sync_data(&data);
        let data = data.to_bytes()?;
        let data = signer
            .ecies_encrypt(data)
            .map_err(|err| anyhow::anyhow!("Could not encrypt sync data: {err:?}"))?;
        let schema_version = CURRENT_SCHEMA_VERSION.to_string();
        Ok(Self {
            id,
            revision,
            schema_version,
            data,
        })
    }

    fn id(prefix: String, data_id: &str) -> String {
        sha256((prefix + ":" + data_id).as_bytes()).to_lower_hex_string()
    }

    pub(crate) fn get_id_from_sync_data(data: &SyncData) -> String {
        let prefix = match data {
            SyncData::Chain(_) => "chain-swap",
            SyncData::Send(_) => "send-swap",
            SyncData::Receive(_) => "receive-swap",
        }
        .to_string();
        Self::id(prefix, data.id())
    }

    pub(crate) fn get_id_from_record_type(record_type: RecordType, data_id: &str) -> String {
        let prefix = match record_type {
            RecordType::Chain => "chain-swap",
            RecordType::Send => "send-swap",
            RecordType::Receive => "receive-swap",
        }
        .to_string();
        Self::id(prefix, data_id)
    }

    pub(crate) fn is_applicable(&self) -> Result<bool> {
        let record_version = Version::parse(&self.schema_version)?;
        Ok(CURRENT_SCHEMA_VERSION.major >= record_version.major)
    }

    pub(crate) fn decrypt(self, signer: Arc<Box<dyn Signer>>) -> Result<DecryptedRecord> {
        let dec_data = signer.ecies_decrypt(self.data)?;
        let data = serde_json::from_slice(&dec_data)?;
        Ok(DecryptedRecord {
            id: self.id,
            revision: self.revision,
            schema_version: self.schema_version,
            data,
        })
    }
}
