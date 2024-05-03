use std::sync::Arc;

use anyhow::Result;
use breez_liquid_sdk::{error::*, model::*, sdk::LiquidSdk};

pub fn connect(req: ConnectRequest) -> Result<Arc<BindingWallet>, LsSdkError> {
    let ln_sdk = LiquidSdk::connect(req)?;
    Ok(Arc::from(BindingWallet { ln_sdk }))
}

pub struct BindingWallet {
    ln_sdk: Arc<LiquidSdk>,
}

impl BindingWallet {
    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LsSdkError> {
        self.ln_sdk.get_info(req).map_err(Into::into)
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.ln_sdk.prepare_send_payment(req)
    }

    pub fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ln_sdk.send_payment(&req)
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.ln_sdk.prepare_receive_payment(&req)
    }

    pub fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ln_sdk.receive_payment(&req)
    }

    pub fn backup(&self) -> Result<(), LsSdkError> {
        self.ln_sdk.backup().map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<(), LsSdkError> {
        self.ln_sdk.restore(req).map_err(Into::into)
    }
}

uniffi::include_scaffolding!("breez_liquid_sdk");
