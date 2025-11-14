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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let error = Error::Config("test config error".to_string());
        assert_eq!(error.to_string(), "Configuration error: test config error");
    }

    #[test]
    fn test_connection_error_display() {
        let error = Error::Connection("failed to connect".to_string());
        assert_eq!(error.to_string(), "Connection error: failed to connect");
    }

    #[test]
    fn test_transaction_error_display() {
        let error = Error::Transaction("invalid transaction".to_string());
        assert_eq!(error.to_string(), "Transaction error: invalid transaction");
    }

    #[test]
    fn test_unsupported_chain_error_display() {
        let error = Error::UnsupportedChain("Unknown".to_string());
        assert_eq!(error.to_string(), "Chain not supported: Unknown");
    }

    #[test]
    fn test_invalid_address_error_display() {
        let error = Error::InvalidAddress("0xinvalid".to_string());
        assert_eq!(error.to_string(), "Invalid address format: 0xinvalid");
    }

    #[test]
    fn test_serialization_error_display() {
        let error = Error::Serialization("JSON parse error".to_string());
        assert_eq!(error.to_string(), "Serialization error: JSON parse error");
    }

    #[test]
    fn test_other_error_display() {
        let error = Error::Other("generic error".to_string());
        assert_eq!(error.to_string(), "generic error");
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("test error");
        let error: Error = anyhow_err.into();
        assert!(matches!(error, Error::Other(_)));
        assert_eq!(error.to_string(), "test error");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<Error>();
        assert_sync::<Error>();
    }
}
