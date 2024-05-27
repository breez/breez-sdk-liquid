use std::sync::Arc;

use anyhow::Result;

use crate::{error::*, frb::bridge::StreamSink, model::*, sdk::LiquidSdk};

struct BindingEventListener {
    stream: StreamSink<LiquidSdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: LiquidSdkEvent) {
        let _ = self.stream.add(e);
    }
}

pub fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, LiquidSdkError> {
    let ln_sdk = LiquidSdk::connect(req)?;
    Ok(BindingLiquidSdk { sdk: ln_sdk })
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LiquidSdkError> {
        self.sdk.get_info(req).map_err(Into::into)
    }

    pub fn add_event_listener(
        &self,
        listener: StreamSink<LiquidSdkEvent>,
    ) -> Result<String, LiquidSdkError> {
        self.sdk
            .add_event_listener(Box::new(BindingEventListener { stream: listener }))
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk.prepare_send_payment(&req)
    }

    pub fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.send_payment(&req)
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req)
    }

    pub fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req)
    }

    pub fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments()
    }

    pub fn sync(&self) -> Result<(), LiquidSdkError> {
        self.sdk.sync().map_err(Into::into)
    }

    pub fn empty_wallet_cache(&self) -> Result<(), LiquidSdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self) -> Result<(), LiquidSdkError> {
        self.sdk.backup().map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<(), LiquidSdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }
}
