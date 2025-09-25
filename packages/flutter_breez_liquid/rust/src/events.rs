use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;

pub use breez_sdk_liquid::model::{EventListener, Payment, SdkEvent};

#[frb(mirror(SdkEvent))]
pub enum _SdkEvent {
    PaymentFailed {
        details: Payment,
    },
    PaymentPending {
        details: Payment,
    },
    PaymentRefundable {
        details: Payment,
    },
    PaymentRefunded {
        details: Payment,
    },
    PaymentRefundPending {
        details: Payment,
    },
    PaymentSucceeded {
        details: Payment,
    },
    PaymentWaitingConfirmation {
        details: Payment,
    },
    PaymentWaitingFeeAcceptance {
        details: Payment,
    },
    /// Synced with mempool and onchain data
    Synced,
    /// Synced with real-time data sync
    DataSynced {
        /// Indicates new data was pulled from other instances.
        did_pull_new_records: bool,
    },
}

pub struct BreezEventListener {
    pub stream: StreamSink<SdkEvent>,
}

impl EventListener for BreezEventListener {
    fn on_event(&self, e: SdkEvent) {
        let _ = self.stream.add(e);
    }
}
