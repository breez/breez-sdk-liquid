use anyhow::{anyhow, Error, Result};

use log::debug;
use sdk_common::grpc::transport::{GrpcClient, Transport};
use tokio::sync::Mutex;
use tonic::{
    metadata::{errors::InvalidMetadataValue, Ascii, MetadataValue},
    service::{interceptor::InterceptedService, Interceptor},
    Request, Status, Streaming,
};

use super::model::{
    syncer_client::SyncerClient as ProtoSyncerClient, ListChangesReply, ListChangesRequest,
    ListenChangesRequest, Notification, SetRecordReply, SetRecordRequest,
};

#[sdk_macros::async_trait]
pub(crate) trait SyncerClient: Send + Sync {
    async fn connect(&self, connect_url: String) -> Result<()>;
    async fn push(&self, req: SetRecordRequest) -> Result<SetRecordReply>;
    async fn pull(&self, req: ListChangesRequest) -> Result<ListChangesReply>;
    async fn listen(&self, req: ListenChangesRequest) -> Result<Streaming<Notification>>;
    async fn disconnect(&self) -> Result<()>;
}

pub(crate) struct BreezSyncerClient {
    grpc_channel: Mutex<Option<Transport>>,
    api_key: Option<String>,
}

impl BreezSyncerClient {
    pub(crate) fn new(api_key: Option<String>) -> Self {
        Self {
            grpc_channel: Mutex::new(None),
            api_key,
        }
    }

    fn api_key_metadata(&self) -> Result<Option<MetadataValue<Ascii>>, Error> {
        match &self.api_key {
            Some(key) => Ok(Some(format!("Bearer {key}").parse().map_err(
                |e: InvalidMetadataValue| {
                    anyhow!(format!(
                        "(Breez: {:?}) Failed parse API key: {e}",
                        self.api_key
                    ))
                },
            )?)),
            _ => Ok(None),
        }
    }
}

impl BreezSyncerClient {
    async fn get_client(
        &self,
    ) -> Result<ProtoSyncerClient<InterceptedService<Transport, ApiKeyInterceptor>>, Error> {
        let Some(channel) = self.grpc_channel.lock().await.clone() else {
            return Err(anyhow!("Cannot get sync client: not connected"));
        };
        let api_key_metadata = self.api_key_metadata()?;
        Ok(ProtoSyncerClient::with_interceptor(
            channel,
            ApiKeyInterceptor { api_key_metadata },
        ))
    }
}

#[sdk_macros::async_trait]
impl SyncerClient for BreezSyncerClient {
    async fn connect(&self, connect_url: String) -> Result<()> {
        let mut grpc_channel = self.grpc_channel.lock().await;
        *grpc_channel = Some(GrpcClient::new(connect_url.clone())?.into_inner());
        debug!("Successfully connected to {connect_url}");
        Ok(())
    }

    async fn push(&self, req: SetRecordRequest) -> Result<SetRecordReply> {
        Ok(self.get_client().await?.set_record(req).await?.into_inner())
    }

    async fn pull(&self, req: ListChangesRequest) -> Result<ListChangesReply> {
        Ok(self
            .get_client()
            .await?
            .list_changes(req)
            .await?
            .into_inner())
    }

    async fn listen(&self, req: ListenChangesRequest) -> Result<Streaming<Notification>> {
        Ok(self
            .get_client()
            .await?
            .listen_changes(req)
            .await?
            .into_inner())
    }

    async fn disconnect(&self) -> Result<()> {
        let mut channel = self.grpc_channel.lock().await;
        *channel = None;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ApiKeyInterceptor {
    api_key_metadata: Option<MetadataValue<Ascii>>,
}

impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        if let Some(api_key_metadata) = &self.api_key_metadata {
            req.metadata_mut()
                .insert("authorization", api_key_metadata.clone());
        }
        Ok(req)
    }
}
