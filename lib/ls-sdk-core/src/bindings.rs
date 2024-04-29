use anyhow::{anyhow, Result};
pub(crate) use boltz_client::util::secrets::LBtcReverseRecovery;
use std::sync::{Arc, OnceLock};

use crate::{
    error::PaymentError,
    model::{
        Network, PrepareReceiveRequest, PrepareReceiveResponse, PrepareSendResponse,
        ReceivePaymentResponse, SendPaymentResponse, WalletInfo,
    },
    wallet::Wallet,
};

use super::model::Payment;

static WALLET_INSTANCE: OnceLock<Arc<Wallet>> = OnceLock::new();

pub fn init(mnemonic: String, data_dir: Option<String>, network: Network) -> Result<()> {
    let wallet = Wallet::init(&mnemonic, data_dir, network)?;
    WALLET_INSTANCE.get_or_init(|| wallet);
    Ok(())
}

pub fn get_info(with_scan: bool) -> Result<WalletInfo> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))?
        .get_info(with_scan)
}

pub fn prepare_send_payment(invoice: String) -> Result<PrepareSendResponse, PaymentError> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .prepare_send_payment(&invoice)
}

pub fn send_payment(req: PrepareSendResponse) -> Result<SendPaymentResponse, PaymentError> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .send_payment(&req)
}

pub fn prepare_receive_payment(
    req: PrepareReceiveRequest,
) -> Result<PrepareReceiveResponse, PaymentError> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .prepare_receive_payment(&req)
}

pub fn receive_payment(
    req: PrepareReceiveResponse,
) -> Result<ReceivePaymentResponse, PaymentError> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .receive_payment(&req)
}

pub fn list_payments(with_scan: bool, include_pending: bool) -> Result<Vec<Payment>> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .list_payments(with_scan, include_pending)
}

pub fn recover_funds(recovery: LBtcReverseRecovery) -> Result<String> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .recover_funds(&recovery)
}

pub fn empty_wallet_cache() -> Result<()> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .empty_wallet_cache()
}

pub fn backup() -> Result<()> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .backup()
}

pub fn restore(backup_path: Option<String>) -> Result<()> {
    WALLET_INSTANCE
        .get()
        .ok_or(anyhow!("Not initialized"))
        .map_err(|e| PaymentError::Generic { err: e.to_string() })?
        .restore(backup_path)
}
