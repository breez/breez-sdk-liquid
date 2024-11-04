pub(crate) mod client;
pub(crate) mod model;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures_util::TryFutureExt;
use log::{debug, warn};
use std::collections::HashMap;
use tokio::sync::{watch, Mutex};
use tokio_stream::StreamExt as _;

use self::client::SyncerClient;
use self::model::sync::TrackChangesRequest;
use self::model::{
    sync::{ListChangesRequest, Record, SetRecordReply, SetRecordRequest, SetRecordStatus},
    DecryptedRecord, SyncData,
};
use self::model::{Merge, OutboundChange, SyncDetails};
use crate::prelude::{ChainSwap, ReceiveSwap, SendSwap};
use crate::{model::Signer, persist::Persister, utils};

const CURRENT_SCHEMA_VERSION: f32 = 0.01;
const MAX_SET_RECORD_REATTEMPTS: u8 = 5;

pub(crate) struct SyncService {
    connect_url: String,
    persister: Arc<Persister>,
    signer: Arc<Box<dyn Signer>>,
    client: Box<dyn SyncerClient>,
    outbound_changes: Mutex<HashMap<String, OutboundChange>>,
}

impl SyncService {
    pub(crate) fn new(
        connect_url: String,
        persister: Arc<Persister>,
        signer: Arc<Box<dyn Signer>>,
        client: Box<dyn SyncerClient>,
    ) -> Arc<Self> {
        Arc::new(Self {
            connect_url,
            persister,
            signer,
            client,
            outbound_changes: Default::default(),
        })
    }

    /// Connects to the gRPC endpoint specified in [SyncService::connect_url] and starts listening
    /// for changes
    /// Additionally, this method pulls the latest changes from the remote and applies them
    pub(crate) async fn connect(
        self: Arc<Self>,
        shutdown_receiver: watch::Receiver<()>,
    ) -> Result<()> {
        self.client.connect(self.connect_url.clone()).await?;
        self.sync_with_tip().await?;
        self.listen(shutdown_receiver).await?;
        Ok(())
    }

