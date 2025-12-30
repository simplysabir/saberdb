use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaberError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Adapter error: {0}")]
    Adapter(String),
}

pub type Result<T> = std::result::Result<T, SaberError>;
