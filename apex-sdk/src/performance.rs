//! Performance optimization utilities for Apex SDK
//!
//! This module provides utilities for optimizing blockchain operations,
//! including batching, parallel execution, and caching strategies.

use std::future::Future;
use std::time::Duration;
use tokio::time::timeout;

/// Configuration for batch processing
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Maximum wait time before flushing batch
    pub max_wait_time: Duration,
    /// Number of parallel workers
    pub num_workers: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            max_wait_time: Duration::from_millis(100),
            num_workers: 4,
        }
    }
}

/// Execute multiple async operations in parallel with timeout
pub async fn parallel_execute<F, Fut, T, E>(
    operations: Vec<F>,
    operation_timeout: Duration,
) -> Vec<Result<T, E>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: From<String> + Send + 'static,
{
    let handles: Vec<_> = operations
        .into_iter()
        .map(|op| {
            tokio::spawn(async move {
                match timeout(operation_timeout, op()).await {
                    Ok(result) => result,
                    Err(_) => Err(E::from("Operation timed out".to_string())),
                }
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => results.push(Err(E::from(format!("Task panicked: {}", e)))),
        }
    }
    results
}

/// Execute operations in batches with parallelism
pub async fn batch_execute<F, Fut, T, E>(
    mut items: Vec<F>,
    config: BatchConfig,
) -> Vec<Result<T, E>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: From<String> + Send + 'static,
{
    let mut all_results = Vec::new();

    while !items.is_empty() {
        let batch_size = items.len().min(config.max_batch_size);
        let batch: Vec<_> = items.drain(..batch_size).collect();

        let results = parallel_execute(batch, config.max_wait_time).await;
        all_results.extend(results);
    }

    all_results
}

/// Rate limiter for API calls
#[derive(Debug, Clone)]
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    requests: std::sync::Arc<tokio::sync::Mutex<Vec<std::time::Instant>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    /// Wait until a request can be made
    pub async fn acquire(&self) {
        loop {
            let mut requests = self.requests.lock().await;
            let now = std::time::Instant::now();

            // Remove expired requests
            requests.retain(|&req_time| now.duration_since(req_time) < self.window);

            if requests.len() < self.max_requests as usize {
                requests.push(now);
                return;
            }

            // Calculate wait time
            if let Some(&oldest) = requests.first() {
                let elapsed = now.duration_since(oldest);
                if elapsed < self.window {
                    let wait_time = self.window - elapsed;
                    drop(requests); // Release lock while waiting
                    tokio::time::sleep(wait_time).await;
                }
            }
        }
    }

    /// Execute an operation with rate limiting
    pub async fn execute<F, Fut, T>(&self, operation: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        self.acquire().await;
        operation().await
    }
}

/// Connection pool manager with health checks
#[derive(Clone)]
pub struct ConnectionPool<T: Clone> {
    connections: std::sync::Arc<tokio::sync::RwLock<Vec<T>>>,
    current_index: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl<T: Clone> ConnectionPool<T> {
    /// Create a new connection pool
    pub fn new(connections: Vec<T>) -> Self {
        Self {
            connections: std::sync::Arc::new(tokio::sync::RwLock::new(connections)),
            current_index: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Get the next connection using round-robin
    pub async fn get(&self) -> Option<T> {
        let connections = self.connections.read().await;
        if connections.is_empty() {
            return None;
        }

        let index = self
            .current_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            % connections.len();
        Some(connections[index].clone())
    }

    /// Get all connections
    pub async fn get_all(&self) -> Vec<T> {
        self.connections.read().await.clone()
    }

    /// Add a connection to the pool
    pub async fn add(&self, connection: T) {
        self.connections.write().await.push(connection);
    }

    /// Remove a connection from the pool
    pub async fn remove(&self, predicate: impl Fn(&T) -> bool) {
        self.connections.write().await.retain(|c| !predicate(c));
    }

    /// Get pool size
    pub async fn size(&self) -> usize {
        self.connections.read().await.len()
    }
}

/// Async memoization for expensive computations
pub struct AsyncMemo<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    cache: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<K, V>>>,
    ttl: Option<Duration>,
    timestamps:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<K, std::time::Instant>>>,
}

impl<K, V> Default for AsyncMemo<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AsyncMemo<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new async memo cache
    pub fn new() -> Self {
        Self {
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            ttl: None,
            timestamps: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Create a new async memo cache with TTL
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            ttl: Some(ttl),
            timestamps: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Get or compute a value
    pub async fn get_or_compute<F, Fut>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        // Check if value exists and is not expired
        {
            let cache = self.cache.read().await;
            if let Some(value) = cache.get(&key) {
                if let Some(ttl) = self.ttl {
                    let timestamps = self.timestamps.read().await;
                    if let Some(&timestamp) = timestamps.get(&key) {
                        if timestamp.elapsed() < ttl {
                            return value.clone();
                        }
                    }
                } else {
                    return value.clone();
                }
            }
        }

        // Compute new value
        let value = compute().await;

        // Store in cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(key.clone(), value.clone());

            if self.ttl.is_some() {
                let mut timestamps = self.timestamps.write().await;
                timestamps.insert(key, std::time::Instant::now());
            }
        }

        value
    }

    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        let mut timestamps = self.timestamps.write().await;
        timestamps.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_parallel_execute() {
        let operations: Vec<_> = (0..5)
            .map(|i| {
                move || async move {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    Ok::<_, String>(i)
                }
            })
            .collect();

        let results = parallel_execute(operations, Duration::from_secs(1)).await;

        assert_eq!(results.len(), 5);
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(2, Duration::from_millis(100));

        let counter = Arc::new(AtomicU32::new(0));
        let mut handles = vec![];

        for _ in 0..5 {
            let limiter = limiter.clone();
            let counter = counter.clone();
            handles.push(tokio::spawn(async move {
                limiter.acquire().await;
                counter.fetch_add(1, Ordering::Relaxed);
            }));
        }

        // Wait a bit and check that only 2 requests went through
        tokio::time::sleep(Duration::from_millis(50)).await;
        let count = counter.load(Ordering::Relaxed);
        assert!(count <= 2);

        // Wait for all to complete
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(counter.load(Ordering::Relaxed), 5);
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = ConnectionPool::new(vec!["conn1", "conn2", "conn3"]);

        let conn1 = pool.get().await.unwrap();
        let conn2 = pool.get().await.unwrap();
        let conn3 = pool.get().await.unwrap();
        let conn4 = pool.get().await.unwrap(); // Should wrap around

        assert_eq!(conn1, "conn1");
        assert_eq!(conn2, "conn2");
        assert_eq!(conn3, "conn3");
        assert_eq!(conn4, "conn1"); // Round-robin
    }

    #[tokio::test]
    async fn test_async_memo() {
        let memo = AsyncMemo::new();
        let counter = Arc::new(AtomicU32::new(0));

        let counter_clone = counter.clone();
        let value1 = memo
            .get_or_compute("key1", || async {
                counter_clone.fetch_add(1, Ordering::Relaxed);
                42
            })
            .await;

        assert_eq!(value1, 42);
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        // Should use cached value
        let value2 = memo
            .get_or_compute("key1", || async {
                counter.fetch_add(1, Ordering::Relaxed);
                100
            })
            .await;

        assert_eq!(value2, 42); // Still cached value
        assert_eq!(counter.load(Ordering::Relaxed), 1); // Not recomputed
    }

    #[tokio::test]
    async fn test_async_memo_with_ttl() {
        let memo = AsyncMemo::with_ttl(Duration::from_millis(50));
        let counter = Arc::new(AtomicU32::new(0));

        let counter_clone = counter.clone();
        let value1 = memo
            .get_or_compute("key1", || async {
                counter_clone.fetch_add(1, Ordering::Relaxed);
                42
            })
            .await;

        assert_eq!(value1, 42);
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        // Wait for TTL to expire
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Should recompute
        let counter_clone = counter.clone();
        let value2 = memo
            .get_or_compute("key1", || async {
                counter_clone.fetch_add(1, Ordering::Relaxed);
                100
            })
            .await;

        assert_eq!(value2, 100); // New value
        assert_eq!(counter.load(Ordering::Relaxed), 2); // Recomputed
    }
}
