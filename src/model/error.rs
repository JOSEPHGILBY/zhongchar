// stolen from https://github.com/penumbra-zone/penumbra/blob/main/crates/wasm/src/error.rs which is MIT or Apache 2.0 attribute appropriately

use std::convert::Infallible;

//use base64::DecodeError;
//use hex::FromHexError;
use serde_wasm_bindgen::Error;
use thiserror::Error;
use js_sys::wasm_bindgen::{JsError, JsValue};
use web_sys::DomException;

//use penumbra_tct::error::{InsertBlockError, InsertEpochError, InsertError};

pub type WasmResult<T> = Result<T, WasmError>;

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),

    // #[error("{0}")]
    // DecodeError(#[from] DecodeError),

    #[error("{0}")]
    Dom(#[from] DomError),

    // #[error("{0}")]
    // FromHexError(#[from] FromHexError),

    #[error("{0}")]
    Infallible(#[from] Infallible),

    // #[error("{0}")]
    // InsertBlockError(#[from] InsertBlockError),

    // #[error("{0}")]
    // InsertEpochError(#[from] InsertEpochError),

    // #[error("{0}")]
    // InsertError(#[from] InsertError),

    #[error("{0}")]
    Wasm(#[from] serde_wasm_bindgen::Error),

    #[error("{0}")]
    Reqwasm(#[from] reqwasm::Error),

    #[error("{0}")]
    Csv(#[from] csv::Error),
}

impl From<WasmError> for serde_wasm_bindgen::Error {
    fn from(wasm_err: WasmError) -> Self {
        Error::new(wasm_err.to_string())
    }
}

impl From<WasmError> for JsValue {
    fn from(error: WasmError) -> Self {
        JsError::from(error).into()
    }
}

#[derive(Debug)]
pub struct DomError(DomException);

impl std::fmt::Display for DomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DOM Exception: {:?}", self.0)
    }
}

impl std::error::Error for DomError {}

impl From<DomException> for WasmError {
    fn from(dom_exception: DomException) -> Self {
        WasmError::Dom(DomError(dom_exception))
    }
}

impl PartialEq for WasmError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WasmError::Anyhow(e1), WasmError::Anyhow(e2)) => e1.to_string() == e2.to_string(),
            (WasmError::Dom(e1), WasmError::Dom(e2)) => e1.to_string() == e2.to_string(),
            (WasmError::Infallible(e1), WasmError::Infallible(e2)) => e1.to_string() == e2.to_string(),
            (WasmError::Wasm(e1), WasmError::Wasm(e2)) => e1.to_string() == e2.to_string(),
            (WasmError::Reqwasm(e1), WasmError::Reqwasm(e2)) => e1.to_string() == e2.to_string(),
            (WasmError::Csv(e1), WasmError::Csv(e2)) => e1.to_string() == e2.to_string(),
            _ => false,
        }
    }
}
