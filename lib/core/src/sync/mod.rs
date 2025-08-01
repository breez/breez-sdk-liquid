use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use log::{info, trace, warn};
use model::{PullError, PushError};
use sdk_common::utils::Arc;
use tokio::sync::{broadcast, watch};
use tokio::time::sleep;
use tokio_stream::StreamExt as _;
use tokio_with_wasm::alias as tokio;
use tonic::Streaming;

use self::client::SyncerClient;
use self::model::{data::SyncData, ListChangesRequest, RecordType, SyncState};
use self::model::{ListenChangesRequest, Notification, SyncOutgoingChanges};
use crate::prelude::Swap;
use crate::recover::recoverer::Recoverer;
use crate::sync::model::data::{
    Bolt12OfferSyncData, ChainSyncData, PaymentDetailsSyncData, ReceiveSyncData, SendSyncData,
};
use crate::sync::model::{DecryptionInfo, Record, SetRecordRequest, SetRecordStatus};
use crate::utils;
use crate::{
    persist::{cache::KEY_LAST_DERIVATION_INDEX, Persister},
    prelude::Signer,
};

pub(crate) mod client;
pub(crate) mod model;

const MAX_FAILED_PUSH: u8 = 5;

#[derive(Clone, Debug)]
pub(crate) enum Event {
    SyncedCompleted { data: SyncCompletedData },
}

#[derive(Clone, Debug)]
pub(crate) struct SyncCompletedData {
    pub(crate) pulled_records_count: u32,
    pub(crate) pushed_records_count: u32,
}

pub struct SyncService {
    remote_url: String,
    client_id: String,
    persister: std::sync::Arc<Persister>,
    recoverer: Arc<Recoverer>,
    signer: Arc<Box<dyn Signer>>,
    client: Box<dyn SyncerClient>,
    subscription_notifier: broadcast::Sender<Event>,
}

impl SyncService {
    pub(crate) fn new(
        remote_url: String,
        persister: std::sync::Arc<Persister>,
        recoverer: Arc<Recoverer>,
        signer: Arc<Box<dyn Signer>>,
        client: Box<dyn SyncerClient>,
    ) -> Self {
        let client_id = uuid::Uuid::new_v4().to_string();
        let (subscription_notifier, _) = broadcast::channel::<Event>(30);
        Self {
            client_id,
            remote_url,
            persister,
            recoverer,
            signer,
            client,
            subscription_notifier,
        }
    }

    pub(crate) fn subscribe_events(&self) -> broadcast::Receiver<Event> {
        self.subscription_notifier.subscribe()
    }

    fn check_remote_change(&self) -> Result<()> {
        match self
            .persister
            .get_sync_settings()?
            .remote_url
            .is_some_and(|url| url == self.remote_url)
        {
            true => Ok(()),
            false => self.persister.set_new_remote(self.remote_url.clone()),
        }
    }

    async fn run_event_loop(&self, failed_push_counter: &mut u8) {
        info!("realtime-sync: Running sync event loop");

        if *failed_push_counter > MAX_FAILED_PUSH {
            warn!("realtime-sync: Outward sync has conflicted more than {MAX_FAILED_PUSH} times, wiping database and retrying.");
            if let Err(err) = self.persister.reset_sync() {
                warn!("realtime-sync: Could not wipe sync database: {err:?}");
                return;
            }
            *failed_push_counter = 0;
        }

        let pulled_records_count = match self.pull().await {
            Ok(pulled_records_count) => pulled_records_count,
            Err(err) => {
                warn!("realtime-sync: Could not pull records - {err}");
                return;
            }
        };

        let pushed_records_count = match self.push().await {
            Ok(pushed_records_count) => pushed_records_count,
            Err(err) => {
                if let PushError::ExcessiveRecordConflicts { .. } = err {
                    *failed_push_counter += 1
                }
                warn!("realtime-sync: Could not push records - {err}");
                return;
            }
        };

        if let Err(e) = self.subscription_notifier.send(Event::SyncedCompleted {
            data: SyncCompletedData {
                pulled_records_count,
                pushed_records_count,
            },
        }) {
            warn!("realtime-sync: Could not send sync completed event {e:?}");
        }

        *failed_push_counter = 0;
    }

