use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

pub(crate) mod client;
pub(crate) mod sync;

const MESSAGE_PREFIX: &[u8; 13] = b"realtimesync:";

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
