//! Mock implementations for testing
//!
//! This module provides mock adapters and utilities for testing blockchain interactions
//! without requiring actual chain connections.

use crate::ChainAdapter;
use apex_sdk_types::{Address, TransactionStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock chain adapter for testing
#[derive(Clone)]
pub struct MockChainAdapter {
    chain_name: String,
    tx_statuses: Arc<Mutex<HashMap<String, TransactionStatus>>>,
    valid_addresses: Arc<Mutex<Vec<Address>>>,
    should_fail: Arc<Mutex<bool>>,
    call_count: Arc<Mutex<u32>>,
}

impl MockChainAdapter {
    /// Create a new mock adapter
    pub fn new(chain_name: impl Into<String>) -> Self {
        Self {
            chain_name: chain_name.into(),
            tx_statuses: Arc::new(Mutex::new(HashMap::new())),
            valid_addresses: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add a transaction status to mock
    pub fn add_tx_status(&self, tx_hash: String, status: TransactionStatus) {
        self.tx_statuses.lock().unwrap().insert(tx_hash, status);
    }

    /// Add a valid address
    pub fn add_valid_address(&self, address: Address) {
        self.valid_addresses.lock().unwrap().push(address);
    }

    /// Set whether operations should fail
    pub fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().unwrap() = should_fail;
    }

    /// Get the number of times methods were called
    pub fn get_call_count(&self) -> u32 {
        *self.call_count.lock().unwrap()
    }

    /// Reset call count
    pub fn reset_call_count(&self) {
        *self.call_count.lock().unwrap() = 0;
    }

    fn increment_call_count(&self) {
        *self.call_count.lock().unwrap() += 1;
    }
}

#[async_trait]
impl ChainAdapter for MockChainAdapter {
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, String> {
        self.increment_call_count();

        if *self.should_fail.lock().unwrap() {
            return Err("Mock failure triggered".to_string());
        }

        self.tx_statuses
            .lock()
            .unwrap()
            .get(tx_hash)
            .cloned()
            .ok_or_else(|| format!("Transaction not found: {}", tx_hash))
    }

    fn validate_address(&self, address: &Address) -> bool {
        self.valid_addresses
            .lock()
            .unwrap()
            .iter()
            .any(|addr| match (addr, address) {
                (Address::Substrate(a), Address::Substrate(b)) => a == b,
                (Address::Evm(a), Address::Evm(b)) => a.eq_ignore_ascii_case(b),
                _ => false,
            })
    }

    fn chain_name(&self) -> &str {
        &self.chain_name
    }
}

/// Builder for creating mock chain adapters with fluent API
pub struct MockChainAdapterBuilder {
    adapter: MockChainAdapter,
}

impl MockChainAdapterBuilder {
    /// Create a new builder
    pub fn new(chain_name: impl Into<String>) -> Self {
        Self {
            adapter: MockChainAdapter::new(chain_name),
        }
    }

    /// Add a transaction status
    pub fn with_tx_status(self, tx_hash: String, status: TransactionStatus) -> Self {
        self.adapter.add_tx_status(tx_hash, status);
        self
    }

    /// Add a valid address
    pub fn with_valid_address(self, address: Address) -> Self {
        self.adapter.add_valid_address(address);
        self
    }

    /// Set failure mode
    pub fn with_failure(self, should_fail: bool) -> Self {
        self.adapter.set_should_fail(should_fail);
        self
    }

    /// Build the mock adapter
    pub fn build(self) -> MockChainAdapter {
        self.adapter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_adapter_basic() {
        let adapter = MockChainAdapter::new("TestChain");
        assert_eq!(adapter.chain_name(), "TestChain");
    }

    #[tokio::test]
    async fn test_mock_adapter_tx_status() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.add_tx_status(
            "0x123".to_string(),
            TransactionStatus::Confirmed {
                block_hash: "0xabc".to_string(),
                block_number: Some(100),
            },
        );

        let status = adapter.get_transaction_status("0x123").await.unwrap();
        match status {
            TransactionStatus::Confirmed { block_number, .. } => {
                assert_eq!(block_number, Some(100));
            }
            _ => panic!("Wrong status type"),
        }
    }

    #[tokio::test]
    async fn test_mock_adapter_address_validation() {
        let adapter = MockChainAdapter::new("TestChain");
        let addr = Address::Evm("0x1234567890123456789012345678901234567890".to_string());
        adapter.add_valid_address(addr.clone());

        assert!(adapter.validate_address(&addr));
        assert!(!adapter.validate_address(&Address::Evm("0xinvalid".to_string())));
    }

    #[tokio::test]
    async fn test_mock_adapter_failure_mode() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.set_should_fail(true);

        let result = adapter.get_transaction_status("0x123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_adapter_call_count() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.add_tx_status("0x123".to_string(), TransactionStatus::Pending);

        let _ = adapter.get_transaction_status("0x123").await;
        let _ = adapter.get_transaction_status("0x123").await;

        assert_eq!(adapter.get_call_count(), 2);

        adapter.reset_call_count();
        assert_eq!(adapter.get_call_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_adapter_builder() {
        let adapter = MockChainAdapterBuilder::new("TestChain")
            .with_tx_status("0x123".to_string(), TransactionStatus::Pending)
            .with_valid_address(Address::Evm(
                "0x1234567890123456789012345678901234567890".to_string(),
            ))
            .build();

        assert_eq!(adapter.chain_name(), "TestChain");
        assert!(adapter.get_transaction_status("0x123").await.is_ok());
    }
}
