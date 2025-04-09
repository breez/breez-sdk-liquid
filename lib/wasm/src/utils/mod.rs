pub(crate) mod node;

use js_sys::{global, Reflect};

pub(crate) fn is_indexed_db_supported() -> bool {
    let global = global();
    Reflect::get(&global, &"indexedDB".into()).is_ok_and(|v| !v.is_undefined())
}
