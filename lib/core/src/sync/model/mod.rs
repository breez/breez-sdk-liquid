use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

pub(crate) mod sync;


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

