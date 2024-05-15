use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Result};

use crate::{error::*, model::*, sdk::LiquidSdk};

use super::model::Payment;

static LIQUID_SDK_INSTANCE: OnceLock<Arc<LiquidSdk>> = OnceLock::new();

pub fn connect(req: ConnectRequest) -> Result<()> {
    let sdk = LiquidSdk::connect(req)?;
    LIQUID_SDK_INSTANCE.get_or_init(|| sdk);
    Ok(())
}

pub fn get_info(req: GetInfoRequest) -> Result<GetInfoResponse> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))?
        .get_info(req)
}

pub fn prepare_send_payment(req: PrepareSendRequest) -> Result<PrepareSendResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .prepare_send_payment(&req)
}

pub fn send_payment(req: PrepareSendResponse) -> Result<SendPaymentResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .send_payment(&req)
}

pub fn prepare_receive_payment(
    req: PrepareReceiveRequest,
) -> Result<PrepareReceiveResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .prepare_receive_payment(&req)
}

pub fn receive_payment(
    req: PrepareReceiveResponse,
) -> Result<ReceivePaymentResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .receive_payment(&req)
}

pub fn list_payments(with_scan: bool, include_pending: bool) -> Result<Vec<Payment>> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .list_payments(with_scan, include_pending)
}

pub fn empty_wallet_cache() -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .empty_wallet_cache()
}

pub fn backup() -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .backup()
}

pub fn restore(req: RestoreRequest) -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LiquidSdkError::Generic { err: e.to_string() })?
        .restore(req)
}
