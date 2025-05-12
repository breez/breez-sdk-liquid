use anyhow::{anyhow, Context};
use js_sys::Reflect;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = writeFileSync, catch)]
    pub(crate) fn write_file_sync(path: &str, data: &js_sys::Uint8Array) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = readFileSync, catch)]
    pub(crate) fn read_file_sync(path: &str) -> Result<js_sys::Uint8Array, JsValue>;

    #[wasm_bindgen(js_name = existsSync)]
    pub(crate) fn exists_sync(path: &str) -> bool;

    #[wasm_bindgen(js_name = mkdirSync, catch)]
    pub(crate) fn mkdir_sync(path: &str, options: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = rmSync, catch)]
    pub(crate) fn rm_sync(path: &str, options: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = readdirSync, catch)]
    pub(crate) fn readdir_sync(path: &str) -> Result<js_sys::Array, JsValue>;
}

pub(crate) fn ensure_dir_exists(path: &str) -> anyhow::Result<()> {
    if !exists_sync(path) {
        let options = js_sys::Object::new();
        Reflect::set(&options, &"recursive".into(), &true.into())
            .map_err(js_value_to_err)
            .context("Failed to set recursive option")?;
        mkdir_sync(path, &options)
            .map_err(js_value_to_err)
            .context("Failed to call mkdir_sync")?;
    }
    Ok(())
}

pub(crate) fn js_value_to_err(err: JsValue) -> anyhow::Error {
    anyhow!(err
        .as_string()
        .unwrap_or_else(|| "Unknown error".to_string()))
}

pub(crate) fn read_file_vec(path: &str) -> anyhow::Result<Vec<u8>> {
    read_file_sync(path)
        .map(|arr| arr.to_vec())
        .map_err(js_value_to_err)
        .context("Failed to call read_file_sync")
}

pub(crate) fn write_file_vec(path: &str, data: &[u8]) -> anyhow::Result<()> {
    let js_arr = js_sys::Uint8Array::from(data);
    write_file_sync(path, &js_arr)
        .map_err(js_value_to_err)
        .context("Failed to call write_file_sync")
}
