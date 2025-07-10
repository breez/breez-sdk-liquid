use breez_sdk_liquid::{
    error::{PaymentError, SdkError},
    signer::{NewError, SignError},
    LnUrlAuthError, LnUrlPayError, LnUrlWithdrawError,
};
use std::fmt::Display;
use wasm_bindgen::{JsError, JsValue};

#[derive(Clone, Debug)]
pub struct WasmError(JsValue);

impl WasmError {
    pub fn new<T: Display>(val: T) -> Self {
        WasmError(JsValue::from(format!("{val}")))
    }
}

impl From<WasmError> for JsValue {
    fn from(err: WasmError) -> Self {
        err.0
    }
}

impl From<JsValue> for WasmError {
    fn from(err: JsValue) -> Self {
        Self(err)
    }
}

macro_rules! wasm_error_wrapper {
    ($($t:ty),*) => {
        $(
            impl From<$t> for WasmError {
                fn from(err: $t) -> Self {
                    WasmError(JsError::new(format!("{}", err).as_str()).into())
                }
            }
        )*
    }
}

wasm_error_wrapper!(
    anyhow::Error,
    LnUrlAuthError,
    LnUrlPayError,
    LnUrlWithdrawError,
    log::ParseLevelError,
    PaymentError,
    SdkError,
    NewError,
    SignError,
    &str,
    String
);
