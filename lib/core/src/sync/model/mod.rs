use serde::{Deserialize, Serialize};

pub(crate) mod sync;

#[derive(Serialize, Deserialize)]
pub(crate) enum ChainSyncData {}

#[derive(Serialize, Deserialize)]
pub(crate) enum SendSyncData {}

#[derive(Serialize, Deserialize)]
pub(crate) enum ReceiveSyncData {}

#[derive(Serialize, Deserialize)]
#[serde(tag = "data_type", content = "data")]
pub(crate) enum SyncData {
    Chain(ChainSyncData),
    Send(SendSyncData),
    Receive(ReceiveSyncData),
}
