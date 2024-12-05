use anyhow::{anyhow, Result};

use async_trait::async_trait;
use log::debug;
use tokio::sync::Mutex;

use super::model::sync::{
    syncer_client::SyncerClient as ProtoSyncerClient, ListChangesReply, ListChangesRequest,
    SetRecordReply, SetRecordRequest,
};

#[async_trait]
pub(crate) trait SyncerClient: Send + Sync {
    async fn connect(&self, connect_url: String) -> Result<()>;
    async fn push(&self, req: SetRecordRequest) -> Result<SetRecordReply>;
    async fn pull(&self, req: ListChangesRequest) -> Result<ListChangesReply>;
    async fn disconnect(&self) -> Result<()>;
}

pub(crate) struct BreezSyncerClient {
    inner: Mutex<Option<ProtoSyncerClient<tonic::transport::Channel>>>,
}

impl BreezSyncerClient {
    pub(crate) fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

#[async_trait]
impl SyncerClient for BreezSyncerClient {
    async fn connect(&self, connect_url: String) -> Result<()> {
        let mut client = self.inner.lock().await;
        *client = Some(ProtoSyncerClient::connect(connect_url.clone()).await?);
        debug!("Successfully connected to {connect_url}");
        Ok(())
    }

    async fn push(&self, req: SetRecordRequest) -> Result<SetRecordReply> {
        let Some(mut client) = self.inner.lock().await.clone() else {
            return Err(anyhow!("Cannot run `set_record`: client not connected"));
        };
        Ok(client.set_record(req).await?.into_inner())
    }
    async fn pull(&self, req: ListChangesRequest) -> Result<ListChangesReply> {
        let Some(mut client) = self.inner.lock().await.clone() else {
            return Err(anyhow!("Cannot run `list_changes`: client not connected"));
        };
        Ok(client.list_changes(req).await?.into_inner())
    }

    async fn disconnect(&self) -> Result<()> {
        let mut client = self.inner.lock().await;
        *client = None;
        Ok(())
    }
}
