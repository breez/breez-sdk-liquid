use crate::model::LogEntry;
use log::{Metadata, Record};

pub(crate) struct GlobalSdkLogger {
    /// SDK internal logger, which logs to file
    pub(crate) logger: env_logger::Logger,
    /// Optional external log listener, that can receive a stream of log statements
    pub(crate) log_listener: Option<Box<dyn log::Log>>,
}
impl log::Log for GlobalSdkLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logger.log(record);

            if let Some(s) = &self.log_listener.as_ref() {
                if s.enabled(record.metadata()) {
                    s.log(record);
                }
            }
        }
    }

    fn flush(&self) {}
}

pub trait LogStream: Send + Sync {
    fn log(&self, l: LogEntry);
}
