//! Metrics and monitoring for EVM operations
//!
//! This module provides:
//! - Performance metrics tracking
//! - RPC call statistics
//! - Transaction success/failure rates
//! - Gas price tracking
//! - Prometheus-compatible metrics export

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Metrics for RPC calls
#[derive(Debug, Default)]
pub struct RpcMetrics {
    /// Total number of RPC calls
    pub total_calls: AtomicU64,
    /// Number of successful calls
    pub successful_calls: AtomicU64,
    /// Number of failed calls
    pub failed_calls: AtomicU64,
    /// Total latency in milliseconds
    pub total_latency_ms: AtomicU64,
    /// Number of retries
    pub retries: AtomicU64,
}

impl RpcMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a successful RPC call
    pub fn record_success(&self, latency_ms: u64) {
        self.total_calls.fetch_add(1, Ordering::Relaxed);
        self.successful_calls.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms
            .fetch_add(latency_ms, Ordering::Relaxed);
    }

    /// Record a failed RPC call
    pub fn record_failure(&self, latency_ms: u64) {
        self.total_calls.fetch_add(1, Ordering::Relaxed);
        self.failed_calls.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms
            .fetch_add(latency_ms, Ordering::Relaxed);
    }

    /// Record a retry
    pub fn record_retry(&self) {
        self.retries.fetch_add(1, Ordering::Relaxed);
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        let total = self.total_calls.load(Ordering::Relaxed);
        if total == 0 {
            return 100.0;
        }
        let successful = self.successful_calls.load(Ordering::Relaxed);
        (successful as f64 / total as f64) * 100.0
    }

    /// Get average latency in milliseconds
    pub fn avg_latency_ms(&self) -> f64 {
        let total = self.total_calls.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let latency = self.total_latency_ms.load(Ordering::Relaxed);
        latency as f64 / total as f64
    }
}

/// Metrics for transactions
#[derive(Debug, Default)]
pub struct TransactionMetrics {
    /// Total transactions submitted
    pub submitted: AtomicU64,
    /// Successful transactions
    pub successful: AtomicU64,
    /// Failed transactions
    pub failed: AtomicU64,
    /// Pending transactions
    pub pending: AtomicU64,
    /// Total gas used
    pub total_gas_used: AtomicU64,
    /// Total transaction cost in wei
    pub total_cost_wei: AtomicU64,
}

impl TransactionMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a transaction submission
    pub fn record_submission(&self) {
        self.submitted.fetch_add(1, Ordering::Relaxed);
        self.pending.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a successful transaction
    pub fn record_success(&self, gas_used: u64, cost_wei: u128) {
        self.successful.fetch_add(1, Ordering::Relaxed);
        self.pending.fetch_sub(1, Ordering::Relaxed);
        self.total_gas_used.fetch_add(gas_used, Ordering::Relaxed);
        // Truncate to u64 for atomic operations
        self.total_cost_wei
            .fetch_add((cost_wei & u64::MAX as u128) as u64, Ordering::Relaxed);
    }

    /// Record a failed transaction
    pub fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
        self.pending.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        let completed =
            self.successful.load(Ordering::Relaxed) + self.failed.load(Ordering::Relaxed);
        if completed == 0 {
            return 100.0;
        }
        let successful = self.successful.load(Ordering::Relaxed);
        (successful as f64 / completed as f64) * 100.0
    }

    /// Get average gas used
    pub fn avg_gas_used(&self) -> f64 {
        let successful = self.successful.load(Ordering::Relaxed);
        if successful == 0 {
            return 0.0;
        }
        let gas = self.total_gas_used.load(Ordering::Relaxed);
        gas as f64 / successful as f64
    }
}

/// Gas price tracking
#[derive(Debug, Clone)]
pub struct GasPriceSnapshot {
    /// Timestamp of snapshot
    pub timestamp: Instant,
    /// Base fee per gas (EIP-1559)
    pub base_fee_gwei: f64,
    /// Priority fee per gas (EIP-1559)
    pub priority_fee_gwei: f64,
    /// Legacy gas price
    pub gas_price_gwei: f64,
}

/// Metrics for gas prices
pub struct GasMetrics {
    recent_snapshots: Arc<RwLock<Vec<GasPriceSnapshot>>>,
    max_snapshots: usize,
}