    async fn new_listener(&self) -> Result<Streaming<Notification>> {
        let req = ListenChangesRequest::new(self.signer.clone())?;
        self.client.listen(req).await
    }

    pub(crate) fn start(self: Arc<Self>, mut shutdown: watch::Receiver<()>) {
        info!("realtime-sync: sync-reconnect start service");
        tokio::spawn(async move {
            if let Err(err) = self.client.connect(self.remote_url.clone()).await {
                warn!("realtime-sync: Could not connect to sync service: {err:?}");
                return;
            }
            if let Err(err) = self.check_remote_change() {
                warn!("realtime-sync: Could not check for remote change: {err:?}");
                return;
            }
            let Ok(mut local_sync_trigger) = self.persister.subscribe_sync_trigger() else {
                warn!("realtime-sync: Could not subscribe to local sync trigger");
                return;
            };

            info!("realtime-sync: Starting real-time sync event loop");
            loop {
                if shutdown.has_changed().unwrap_or(true) {
                    return;
                }
                let mut remote_sync_trigger = match self.new_listener().await {
                    Ok(trigger) => trigger,
                    Err(e) => {
                        warn!(
                            "realtime-sync: new_listener returned error: {e:?} waiting 3 seconds"
                        );
                        sleep(Duration::from_secs(3)).await;
                        continue;
                    }
                };
                let mut failed_push_counter = 0;
                info!("realtime-sync: Starting sync service loop");
                loop {
                    info!("realtime-sync: before tokio_select");
                    tokio::select! {
                        local_event = local_sync_trigger.recv() => {
                          info!("realtime-sync: Received local message");
                          match local_event {
                            Ok(_) => self.run_event_loop(&mut failed_push_counter).await,
                            Err(err) => {
                                warn!("realtime-sync: local trigger received error, probably lagging behind {err:?}");
                            }
                          }
                        }
                        Some(msg) = remote_sync_trigger.next() => {
                          info!("realtime-sync: Received remote message");
                          match msg {
                            Ok(Notification { client_id }) => {
                                if client_id.is_some_and(|id| id == self.client_id) {
                                    info!("realtime-sync: Notification client_id matches our own, skipping update.");
                                    continue;
                                }
                                self.run_event_loop(&mut failed_push_counter).await;
                            }
                            Err(err) => {
                                warn!("realtime-sync: Received status {} from remote, attempting to reconnect.", err.message());
                                break; // break the inner loop which will start the main loop by reconnecting
                            }
                          }
                        },
                        _ = shutdown.changed() => {
                            info!("realtime-sync: Received shutdown signal, exiting realtime sync service loop");
                            if let Err(err) = self.client.disconnect().await {
                                warn!("realtime-sync: Could not disconnect sync service client: {err:?}");
                            };
                            return;
                        }
                    }
                }
            }
        });
    }

