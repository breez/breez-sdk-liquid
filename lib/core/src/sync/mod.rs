pub(crate) mod client;
pub(crate) mod model;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures_util::TryFutureExt;
use log::{debug, warn};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tokio_stream::StreamExt as _;

use self::client::SyncerClient;
use self::model::sync::TrackChangesRequest;
use self::model::SyncDetails;
use self::model::{
    sync::{ListChangesRequest, Record, SetRecordReply, SetRecordRequest, SetRecordStatus},
    DecryptedRecord, SyncData,
};
use crate::prelude::{ChainSwap, ReceiveSwap, SendSwap};
use crate::{model::Signer, persist::Persister, utils};

const CURRENT_SCHEMA_VERSION: f32 = 0.01;
const MAX_SET_RECORD_REATTEMPTS: u8 = 5;

pub(crate) struct SyncService {
    connect_url: String,
    persister: Arc<Persister>,
    signer: Arc<Box<dyn Signer>>,
    client: Box<dyn SyncerClient>,
    sent_counter: Mutex<HashMap<String, u8>>,
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
            sent_counter: Default::default(),
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
    /// Connects to the gRPC endpoint specified in [SyncService::connect_url]
    /// Additionally, this method pulls the latest changes from the remote and applies them
    pub(crate) async fn connect(&self) -> Result<()> {
        self.client.connect(self.connect_url.clone()).await?;
        self.sync_with_tip().await?;
        Ok(())
    }

    /// Listens to updates for new changes.
    /// This method ignores changes we broadcasted by referring to the `sent_counter` inner hashset
    /// Records which are received from an external instance are instantly applied to the local
    /// database. Errors are skipped.
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
                        let mut sent = cloned.sent_counter.lock().await;
                        if sent.get(&record.id).is_some() {
                            sent.remove(&record.id);
                            continue;
                        }

                        let record_id = record.id.clone();
                        let record_revision = record.revision;
                        let record_schema_version = record.schema_version;

                        debug!(
                            "Received new record - record_id {record_id} record_revision {record_revision} record_schema_version {record_schema_version}"
                        );

                        if let Err(err) = cloned.apply_changes(&[record]) {
                            warn!("Could not apply incoming changes: {err:?}")
                        };

