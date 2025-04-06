#![cfg(feature = "node-js")]

use crate::utils::PathExt;
use anyhow::Result;
use js_sys::Reflect;
use std::path::{Path, PathBuf};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = writeFileSync, catch)]
    fn write_file_sync(path: &str, data: &js_sys::Uint8Array) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file_sync(path: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = existsSync)]
    fn exists_sync(path: &str) -> bool;

    #[wasm_bindgen(js_name = mkdirSync, catch)]
    fn mkdir_sync(path: &str, options: &JsValue) -> Result<(), JsValue>;
}

fn get_backup_file_path(backup_dir_path: &Path) -> PathBuf {
    backup_dir_path.join("backup.sql")
}

pub fn ensure_dir_exists(path: &str) -> Result<(), JsValue> {
    if !exists_sync(path) {
        let options = js_sys::Object::new();
        Reflect::set(&options, &"recursive".into(), &true.into())?;
        mkdir_sync(path, &options)?;
    }
    Ok(())
}

pub(crate) fn backup_to_file_system(db_bytes: Vec<u8>, backup_dir_path: &Path) -> Result<()> {
    let uint8_array = js_sys::Uint8Array::from(db_bytes.as_slice());

    ensure_dir_exists(backup_dir_path.to_str_safe()?)
        .map_err(|e| anyhow::anyhow!("Failed to create backup directory: {:?}", e))?;

    let backup_file_path = get_backup_file_path(backup_dir_path);
    write_file_sync(backup_file_path.to_str_safe()?, &uint8_array).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write backup to file system using fs.writeFileSync: {:?}",
            e
        )
    })?;

    Ok(())
}

pub(crate) fn load_file_system_backup(backup_dir_path: &Path) -> Result<Option<Vec<u8>>> {
    let backup_file_path = get_backup_file_path(backup_dir_path);
    let backup_file_path_str = backup_file_path.to_str_safe()?;
    if !exists_sync(backup_file_path_str) {
        log::debug!("Backup file '{backup_file_path:?}' not found.");
        return Ok(None);
    }
    log::debug!("Backup file '{backup_file_path:?}' found, attempting to read.",);

    let buffer = read_file_sync(backup_file_path_str).map_err(|e| {
        anyhow::anyhow!("Failed to read backup file using fs.readFileSync: {:?}", e)
    })?;

    if !buffer.is_undefined() && !buffer.is_null() {
        let uint8_array = js_sys::Uint8Array::new(&buffer);
        let mut data = vec![0; uint8_array.length() as usize];
        uint8_array.copy_to(&mut data);
        Ok(Some(data))
    } else {
        Err(anyhow::anyhow!(
            "readFileSync returned null or undefined for '{backup_file_path:?}'"
        ))
    }
}
