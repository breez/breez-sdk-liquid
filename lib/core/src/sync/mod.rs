pub(crate) mod client;
pub(crate) mod model;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::{debug, warn};
use std::collections::HashSet;
use tokio::sync::Mutex;
use tokio_stream::StreamExt as _;

use self::client::SyncerClient;
use self::model::sync::TrackChangesRequest;
use self::model::{
    sync::{ListChangesRequest, Record, SetRecordReply, SetRecordRequest, SetRecordStatus},
    DecryptedRecord, SyncData,
};
use crate::{model::Signer, persist::Persister, prelude::Direction, utils};

const CURRENT_SCHEMA_VERSION: f32 = 0.01;

pub(crate) struct SyncService {
    connect_url: String,
    persister: Arc<Persister>,
    sent: Mutex<HashSet<i64>>,
    signer: Arc<Box<dyn Signer>>,
    client: Box<dyn SyncerClient>,
}

impl SyncService {
    pub(crate) fn new(
        connect_url: String,
        persister: Arc<Persister>,
        signer: Arc<Box<dyn Signer>>,
        client: Box<dyn SyncerClient>,
    ) -> Self {
        Self {
            connect_url,
            persister,
            signer,
            client,
            sent: Default::default(),
        }
    }

    fn collect_records<'a>(
        &self,
        records: &'a [Record],
    ) -> (Vec<DecryptedRecord>, Vec<&'a Record>) {
        let mut failed_records = vec![];
        let mut updatable_records = vec![];

        for record in records {
            // If it's a major version ahead, we skip
            if record.schema_version.floor() > CURRENT_SCHEMA_VERSION.floor() {
                failed_records.push(record);
                continue;
            }

            let decrypted_record =
                match DecryptedRecord::try_from_record(self.signer.clone(), record) {
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

impl SyncService {
    pub(crate) async fn connect(&self) -> Result<()> {
        self.client.connect(self.connect_url.clone()).await
    }

    pub(crate) async fn listen(self: Arc<Self>) -> Result<()> {
        let request = TrackChangesRequest::new(utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign ListenChangesRequest: {err:?}"))?;

        let cloned = self.clone();
        tokio::spawn(async move {
            let mut stream = match cloned.client.track_changes(request).await {
                Ok(stream) => stream,
                Err(err) => return warn!("Could not listen to changes: {err:?}"),
            };

            debug!("Started listening to changes");
            while let Some(message) = stream.next().await {
                match message {
                    Ok(record) => {
                        let mut sent = cloned.sent.lock().await;
                        if sent.get(&record.revision).is_some() {
                            sent.remove(&record.revision);
                            continue;
                        }

                        debug!(
                            "Received new record - record_id {} record_revision {} record_schema_version {}",
                            record.id, record.revision, record.schema_version
                        );

                        let record_revision = record.revision;
                        if let Err(err) = cloned.apply_changes(&[record]) {
                            warn!("Could not apply incoming changes: {err:?}")
                        };
                        if let Err(err) = cloned.persister.set_latest_revision(record_revision) {
                            warn!("Could not update latest record revision from stream: {err:?}")
                        };
                    }
                    Err(err) => warn!("An error occured while listening for records: {err:?}"),
                }
            }
        });

        Ok(())
    }

    fn apply_record(&self, record: DecryptedRecord) -> Result<()> {
        match record.data {
            SyncData::Chain(chain_data) => self.persister.insert_chain_swap(&chain_data.into()),
            SyncData::Send(send_data) => self.persister.insert_send_swap(&send_data.into()),
            SyncData::Receive(receive_data) => {
                self.persister.insert_receive_swap(&receive_data.into())
            }
        }
    }

    pub(crate) fn apply_changes(&self, records: &[Record]) -> Result<()> {
        let (updatable_records, failed_records) = self.collect_records(records);

        // We persist records which we cannot update (> CURRENT_SCHEMA_VERSION)
        for record in failed_records {
            self.persister.insert_record(record, Direction::Incoming)?;
        }

        // We apply records which we can update (<= CURRENT_SCHEMA_VERSION)
        for record in updatable_records {
            if let Err(err) = self.apply_record(record) {
                warn!("Could not apply record changes: {err:?}");
            }
        }

        Ok(())
    }

    async fn get_changes_since(&self, from_id: i64) -> Result<Vec<Record>> {
        let request = ListChangesRequest::new(from_id, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign ListChangesRequest: {err:?}"))?;
        let records = self.client.list_changes(request).await?.changes;

        Ok(records)
    }

    pub(crate) async fn get_latest_changes(&self) -> Result<Vec<Record>> {
        let latest_revision = self.persister.get_latest_revision()?;

        let records = self.get_changes_since(latest_revision).await?;

        if let Some(last_record) = records.last() {
            self.persister.set_latest_revision(last_record.revision)?;
        }

        Ok(records)
    }

    pub(crate) async fn set_record(&self, data: SyncData) -> Result<()> {
        let record = Record::new(data, None, self.signer.clone())?;
        let request = SetRecordRequest::new(record, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign SetRecordRequest: {err:?}"))?;

        let SetRecordReply {
            status,
            new_revision,
        } = self.client.set_record(request).await?;

        if SetRecordStatus::try_from(status)? == SetRecordStatus::Conflict {
            return Err(anyhow!("Could not set record: revision conflict."));
        }

        self.sent.lock().await.insert(new_revision);
        self.persister.set_latest_revision(new_revision)?;
        Ok(())
    }

    pub(crate) fn cleanup(&self) -> Result<()> {
        let pending_records = self
            .persister
            .get_records(Some(Direction::Incoming))
            .map_err(|err| anyhow!("Could not fetch pending records from database: {err:?}"))?;

        let (updatable_records, _) = self.collect_records(&pending_records);

        for record in updatable_records {
            let record_id = record.id.clone();
            if self.apply_record(record).is_err() {
                continue;
            }
            self.persister.delete_record(record_id)?;
        }

        Ok(())
    }

    pub(crate) async fn disconnect(&self) -> Result<()> {
        self.client.disconnect().await
    }
}
