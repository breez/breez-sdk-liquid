use std::sync::Arc;

use anyhow::Result;
use breez_liquid_sdk::logger::LogStream;
use breez_liquid_sdk::{error::*, model::*, sdk::LiquidSdk};
use log::{Metadata, Record};
use once_cell::sync::{Lazy, OnceCell};
use tokio::runtime::Runtime;
use uniffi::deps::log::{Level, LevelFilter};

static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

fn rt() -> &'static Runtime {
    &RT
}

static LOG_INIT: OnceCell<bool> = OnceCell::new();

struct UniffiBindingLogger {
    log_stream: Box<dyn LogStream>,
}

impl UniffiBindingLogger {
    fn init(log_stream: Box<dyn LogStream>) {
        let binding_logger = UniffiBindingLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger)).unwrap();
        log::set_max_level(LevelFilter::Trace);
    }
}

impl log::Log for UniffiBindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        // ignore the internal uniffi log to prevent infinite loop.
        return m.level() <= Level::Trace && *m.target() != *"breez_liquid_sdk_bindings";
    }

    fn log(&self, record: &Record) {
        self.log_stream.log(LogEntry {
            line: record.args().to_string(),
            level: record.level().as_str().to_string(),
        });
    }
    fn flush(&self) {}
}

/// If used, this must be called before `connect`
pub fn set_log_stream(log_stream: Box<dyn LogStream>) -> Result<(), LiquidSdkError> {
    LOG_INIT.set(true).map_err(|_| LiquidSdkError::Generic {
        err: "Log stream already created".into(),
    })?;
    UniffiBindingLogger::init(log_stream);
    Ok(())
}

pub fn connect(req: ConnectRequest) -> Result<Arc<BindingLiquidSdk>, LiquidSdkError> {
    rt().block_on(async {
        let sdk = LiquidSdk::connect(req).await?;
        Ok(Arc::from(BindingLiquidSdk { sdk }))
    })
}

pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> LiquidSdkResult<String> {
        rt().block_on(self.sdk.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, id: String) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.remove_event_listener(id))
    }

    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LiquidSdkError> {
        rt().block_on(self.sdk.get_info(req)).map_err(Into::into)
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_send_payment(&req))
    }

    pub fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.sdk.send_payment(&req))
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_receive_payment(&req))
    }

    pub fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.sdk.receive_payment(&req))
    }

    pub fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.sdk.list_payments())
    }

    pub fn sync(&self) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.sync()).map_err(Into::into)
    }

    pub fn empty_wallet_cache(&self) -> LiquidSdkResult<()> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self, req: BackupRequest) -> LiquidSdkResult<()> {
        self.sdk.backup(req).map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> LiquidSdkResult<()> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub fn disconnect(&self) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.disconnect())
    }
}

uniffi::include_scaffolding!("breez_liquid_sdk");