impl GasMetrics {
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            recent_snapshots: Arc::new(RwLock::new(Vec::new())),
            max_snapshots,
        }
    }

    /// Record a gas price snapshot
    pub async fn record_snapshot(&self, snapshot: GasPriceSnapshot) {
        let mut snapshots = self.recent_snapshots.write().await;

        // Keep only recent snapshots
        if snapshots.len() >= self.max_snapshots {
            snapshots.remove(0);
        }

        snapshots.push(snapshot);
    }

    /// Get average base fee over recent snapshots
    pub async fn avg_base_fee_gwei(&self) -> f64 {
        let snapshots = self.recent_snapshots.read().await;
        if snapshots.is_empty() {
            return 0.0;
        }

        let sum: f64 = snapshots.iter().map(|s| s.base_fee_gwei).sum();
        sum / snapshots.len() as f64
    }

    /// Get average priority fee over recent snapshots
    pub async fn avg_priority_fee_gwei(&self) -> f64 {
        let snapshots = self.recent_snapshots.read().await;
        if snapshots.is_empty() {
            return 0.0;
        }

        let sum: f64 = snapshots.iter().map(|s| s.priority_fee_gwei).sum();
        sum / snapshots.len() as f64
    }

    /// Get gas price trend (increasing, stable, decreasing)
    pub async fn gas_price_trend(&self) -> String {
        let snapshots = self.recent_snapshots.read().await;
        if snapshots.len() < 2 {
            return "unknown".to_string();
        }

        let recent = &snapshots[snapshots.len() - 1];
        let older = &snapshots[snapshots.len() / 2];

        let diff_percent =
            ((recent.base_fee_gwei - older.base_fee_gwei) / older.base_fee_gwei) * 100.0;

        if diff_percent > 10.0 {
            "increasing".to_string()
        } else if diff_percent < -10.0 {
            "decreasing".to_string()
        } else {
            "stable".to_string()
        }
    }
}

