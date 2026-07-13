use std::path::PathBuf;
use thiserror::Error;

/// Unified error type for all node operations.
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum NodeError {
    #[error("I/O error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("TOML deserialisation error in {path}: {source}")]
    Toml {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("JSON deserialisation error in {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("Network bind error on {addr}: {source}")]
    Bind {
        addr: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid listen address: {0}")]
    InvalidAddress(String),

    #[error("Genesis validation failed: {0}")]
    Genesis(String),

    #[error("Certificate error: {0}")]
    Certificate(String),
}

/// Small helper to create an Io error with a path.
pub fn io_err(path: impl Into<PathBuf>, source: std::io::Error) -> NodeError {
    NodeError::Io {
        path: path.into(),
        source,
    }
}
