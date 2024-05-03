use anyhow::{anyhow, Result};
pub(crate) use boltz_client::util::secrets::LBtcReverseRecovery;
use std::sync::{Arc, OnceLock};

use crate::{
    error::{LsSdkError, PaymentError},
    model::*,
    wallet::Wallet,
};

use super::model::Payment;

static LIQUID_SDK_INSTANCE: OnceLock<Arc<Wallet>> = OnceLock::new();

pub fn connect(req: ConnectRequest) -> Result<()> {
    let wallet = Wallet::connect(req)?;
    LIQUID_SDK_INSTANCE.get_or_init(|| wallet);
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
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .prepare_send_payment(req)
}

pub fn send_payment(req: PrepareSendResponse) -> Result<SendPaymentResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .send_payment(&req)
}

pub fn prepare_receive_payment(
    req: PrepareReceiveRequest,
) -> Result<PrepareReceiveResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .prepare_receive_payment(&req)
}

pub fn receive_payment(
    req: PrepareReceiveResponse,
) -> Result<ReceivePaymentResponse, PaymentError> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .receive_payment(&req)
}

pub fn list_payments(with_scan: bool, include_pending: bool) -> Result<Vec<Payment>> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .list_payments(with_scan, include_pending)
}

pub fn recover_funds(recovery: LBtcReverseRecovery) -> Result<String> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .recover_funds(&recovery)
}

pub fn empty_wallet_cache() -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .empty_wallet_cache()
}

pub fn backup() -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .backup()
}

pub fn restore(req: RestoreRequest) -> Result<()> {
    LIQUID_SDK_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| LsSdkError::Generic { err: e.to_string() })?
        .restore(req)
}
