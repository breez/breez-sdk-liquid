pub(crate) mod model;

use anyhow::Result;
use async_trait::async_trait;

use self::model::{sync::Record, SyncData};

#[async_trait]
pub trait SyncModule {
    /// Connects to a gRPC stream
    async fn connect(&self) -> Result<()>;

    /// Applies the changes received from the stream to the local database
    async fn apply_changes(&self, changes: &[Record]) -> Result<()>;

    /// Retrieves the changes since a specified [record_index](Record::record_index)
    async fn get_changes_since(&self, record_index: u64) -> Result<Vec<Record>>;

    /// Adds a record to the remote
    async fn set_record(&self, data: SyncData) -> Result<()>;

    /// Attemps to clean up local changes by applying them
    async fn cleanup(&self) -> Result<()>;
}

pub(crate) struct BreezSyncModule {}

#[async_trait]
impl SyncModule for BreezSyncModule {
    async fn connect(&self) -> Result<()> {
        unimplemented!()
    }

    async fn apply_changes(&self, changes: &[Record]) -> Result<()> {
        unimplemented!()
    }

    async fn get_changes_since(&self, record_index: u64) -> Result<Vec<Record>> {
        unimplemented!()
    }

    async fn set_record(&self, data: SyncData) -> Result<()> {
        unimplemented!()
    }

    async fn cleanup(&self) -> Result<()> {
        unimplemented!()
    }
}
