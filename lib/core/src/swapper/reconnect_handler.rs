use std::sync::Arc;

use async_trait::async_trait;
use log::{error, info};

use crate::persist::Persister;

use super::SwapperStatusStream;

#[async_trait]
pub trait ReconnectHandler: Send + Sync {
    async fn on_stream_reconnect(&self);
}

pub(crate) struct SwapperReconnectHandler {
    persister: Arc<Persister>,
    status_stream: Arc<dyn SwapperStatusStream>,
}

impl SwapperReconnectHandler {
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

#[async_trait]
impl ReconnectHandler for SwapperReconnectHandler {
    async fn on_stream_reconnect(&self) {
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
