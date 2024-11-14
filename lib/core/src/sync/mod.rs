use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::sync::model::sync::{Record, SetRecordRequest, SetRecordStatus};
use crate::utils;
use crate::{persist::Persister, prelude::Signer};

use self::client::SyncerClient;
use self::model::{
    data::{ChainSyncData, ReceiveSyncData, SendSyncData, SyncData},
    sync::ListChangesRequest,
    RecordType, SyncState,
};
use self::model::{DecryptedRecord, SyncOutgoingChanges};

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

    async fn handle_push(
        &self,
        record_id: &str,
        data_id: &str,
        record_type: RecordType,
    ) -> Result<()> {
        log::info!("Handling push for record record_id {record_id} data_id {data_id}");

        // Step 1: Get the sync state, if it exists, to compute the revision
        let maybe_sync_state = self.persister.get_sync_state_by_record_id(record_id)?;
        let record_revision = maybe_sync_state
            .as_ref()
            .map(|s| s.record_revision)
            .unwrap_or(0);
        let is_local = maybe_sync_state.map(|s| s.is_local).unwrap_or(true);

        // Step 2: Fetch the sync data
        let sync_data = self.load_sync_data(data_id, record_type)?;

        // Step 3: Create the record to push outwards
        let record = Record::new(sync_data, record_revision, self.signer.clone())?;

        // Step 4: Push the record
        let req = SetRecordRequest::new(record, utils::now(), self.signer.clone())?;
        let reply = self.client.push(req).await?;

        // Step 5: Check for conflict. If present, skip and retry on the next call
        if reply.status() == SetRecordStatus::Conflict {
            return Err(anyhow!(
                "Got conflict status when attempting to push record"
            ));
        }

        // Step 6: Set/update the state revision
        self.persister.set_sync_state(SyncState {
            data_id: data_id.to_string(),
            record_id: record_id.to_string(),
            record_revision: reply.new_revision,
            is_local,
        })?;

        log::info!("Successfully pushed record record_id {record_id}");

        Ok(())
    }

    async fn push(&self) -> Result<()> {
        let outgoing_changes = self.persister.get_sync_outgoing_changes()?;

        let mut succeded = vec![];
        for SyncOutgoingChanges {
            record_id,
            data_id,
            record_type,
            ..
        } in outgoing_changes
        {
            if let Err(err) = self.handle_push(&record_id, &data_id, record_type).await {
                log::debug!("Could not handle push for record {record_id}: {err:?}");
                continue;
            }
            succeded.push(record_id);
        }

        if !succeded.is_empty() {
            self.persister.remove_sync_outgoing_changes(succeded)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use std::sync::Arc;
    use tokio::sync::mpsc;

    use crate::{
        prelude::Signer,
        test_utils::{
            persist::new_persister,
            sync::{
                new_chain_sync_data, new_receive_sync_data, new_send_sync_data, MockSyncerClient,
            },
            wallet::MockSigner,
        },
    };

    use super::{
        model::{data::SyncData, sync::Record},
        SyncService,
    };

    #[tokio::test]
    async fn test_incoming_sync_create_and_update() -> Result<()> {
        let (_temp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);

        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()));

        let sync_data = vec![
            SyncData::Receive(new_receive_sync_data()),
            SyncData::Send(new_send_sync_data(None)),
            SyncData::Chain(new_chain_sync_data(None)),
        ];
        let incoming_records = vec![
            Record::new(sync_data[0].clone(), 1, signer.clone())?,
            Record::new(sync_data[1].clone(), 2, signer.clone())?,
            Record::new(sync_data[2].clone(), 3, signer.clone())?,
        ];

        let (incoming_tx, incoming_rx) = mpsc::channel::<Record>(10);
        let client = Box::new(MockSyncerClient::new(incoming_rx));
        let sync_service =
            SyncService::new("".to_string(), persister.clone(), signer.clone(), client);

        for record in incoming_records {
            incoming_tx.send(record).await?;
        }
        sync_service.pull().await?;

        if let Some(receive_swap) = persister.fetch_receive_swap_by_id(&sync_data[0].id())? {
            assert!(receive_swap.description.is_none());
            assert!(receive_swap.payment_hash.is_none());
        } else {
            return Err(anyhow!("Receive swap not found"));
        }
        if let Some(send_swap) = persister.fetch_send_swap_by_id(&sync_data[1].id())? {
            assert!(send_swap.preimage.is_none());
            assert!(send_swap.description.is_none());
            assert!(send_swap.payment_hash.is_none());
        } else {
            return Err(anyhow!("Send swap not found"));
        }
        if let Some(chain_swap) = persister.fetch_chain_swap_by_id(&sync_data[2].id())? {
            assert!(chain_swap.claim_address.is_none());
            assert!(chain_swap.description.is_none());
            assert!(chain_swap.accept_zero_conf.eq(&true));
        } else {
            return Err(anyhow!("Chain swap not found"));
        }

        let new_preimage = Some("preimage".to_string());
        let new_accept_zero_conf = false;
        let new_server_lockup_tx_id = Some("server_lockup_tx_id".to_string());
        let sync_data = vec![
            SyncData::Send(new_send_sync_data(new_preimage.clone())),
            SyncData::Chain(new_chain_sync_data(Some(new_accept_zero_conf))),
        ];
        let incoming_records = vec![
            Record::new(sync_data[0].clone(), 4, signer.clone())?,
            Record::new(sync_data[1].clone(), 5, signer.clone())?,
            Record::new(sync_data[2].clone(), 6, signer.clone())?,
        ];

        for record in incoming_records {
            incoming_tx.send(record).await?;
        }
        sync_service.pull().await?;

        if let Some(send_swap) = persister.fetch_send_swap_by_id(&sync_data[1].id())? {
            assert_eq!(send_swap.preimage, new_preimage);
        } else {
            return Err(anyhow!("Send swap not found"));
        }
        if let Some(chain_swap) = persister.fetch_chain_swap_by_id(&sync_data[2].id())? {
            assert_eq!(chain_swap.accept_zero_conf, new_accept_zero_conf);
            assert_eq!(chain_swap.server_lockup_tx_id, new_server_lockup_tx_id);
        } else {
            return Err(anyhow!("Chain swap not found"));
        }

        Ok(())
    }
}
