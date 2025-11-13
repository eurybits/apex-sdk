//! Error types for Apex SDK

use thiserror::Error;

/// Result type alias for Apex SDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Apex SDK error types
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Chain not supported
    #[error("Chain not supported: {0}")]
    UnsupportedChain(String),

    /// Invalid address format
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),

    /// Substrate adapter error
    #[error("Substrate adapter error: {0}")]
    Substrate(#[from] apex_sdk_substrate::Error),

    /// EVM adapter error
    #[error("EVM adapter error: {0}")]
    Evm(#[from] apex_sdk_evm::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}
