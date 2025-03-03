use std::{sync::Arc};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ArcZhongCharError {
    #[error("{0}")]
    Error(Arc<ZhongCharError>),

}

impl From<ZhongCharError> for ArcZhongCharError {
    fn from(error: ZhongCharError) -> Self {
        ArcZhongCharError::Error(Arc::new(error))
    }
}

pub type ZhongCharResult<T> = Result<T, ZhongCharError>;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum ZhongCharError {
    #[error("{0}")]
    Wasm(String),
    #[error("{0}")]
    Reqwasm(#[from] reqwasm::Error),
    #[error("{0}")]
    Csv(#[from] csv::Error),
}

impl From<serde_wasm_bindgen::Error> for ZhongCharError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        ZhongCharError::Wasm(error.to_string()) // Convert to String
    }
}
