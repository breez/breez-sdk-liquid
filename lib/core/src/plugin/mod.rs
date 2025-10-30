use std::sync::Arc;
use std::sync::Weak;

use crate::error::*;
use crate::model::*;
use crate::sdk::LiquidSdk;

mod storage;

use anyhow::Context as _;
pub use storage::*;

#[sdk_macros::async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    async fn on_start(&self, plugin_sdk: PluginSdk, storage: PluginStorage);
    async fn on_stop(&self);
}

#[derive(Clone)]
pub struct PluginSdk {
    sdk: Weak<LiquidSdk>,
}

impl PluginSdk {
    pub(crate) fn new(sdk: Weak<LiquidSdk>) -> Self {
        Self { sdk }
    }

    fn sdk(&self) -> anyhow::Result<Arc<LiquidSdk>> {
        self.sdk
            .upgrade()
            .context("Could not reach running SDK instance.")
    }

    pub async fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        self.sdk()?.get_info().await
    }

    pub async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk()?.prepare_send_payment(req).await
    }

    pub async fn send_payment(
        &self,
        req: &SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk()?.send_payment(req).await
    }

    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk()?.prepare_receive_payment(req).await
    }

    pub async fn receive_payment(
        &self,
        req: &ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk()?.receive_payment(req).await
    }

    pub async fn list_payments(
        &self,
        req: &ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.sdk()?.list_payments(req).await
    }

    pub async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        self.sdk()?.add_event_listener(listener).await
    }

    pub async fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        self.sdk()?.remove_event_listener(id).await
    }
}
