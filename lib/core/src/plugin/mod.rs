use std::sync::Arc;
use std::sync::Weak;

use crate::error::*;
use crate::model::*;
use crate::sdk::LiquidSdk;

mod storage;

use anyhow::Context as _;
use sdk_common::prelude::InputType;
pub use storage::*;

#[sdk_macros::async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    async fn on_start(&self, plugin_sdk: Arc<dyn PluginSdk>, storage: Arc<dyn PluginStorage>);
    async fn on_stop(&self);
}

#[sdk_macros::async_trait]
pub trait PluginSdk: Send + Sync {
    async fn get_info(&self) -> SdkResult<GetInfoResponse>;
    async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError>;
    async fn send_payment(
        &self,
        req: &SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError>;
    async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError>;
    async fn receive_payment(
        &self,
        req: &ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError>;
    async fn parse(&self, input: &str) -> Result<InputType, PaymentError>;
    async fn list_payments(&self, req: &ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError>;
    async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String>;
    async fn remove_event_listener(&self, id: String) -> SdkResult<()>;
}

#[derive(Clone)]
pub struct BreezPluginSdk {
    sdk: Weak<LiquidSdk>,
}

impl BreezPluginSdk {
    pub(crate) fn new(sdk: Weak<LiquidSdk>) -> Self {
        Self { sdk }
    }

    fn sdk(&self) -> anyhow::Result<Arc<LiquidSdk>> {
        self.sdk
            .upgrade()
            .context("Could not reach running SDK instance.")
    }
}

#[sdk_macros::async_trait]
impl PluginSdk for BreezPluginSdk {
    async fn get_info(&self) -> SdkResult<GetInfoResponse> {
        self.sdk()?.get_info().await
    }

    async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk()?.prepare_send_payment(req).await
    }

    async fn send_payment(
        &self,
        req: &SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk()?.send_payment(req).await
    }

    async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk()?.prepare_receive_payment(req).await
    }

    async fn receive_payment(
        &self,
        req: &ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk()?.receive_payment(req).await
    }

    async fn list_payments(&self, req: &ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError> {
        self.sdk()?.list_payments(req).await
    }

    async fn parse(&self, input: &str) -> Result<InputType, PaymentError> {
        self.sdk()?.parse(input).await
    }

    async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        self.sdk()?.add_event_listener(listener).await
    }

    async fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        self.sdk()?.remove_event_listener(id).await
    }
}