                        debug!("Successfully applied incoming changes for record {record_id}",)
                    }
                    Err(err) => warn!("An error occured while listening for records: {err:?}"),
                }
            }
        });

        Ok(())
    }

    fn apply_record(&self, record: DecryptedRecord) -> Result<()> {
        debug!(
            "Applying record - id {} revision {}",
            &record.id, record.revision
        );
        match record.data {
            SyncData::Chain(chain_data) => self.persister.insert_chain_swap(
                &ChainSwap::from_sync_data(chain_data, record.revision, record.id),
            ),
            SyncData::Send(send_data) => self.persister.insert_send_swap(
                &SendSwap::from_sync_data(send_data, record.revision, record.id),
            ),
            SyncData::Receive(receive_data) => {
                self.persister
                    .insert_receive_swap(&ReceiveSwap::from_sync_data(
                        receive_data,
                        record.revision,
                        record.id,
                    ))
            }
        }
    }

    /// Applies a given set of changes into the local database
    /// For each record, if its (schema_version)[Record::schema_version] is greater than the
    /// [CURRENT_SCHEMA_VERSION], the record will be persisted into the `pending_sync_records` and
    /// applied later.
    /// Instead, if the schema_version is less or equal to the client's current version, it will try
    /// and apply the changes, skipping errors if any
    pub(crate) fn apply_changes(&self, records: &[Record]) -> Result<()> {
        let (updatable_records, failed_records) = self.collect_records(records);

        // We persist records which we cannot update (> CURRENT_SCHEMA_VERSION)
        for record in failed_records {
            self.persister.insert_pending_record(record)?;
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

    /// Pulls the latest changes from the remote, *without* updating the local database
    pub(crate) async fn get_latest_changes(&self) -> Result<Vec<Record>> {
        let latest_revision = self.persister.get_latest_revision()?;
        let records = self.get_changes_since(latest_revision).await?;
        Ok(records)
    }

    async fn sync_with_tip(&self) -> Result<()> {
        self.get_latest_changes()
            .await
            .and_then(|records| self.apply_changes(&records))
    }

    async fn increment_sent_counter(&self, record_id: String) -> u8 {
        let mut sent_counter = self.sent_counter.lock().await;
        let mut num_attempts = *sent_counter.get(&record_id).unwrap_or(&0);
        num_attempts += 1;
        sent_counter.insert(record_id.to_string(), num_attempts);
        return num_attempts;
    }

    /// Syncs the given data outwards
    /// If `is_update` is specified, the method will look into the local `sync_details` cache and
    /// get the correct revision number and record id
    pub(crate) async fn set_record(&self, data: SyncData, is_update: bool) -> Result<()> {
        let data_identifier = data.id();
        let (record_id, revision) = match is_update {
            true => {
                let existing_sync_details = self.persister.get_sync_details(&data_identifier)?;
                (
                    existing_sync_details.record_id.expect(
                        "Expecting valid record_id when calling `set_record` with `is_update` flag",
                    ),
                    existing_sync_details.revision.expect(
                        "Expecting valid revision when calling `set_record` with `is_update` flag",
                    ),
                )
            }
            false => (uuid::Uuid::new_v4().to_string(), 0),
        };
        let record = Record::new(
            record_id.clone(),
            data.clone(),
            revision,
            self.signer.clone(),
        )?;

        debug!("Starting outward sync (set_record) for record {record_id} is_update {is_update}");

        let num_attempts = self.increment_sent_counter(record_id.clone()).await;
        let request = SetRecordRequest::new(record, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign SetRecordRequest: {err:?}"))?;

        match self.client.set_record(request).await {
            Ok(SetRecordReply {
                status,
                new_revision,
            }) => {
                if SetRecordStatus::try_from(status)? == SetRecordStatus::Conflict {
                    if num_attempts > MAX_SET_RECORD_REATTEMPTS {
                        return Err(anyhow!(
                            "Could not set record: revision conflict and max reattempts reached."
                        ));
                    }

                    return Box::pin(
                        self.sync_with_tip()
                            .and_then(|_| self.set_record(data, is_update)),
                    )
                    .await;
                }

                self.persister.insert_or_update_sync_details(
                    &data_identifier,
                    &SyncDetails {
                        is_local: true,
                        revision: Some(new_revision),
                        record_id: Some(record_id.clone()),
                    },
                )?;

                debug!("Successfully synced (set_record) id {record_id} is_update {is_update} revision {new_revision}");
                Ok(())
            }
            Err(err) => {
                self.sent_counter.lock().await.remove(&record_id);
                debug!("Could not sync record (set_record) id {record_id} is_update {is_update}: {err:?}");
                Err(err)
            }
        }
    }

    /// Cleans up the cached pending records, by trying to apply each one
    /// This method is especially useful once a client has upgraded, and is now able to
    /// successfully apply a record with a higher schema_version (see [SyncService::apply_changes])
    pub(crate) fn cleanup(&self) -> Result<()> {
        let pending_records = self
            .persister
            .get_pending_records()
            .map_err(|err| anyhow!("Could not fetch pending records from database: {err:?}"))?;

        let (updatable_records, _) = self.collect_records(&pending_records);

        for record in updatable_records {
            let record_id = record.id.clone();
            if self.apply_record(record).is_err() {
                continue;
            }
            self.persister.delete_pending_record(record_id)?;
        }

        Ok(())
    }

    /// Disconnects from the gRPC endpoint
    /// TODO: Add shutdown signal for stream
    pub(crate) async fn disconnect(&self) -> Result<()> {
        self.client.disconnect().await
    }
}
