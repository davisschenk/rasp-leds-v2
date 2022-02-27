use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, LedError>;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum LedError {
    #[error("There is no last pattern in history")]
    NoHistory,

    #[error("Couldnt send command to inner thread!")]
    SendError,

    #[error("Pattern Error")]
    PatternError,
}
