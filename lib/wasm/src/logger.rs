use log::{Level, Log, Metadata, Record};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::model::LogEntry;

pub struct WasmLogger {
    pub logger: Logger,
}

impl Log for WasmLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        self.logger.log(LogEntry {
            line: record.args().to_string(),
            level: record.level().as_str().to_string(),
        });
    }
    fn flush(&self) {}
}

#[wasm_bindgen(typescript_custom_section)]
const LOGGER: &'static str = r#"export interface Logger {
    log: (l: LogEntry) => void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Logger")]
    pub type Logger;

    #[wasm_bindgen(structural, method, js_name = log)]
    pub fn log(this: &Logger, l: LogEntry);
}
