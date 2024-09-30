pub(crate) mod model;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rusqlite::params;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use self::model::{
    sync::{
        syncer_client::SyncerClient, ListChangesReply, ListChangesRequest, Record, SetRecordReply,
        SetRecordRequest, SetRecordStatus,
    },
    SyncData,
};
use crate::{persist::Persister, utils};

const CURRENT_SCHEMA_VERSION: f32 = 0.1;

#[async_trait]
pub trait SyncModule {
    /// Connects to a gRPC stream
    async fn connect(&self) -> Result<()>;

    /// Applies the changes received from the stream to the local database
    async fn apply_changes(&self, changes: &[Record]) -> Result<()>;

    /// Retrieves the changes since a specified [id](Record::id)
    async fn get_changes_since(&self, from_id: u64) -> Result<Vec<Record>>;

    /// Adds a record to the remote
    async fn set_record(&self, data: SyncData) -> Result<()>;

    /// Attemps to clean up local changes by applying them
    async fn cleanup(&self) -> Result<()>;

    /// Disconnects from the gRPC stream
    async fn disconnect(&self) -> Result<()>;
}

pub(crate) struct BreezSyncModule {
    connect_url: String,
    persister: Arc<Persister>,
    client: Mutex<Option<SyncerClient<Channel>>>,
}

impl BreezSyncModule {
    fn get_latest_record_id(&self) -> Result<i64> {
        let con = self.persister.get_connection()?;

        let latest_record_id: i64 = con.query_row(
            "SELECT latestRecordId FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )?;

        Ok(latest_record_id)
    }

    fn set_latest_record_id(&self, new_latest_id: i64) -> Result<()> {
        let con = self.persister.get_connection()?;

        con.execute(
            "INSERT OR REPLACE INTO settings(id, latestRecordId) VALUES(1, ?)",
            params![new_latest_id],
        )
        .map_err(|err| anyhow!("Could not write latest record id to database: {err}"))?;

        Ok(())
    }
}

#[async_trait]
impl SyncModule for BreezSyncModule {
    async fn connect(&self) -> Result<()> {
        let mut client = self.client.lock().await;
        *client = Some(SyncerClient::connect(self.connect_url.clone()).await?);
        Ok(())
    }

    async fn apply_changes(&self, changes: &[Record]) -> Result<()> {
        unimplemented!()
    }

    async fn get_changes_since(&self, from_id: u64) -> Result<Vec<Record>> {
        let Some(ref mut client) = *self.client.lock().await else {
            return Err(anyhow!(
                "Cannot run `get_changes_since`: client not connected"
            ));
        };

        let ListChangesReply { changes } = client
            .list_changes(ListChangesRequest {
                from_id: from_id as i64,
                request_time: utils::now(),
                signature: todo!(),
            })
            .await?
            .into_inner();

        todo!()
    }

    async fn set_record(&self, data: SyncData) -> Result<()> {
        let Some(ref mut client) = *self.client.lock().await else {
            return Err(anyhow!("Cannot run `set_record`: client not connected"));
        };

        let id = self.get_latest_record_id()? + 1;
        let data = todo!();
        let record = Some(Record {
            id,
            version: CURRENT_SCHEMA_VERSION,
            data,
        });
        let SetRecordReply { status, new_id } = client
            .set_record(SetRecordRequest {
                record,
                request_time: utils::now(),
                signature: todo!(),
            })
            .await?
            .into_inner();

        if status == SetRecordStatus::Conflict as i32 {
            return Err(anyhow!("Cannot set record: Local head is behind remote"));
        }

        self.set_latest_record_id(new_id)?;
        Ok(())
    }

    async fn cleanup(&self) -> Result<()> {
        unimplemented!()
    }

    async fn disconnect(&self) -> Result<()> {
        let mut client = self.client.lock().await;
        *client = None;
        Ok(())
    }
}
