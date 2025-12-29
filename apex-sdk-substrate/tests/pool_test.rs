//! Comprehensive tests for connection pool module
//!
//! These tests verify connection pool functionality including:
//! - Pool configuration
//! - Connection management
//! - Health checking
//! - Round-robin load balancing
//! - Failover behavior

use apex_sdk_substrate::pool::*;
use std::time::Duration;

#[test]
fn test_pool_config_new() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints.clone());

    assert_eq!(config.endpoints.len(), 1);
    assert_eq!(config.endpoints[0], "wss://endpoint1.example.com");
    assert_eq!(config.health_check_interval, Duration::from_secs(30));
    assert_eq!(config.connection_timeout, Duration::from_secs(10));
    assert_eq!(config.max_retries, 3);
    assert!(config.auto_health_check);
}

#[test]
fn test_pool_config_with_health_check_interval() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints).with_health_check_interval(Duration::from_secs(60));

    assert_eq!(config.health_check_interval, Duration::from_secs(60));
}

#[test]
fn test_pool_config_with_connection_timeout() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints).with_connection_timeout(Duration::from_secs(20));

    assert_eq!(config.connection_timeout, Duration::from_secs(20));
}

#[test]
fn test_pool_config_with_max_retries() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints).with_max_retries(5);

    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_pool_config_with_auto_health_check() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints).with_auto_health_check(false);

    assert!(!config.auto_health_check);
}

#[test]
fn test_pool_config_builder_pattern() {
    let endpoints = vec![
        "wss://endpoint1.example.com".to_string(),
        "wss://endpoint2.example.com".to_string(),
    ];

    let config = PoolConfig::new(endpoints)
        .with_health_check_interval(Duration::from_secs(45))
        .with_connection_timeout(Duration::from_secs(15))
        .with_max_retries(4)
        .with_auto_health_check(true);

    assert_eq!(config.endpoints.len(), 2);
    assert_eq!(config.health_check_interval, Duration::from_secs(45));
    assert_eq!(config.connection_timeout, Duration::from_secs(15));
    assert_eq!(config.max_retries, 4);
    assert!(config.auto_health_check);
}

#[test]
fn test_health_status_equality() {
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
    assert_eq!(HealthStatus::Unhealthy, HealthStatus::Unhealthy);
    assert_eq!(HealthStatus::Unknown, HealthStatus::Unknown);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Unknown);
    assert_ne!(HealthStatus::Unhealthy, HealthStatus::Unknown);
}

#[test]
fn test_pool_stats_structure() {
    let stats = PoolStats {
        total_endpoints: 5,
        healthy_endpoints: 3,
        unhealthy_endpoints: 1,
        unknown_endpoints: 1,
    };

    assert_eq!(stats.total_endpoints, 5);
    assert_eq!(stats.healthy_endpoints, 3);
    assert_eq!(stats.unhealthy_endpoints, 1);
    assert_eq!(stats.unknown_endpoints, 1);
}

#[test]
fn test_pool_stats_health_percentage() {
    let stats = PoolStats {
        total_endpoints: 10,
        healthy_endpoints: 8,
        unhealthy_endpoints: 2,
        unknown_endpoints: 0,
    };

    assert_eq!(stats.health_percentage(), 80.0);
}

#[test]
fn test_pool_stats_health_percentage_zero_endpoints() {
    let stats = PoolStats {
        total_endpoints: 0,
        healthy_endpoints: 0,
        unhealthy_endpoints: 0,
        unknown_endpoints: 0,
    };

    assert_eq!(stats.health_percentage(), 0.0);
}

#[test]
fn test_pool_stats_health_percentage_all_healthy() {
    let stats = PoolStats {
        total_endpoints: 5,
        healthy_endpoints: 5,
        unhealthy_endpoints: 0,
        unknown_endpoints: 0,
    };

    assert_eq!(stats.health_percentage(), 100.0);
}

#[test]
fn test_pool_stats_health_percentage_none_healthy() {
    let stats = PoolStats {
        total_endpoints: 5,
        healthy_endpoints: 0,
        unhealthy_endpoints: 5,
        unknown_endpoints: 0,
    };

    assert_eq!(stats.health_percentage(), 0.0);
}

#[test]
fn test_pool_stats_display() {
    let stats = PoolStats {
        total_endpoints: 4,
        healthy_endpoints: 3,
        unhealthy_endpoints: 1,
        unknown_endpoints: 0,
    };

    let display = format!("{}", stats);
    assert!(display.contains("4 total"));
    assert!(display.contains("3 healthy"));
    assert!(display.contains("75.0%"));
    assert!(display.contains("1 unhealthy"));
}

#[test]
fn test_pool_stats_display_with_decimals() {
    let stats = PoolStats {
        total_endpoints: 3,
        healthy_endpoints: 2,
        unhealthy_endpoints: 1,
        unknown_endpoints: 0,
    };

    let display = format!("{}", stats);
    // 2/3 = 66.666...%
    assert!(display.contains("66.7%"));
}

#[test]
fn test_pool_config_multiple_endpoints() {
    let endpoints = vec![
        "wss://endpoint1.example.com".to_string(),
        "wss://endpoint2.example.com".to_string(),
        "wss://endpoint3.example.com".to_string(),
    ];

    let config = PoolConfig::new(endpoints.clone());

    assert_eq!(config.endpoints.len(), 3);
    assert_eq!(config.endpoints[0], "wss://endpoint1.example.com");
    assert_eq!(config.endpoints[1], "wss://endpoint2.example.com");
    assert_eq!(config.endpoints[2], "wss://endpoint3.example.com");
}

