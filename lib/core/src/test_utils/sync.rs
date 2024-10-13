use anyhow::Result;
use std::sync::Arc;

use async_trait::async_trait;

use crate::sync::{
    model::{sync::Record, SyncData},
    SyncService,
};

pub(crate) struct MockSyncService {}

impl MockSyncService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SyncService for MockSyncService {
    async fn connect(&self) -> Result<()> {
        Ok(())
    }

    async fn listen(self: Arc<Self>) -> Result<()> {
        Ok(())
    }

    fn apply_changes(&self, _changes: &[Record]) -> Result<()> {
        Ok(())
    }

    async fn get_changes_since(&self, _from_id: i64) -> Result<Vec<Record>> {
        unimplemented!()
    }

    async fn set_record(&self, _data: SyncData) -> Result<()> {
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        Ok(())
    }
}
