//! Dart / flutter bindings

use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::frb;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use sdk_common::prelude::{LnUrlPayRequest, LnUrlWithdrawRequest};

use crate::model::lnurl::LnUrlPayResult;
use crate::{error::*, frb_generated::StreamSink, model::*, sdk::LiquidSdk, *};

pub struct BindingEventListener {
    pub stream: StreamSink<LiquidSdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: LiquidSdkEvent) {
        let _ = self.stream.add(e);
    }
}

struct DartBindingLogger {
    log_stream: StreamSink<LogEntry>,
}

impl DartBindingLogger {
    fn init(log_stream: StreamSink<LogEntry>) -> Result<(), SetLoggerError> {
        let binding_logger: DartBindingLogger = DartBindingLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
    }
}

impl log::Log for DartBindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.log_stream.add(LogEntry {
                line: record.args().to_string(),
                level: record.level().as_str().to_string(),
            });
        }
    }
    fn flush(&self) {}
}

pub async fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, LiquidSdkError> {
    let ln_sdk = LiquidSdk::connect(req).await?;
    Ok(BindingLiquidSdk { sdk: ln_sdk })
}

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    DartBindingLogger::init(s).map_err(|_| LiquidSdkError::Generic {
        err: "Log stream already created".into(),
    })?;
    Ok(())
}

#[frb(sync)]
pub fn default_config(network: LiquidSdkNetwork) -> Config {
    LiquidSdk::default_config(network)
}

pub async fn parse(input: String) -> Result<InputType, PaymentError> {
    LiquidSdk::parse(&input).await
}