/// Comprehensive metrics collector
pub struct MetricsCollector {
    /// RPC call metrics
    pub rpc: Arc<RpcMetrics>,
    /// Transaction metrics
    pub transactions: Arc<TransactionMetrics>,
    /// Gas price metrics
    pub gas: Arc<GasMetrics>,
    /// Start time of the collector
    start_time: Instant,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            rpc: Arc::new(RpcMetrics::new()),
            transactions: Arc::new(TransactionMetrics::new()),
            gas: Arc::new(GasMetrics::new(100)),
            start_time: Instant::now(),
        }
    }

    /// Get uptime in seconds
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Export metrics in Prometheus format
    pub async fn export_prometheus(&self) -> String {
        let mut output = String::new();

        // RPC metrics
        output.push_str("# HELP apex_evm_rpc_calls_total Total number of RPC calls\n");
        output.push_str("# TYPE apex_evm_rpc_calls_total counter\n");
        output.push_str(&format!(
            "apex_evm_rpc_calls_total {}\n",
            self.rpc.total_calls.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_rpc_calls_successful Successful RPC calls\n");
        output.push_str("# TYPE apex_evm_rpc_calls_successful counter\n");
        output.push_str(&format!(
            "apex_evm_rpc_calls_successful {}\n",
            self.rpc.successful_calls.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_rpc_calls_failed Failed RPC calls\n");
        output.push_str("# TYPE apex_evm_rpc_calls_failed counter\n");
        output.push_str(&format!(
            "apex_evm_rpc_calls_failed {}\n",
            self.rpc.failed_calls.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_rpc_latency_avg Average RPC latency in milliseconds\n");
        output.push_str("# TYPE apex_evm_rpc_latency_avg gauge\n");
        output.push_str(&format!(
            "apex_evm_rpc_latency_avg {}\n",
            self.rpc.avg_latency_ms()
        ));

        output.push_str("# HELP apex_evm_rpc_success_rate RPC success rate percentage\n");
        output.push_str("# TYPE apex_evm_rpc_success_rate gauge\n");
        output.push_str(&format!(
            "apex_evm_rpc_success_rate {}\n",
            self.rpc.success_rate()
        ));

        // Transaction metrics
        output.push_str("# HELP apex_evm_transactions_submitted Total transactions submitted\n");
        output.push_str("# TYPE apex_evm_transactions_submitted counter\n");
        output.push_str(&format!(
            "apex_evm_transactions_submitted {}\n",
            self.transactions.submitted.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_transactions_successful Successful transactions\n");
        output.push_str("# TYPE apex_evm_transactions_successful counter\n");
        output.push_str(&format!(
            "apex_evm_transactions_successful {}\n",
            self.transactions.successful.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_transactions_failed Failed transactions\n");
        output.push_str("# TYPE apex_evm_transactions_failed counter\n");
        output.push_str(&format!(
            "apex_evm_transactions_failed {}\n",
            self.transactions.failed.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_transactions_pending Pending transactions\n");
        output.push_str("# TYPE apex_evm_transactions_pending gauge\n");
        output.push_str(&format!(
            "apex_evm_transactions_pending {}\n",
            self.transactions.pending.load(Ordering::Relaxed)
        ));

        output.push_str("# HELP apex_evm_transactions_success_rate Transaction success rate\n");
        output.push_str("# TYPE apex_evm_transactions_success_rate gauge\n");
        output.push_str(&format!(
            "apex_evm_transactions_success_rate {}\n",
            self.transactions.success_rate()
        ));

        output.push_str("# HELP apex_evm_gas_avg Average gas used per transaction\n");
        output.push_str("# TYPE apex_evm_gas_avg gauge\n");
        output.push_str(&format!(
            "apex_evm_gas_avg {}\n",
            self.transactions.avg_gas_used()
        ));

        // Gas price metrics
        output.push_str("# HELP apex_evm_gas_base_fee_avg Average base fee in gwei\n");
        output.push_str("# TYPE apex_evm_gas_base_fee_avg gauge\n");
        output.push_str(&format!(
            "apex_evm_gas_base_fee_avg {}\n",
            self.gas.avg_base_fee_gwei().await
        ));

        output.push_str("# HELP apex_evm_gas_priority_fee_avg Average priority fee in gwei\n");
        output.push_str("# TYPE apex_evm_gas_priority_fee_avg gauge\n");
        output.push_str(&format!(
            "apex_evm_gas_priority_fee_avg {}\n",
            self.gas.avg_priority_fee_gwei().await
        ));

        // Uptime
        output.push_str("# HELP apex_evm_uptime_seconds Uptime in seconds\n");
        output.push_str("# TYPE apex_evm_uptime_seconds counter\n");
        output.push_str(&format!("apex_evm_uptime_seconds {}\n", self.uptime_secs()));

        output
    }

    /// Print human-readable metrics summary
    pub async fn print_summary(&self) {
        println!("=== Apex EVM Metrics Summary ===");
        println!("\nRPC Calls:");
        println!("  Total: {}", self.rpc.total_calls.load(Ordering::Relaxed));
        println!(
            "  Successful: {}",
            self.rpc.successful_calls.load(Ordering::Relaxed)
        );
        println!(
            "  Failed: {}",
            self.rpc.failed_calls.load(Ordering::Relaxed)
        );
        println!("  Success Rate: {:.2}%", self.rpc.success_rate());
        println!("  Avg Latency: {:.2}ms", self.rpc.avg_latency_ms());
        println!("  Retries: {}", self.rpc.retries.load(Ordering::Relaxed));

        println!("\nTransactions:");
        println!(
            "  Submitted: {}",
            self.transactions.submitted.load(Ordering::Relaxed)
        );
        println!(
            "  Successful: {}",
            self.transactions.successful.load(Ordering::Relaxed)
        );
        println!(
            "  Failed: {}",
            self.transactions.failed.load(Ordering::Relaxed)
        );
        println!(
            "  Pending: {}",
            self.transactions.pending.load(Ordering::Relaxed)
        );
        println!("  Success Rate: {:.2}%", self.transactions.success_rate());
        println!("  Avg Gas Used: {:.0}", self.transactions.avg_gas_used());

        println!("\nGas Prices:");
        println!(
            "  Avg Base Fee: {:.2} gwei",
            self.gas.avg_base_fee_gwei().await
        );
        println!(
            "  Avg Priority Fee: {:.2} gwei",
            self.gas.avg_priority_fee_gwei().await
        );
        println!("  Trend: {}", self.gas.gas_price_trend().await);

        println!("\nSystem:");
        println!("  Uptime: {}s", self.uptime_secs());
        println!("================================");
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_metrics() {
        let metrics = RpcMetrics::new();

        metrics.record_success(100);
        metrics.record_success(200);
        metrics.record_failure(150);

        assert_eq!(metrics.total_calls.load(Ordering::Relaxed), 3);
        assert_eq!(metrics.successful_calls.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.failed_calls.load(Ordering::Relaxed), 1);
        assert!((metrics.success_rate() - 66.67).abs() < 0.1);
        assert!((metrics.avg_latency_ms() - 150.0).abs() < 0.1);
    }

    #[test]
    fn test_transaction_metrics() {
        let metrics = TransactionMetrics::new();

        metrics.record_submission();
        metrics.record_submission();
        metrics.record_success(21000, 21000000000000);
        metrics.record_failure();

        assert_eq!(metrics.submitted.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.successful.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.failed.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.success_rate(), 50.0);
    }

    #[tokio::test]
    async fn test_metrics_collector_prometheus() {
        let collector = MetricsCollector::new();

        collector.rpc.record_success(100);
        collector.transactions.record_submission();

        let output = collector.export_prometheus().await;

        assert!(output.contains("apex_evm_rpc_calls_total"));
        assert!(output.contains("apex_evm_transactions_submitted"));
        assert!(output.contains("apex_evm_uptime_seconds"));
    }
}