    fn commit_record(&self, decryption_info: &DecryptionInfo, swap: Option<Swap>) -> Result<()> {
        let DecryptionInfo {
            record,
            new_sync_state,
            last_commit_time,
            ..
        } = decryption_info;
        match record.data.clone() {
            SyncData::Chain(_) | SyncData::Receive(_) | SyncData::Send(_) => {
                let Some(swap) = swap else {
                    return Err(anyhow!(
                        "Cannot commit a swap-related record without specifying a swap."
                    ));
                };
                self.persister
                    .commit_incoming_swap(&swap, new_sync_state, *last_commit_time)
            }
            SyncData::LastDerivationIndex(new_address_index) => {
                self.persister.commit_incoming_address_index(
                    new_address_index,
                    new_sync_state,
                    *last_commit_time,
                )
            }
            SyncData::PaymentDetails(payment_details_sync_data) => {
                self.persister.commit_incoming_payment_details(
                    payment_details_sync_data.into(),
                    new_sync_state,
                    *last_commit_time,
                )
            }
            SyncData::Bolt12Offer(bolt12_offer_data) => {
                self.persister.commit_incoming_bolt12_offer(
                    bolt12_offer_data.into(),
                    new_sync_state,
                    *last_commit_time,
                )
            }
        }
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
            RecordType::LastDerivationIndex => SyncData::LastDerivationIndex(
                self.persister
                    .get_cached_item(KEY_LAST_DERIVATION_INDEX)?
                    .ok_or(anyhow!("Could not find last derivation index"))?
                    .parse()?,
            ),
            RecordType::PaymentDetails => {
                let payment_details_data: PaymentDetailsSyncData = self
                    .persister
                    .get_payment_details(data_id)?
                    .ok_or(anyhow!("Could not find Payment Details {data_id}"))?
                    .into();
                SyncData::PaymentDetails(payment_details_data)
            }
            RecordType::Bolt12Offer => {
                let bolt12_offer_data: Bolt12OfferSyncData = self
                    .persister
                    .fetch_bolt12_offer_by_id(data_id)?
                    .ok_or(anyhow!("Could not find Bolt12 Offer {data_id}"))?
                    .into();
                SyncData::Bolt12Offer(bolt12_offer_data)
            }
        };
        Ok(data)
    }

    async fn fetch_and_save_records(&self) -> Result<(), PullError> {
        info!("realtime-sync: Initiating record pull");

        let local_latest_revision = self
            .persister
            .get_sync_settings()
            .map_err(PullError::persister)?
            .latest_revision
            .unwrap_or(0);
        let req = ListChangesRequest::new(local_latest_revision, self.signer.clone())
            .map_err(PullError::signing)?;
        let incoming_records = self
            .client
            .pull(req)
            .await
            .map_err(PullError::network)?
            .changes;

        self.persister
            .set_incoming_records(&incoming_records)
            .map_err(PullError::persister)?;
        let remote_latest_revision = incoming_records.last().map(|record| record.revision);
        if let Some(latest_revision) = remote_latest_revision {
            self.persister
                .set_sync_settings(HashMap::from([(
                    "latest_revision",
                    latest_revision.to_string(),
                )]))
                .map_err(PullError::persister)?;
            info!(
                "realtime-sync: Successfully pulled and persisted records. New latest revision: {latest_revision}"
            );
        } else {
            info!("realtime-sync: No new records found. Local latest revision: {local_latest_revision}");
        }

        Ok(())
    }

    async fn handle_decryption(&self, new_record: Record) -> Result<DecryptionInfo, PullError> {
        info!(
            "realtime-sync: Handling decryption for record record_id {}",
            &new_record.id
        );

        // Step 3: Check whether or not record is applicable (from its schema_version)
        if !new_record
            .is_applicable()
            .map_err(PullError::invalid_record_version)?
        {
            return Err(PullError::SchemaNotApplicable);
        }

        // Step 4: Check whether we already have this record, and if the revision is newer
        let maybe_sync_state = self
            .persister
            .get_sync_state_by_record_id(&new_record.id)
            .map_err(PullError::persister)?;
        if let Some(sync_state) = &maybe_sync_state {
            if sync_state.record_revision >= new_record.revision {
                return Err(PullError::AlreadyPersisted);
            }
        }

        // Step 5: Decrypt the incoming record
        let mut decrypted_record = new_record.decrypt(self.signer.clone())?;

        // Step 6: Merge with outgoing records, if present
        let maybe_outgoing_changes = self
            .persister
            .get_sync_outgoing_changes_by_id(&decrypted_record.id)
            .map_err(PullError::persister)?;

        if let Some(outgoing_changes) = &maybe_outgoing_changes {
            if let Some(updated_fields) = &outgoing_changes.updated_fields {
                let local_data = self
                    .load_sync_data(decrypted_record.data.id(), outgoing_changes.record_type)
                    .map_err(PullError::persister)?;
                decrypted_record
                    .data
                    .merge(&local_data, updated_fields)
                    .map_err(PullError::merge)?;
            }
        }

        let new_sync_state = SyncState {
            data_id: decrypted_record.data.id().to_string(),
            record_id: decrypted_record.id.clone(),
            record_revision: decrypted_record.revision,
            is_local: maybe_sync_state
                .as_ref()
                .map(|state| state.is_local)
                .unwrap_or(false),
        };
        let last_commit_time = maybe_outgoing_changes.map(|details| details.commit_time);

        info!(
            "realtime-sync: Successfully decrypted record {}",
            &decrypted_record.id
        );

        Ok(DecryptionInfo {
            new_sync_state,
            record: decrypted_record,
            last_commit_time,
        })
    }

    async fn handle_recovery(
        &self,
        swap_decryption_info: Vec<DecryptionInfo>,
    ) -> Result<Vec<(DecryptionInfo, Swap)>, PullError> {
        let mut succeded = vec![];
        let mut swaps = vec![];

        // Step 1: Convert each record into a swap, if possible
        for decryption_info in swap_decryption_info {
            let record = &decryption_info.record;
            match TryInto::<Swap>::try_into(record.data.clone()) {
                Ok(mut swap) => {
                    // If there is a local swap, take its version to prevent races between the
                    //  recovery of step 2 and other potential changes occurring in parallel
                    //  (e.g. a refund tx being broadcasted)
                    if let Ok(version) = self
                        .persister
                        .fetch_swap_by_id(&swap.id())
                        .map(|s| s.version())
                    {
                        swap.set_version(version);
                    }
                    succeded.push(decryption_info);
                    swaps.push(swap);
                }
                Err(e) => {
                    warn!("realtime-sync: Could not convert sync data to swap: {e}");
                    continue;
                }
            };
        }

        // Step 2: Recover each swap's data from chain
        self.recoverer
            .recover_from_onchain(&mut swaps, None)
            .await
            .map_err(PullError::recovery)?;

        Ok(succeded.into_iter().zip(swaps.into_iter()).collect())
    }

    pub(crate) async fn pull(&self) -> Result<u32, PullError> {
        // Step 1: Fetch and save incoming records from remote, then update local tip
        self.fetch_and_save_records().await?;

        // Step 2: Grab all pending incoming records from the database
        let incoming_records = self
            .persister
            .get_incoming_records()
            .map_err(PullError::persister)?;

        // Step 3: Decrypt all the records, if possible. Filter those whose revision/schema is not
        // applicable
        let mut succeded = vec![];
        let mut decrypted: Vec<DecryptionInfo> = vec![];
        for record in incoming_records {
            let record_id = record.id.clone();
            match self.handle_decryption(record).await {
                Ok(decryption_info) => decrypted.push(decryption_info),
                Err(err) => {
                    warn!("realtime-sync: Could not handle decryption of incoming record {record_id}: {err}");
                    match err {
                        // If we already have this record, it should be cleaned from sync_incoming
                        PullError::AlreadyPersisted => succeded.push(record_id),
                        _ => continue,
                    }
                }
            }
        }

        // Step 4: Split each record into swap, last derivation index and others
        let (decrypted_swap_info, decrypted_non_swap_info): (Vec<_>, Vec<_>) = decrypted
            .into_iter()
            .partition(|result| result.record.data.is_swap());

        let (decrypted_last_derivation_index_info, decrypted_others_info): (Vec<_>, Vec<_>) =
            decrypted_non_swap_info
                .into_iter()
                .partition(|result| matches!(result.record.data, SyncData::LastDerivationIndex(_)));

        // Step 5: Apply derivation index updates (step 6 requires having knowledge of the last derivation index)
        for decryption_info in decrypted_last_derivation_index_info {
            if let Err(e) = self.commit_record(&decryption_info, None) {
                warn!("realtime-sync: Could not commit last derivation index record: {e:?}");
                continue;
            }
            succeded.push(decryption_info.record.id);
        }

        // Step 6: Recover the swap records' data from onchain, and commit it
        for (decryption_info, swap) in self.handle_recovery(decrypted_swap_info).await? {
            if let Err(e) = self.commit_record(&decryption_info, Some(swap)) {
                warn!("realtime-sync: Could not commit swap record: {e:?}");
                continue;
            }
            succeded.push(decryption_info.record.id);
        }

        // Step 7: Commit non-swap-related data
        for decryption_info in decrypted_others_info {
            if let Err(e) = self.commit_record(&decryption_info, None) {
                warn!("realtime-sync: Could not commit generic record: {e:?}");
                continue;
            }
            succeded.push(decryption_info.record.id);
        }

        // Step 8: Clear succeded records
        let succeded_len = succeded.len();
        if succeded_len > 0 {
            info!("realtime-sync: Cleaning {succeded_len} incoming records",);
            self.persister
                .remove_incoming_records(succeded.clone())
                .map_err(PullError::persister)?;
            info!("realtime-sync: Successfully cleaned records");
        }

        Ok(succeded_len as u32)
    }

    async fn handle_push(
        &self,
        record_id: &str,
        data_id: &str,
        record_type: RecordType,
    ) -> Result<(), PushError> {
        info!("realtime-sync: Handling push for record record_id {record_id} data_id {data_id}");

        // Step 1: Get the sync state, if it exists, to compute the revision
        let maybe_sync_state = self
            .persister
            .get_sync_state_by_record_id(record_id)
            .map_err(PushError::persister)?;
        trace!(
            "realtime-sync: Got sync state: {}",
            maybe_sync_state.is_some()
        );
        let record_revision = maybe_sync_state
            .as_ref()
            .map(|s| s.record_revision)
            .unwrap_or(0);
        trace!("realtime-sync: Got revision: {record_revision}");
        let is_local = maybe_sync_state.map(|s| s.is_local).unwrap_or(true);
        trace!("realtime-sync: Got is local: {is_local}");

        // Step 2: Fetch the sync data
        let sync_data = self
            .load_sync_data(data_id, record_type)
            .map_err(PushError::persister)?;
        trace!("realtime-sync: Got sync data: {sync_data:?}");

        // Step 3: Create the record to push outwards
        let record = Record::new(sync_data, record_revision, self.signer.clone())?;
        trace!("realtime-sync: Got record: {record:?}");

        // Step 4: Push the record
        let req = SetRecordRequest::new(
            record,
            utils::now(),
            self.signer.clone(),
            self.client_id.clone(),
        )
        .map_err(PushError::signing)?;
        trace!("realtime-sync: Got set record request: {req:?}");
        let reply = self.client.push(req).await.map_err(PushError::network)?;
        trace!("realtime-sync: Got reply: {reply:?}");

        // Step 5: Check for conflict. If present, skip and retry on the next call
        if reply.status() == SetRecordStatus::Conflict {
            return Err(PushError::RecordConflict);
        }

        // Step 6: Set/update the state revision
        self.persister
            .set_sync_state(SyncState {
                data_id: data_id.to_string(),
                record_id: record_id.to_string(),
                record_revision: reply.new_revision,
                is_local,
            })
            .map_err(PushError::persister)?;

        info!("realtime-sync: Successfully pushed record record_id {record_id}");

        Ok(())
    }

    async fn push(&self) -> Result<u32, PushError> {
        let outgoing_changes = self
            .persister
            .get_sync_outgoing_changes()
            .map_err(PushError::persister)?;
        let total_changes = outgoing_changes.len();

        let mut succeded = vec![];
        let mut recoverable_errors = 0;
        for SyncOutgoingChanges {
            record_id,
            data_id,
            record_type,
            ..
        } in outgoing_changes
        {
            match self.handle_push(&record_id, &data_id, record_type).await {
                Ok(_) => succeded.push(record_id),
                Err(err) => {
                    warn!("realtime-sync: Could not handle push for record {record_id}: {err}");
                    match err {
                        // If we get a record conflict, we want to increment the failed counter.
                        // This way, we will reset the local sync tables after too many failed
                        // attempts (something is probably wrong with the local revisions)
                        PushError::RecordConflict => {}
                        _ => recoverable_errors += 1,
                    }
                    continue;
                }
            }
        }

        let succeded_len = succeded.len();
        if succeded_len > 0 {
            self.persister
                .remove_sync_outgoing_changes(succeded.clone())
                .map_err(PushError::persister)?;
        }

        if succeded_len != total_changes - recoverable_errors {
            return Err(PushError::ExcessiveRecordConflicts {
                succeded: succeded_len,
                total: total_changes,
                recoverable: recoverable_errors,
            });
        }

        Ok(succeded_len as u32)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use sdk_common::utils::Arc;
    use std::collections::HashMap;

    use crate::{
        persist::{cache::KEY_LAST_DERIVATION_INDEX, Persister},
        prelude::{Direction, PaymentState, Signer},
        sync::model::{data::LAST_DERIVATION_INDEX_DATA_ID, SyncState},
        test_utils::{
            chain_swap::new_chain_swap,
            persist::{create_persister, new_receive_swap, new_send_swap},
            recover::new_recoverer,
            swapper::MockSwapper,
            sync::{
                new_chain_sync_data, new_receive_sync_data, new_send_sync_data, new_sync_service,
            },
            wallet::{MockSigner, MockWallet},
        },
    };

    use super::model::{data::SyncData, Record, RecordType};

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_incoming_sync_create_and_update() -> Result<()> {
        create_persister!(persister);
        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
        let swapper = Arc::new(MockSwapper::new());
        let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
        let recoverer = Arc::new(new_recoverer(
            signer.clone(),
            swapper.clone(),
            onchain_wallet.clone(),
            persister.clone(),
        )?);

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

        let (incoming_tx, _outgoing_records, sync_service) =
            new_sync_service(persister.clone(), recoverer, signer.clone())?;

        for record in incoming_records {
            incoming_tx.send(record).await?;
        }
        sync_service.pull().await?;

        if let Some(receive_swap) = persister.fetch_receive_swap_by_id(sync_data[0].id())? {
            assert!(receive_swap.description.is_none());
            assert!(receive_swap.payment_hash.is_none());
        } else {
            return Err(anyhow!("Receive swap not found"));
        }
        if let Some(send_swap) = persister.fetch_send_swap_by_id(sync_data[1].id())? {
            assert!(send_swap.preimage.is_none());
            assert!(send_swap.description.is_none());
            assert!(send_swap.payment_hash.is_none());
        } else {
            return Err(anyhow!("Send swap not found"));
        }
        if let Some(chain_swap) = persister.fetch_chain_swap_by_id(sync_data[2].id())? {
            assert!(chain_swap.claim_address.is_none());
            assert!(chain_swap.description.is_none());
            assert!(chain_swap.accept_zero_conf.eq(&true));
        } else {
            return Err(anyhow!("Chain swap not found"));
        }

        let new_preimage = Some("preimage".to_string());
        let new_accept_zero_conf = false;
        let sync_data = vec![
            SyncData::Send(new_send_sync_data(new_preimage.clone())),
            SyncData::Chain(new_chain_sync_data(Some(new_accept_zero_conf))),
        ];
        let incoming_records = vec![
            Record::new(sync_data[0].clone(), 4, signer.clone())?,
            Record::new(sync_data[1].clone(), 5, signer.clone())?,
        ];

        for record in incoming_records {
            incoming_tx.send(record).await?;
        }
        sync_service.pull().await?;

        if let Some(send_swap) = persister.fetch_send_swap_by_id(sync_data[0].id())? {
            assert_eq!(send_swap.preimage, new_preimage);
        } else {
            return Err(anyhow!("Send swap not found"));
        }
        if let Some(chain_swap) = persister.fetch_chain_swap_by_id(sync_data[1].id())? {
            assert_eq!(chain_swap.accept_zero_conf, new_accept_zero_conf);
        } else {
            return Err(anyhow!("Chain swap not found"));
        }

        Ok(())
    }

    fn get_outgoing_record<'a>(
        persister: std::sync::Arc<Persister>,
        outgoing: &'a HashMap<String, Record>,
        data_id: &str,
        record_type: RecordType,
    ) -> Result<&'a Record> {
        let record_id = Record::get_id_from_record_type(record_type, data_id);
        let sync_state = persister
            .get_sync_state_by_record_id(&record_id)?
            .ok_or(anyhow::anyhow!("Expected existing swap state"))?;
        let Some(record) = outgoing.get(&sync_state.record_id) else {
            return Err(anyhow::anyhow!(
                "Expecting existing record in client's outgoing list"
            ));
        };
        Ok(record)
    }

    #[sdk_macros::async_test_all]
    async fn test_outgoing_sync() -> Result<()> {
        create_persister!(persister);
        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
        let swapper = Arc::new(MockSwapper::new());
        let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
        let recoverer = Arc::new(new_recoverer(
            signer.clone(),
            swapper.clone(),
            onchain_wallet.clone(),
            persister.clone(),
        )?);

        let (_incoming_tx, outgoing_records, sync_service) =
            new_sync_service(persister.clone(), recoverer, signer.clone())?;

        // Test insert
        persister.insert_or_update_receive_swap(&new_receive_swap(None, None))?;
        persister.insert_or_update_send_swap(&new_send_swap(None, None))?;
        persister.insert_or_update_chain_swap(&new_chain_swap(
            Direction::Incoming,
            None,
            true,
            None,
            false,
            false,
            None,
        ))?;

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;
        assert_eq!(outgoing.len(), 3);
        drop(outgoing);

        // Test conflict
        let swap = new_receive_swap(None, None);
        persister.insert_or_update_receive_swap(&swap)?;

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;
        assert_eq!(outgoing.len(), 4);
        let record =
            get_outgoing_record(persister.clone(), &outgoing, &swap.id, RecordType::Receive)?;
        persister.set_sync_state(SyncState {
            data_id: swap.id.clone(),
            record_id: record.id.clone(),
            record_revision: 90, // Set a wrong record revision
            is_local: true,
        })?;
        drop(outgoing);

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;
        assert_eq!(outgoing.len(), 4); // No records were added
        drop(outgoing);

        // Test update before push
        let swap = new_send_swap(None, None);
        persister.insert_or_update_send_swap(&swap)?;
        let new_preimage = Some("new-preimage");
        persister.try_handle_send_swap_update(
            &swap.id,
            PaymentState::Pending,
            new_preimage,
            None,
            None,
        )?;

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;

        let record = get_outgoing_record(persister.clone(), &outgoing, &swap.id, RecordType::Send)?;
        let decrypted_record = record.clone().decrypt(signer.clone())?;
        assert_eq!(decrypted_record.data.id(), &swap.id);
        match decrypted_record.data {
            SyncData::Send(data) => {
                assert_eq!(data.preimage, new_preimage.map(|p| p.to_string()));
            }
            _ => {
                return Err(anyhow::anyhow!("Unexpected sync data type received."));
            }
        }
        drop(outgoing);

        // Test update after push
        let swap = new_send_swap(None, None);
        persister.insert_or_update_send_swap(&swap)?;

        sync_service.push().await?;

        let new_preimage = Some("new-preimage");
        persister.try_handle_send_swap_update(
            &swap.id,
            PaymentState::Pending,
            new_preimage,
            None,
            None,
        )?;

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;
        let record = get_outgoing_record(persister.clone(), &outgoing, &swap.id, RecordType::Send)?;
        let decrypted_record = record.clone().decrypt(signer.clone())?;
        assert_eq!(decrypted_record.data.id(), &swap.id);
        match decrypted_record.data {
            SyncData::Send(data) => {
                assert_eq!(data.preimage, new_preimage.map(|p| p.to_string()),);
            }
            _ => {
                return Err(anyhow::anyhow!("Unexpected sync data type received."));
            }
        }

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_sync_clean() -> Result<()> {
        create_persister!(persister);
        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
        let swapper = Arc::new(MockSwapper::new());
        let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
        let recoverer = Arc::new(new_recoverer(
            signer.clone(),
            swapper.clone(),
            onchain_wallet.clone(),
            persister.clone(),
        )?);

        let (incoming_tx, _outgoing_records, sync_service) =
            new_sync_service(persister.clone(), recoverer, signer.clone())?;

        // Clean incoming
        let record = Record::new(
            SyncData::Receive(new_receive_sync_data()),
            1,
            signer.clone(),
        )?;
        incoming_tx.send(record).await?;
        sync_service.pull().await?;

        let incoming_records = persister.get_incoming_records()?;
        assert_eq!(incoming_records.len(), 0); // Records have been cleaned

        let mut inapplicable_record = Record::new(
            SyncData::Receive(new_receive_sync_data()),
            2,
            signer.clone(),
        )?;
        inapplicable_record.schema_version = "9.9.9".to_string();
        incoming_tx.send(inapplicable_record).await?;
        sync_service.pull().await?;

        let incoming_records = persister.get_incoming_records()?;
        assert_eq!(incoming_records.len(), 1); // Inapplicable records are stored for later

        // Clean outgoing
        let swap = new_send_swap(None, None);
        persister.insert_or_update_send_swap(&swap)?;
        let outgoing_changes = persister.get_sync_outgoing_changes()?;
        assert_eq!(outgoing_changes.len(), 1); // Changes have been set

        sync_service.push().await?;
        let outgoing_changes = persister.get_sync_outgoing_changes()?;
        assert_eq!(outgoing_changes.len(), 0); // Changes have been cleaned

        let new_preimage = Some("new-preimage");
        persister.try_handle_send_swap_update(
            &swap.id,
            PaymentState::Pending,
            new_preimage,
            None,
            None,
        )?;
        let outgoing_changes = persister.get_sync_outgoing_changes()?;
        assert_eq!(outgoing_changes.len(), 1); // Changes have been set

        sync_service.push().await?;
        let outgoing_changes = persister.get_sync_outgoing_changes()?;
        assert_eq!(outgoing_changes.len(), 0); // Changes have been cleaned

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_last_derivation_index_update() -> Result<()> {
        create_persister!(persister);
        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
        let swapper = Arc::new(MockSwapper::new());
        let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
        let recoverer = Arc::new(new_recoverer(
            signer.clone(),
            swapper.clone(),
            onchain_wallet.clone(),
            persister.clone(),
        )?);

        let (incoming_tx, outgoing_records, sync_service) =
            new_sync_service(persister.clone(), recoverer, signer.clone())?;

        // Check pull
        assert_eq!(persister.get_cached_item(KEY_LAST_DERIVATION_INDEX)?, None);

        let new_last_derivation_index = 10;
        let data = SyncData::LastDerivationIndex(new_last_derivation_index);
        incoming_tx
            .send(Record::new(data, 0, signer.clone())?)
            .await?;

        sync_service.pull().await?;

        assert_eq!(
            persister.get_cached_item(KEY_LAST_DERIVATION_INDEX)?,
            Some(new_last_derivation_index.to_string())
        );

        // Check push
        let new_last_derivation_index = 20;
        persister.set_last_derivation_index(new_last_derivation_index)?;

        sync_service.push().await?;

        let outgoing = outgoing_records.lock().await;
        let record = get_outgoing_record(
            persister.clone(),
            &outgoing,
            LAST_DERIVATION_INDEX_DATA_ID,
            RecordType::LastDerivationIndex,
        )?;
        let decrypted_record = record.clone().decrypt(signer.clone())?;
        match decrypted_record.data {
            SyncData::LastDerivationIndex(last_derivation_index) => {
                assert_eq!(last_derivation_index, new_last_derivation_index);
            }
            _ => {
                return Err(anyhow::anyhow!("Unexpected sync data type received."));
            }
        }

        // Check pull with merge
        let new_local_last_derivation_index = 30;
        persister.set_last_derivation_index(new_local_last_derivation_index)?;

        let new_remote_last_derivation_index = 25;
        let data = SyncData::LastDerivationIndex(new_remote_last_derivation_index);
        incoming_tx
            .send(Record::new(data, 0, signer.clone())?)
            .await?;

        sync_service.pull().await?;

        // Newer one is persisted (local > remote)
        assert_eq!(
            persister.get_cached_item(KEY_LAST_DERIVATION_INDEX)?,
            Some(new_local_last_derivation_index.to_string())
        );

        let new_local_last_derivation_index = 35;
        persister.set_last_derivation_index(new_local_last_derivation_index)?;

        let new_remote_last_derivation_index = 40;
        let data = SyncData::LastDerivationIndex(new_remote_last_derivation_index);
        incoming_tx
            .send(Record::new(data, 2, signer.clone())?)
            .await?;

        sync_service.pull().await?;

        // Newer one is persisted (remote > local)
        assert_eq!(
            persister.get_cached_item(KEY_LAST_DERIVATION_INDEX)?,
            Some(new_remote_last_derivation_index.to_string())
        );

        Ok(())
    }
}
