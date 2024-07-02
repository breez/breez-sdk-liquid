#![cfg(test)]

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::boltzv2;
use std::sync::Arc;

use tokio::sync::{broadcast, watch};

use crate::swapper::{ReconnectHandler, SwapperStatusStream};

pub(crate) struct MockStatusStream {
    pub update_notifier: broadcast::Sender<boltzv2::Update>,
}

impl MockStatusStream {
    pub(crate) fn new() -> Self {
        let (update_notifier, _) = broadcast::channel::<boltzv2::Update>(30);

        Self { update_notifier }
    }

    pub(crate) async fn send_mock_update(self: Arc<Self>, update: boltzv2::Update) -> Result<()> {
        tokio::spawn(async move {
            self.update_notifier.send(update).unwrap();
        })
        .await?;
        Ok(())
    }
}

#[async_trait]
impl SwapperStatusStream for MockStatusStream {
    async fn start(
        self: Arc<Self>,
        _callback: Box<dyn ReconnectHandler>,
        _shutdown: watch::Receiver<()>,
    ) {
    }

    fn track_swap_id(&self, _swap_id: &str) -> Result<()> {
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltzv2::Update> {
        self.update_notifier.subscribe()
    }
}
