use serde::Serialize;
use thiserror::Error;

#[cfg(feature = "spotify")]
use serde::Serializer;

#[cfg(feature = "spotify")]
use rspotify::ClientError;

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

    #[cfg(feature = "spotify")]
    #[error("Spotify Error")]
    #[serde(serialize_with = "serialize_debug")]
    SpotifyError(#[from] ClientError),
}

#[cfg(feature = "spotify")]
fn serialize_debug<S, T>(error: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Debug,
{
    serializer.serialize_str(&format!("{:?}", error))
}
