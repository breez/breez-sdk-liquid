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

