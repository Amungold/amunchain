use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrchestratorError {
    #[error("I/O error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Build failed: {0}")]
    Build(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Genesis error: {0}")]
    Genesis(String),

    #[error("Certificate error: {0}")]
    Certificate(String),

    #[error("Certificate expired for {0}")]
    CertificateExpired(String),

    #[error("Validator error ({name}): {message}")]
    Validator { name: String, message: String },

    #[error("Service error ({service}): {message}")]
    Service { service: String, message: String },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Deployment error: {0}")]
    Deployment(String),

    #[error("Quorum not reached after {elapsed_secs}s: {details}")]
    Quorum { elapsed_secs: u64, details: String },

    #[error("Health check failed ({component}): {reason}")]
    Health { component: String, reason: String },

    #[error("State transition error: from {from} to {to}")]
    StateTransition { from: String, to: String },

    #[error("Timeout after {seconds}s: {context}")]
    Timeout { seconds: u64, context: String },

    #[error("Process error ({command}): {message}")]
    Process { command: String, message: String },

    #[error("RPC error ({endpoint}): {message}")]
    Rpc { endpoint: String, message: String },

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, OrchestratorError>;
