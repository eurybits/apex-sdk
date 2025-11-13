//! Core functionality for Apex SDK

use apex_sdk_types::{Address, TransactionStatus};
use async_trait::async_trait;

/// Trait for blockchain adapters
#[async_trait]
pub trait ChainAdapter: Send + Sync {
    /// Get the transaction status
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, String>;

    /// Validate an address for this chain
    fn validate_address(&self, address: &Address) -> bool;

    /// Get the chain name
    fn chain_name(&self) -> &str;
}

/// Transaction builder trait
#[async_trait]
pub trait TransactionBuilder {
    /// Set the sender address
    fn from(&mut self, address: Address) -> &mut Self;

    /// Set the recipient address
    fn to(&mut self, address: Address) -> &mut Self;

    /// Set the amount
    fn amount(&mut self, amount: u128) -> &mut Self;

    /// Build the transaction
    fn build(&self) -> Result<Vec<u8>, String>;
}
