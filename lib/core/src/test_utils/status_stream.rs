#![cfg(test)]

use anyhow::Result;
use boltz_client::boltz;
use std::sync::Arc;

use tokio::sync::{broadcast, watch};

use crate::swapper::{ReconnectHandler, SwapperStatusStream};

pub(crate) struct MockStatusStream {
    pub update_notifier: broadcast::Sender<boltz::Update>,
}

impl MockStatusStream {
    pub(crate) fn new() -> Self {
        let (update_notifier, _) = broadcast::channel::<boltz::Update>(30);

        Self { update_notifier }
    }

    pub(crate) async fn send_mock_update(self: Arc<Self>, update: boltz::Update) -> Result<()> {
        tokio::spawn(async move {
            self.update_notifier.send(update).unwrap();
        })
        .await?;
        Ok(())
    }
}

impl SwapperStatusStream for MockStatusStream {
    fn start(
        self: Arc<Self>,
        _callback: Box<dyn ReconnectHandler>,
        _shutdown: watch::Receiver<()>,
    ) {
    }

    fn track_swap_id(&self, _swap_id: &str) -> Result<()> {
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::Update> {
        self.update_notifier.subscribe()
    }
}
