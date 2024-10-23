use anyhow::{Result};
use async_trait::async_trait;

use crate::sync::client::SyncerClient;
use crate::sync::model::sync::{
    ListChangesReply, ListChangesRequest, ListenChangesRequest, Record, SetRecordReply,
    SetRecordRequest,
};

pub(crate) struct MockSyncerClient {}

impl MockSyncerClient {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SyncerClient for MockSyncerClient {
    async fn connect(&self, _connect_url: String) -> Result<()> {
        Ok(())
    }

    async fn set_record(&self, _req: SetRecordRequest) -> Result<SetRecordReply> {
        unimplemented!()
    }
    async fn list_changes(&self, _req: ListChangesRequest) -> Result<ListChangesReply> {
        unimplemented!()
    }

    async fn listen_changes(
        &self,
        _req: ListenChangesRequest,
    ) -> Result<tonic::codec::Streaming<Record>> {
        unimplemented!()
    }

    async fn disconnect(&self) -> Result<()> {
        Ok(())
    }
}
