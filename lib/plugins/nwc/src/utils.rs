use web_time::{SystemTime, UNIX_EPOCH};

pub(crate) fn now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

pub(crate) fn mins_to_seconds(mins: u32) -> u32 {
    mins.div_ceil(60)
}
