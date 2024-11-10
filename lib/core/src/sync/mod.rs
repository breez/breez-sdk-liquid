use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::{persist::Persister, prelude::Signer};

use self::client::SyncerClient;
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

}
