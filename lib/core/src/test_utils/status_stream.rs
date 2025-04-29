use anyhow::Result;
use boltz_client::boltz;
use sdk_common::utils::Arc;

use tokio::sync::{broadcast, watch};
use tokio_with_wasm::alias as tokio;

use crate::swapper::{SubscriptionHandler, SwapperStatusStream};

pub(crate) struct MockStatusStream {
    pub update_notifier: broadcast::Sender<boltz::SwapStatus>,
    pub invoice_request_notifier: broadcast::Sender<boltz::InvoiceRequest>,
}

impl MockStatusStream {
    pub(crate) fn new() -> Self {
        let (update_notifier, _) = broadcast::channel::<boltz::SwapStatus>(30);
        let (invoice_request_notifier, _) = broadcast::channel::<boltz::InvoiceRequest>(30);

        Self {
            update_notifier,
            invoice_request_notifier,
        }
    }

    pub(crate) async fn send_mock_update(self: Arc<Self>, update: boltz::SwapStatus) -> Result<()> {
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
        _callback: Box<dyn SubscriptionHandler>,
        _shutdown: watch::Receiver<()>,
    ) {
    }

    fn track_swap_id(&self, _swap_id: &str) -> Result<()> {
        Ok(())
    }

    fn track_offer(&self, _offer: &str, _signature: &str) -> Result<()> {
        Ok(())
    }

    fn send_invoice_created(&self, _id: &str, _invoice: &str) -> Result<()> {
        Ok(())
    }

    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::SwapStatus> {
        self.update_notifier.subscribe()
    }

    fn subscribe_invoice_requests(&self) -> broadcast::Receiver<boltz::InvoiceRequest> {
        self.invoice_request_notifier.subscribe()
    }
}
