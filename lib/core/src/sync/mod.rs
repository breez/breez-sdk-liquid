pub(crate) mod model;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::warn;
use tokio::sync::Mutex;
use tokio_stream::StreamExt as _;
use tonic::transport::Channel;

use self::model::{
    sync::{
        syncer_client::SyncerClient, ListChangesRequest, ListenChangesRequest, Record,
        SetRecordReply, SetRecordRequest, SetRecordStatus,
    },
    DecryptedRecord, SyncData,
};
use crate::{persist::Persister, signer::SdkSigner, utils};

const CURRENT_SCHEMA_VERSION: f32 = 0.01;

#[async_trait]
pub trait SyncModule {
    /// Connects to a gRPC endpoint
    async fn connect(&self) -> Result<()>;

    /// Listens to the incoming changes stream
    async fn listen(self: Arc<Self>) -> Result<()>;

    /// Applies the changes received from the stream to the local database
    async fn apply_changes(&self, changes: &[Record]) -> Result<()>;

    /// Retrieves the changes since a specified [id](Record::id)
    async fn get_changes_since(&self, from_id: i64) -> Result<Vec<Record>>;

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
    signer: Arc<SdkSigner>,
    client: Mutex<Option<SyncerClient<Channel>>>,
}

impl BreezSyncModule {
    fn collect_records<'a>(
        &self,
        records: &'a [Record],
    ) -> (Vec<DecryptedRecord>, Vec<&'a Record>) {
        let mut failed_records = vec![];
        let mut updatable_records = vec![];

        for record in records {
            // If it's a major version ahead, we skip
            if record.version.floor() > CURRENT_SCHEMA_VERSION.floor() {
                failed_records.push(record);
                continue;
            }

            let decrypted_record =
                match DecryptedRecord::try_from_record(&self.signer.seed(), record) {
                    Ok(record) => record,
                    Err(err) => {
                        warn!("Could not decrypt record: {err:?}");
                        continue;
                    }
                };

            updatable_records.push(decrypted_record)
        }

        (updatable_records, failed_records)
    }
}

#[async_trait]
impl SyncModule for BreezSyncModule {
    async fn connect(&self) -> Result<()> {
        let mut client = self.client.lock().await;
        *client = Some(SyncerClient::connect(self.connect_url.clone()).await?);
        Ok(())
    }

    async fn listen(self: Arc<Self>) -> Result<()> {
        let Some(ref mut client) = *self.client.lock().await else {
            return Err(anyhow!(
                "Cannot run `get_changes_since`: client not connected"
            ));
        };

        let request = ListenChangesRequest::new(utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign ListenChangesRequest: {err:?}"))?;
        let mut stream = client.listen_changes(request).await?.into_inner();

        let cloned = self.clone();
        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(record) => {
                        if let Err(err) = cloned.apply_changes(&[record]).await {
                            warn!("Could not apply incoming changes: {err:?}")
                        };
                    }
                    Err(err) => warn!("An error occured while listening for records: {err:?}"),
                }
            }
        });

        Ok(())
    }

    async fn apply_changes(&self, records: &[Record]) -> Result<()> {
        let (updatable_records, failed_records) = self.collect_records(records);

        // We persist records which we cannot update (> CURRENT_SCHEMA_VERSION)
        for record in failed_records {
            self.persister.insert_record(record)?;
        }

        // We apply records which we can update (<= CURRENT_SCHEMA_VERSION)
        for record in updatable_records {
            if let Err(err) = self.persister.apply_record(record) {
                warn!("Could not apply record changes: {err:?}");
            }
        }

        Ok(())
    }

    async fn get_changes_since(&self, from_id: i64) -> Result<Vec<Record>> {
        let Some(ref mut client) = *self.client.lock().await else {
            return Err(anyhow!(
                "Cannot run `get_changes_since`: client not connected"
            ));
        };

        let request = ListChangesRequest::new(from_id, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign ListChangesRequest: {err:?}"))?;
        let records = client.list_changes(request).await?.into_inner().changes;

        Ok(records)
    }

    async fn set_record(&self, data: SyncData) -> Result<()> {
        let Some(ref mut client) = *self.client.lock().await else {
            return Err(anyhow!("Cannot run `set_record`: client not connected"));
        };

        let id = self.persister.get_latest_record_id()? + 1;
        let data = utils::encrypt(&self.signer.seed(), &data.to_bytes()?)?;
        let record = Record {
            id,
            version: CURRENT_SCHEMA_VERSION,
            data,
        };
        let request = SetRecordRequest::new(record, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign SetRecordRequest: {err:?}"))?;

        let SetRecordReply { status, new_id } = client.set_record(request).await?.into_inner();

        if status == SetRecordStatus::Conflict as i32 {
            return Err(anyhow!("Cannot set record: Local head is behind remote"));
        }

        self.persister.set_latest_record_id(new_id)?;
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