use anyhow::{anyhow, Result};

use async_trait::async_trait;
use http::Uri;
use log::debug;
use rustls::ClientConfig;
use rustls_platform_verifier::BuilderVerifierExt;
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
    inner: Mutex<Option<ProtoSyncerClient<tonic_rustls::Channel>>>,
}

impl BreezSyncerClient {
    pub(crate) fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl ProtoSyncerClient<tonic_rustls::Channel> {
    /// Attempt to create a new client by connecting to a given endpoint.
    pub async fn connect<D>(dst: D) -> Result<Self, tonic_rustls::Error>
    where
        D: TryInto<tonic_rustls::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let conn = tonic_rustls::Endpoint::new(dst)?.connect().await?;
        Ok(Self::new(conn))
    }
}

#[async_trait]
impl SyncerClient for BreezSyncerClient {
    async fn connect(&self, connect_url: String) -> Result<()> {
        let mut client = self.inner.lock().await;

        let uri: Uri = connect_url.parse()?;
        let mut endpoint = tonic_rustls::Endpoint::from(uri.clone());
        if let Some("https") = uri.scheme_str() {
            let crypto_provider = std::sync::Arc::new(rustls::crypto::ring::default_provider());
            let tls_config = ClientConfig::builder_with_provider(crypto_provider)
                .with_safe_default_protocol_versions()?
                .with_platform_verifier()
                .with_no_client_auth();
            endpoint = endpoint.tls_config(tls_config)?;
        };

        *client = Some(ProtoSyncerClient::<tonic_rustls::Channel>::connect(endpoint).await?);
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