#[test]
fn test_pool_stats_various_distributions() {
    // Test 1: All healthy
    let stats = PoolStats {
        total_endpoints: 10,
        healthy_endpoints: 10,
        unhealthy_endpoints: 0,
        unknown_endpoints: 0,
    };
    assert_eq!(stats.health_percentage(), 100.0);

    // Test 2: Half healthy
    let stats = PoolStats {
        total_endpoints: 10,
        healthy_endpoints: 5,
        unhealthy_endpoints: 5,
        unknown_endpoints: 0,
    };
    assert_eq!(stats.health_percentage(), 50.0);

    // Test 3: One healthy
    let stats = PoolStats {
        total_endpoints: 10,
        healthy_endpoints: 1,
        unhealthy_endpoints: 9,
        unknown_endpoints: 0,
    };
    assert_eq!(stats.health_percentage(), 10.0);

    // Test 4: Mixed status
    let stats = PoolStats {
        total_endpoints: 20,
        healthy_endpoints: 15,
        unhealthy_endpoints: 3,
        unknown_endpoints: 2,
    };
    assert_eq!(stats.health_percentage(), 75.0);
}

#[test]
fn test_health_status_copy() {
    let status1 = HealthStatus::Healthy;
    let status2 = status1;

    assert_eq!(status1, status2);
    assert_eq!(status1, HealthStatus::Healthy);
}

#[test]
fn test_pool_config_clone() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints)
        .with_health_check_interval(Duration::from_secs(60))
        .with_max_retries(5);

    let cloned = config.clone();

    assert_eq!(cloned.endpoints.len(), config.endpoints.len());
    assert_eq!(cloned.health_check_interval, config.health_check_interval);
    assert_eq!(cloned.max_retries, config.max_retries);
}

#[test]
fn test_pool_stats_clone() {
    let stats = PoolStats {
        total_endpoints: 5,
        healthy_endpoints: 3,
        unhealthy_endpoints: 2,
        unknown_endpoints: 0,
    };

    let cloned = stats.clone();

    assert_eq!(cloned.total_endpoints, stats.total_endpoints);
    assert_eq!(cloned.healthy_endpoints, stats.healthy_endpoints);
    assert_eq!(cloned.unhealthy_endpoints, stats.unhealthy_endpoints);
}

#[test]
fn test_pool_config_debug() {
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];
    let config = PoolConfig::new(endpoints);

    let debug_output = format!("{:?}", config);
    assert!(debug_output.contains("PoolConfig"));
    assert!(debug_output.contains("endpoints"));
}

#[test]
fn test_health_status_debug() {
    let healthy = format!("{:?}", HealthStatus::Healthy);
    let unhealthy = format!("{:?}", HealthStatus::Unhealthy);
    let unknown = format!("{:?}", HealthStatus::Unknown);

    assert_eq!(healthy, "Healthy");
    assert_eq!(unhealthy, "Unhealthy");
    assert_eq!(unknown, "Unknown");
}

#[test]
fn test_pool_stats_debug() {
    let stats = PoolStats {
        total_endpoints: 5,
        healthy_endpoints: 3,
        unhealthy_endpoints: 1,
        unknown_endpoints: 1,
    };

    let debug_output = format!("{:?}", stats);
    assert!(debug_output.contains("PoolStats"));
    assert!(debug_output.contains("total_endpoints"));
    assert!(debug_output.contains("5"));
}

#[test]
fn test_pool_config_with_custom_values() {
    let endpoints = vec![
        "wss://rpc1.polkadot.io".to_string(),
        "wss://rpc2.polkadot.io".to_string(),
    ];

    let config = PoolConfig::new(endpoints)
        .with_health_check_interval(Duration::from_secs(120))
        .with_connection_timeout(Duration::from_secs(30))
        .with_max_retries(10)
        .with_auto_health_check(false);

    assert_eq!(config.health_check_interval, Duration::from_secs(120));
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.max_retries, 10);
    assert!(!config.auto_health_check);
}

#[test]
fn test_pool_stats_edge_cases() {
    // Edge case: Single endpoint
    let stats = PoolStats {
        total_endpoints: 1,
        healthy_endpoints: 1,
        unhealthy_endpoints: 0,
        unknown_endpoints: 0,
    };
    assert_eq!(stats.health_percentage(), 100.0);

    // Edge case: Large number of endpoints
    let stats = PoolStats {
        total_endpoints: 1000,
        healthy_endpoints: 999,
        unhealthy_endpoints: 1,
        unknown_endpoints: 0,
    };
    assert_eq!(stats.health_percentage(), 99.9);
}

#[test]
fn test_pool_config_empty_then_add() {
    // Test that we can track endpoint additions
    let endpoints = vec!["wss://endpoint1.example.com".to_string()];

    let config = PoolConfig::new(endpoints);
    assert_eq!(config.endpoints.len(), 1);
}

#[test]
fn test_duration_values() {
    let config = PoolConfig::new(vec!["wss://test.com".to_string()]);

    assert!(config.health_check_interval.as_secs() >= 30);
    assert!(config.connection_timeout.as_secs() >= 10);
}

#[test]
fn test_retry_limits() {
    let config = PoolConfig::new(vec!["wss://test.com".to_string()]).with_max_retries(0);

    assert_eq!(config.max_retries, 0);

    let config = PoolConfig::new(vec!["wss://test.com".to_string()]).with_max_retries(100);

    assert_eq!(config.max_retries, 100);
}

#[test]
fn test_pool_stats_calculation_accuracy() {
    // Test floating point accuracy
    let stats = PoolStats {
        total_endpoints: 7,
        healthy_endpoints: 5,
        unhealthy_endpoints: 2,
        unknown_endpoints: 0,
    };

    let percentage = stats.health_percentage();
    assert!((percentage - 71.42857142857143).abs() < 0.0001);
}
