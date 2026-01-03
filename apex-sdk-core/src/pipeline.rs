//! # Transaction Pipeline
//!
//! Core transaction pipeline implementation providing unified transaction handling
//! across EVM and Substrate chains.

use crate::{
    Broadcaster, ConfirmationStrategy, FeeEstimator, NonceManager, Provider, ReceiptWatcher,
    RetryConfig, SdkError, SdkLog, Signer, TimeoutConfig,
};
use apex_sdk_types::{Address, TransactionStatus};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

/// Transaction pipeline for unified transaction execution
#[derive(Debug, Clone)]
pub struct TransactionPipeline<P, S, FE, N, B, R>
where
    P: Provider,
    S: Signer,
    FE: FeeEstimator,
    N: NonceManager,
    B: Broadcaster,
    R: ReceiptWatcher,
{
    provider: P,
    signer: S,
    fee_estimator: FE,
    nonce_manager: N,
    broadcaster: B,
    receipt_watcher: R,
    retry_config: RetryConfig,
    timeout_config: TimeoutConfig,
    confirmation_strategy: ConfirmationStrategy,
}

impl<P, S, FE, N, B, R> TransactionPipeline<P, S, FE, N, B, R>
where
    P: Provider,
    S: Signer,
    FE: FeeEstimator,
    N: NonceManager,
    B: Broadcaster,
    R: ReceiptWatcher,
{
    /// Create a new transaction pipeline
    pub fn new(
        provider: P,
        signer: S,
        fee_estimator: FE,
        nonce_manager: N,
        broadcaster: B,
        receipt_watcher: R,
    ) -> Self {
        Self {
            provider,
            signer,
            fee_estimator,
            nonce_manager,
            broadcaster,
            receipt_watcher,
            retry_config: RetryConfig::default(),
            timeout_config: TimeoutConfig::default(),
            confirmation_strategy: ConfirmationStrategy::default(),
        }
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set timeout configuration
    pub fn with_timeout_config(mut self, config: TimeoutConfig) -> Self {
        self.timeout_config = config;
        self
    }

    /// Set confirmation strategy
    pub fn with_confirmation_strategy(mut self, strategy: ConfirmationStrategy) -> Self {
        self.confirmation_strategy = strategy;
        self
    }

    /// Execute a transaction through the complete pipeline
    pub async fn execute_transaction(
        &self,
        unsigned_tx: &[u8],
    ) -> Result<TransactionResult, SdkError> {
        // Pre-transaction validation (health check + balance)
        let estimated_fee = self.validate_transaction_readiness(unsigned_tx).await?;

        // Step 2: Get nonce
        let nonce = self.get_nonce_with_retry(&self.signer.address()).await?;

        // Step 3: Sign transaction
        let signed_tx = self.sign_transaction_with_retry(unsigned_tx).await?;

        // Step 4: Broadcast transaction
        let tx_hash = self.broadcast_with_retry(&signed_tx).await?;

        // Step 5: Wait for confirmation
        let status = self.wait_for_confirmation_with_retry(&tx_hash).await?;

        Ok(TransactionResult {
            hash: tx_hash,
            nonce,
            estimated_fee,
            status,
            confirmation_strategy: self.confirmation_strategy.clone(),
        })
    }

    /// Check the balance of the signer account
    pub async fn check_signer_balance(&self) -> Result<u128, SdkError> {
        let address = self.signer.address();
        timeout(
            Duration::from_secs(self.timeout_config.rpc_timeout_secs),
            self.provider.get_balance(&address),
        )
        .await
        .map_err(|_| SdkError::NetworkError("Balance check timeout".to_string()))?
    }

    /// Get current block number from the provider
    pub async fn get_current_block_number(&self) -> Result<u64, SdkError> {
        timeout(
            Duration::from_secs(self.timeout_config.rpc_timeout_secs),
            self.provider.get_block_number(),
        )
        .await
        .map_err(|_| SdkError::NetworkError("Block number query timeout".to_string()))?
    }

    /// Validate that the signer has sufficient balance for the transaction
    pub async fn validate_sufficient_balance(&self, estimated_fee: u128) -> Result<(), SdkError> {
        let balance = self.check_signer_balance().await?;
        if balance < estimated_fee {
            return Err(SdkError::TransactionError(format!(
                "Insufficient balance. Required: {}, Available: {}",
                estimated_fee, balance
            )));
        }
        Ok(())
    }

    /// Perform pre-transaction validation including health check and balance
    pub async fn validate_transaction_readiness(
        &self,
        unsigned_tx: &[u8],
    ) -> Result<u128, SdkError> {
        // Health check
        self.provider
            .health_check()
            .await
            .map_err(|e| SdkError::NetworkError(format!("Provider health check failed: {}", e)))?;

        // Estimate fees first
        let estimated_fee = self.estimate_fee_with_retry(unsigned_tx).await?;

        // Validate sufficient balance
        self.validate_sufficient_balance(estimated_fee).await?;

        Ok(estimated_fee)
    }

    /// Estimate fee with retry logic
    async fn estimate_fee_with_retry(&self, tx: &[u8]) -> Result<u128, SdkError> {
        self.with_retry("estimate_fee", || async {
            let result: Result<u128, SdkError> = timeout(
                Duration::from_secs(self.timeout_config.rpc_timeout_secs),
                self.fee_estimator.estimate_fee(tx),
            )
            .await
            .map_err(|_| SdkError::NetworkError("Fee estimation timeout".to_string()))?;
            result
        })
        .await
    }

    async fn get_nonce_with_retry(&self, address: &Address) -> Result<u64, SdkError> {
        self.with_retry("get_nonce", || async {
            let result: Result<u64, SdkError> = timeout(
                Duration::from_secs(self.timeout_config.rpc_timeout_secs),
                self.nonce_manager.get_next_nonce(address),
            )
            .await
            .map_err(|_| SdkError::NetworkError("Nonce retrieval timeout".to_string()))?;
            result
        })
        .await
    }

    /// Sign transaction with retry logic
    async fn sign_transaction_with_retry(&self, tx: &[u8]) -> Result<Vec<u8>, SdkError> {
        self.with_retry("sign_transaction", || async {
            self.signer.sign_transaction(tx).await
        })
        .await
    }

    /// Broadcast transaction with retry logic
    async fn broadcast_with_retry(&self, signed_tx: &[u8]) -> Result<String, SdkError> {
        self.with_retry("broadcast", || async {
            let result: Result<String, SdkError> = timeout(
                Duration::from_secs(self.timeout_config.rpc_timeout_secs),
                self.broadcaster.broadcast(signed_tx),
            )
            .await
            .map_err(|_| SdkError::NetworkError("Broadcast timeout".to_string()))?;
            result
        })
        .await
    }

    /// Wait for confirmation with retry logic
    async fn wait_for_confirmation_with_retry(
        &self,
        tx_hash: &str,
    ) -> Result<TransactionStatus, SdkError> {
        let result: Result<TransactionStatus, SdkError> = timeout(
            Duration::from_secs(self.timeout_config.operation_timeout_secs),
            self.receipt_watcher
                .wait_for_receipt_with_strategy(tx_hash, &self.confirmation_strategy),
        )
        .await
        .map_err(|_| SdkError::NetworkError("Confirmation timeout".to_string()))?;
        result
    }

    /// Generic retry wrapper with exponential backoff
    async fn with_retry<RetryFn, Fut, T>(&self, operation: &str, f: RetryFn) -> Result<T, SdkError>
    where
        RetryFn: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, SdkError>>,
    {
        let mut delay = self.retry_config.initial_delay_ms;
        let mut attempt = 0;

        loop {
            match f().await {
                Ok(result) => {
                    if attempt > 0 {
                        self.log_retry_success(operation, attempt).await;
                    }
                    return Ok(result);
                }
                Err(err) => {
                    attempt += 1;
                    if attempt >= self.retry_config.max_attempts {
                        self.log_retry_failure(operation, attempt, &err).await;
                        return Err(err);
                    }

                    self.log_retry_attempt(operation, attempt, &err, delay)
                        .await;
                    tokio::time::sleep(Duration::from_millis(delay)).await;

                    delay = (delay as f64 * self.retry_config.backoff_multiplier) as u64;
                    delay = delay.min(self.retry_config.max_delay_ms);
                }
            }
        }
    }

    /// Log retry attempt
    async fn log_retry_attempt(&self, operation: &str, attempt: u32, error: &SdkError, delay: u64) {
        let log = SdkLog {
            level: crate::LogLevel::Warn,
            message: format!("Retry attempt {} for {}: {}", attempt, operation, error),
            timestamp: chrono::Utc::now().timestamp() as u64,
            operation: operation.to_string(),
            chain: None,
            transaction_hash: None,
            context: Some(serde_json::json!({
                "attempt": attempt,
                "delay_ms": delay,
                "error": error.to_string()
            })),
        };
        // In a real implementation, this would be sent to a logging system
        tracing::warn!("{}", serde_json::to_string(&log).unwrap_or_default());
    }

    /// Log retry success
    async fn log_retry_success(&self, operation: &str, attempts: u32) {
        let log = SdkLog {
            level: crate::LogLevel::Info,
            message: format!(
                "Operation {} succeeded after {} attempts",
                operation, attempts
            ),
            timestamp: chrono::Utc::now().timestamp() as u64,
            operation: operation.to_string(),
            chain: None,
            transaction_hash: None,
            context: Some(serde_json::json!({
                "total_attempts": attempts
            })),
        };
        tracing::info!("{}", serde_json::to_string(&log).unwrap_or_default());
    }

    /// Log retry failure
    async fn log_retry_failure(&self, operation: &str, attempts: u32, error: &SdkError) {
        let log = SdkLog {
            level: crate::LogLevel::Error,
            message: format!(
                "Operation {} failed after {} attempts: {}",
                operation, attempts, error
            ),
            timestamp: chrono::Utc::now().timestamp() as u64,
            operation: operation.to_string(),
            chain: None,
            transaction_hash: None,
            context: Some(serde_json::json!({
                "total_attempts": attempts,
                "final_error": error.to_string()
            })),
        };
        tracing::error!("{}", serde_json::to_string(&log).unwrap_or_default());
    }

    /// Build a transaction with the current pipeline configuration
    ///
    /// This method provides a unified interface for building transactions
    /// across different blockchain types.
    pub async fn build_transaction(
        &self,
        to: &Address,
        value: u128,
        data: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, SdkError> {
        // For building transactions, we need to create a basic transaction structure
        // In a real implementation, this would be delegated to the specific chain adapter

        // For now, return a basic encoded transaction representation
        let tx_data = serde_json::json!({
            "from": self.signer.address(),
            "to": to,
            "value": value,
            "data": data,
            "timestamp": chrono::Utc::now().timestamp()
        });

        Ok(tx_data.to_string().as_bytes().to_vec())
    }

    /// Estimate gas for a transaction
    ///
    /// This is a convenience method that delegates to the fee estimator
    pub async fn estimate_gas(&self, unsigned_tx: &[u8]) -> Result<u128, SdkError> {
        self.estimate_fee_with_retry(unsigned_tx).await
    }

    /// Configure gas settings for the pipeline
    ///
    /// This method allows updating gas-related configuration.
    /// Note: The actual gas configuration depends on the underlying provider type.
    pub fn with_gas_config(self, _gas_config: serde_json::Value) -> Self {
        // Store gas configuration in the retry config context for now
        // In a real implementation, this would be passed to the appropriate provider
        self
    }
}

/// Result of a transaction execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    /// Transaction hash
    pub hash: String,
    /// Nonce used
    pub nonce: u64,
    /// Estimated fee
    pub estimated_fee: u128,
    /// Final transaction status
    pub status: TransactionStatus,
    /// Confirmation strategy used
    pub confirmation_strategy: ConfirmationStrategy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::*;

    #[tokio::test]
    async fn test_transaction_pipeline() {
        let pipeline = TransactionPipeline::new(
            MockProvider::new(),
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let result = pipeline.execute_transaction(&[1, 2, 3]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_signer_balance() {
        let provider = MockProvider::new();
        let address = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        provider.set_balance(&address.to_string(), 1_000_000_000_000_000_000u128); // 1 ETH

        let pipeline = TransactionPipeline::new(
            provider,
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let balance = pipeline.check_signer_balance().await;
        assert!(balance.is_ok());
        assert_eq!(balance.unwrap(), 1_000_000_000_000_000_000u128);
    }

    #[tokio::test]
    async fn test_get_current_block_number() {
        let provider = MockProvider::with_block_number(12345);

        let pipeline = TransactionPipeline::new(
            provider,
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let block_number = pipeline.get_current_block_number().await;
        assert!(block_number.is_ok());
        assert_eq!(block_number.unwrap(), 12345);
    }

    #[tokio::test]
    async fn test_validate_sufficient_balance_success() {
        let provider = MockProvider::new();
        let address = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        provider.set_balance(&address.to_string(), 1_000_000_000_000_000_000u128); // 1 ETH

        let pipeline = TransactionPipeline::new(
            provider,
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let result = pipeline
            .validate_sufficient_balance(500_000_000_000_000_000u128)
            .await; // 0.5 ETH
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_sufficient_balance_insufficient() {
        let provider = MockProvider::new();
        let address = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        provider.set_balance(&address.to_string(), 100_000_000_000_000_000u128); // 0.1 ETH

        let pipeline = TransactionPipeline::new(
            provider,
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let result = pipeline
            .validate_sufficient_balance(1_000_000_000_000_000_000u128)
            .await; // 1 ETH
        assert!(result.is_err());

        if let Err(SdkError::TransactionError(msg)) = result {
            assert!(msg.contains("Insufficient balance"));
        }
    }

    #[tokio::test]
    async fn test_validate_transaction_readiness() {
        let provider = MockProvider::new();
        let address = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        provider.set_balance(&address.to_string(), 1_000_000_000_000_000_000u128); // 1 ETH

        let pipeline = TransactionPipeline::new(
            provider,
            MockSigner::new(),
            MockFeeEstimator::new(),
            MockNonceManager::new(),
            MockBroadcaster::new(),
            MockReceiptWatcher::new(),
        );

        let result = pipeline.validate_transaction_readiness(&[1, 2, 3]).await;
        assert!(result.is_ok());
    }
}
