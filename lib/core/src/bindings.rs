use crate::{error::*, frb::bridge::StreamSink, model::*, sdk::LiquidSdk};
use anyhow::{anyhow, Result};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Arc;
use log::{Level, LevelFilter, Metadata, Record};
use tokio::runtime::Runtime;

static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
static LOG_INIT: OnceCell<bool> = OnceCell::new();

pub(crate) fn rt() -> &'static Runtime {
    &RT
}

struct BindingEventListener {
    stream: StreamSink<LiquidSdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: LiquidSdkEvent) {
        let _ = self.stream.add(e);
    }
}

struct BindingLogger {
    log_stream: StreamSink<LogEntry>,
}

impl BindingLogger {
    fn init(log_stream: StreamSink<LogEntry>) {
        let binding_logger = BindingLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger)).unwrap();
        log::set_max_level(LevelFilter::Trace);
    }
}

impl log::Log for BindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.log_stream.add(LogEntry {
                line: record.args().to_string(),
                level: record.level().as_str().to_string(),
            });
        }
    }
    fn flush(&self) {}
}

pub fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, LiquidSdkError> {
    rt().block_on(async {
        let ln_sdk = LiquidSdk::connect(req).await?;
        Ok(BindingLiquidSdk { sdk: ln_sdk })
    })
}

/// If used, this must be called before `connect`. It can only be called once.
pub fn set_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    LOG_INIT
        .set(true)
        .map_err(|_| anyhow!("Log stream already created"))?;
    BindingLogger::init(s);
    Ok(())
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LiquidSdkError> {
        rt().block_on(self.sdk.get_info(req)).map_err(Into::into)
    }

    pub fn add_event_listener(
        &self,
        listener: StreamSink<LiquidSdkEvent>,
    ) -> Result<String, LiquidSdkError> {
        rt().block_on(
            self.sdk
                .add_event_listener(Box::new(BindingEventListener { stream: listener })),
        )
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

    pub fn sync(&self) -> Result<(), LiquidSdkError> {
        rt().block_on(self.sdk.sync()).map_err(Into::into)
    }

    pub fn empty_wallet_cache(&self) -> Result<(), LiquidSdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self, req: BackupRequest) -> Result<(), LiquidSdkError> {
        self.sdk.backup(req).map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<(), LiquidSdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub fn disconnect(&self) -> Result<(), LiquidSdkError> {
        rt().block_on(self.sdk.disconnect())
    }
}
