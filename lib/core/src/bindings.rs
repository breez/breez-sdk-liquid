use crate::{error::*, frb::bridge::StreamSink, model::*, sdk::LiquidSdk};
use anyhow::Result;
use flutter_rust_bridge::frb;
use std::sync::Arc;

struct BindingEventListener {
    stream: StreamSink<LiquidSdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: LiquidSdkEvent) {
        let _ = self.stream.add(e);
    }
}

pub async fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, LiquidSdkError> {
    let ln_sdk = LiquidSdk::connect(req).await?;
    Ok(BindingLiquidSdk { sdk: ln_sdk })
}

pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub async fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LiquidSdkError> {
        self.sdk.get_info(req).await.map_err(Into::into)
    }

    pub async fn add_event_listener(
        &self,
        listener: StreamSink<LiquidSdkEvent>,
    ) -> Result<String, LiquidSdkError> {
        self.sdk
            .add_event_listener(Box::new(BindingEventListener { stream: listener }))
            .await
    }

    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk.prepare_send_payment(&req).await
    }

    pub async fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.send_payment(&req).await
    }

    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req).await
    }

    pub async fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req).await
    }

    pub async fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments().await
    }

    pub async fn sync(&self) -> Result<(), LiquidSdkError> {
        self.sdk.sync().await.map_err(Into::into)
    }

    #[frb(sync)]
    pub fn empty_wallet_cache(&self) -> Result<(), LiquidSdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    #[frb(sync)]
    pub fn backup(&self, req: BackupRequest) -> Result<(), LiquidSdkError> {
        self.sdk.backup(req).map_err(Into::into)
    }

    #[frb(sync)]
    pub fn restore(&self, req: RestoreRequest) -> Result<(), LiquidSdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub async fn disconnect(&self) -> Result<(), LiquidSdkError> {
        self.sdk.disconnect().await
    }
}
