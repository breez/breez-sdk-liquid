use crate::frb_generated::StreamSink;
use anyhow::Result;
pub use breez_sdk_liquid::model::LogEntry;
use flutter_rust_bridge::frb;

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

use crate::errors::*;

#[frb(mirror(LogEntry))]
pub struct _LogEntry {
    pub line: String,
    pub level: String,
}

pub struct BreezLogger {
    pub log_stream: StreamSink<LogEntry>,
}

impl BreezLogger {
    fn init(log_stream: StreamSink<LogEntry>) -> Result<(), SetLoggerError> {
        let binding_logger: BreezLogger = BreezLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
    }
}

impl log::Log for BreezLogger {
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

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    BreezLogger::init(s).map_err(|_| SdkError::generic("Log stream already created"))?;
    Ok(())
}
