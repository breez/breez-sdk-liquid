use log::{error, info};
use maybe_sync::{MaybeSend, MaybeSync};

use crate::persist::Persister;

use sdk_common::utils::Arc;

use super::SwapperStatusStream;

#[sdk_macros::async_trait]
pub trait SubscriptionHandler: MaybeSend + MaybeSync {
    async fn subscribe_swaps(&self);
}

#[derive(Clone)]
pub(crate) struct SwapperSubscriptionHandler {
    persister: Arc<Persister>,
    status_stream: Arc<dyn SwapperStatusStream>,
}

impl SwapperSubscriptionHandler {
    pub(crate) fn new(
        persister: Arc<Persister>,
        status_stream: Arc<dyn SwapperStatusStream>,
    ) -> Self {
        Self {
            persister,
            status_stream,
        }
    }
}

#[sdk_macros::async_trait]
impl SubscriptionHandler for SwapperSubscriptionHandler {
    async fn subscribe_swaps(&self) {
        match self.persister.list_ongoing_swaps() {
            Ok(initial_ongoing_swaps) => {
                info!(
                    "On stream reconnection, got {} initial ongoing swaps",
                    initial_ongoing_swaps.len()
                );
                for ongoing_swap in initial_ongoing_swaps {
                    match self.status_stream.track_swap_id(&ongoing_swap.id()) {
                        Ok(_) => info!("Tracking ongoing swap: {}", ongoing_swap.id()),
                        Err(e) => error!("Failed to track ongoing swap: {e:?}"),
                    }
                }
            }
            Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
        }
    }
}
