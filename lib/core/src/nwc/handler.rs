pub trait RelayMessageHandler {
    async fn pay_invoice();
    async fn list_transactions();
    async fn get_balance();
}

pub struct BreezRelayMessageHandler {
    sdk: Arc<LiquidSdk>,
    outgoing_sender: mpsc::Sender<String>,
}

impl RelayMessageHandler for BreezRelayMessageHandler {}