#[frb(sync)]
pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub async fn get_info(&self) -> Result<GetInfoResponse, LiquidSdkError> {
        self.sdk.get_info().await.map_err(Into::into)
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

    pub async fn lnurl_pay(
        &self,
        req: LnUrlPayRequest,
    ) -> Result<LnUrlPayResult, duplicates::LnUrlPayError> {
        self.sdk.lnurl_pay(req).await.map_err(Into::into)
    }

    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<duplicates::LnUrlWithdrawResult, duplicates::LnUrlWithdrawError> {
        self.sdk
            .lnurl_withdraw(req)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<duplicates::LnUrlCallbackStatus, duplicates::LnUrlAuthError> {
        self.sdk
            .lnurl_auth(req_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
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

/// External structs that cannot be mirrored for FRB, so are therefore duplicated instead
pub mod duplicates {
    use sdk_common::prelude::*;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::error::PaymentError;

    #[derive(Clone, Debug, Error)]
    pub enum LnUrlPayError {
        /// This error is raised when attempting to pay an invoice that has already being paid.
        #[error("Invoice already paid")]
        AlreadyPaid,

        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the amount from the parsed invoice is not set.
        #[error("Invalid amount: {err}")]
        InvalidAmount { err: String },

        /// This error is raised when the lightning invoice cannot be parsed.
        #[error("Invalid invoice: {err}")]
        InvalidInvoice { err: String },

        /// This error is raised when the lightning invoice is for a different Bitcoin network.
        #[error("Invalid network: {err}")]
        InvalidNetwork { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when the lightning invoice has passed it's expiry time.
        #[error("Invoice expired: {err}")]
        InvoiceExpired { err: String },

        /// This error is raised when attempting to make a payment by the node fails.
        #[error("Payment failed: {err}")]
        PaymentFailed { err: String },

        /// This error is raised when attempting to make a payment takes too long.
        #[error("Payment timeout: {err}")]
        PaymentTimeout { err: String },

        /// This error is raised when no route can be found when attempting to make a
        /// payment by the node.
        #[error("Route not found: {err}")]
        RouteNotFound { err: String },

        /// This error is raised when the route is considered too expensive when
        /// attempting to make a payment by the node.
        #[error("Route too expensive: {err}")]
        RouteTooExpensive { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }
    impl From<sdk_common::prelude::LnUrlPayError> for LnUrlPayError {
        fn from(value: sdk_common::prelude::LnUrlPayError) -> Self {
            match value {
                sdk_common::prelude::LnUrlPayError::AlreadyPaid => Self::AlreadyPaid,
                sdk_common::prelude::LnUrlPayError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlPayError::InvalidAmount { err } => {
                    Self::InvalidAmount { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidInvoice { err } => {
                    Self::InvalidInvoice { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidNetwork { err } => {
                    Self::InvalidNetwork { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidUri { err } => Self::InvalidUri { err },
                sdk_common::prelude::LnUrlPayError::InvoiceExpired { err } => {
                    Self::InvoiceExpired { err }
                }
                sdk_common::prelude::LnUrlPayError::PaymentFailed { err } => {
                    Self::PaymentFailed { err }
                }
                sdk_common::prelude::LnUrlPayError::PaymentTimeout { err } => {
                    Self::PaymentTimeout { err }
                }
                sdk_common::prelude::LnUrlPayError::RouteNotFound { err } => {
                    Self::RouteNotFound { err }
                }
                sdk_common::prelude::LnUrlPayError::RouteTooExpensive { err } => {
                    Self::RouteTooExpensive { err }
                }
                sdk_common::prelude::LnUrlPayError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    impl From<PaymentError> for sdk_common::prelude::LnUrlPayError {
        fn from(value: PaymentError) -> Self {
            Self::Generic {
                err: format!("{value}"),
            }
        }
    }

    #[derive(Debug, Error)]
    pub enum LnUrlWithdrawError {
        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the amount is zero or the amount does not cover
        /// the cost to open a new channel.
        #[error("Invalid amount: {err}")]
        InvalidAmount { err: String },

        /// This error is raised when the lightning invoice cannot be parsed.
        #[error("Invalid invoice: {err}")]
        InvalidInvoice { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when no routing hints were able to be added to the invoice
        /// while trying to receive a payment.
        #[error("No routing hints: {err}")]
        InvoiceNoRoutingHints { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }

    impl From<sdk_common::prelude::LnUrlWithdrawError> for LnUrlWithdrawError {
        fn from(value: sdk_common::prelude::LnUrlWithdrawError) -> Self {
            match value {
                sdk_common::prelude::LnUrlWithdrawError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlWithdrawError::InvalidAmount { err } => {
                    Self::InvalidAmount { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvalidInvoice { err } => {
                    Self::InvalidInvoice { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvalidUri { err } => {
                    Self::InvalidUri { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvoiceNoRoutingHints { err } => {
                    Self::InvoiceNoRoutingHints { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    impl From<PaymentError> for sdk_common::prelude::LnUrlWithdrawError {
        fn from(value: PaymentError) -> Self {
            Self::Generic {
                err: format!("{value}"),
            }
        }
    }

    #[derive(Clone, Serialize)]
    pub enum LnUrlWithdrawResult {
        Ok { data: LnUrlWithdrawSuccessData },
        ErrorStatus { data: LnUrlErrorData },
    }
    impl From<sdk_common::prelude::LnUrlWithdrawResult> for LnUrlWithdrawResult {
        fn from(value: sdk_common::prelude::LnUrlWithdrawResult) -> Self {
            match value {
                sdk_common::prelude::LnUrlWithdrawResult::Ok { data } => {
                    Self::Ok { data: data.into() }
                }
                sdk_common::prelude::LnUrlWithdrawResult::ErrorStatus { data } => {
                    Self::ErrorStatus { data }
                }
            }
        }
    }

    #[derive(Clone, Deserialize, Debug, Serialize)]
    pub struct LnUrlWithdrawSuccessData {
        pub invoice: LNInvoice,
    }
    impl From<sdk_common::prelude::LnUrlWithdrawSuccessData> for LnUrlWithdrawSuccessData {
        fn from(value: sdk_common::prelude::LnUrlWithdrawSuccessData) -> Self {
            Self {
                invoice: value.invoice,
            }
        }
    }

    #[derive(Debug, Error)]
    pub enum LnUrlAuthError {
        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }
    impl From<sdk_common::prelude::LnUrlAuthError> for LnUrlAuthError {
        fn from(value: prelude::LnUrlAuthError) -> Self {
            match value {
                sdk_common::prelude::LnUrlAuthError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlAuthError::InvalidUri { err } => Self::InvalidUri { err },
                sdk_common::prelude::LnUrlAuthError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    /// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
    ///
    /// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
    ///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
    ///  - verified the client signature in the case of LNURL-auth,////// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
    /// description of the reason.
    ///
    /// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
    #[derive(Clone, Deserialize, Debug, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(tag = "status")]
    pub enum LnUrlCallbackStatus {
        /// On-wire format is: `{"status": "OK"}`
        Ok,
        /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
        #[serde(rename = "ERROR")]
        ErrorStatus {
            #[serde(flatten)]
            data: LnUrlErrorData,
        },
    }
    impl From<sdk_common::prelude::LnUrlCallbackStatus> for LnUrlCallbackStatus {
        fn from(value: prelude::LnUrlCallbackStatus) -> Self {
            match value {
                sdk_common::prelude::LnUrlCallbackStatus::Ok => Self::Ok,
                sdk_common::prelude::LnUrlCallbackStatus::ErrorStatus { data } => {
                    Self::ErrorStatus { data }
                }
            }
        }
    }
}