    /// Listens to updates for new changes.
    /// This method ignores changes we broadcasted by referring to the `sent_counter` inner hashset
    /// Records which are received from an external instance are instantly applied to the local
    /// database. Errors are skipped.
    async fn listen(self: Arc<Self>, mut shutdown_receiver: watch::Receiver<()>) -> Result<()> {
        let request = TrackChangesRequest::new(utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign ListenChangesRequest: {err:?}"))?;

        let cloned = self.clone();
        tokio::spawn(async move {
            let mut stream = match cloned.client.track_changes(request).await {
                Ok(stream) => stream,
                Err(err) => return warn!("Could not listen to changes: {err:?}"),
            };

            debug!("Started listening to changes");

            loop {
                tokio::select! {
                    Some(message) = stream.next() => {
                        match message {
                            Ok(record) => {
                                let mut sent = cloned.outbound_changes.lock().await;
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

                                if let Err(err) = cloned.apply_changes(&[record]).await {
                                    warn!("Could not apply incoming changes: {err:?}")
                                };

                                debug!("Successfully applied incoming changes for record {record_id}",)
                            }
                            Err(err) => warn!("An error occured while listening for records: {err:?}"),
                        }
                    }
                    _ = shutdown_receiver.changed() => {
                        debug!("Received shutdown signal, exiting sync loop");
                        if let Err(err) = cloned.client.disconnect().await {
                            debug!("Could not disconnect sync client: {err:?}");
                        }
                        return;
                    }
                }
            }
        });

        Ok(())
    }

    async fn apply_record(&self, record: DecryptedRecord) -> Result<()> {
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

    /// Collects the given records in two categories: upgradable and non-upgradable, based on their
    /// schema_version
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

    /// Applies a given set of changes into the local database
    /// For each record, if its (schema_version)[Record::schema_version] is greater than the
    /// [CURRENT_SCHEMA_VERSION], the record will be persisted into the `pending_sync_records` and
    /// applied later.
    /// Instead, if the schema_version is less or equal to the client's current version, it will try
    /// and apply the changes, skipping errors if any
    pub(crate) async fn apply_changes(&self, records: &[Record]) -> Result<()> {
        let (updatable_records, failed_records) = self.collect_records(records);

        // We persist records which we cannot update (> CURRENT_SCHEMA_VERSION)
        for record in failed_records {
            self.persister.insert_pending_record(record)?;
        }

        let changes = self.outbound_changes.lock().await;

        // We apply records which we can update (<= CURRENT_SCHEMA_VERSION)
        // Additionally, if there is a conflict with an outbound record, it will be resolved
        // before persisting the update
        for mut record in updatable_records {
            if let Some(outgoing_record) = changes.get(&record.id) {
                if let Err(err) = record
                    .data
                    .merge(&outgoing_record.data, &outgoing_record.updated_fields)
                {
                    warn!("Could not merge inbound record with outbound changes: {err:?}");
                    continue;
                }
            }

            if let Err(err) = self.apply_record(record).await {
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
        let records = self.get_latest_changes().await?;
        self.apply_changes(&records).await
    }

    /// Syncs the given data outwards
    /// If `updated_fields` is specified, the method will look into the local `sync_details` cache and
    /// get the correct revision number and record id, as well as calculate the number of
    /// set_record attempts which have been executed so far
    pub(crate) async fn set_record(
        &self,
        data: SyncData,
        updated_fields: Option<&[&'static str]>,
    ) -> Result<()> {
        let data_identifier = data.id();
        let (record_id, revision, attempts) = match updated_fields {
            Some(fields) => {
                let existing_sync_details = self.persister.get_sync_details(&data_identifier)?;

                let record_id = existing_sync_details.record_id.ok_or(anyhow!(
                    "Expecting valid record_id when calling `set_record` with updated fields",
                ))?;
                let revision = existing_sync_details.revision.ok_or(anyhow!(
                    "Expecting valid revision when calling `set_record` with updated fields",
                ))?;

                let mut changes = self.outbound_changes.lock().await;
                let attempts = match changes.get_mut(&record_id) {
                    Some(change) => {
                        change.increment_counter();
                        change.reattempt_counter
                    }
                    None => {
                        let change = OutboundChange::new(data.clone(), fields.into());
                        let reattempt_counter = change.reattempt_counter;
                        changes.insert(record_id.clone(), change);
                        reattempt_counter
                    }
                };

                (record_id, revision, Some(attempts))
            }
            None => (uuid::Uuid::new_v4().to_string(), 0, None),
        };
        let record = Record::new(
            record_id.clone(),
            data.clone(),
            revision,
            self.signer.clone(),
        )?;

        debug!("Starting outward sync (set_record) for record {record_id} updated_fields {updated_fields:?}");

        let request = SetRecordRequest::new(record, utils::now(), self.signer.clone())
            .map_err(|err| anyhow!("Could not sign SetRecordRequest: {err:?}"))?;

        match self.client.set_record(request).await {
            Ok(SetRecordReply {
                status,
                new_revision,
            }) => {
                if SetRecordStatus::try_from(status)? == SetRecordStatus::Conflict {
                    if let Some(attempts) = attempts {
                        if attempts > MAX_SET_RECORD_REATTEMPTS {
                            return Err(anyhow!(
                                "Could not set record: revision conflict and max reattempts reached."
                            ));
                        }

                        return Box::pin(
                            self.sync_with_tip()
                                .and_then(|_| self.set_record(data, updated_fields)),
                        )
                        .await;
                    }

                    // Impossible scenario - we cannot get a conflict on a newly created record
                    return Err(anyhow!("Could not set record: conflict detected by the server on newly created record."));
                }

                self.persister.insert_or_update_sync_details(
                    &data_identifier,
                    &SyncDetails {
                        is_local: true,
                        revision: Some(new_revision),
                        record_id: Some(record_id.clone()),
                    },
                )?;

                debug!("Successfully synced (set_record) id {record_id} updated_fields {updated_fields:?} revision {new_revision}");
                Ok(())
            }
            Err(err) => {
                self.outbound_changes.lock().await.remove(&record_id);
                debug!("Could not sync record (set_record) id {record_id} updated_fields {updated_fields:?}: {err:?}");
                Err(err)
            }
        }
    }

    /// Cleans up the cached pending records, by trying to apply each one
    /// This method is especially useful once a client has upgraded, and is now able to
    /// successfully apply a record with a higher schema_version (see [SyncService::apply_changes])
    pub(crate) async fn cleanup(&self) -> Result<()> {
        let pending_records = self
            .persister
            .get_pending_records()
            .map_err(|err| anyhow!("Could not fetch pending records from database: {err:?}"))?;

        let (updatable_records, _) = self.collect_records(&pending_records);

        for record in updatable_records {
            let record_id = record.id.clone();
            if self.apply_record(record).await.is_err() {
                continue;
            }
            self.persister.delete_pending_record(record_id)?;
        }

        Ok(())
    }
}
