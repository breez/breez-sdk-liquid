use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::{persist::Persister, prelude::Signer};

use self::client::SyncerClient;
use self::model::sync::Record;
use self::model::DecryptedRecord;
use self::model::{
    data::{ChainSyncData, ReceiveSyncData, SendSyncData, SyncData},
    sync::ListChangesRequest,
    RecordType, SyncState,
};

pub(crate) mod client;
pub(crate) mod model;

pub(crate) struct SyncService {
    remote_url: String,
    persister: Arc<Persister>,
    signer: Arc<Box<dyn Signer>>,
    client: Box<dyn SyncerClient>,
}

impl SyncService {
    pub(crate) fn new(
        remote_url: String,
        persister: Arc<Persister>,
        signer: Arc<Box<dyn Signer>>,
        client: Box<dyn SyncerClient>,
    ) -> Self {
        Self {
            remote_url,
            persister,
            signer,
            client,
        }
    }

    fn commit_record(
        &self,
        decrypted_record: &DecryptedRecord,
        sync_state: SyncState,
        is_update: bool,
        last_commit_time: Option<u32>,
    ) -> Result<()> {
        match decrypted_record.data.clone() {
            SyncData::Chain(chain_data) => self.persister.commit_incoming_chain_swap(
                &chain_data,
                sync_state,
                is_update,
                last_commit_time,
            )?,
            SyncData::Send(send_data) => self.persister.commit_incoming_send_swap(
                &send_data,
                sync_state,
                is_update,
                last_commit_time,
            )?,
            SyncData::Receive(receive_data) => self.persister.commit_incoming_receive_swap(
                &receive_data,
                sync_state,
                is_update,
                last_commit_time,
            )?,
        }
        Ok(())
    }

    fn load_sync_data(&self, data_id: &str, record_type: RecordType) -> Result<SyncData> {
        let data = match record_type {
            RecordType::Receive => {
                let receive_data: ReceiveSyncData = self
                    .persister
                    .fetch_receive_swap_by_id(data_id)?
                    .ok_or(anyhow!("Could not find Receive swap {data_id}"))?
                    .into();
                SyncData::Receive(receive_data)
            }
            RecordType::Send => {
                let send_data: SendSyncData = self
                    .persister
                    .fetch_send_swap_by_id(data_id)?
                    .ok_or(anyhow!("Could not find Send swap {data_id}"))?
                    .into();
                SyncData::Send(send_data)
            }
            RecordType::Chain => {
                let chain_data: ChainSyncData = self
                    .persister
                    .fetch_chain_swap_by_id(data_id)?
                    .ok_or(anyhow!("Could not find Chain swap {data_id}"))?
                    .into();
                SyncData::Chain(chain_data)
            }
        };
        Ok(data)
    }

    async fn fetch_and_save_records(&self) -> Result<()> {
        log::info!("Initiating record pull");

        let local_latest_revision = self
            .persister
            .get_sync_settings()?
            .latest_revision
            .unwrap_or(0);
        let req = ListChangesRequest::new(local_latest_revision, self.signer.clone())?;
        let incoming_records = self.client.pull(req).await?.changes;

        self.persister.set_incoming_records(&incoming_records)?;
        let remote_latest_revision = incoming_records.last().map(|record| record.revision);
        if let Some(latest_revision) = remote_latest_revision {
            self.persister.set_sync_settings(HashMap::from([(
                "latest_revision",
                latest_revision.to_string(),
            )]))?;
            log::info!(
                "Successfully pulled and persisted records. New latest revision: {latest_revision}"
            );
        } else {
            log::info!("No new records found. Local latest revision: {local_latest_revision}");
        }

        Ok(())
    }

    async fn handle_pull(&self, new_record: Record) -> Result<()> {
        log::info!("Handling pull for record record_id {}", &new_record.id);

        // Step 3: Check whether or not record is applicable (from its schema_version)
        if !new_record.is_applicable()? {
            return Err(anyhow!("Record is not applicable: schema_version too high"));
        }

        // Step 4: Check whether we already have this record, and if the revision is newer
        let maybe_sync_state = self.persister.get_sync_state_by_record_id(&new_record.id)?;
        if let Some(sync_state) = &maybe_sync_state {
            if sync_state.record_revision >= new_record.revision {
                log::info!("Remote record revision is lower or equal to the persisted one. Skipping update.");
                return Ok(());
            }
        }

        // Step 5: Decrypt the incoming record
        let mut decrypted_record = new_record.decrypt(self.signer.clone())?;

        // Step 6: Merge with outgoing records, if present
        let maybe_outgoing_changes = self
            .persister
            .get_sync_outgoing_changes_by_id(&decrypted_record.id)?;

        if let Some(outgoing_changes) = &maybe_outgoing_changes {
            if let Some(updated_fields) = &outgoing_changes.updated_fields {
                let local_data =
                    self.load_sync_data(decrypted_record.data.id(), outgoing_changes.record_type)?;
                decrypted_record.data.merge(&local_data, updated_fields)?;
            }
        }

        // Step 7: Apply the changes and update sync state
        let new_sync_state = SyncState {
            data_id: decrypted_record.data.id().to_string(),
            record_id: decrypted_record.id.clone(),
            record_revision: decrypted_record.revision,
            is_local: maybe_sync_state
                .as_ref()
                .map(|state| state.is_local)
                .unwrap_or(false),
        };
        let is_update = maybe_sync_state.is_some();
        let last_commit_time = maybe_outgoing_changes.map(|details| details.commit_time);
        self.commit_record(
            &decrypted_record,
            new_sync_state,
            is_update,
            last_commit_time,
        )?;

        log::info!(
            "Successfully pulled record record_id {}",
            &decrypted_record.id
        );

        Ok(())
    }

    async fn pull(&self) -> Result<()> {
        // Step 1: Fetch and save incoming records from remote, then update local tip
        self.fetch_and_save_records().await?;

        // Step 2: Grab all pending incoming records from the database, merge them with
        // outgoing if necessary, then apply
        let mut succeded = vec![];
        let incoming_records = self.persister.get_incoming_records()?;
        for new_record in incoming_records {
            let record_id = new_record.id.clone();
            if let Err(err) = self.handle_pull(new_record).await {
                log::debug!("Could not handle incoming record {record_id}: {err:?}");
                continue;
            }
            succeded.push(record_id);
        }

        if !succeded.is_empty() {
            self.persister.remove_incoming_records(succeded)?;
        }

        Ok(())
    }
}
}
