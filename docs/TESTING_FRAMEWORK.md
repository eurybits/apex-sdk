# Apex SDK Testing Framework Guide

This guide provides comprehensive information about the testing framework, strategies, and best practices for the Apex SDK.

## Table of Contents

- [Overview](#overview)
- [Test Types](#test-types)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Property-Based Testing](#property-based-testing)
- [Benchmark Testing](#benchmark-testing)
- [Test Coverage](#test-coverage)
- [Best Practices](#best-practices)
- [Continuous Integration](#continuous-integration)

## Overview

The Apex SDK uses a comprehensive multi-layered testing approach that includes:

- **Unit Tests**: Test individual functions and modules in isolation
- **Integration Tests**: Test interactions between components
- **Property-Based Tests**: Verify behavior across a wide range of inputs using `proptest`
- **Benchmark Tests**: Measure and track performance using `criterion`
- **Documentation Tests**: Ensure code examples in documentation work correctly

### Test Statistics

Current test coverage:
- **42 unit tests** across all core modules
- **13 property-based tests** for input validation
- **9 benchmark suites** for performance tracking

## Test Types

### 1. Unit Tests

Unit tests are located within the source files using `#[cfg(test)]` modules.

**Location**: `apex-sdk/src/*.rs` (inline with source code)

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_builder_new() {
        let builder = TransactionBuilder::new();
        assert!(builder.from.is_none());
        assert!(builder.to.is_none());
    }

    #[tokio::test]
    async fn test_builder_with_evm_endpoint() {
        let builder = ApexSDKBuilder::new()
            .with_evm_endpoint("https://test.ethereum.io");
        assert_eq!(builder.evm_endpoint, Some("https://test.ethereum.io".to_string()));
    }
}
```

**Coverage**:
- `error.rs`: Error handling and display formatting (9 tests)
- `builder.rs`: SDK builder configuration (7 tests)
- `sdk.rs`: Core SDK functionality (10 tests)
- `transaction.rs`: Transaction building and validation (16 tests)

### 2. Integration Tests

Integration tests verify that components work together correctly.

**Location**: `apex-sdk/tests/integration_tests.rs`

**Example**:
```rust
#[tokio::test]
async fn test_cross_chain_transaction_execution() {
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://test.substrate.io")
        .with_evm_endpoint("https://test.ethereum.io")
        .build()
        .await
        .unwrap();

    let tx = sdk
        .transaction()
        .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .amount(1_000_000)
        .build()
        .unwrap();

    let result = sdk.execute(tx).await;
    assert!(result.is_ok());
}
```

**Test Scenarios**:
- SDK initialization with different adapter configurations
- Transaction building for same-chain and cross-chain operations
- Transaction execution and status queries
- Chain support validation
- Address validation

### 3. Property-Based Tests

Property-based tests use `proptest` to verify behavior across a wide range of inputs.

**Location**: `apex-sdk/tests/property_tests.rs`

**Example**:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_evm_to_evm_transaction_always_succeeds(
        from in evm_address_strategy(),
        to in evm_address_strategy(),
        amount in amount_strategy()
    ) {
        let tx = TransactionBuilder::new()
            .from_evm_address(&from)
            .to_evm_address(&to)
            .amount(amount)
            .build();

        prop_assert!(tx.is_ok());
        prop_assert_eq!(tx.unwrap().amount, amount);
    }
}
```

**Test Properties**:
- Transaction creation always succeeds with valid inputs
- Amount preservation through serialization
- Cross-chain detection accuracy
- Data preservation in transactions
- Error conditions for missing fields

### 4. Benchmark Tests

Performance benchmarks use `criterion` to track and measure performance.

**Location**: `apex-sdk/benches/transaction_benchmarks.rs`

**Example**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_evm_to_evm_transaction(c: &mut Criterion) {
    c.bench_function("build_evm_to_evm_transaction", |b| {
        b.iter(|| {
            TransactionBuilder::new()
                .from_evm_address(black_box("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
                .to_evm_address(black_box("0x1234567890123456789012345678901234567890"))
                .amount(black_box(1000))
                .build()
                .unwrap()
        })
    });
}

criterion_group!(benches, bench_evm_to_evm_transaction);
criterion_main!(benches);
```

**Benchmark Suites**:
- Transaction builder creation
- EVM and Substrate transaction building
- Cross-chain transaction building
- Transaction with varying data sizes
- Hash computation
- Serialization/deserialization

## Running Tests

### All Tests

Run all tests (unit, integration, and doc tests):
```bash
cargo test --workspace
```

### Specific Test Suites

Run only unit tests:
```bash
cargo test --lib
```

Run only integration tests:
```bash
cargo test --test integration_tests
```

Run property-based tests:
```bash
cargo test --test property_tests
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

### Benchmarks

Run all benchmarks:
```bash
cargo bench
```

Run specific benchmark:
```bash
cargo bench --bench transaction_benchmarks
```

Generate benchmark report:
```bash
cargo bench -- --save-baseline my-baseline
```

### Test Coverage

Generate coverage report using `tarpaulin`:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

Generate detailed coverage:
```bash
cargo tarpaulin --out Html --output-dir coverage --all-features --workspace
```

View coverage report:
```bash
open coverage/index.html
```

## Writing Tests

### Unit Test Guidelines

1. **Test one thing at a time**: Each test should verify a single behavior
2. **Use descriptive names**: Test names should clearly indicate what is being tested
3. **Follow AAA pattern**: Arrange, Act, Assert
4. **Test edge cases**: Zero values, maximum values, empty collections
5. **Test error paths**: Verify error handling works correctly

**Example**:
```rust
#[test]
fn test_transaction_builder_missing_sender() {
    // Arrange
    let builder = TransactionBuilder::new()
        .to_evm_address("0x1234567890123456789012345678901234567890")
        .amount(100);

    // Act
    let result = builder.build();

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::Transaction(msg)) => {
            assert!(msg.contains("Sender address required"));
        }
        _ => panic!("Expected Transaction error"),
    }
}
```

### Async Test Guidelines

For async tests, use `#[tokio::test]`:

```rust
#[tokio::test]
async fn test_sdk_initialization() {
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://test.ethereum.io")
        .build()
        .await;

    assert!(sdk.is_ok());
}
```

### Property Test Guidelines

1. **Define input strategies**: Create strategies for generating valid test data
2. **Test invariants**: Properties that should always hold true
3. **Use appropriate sample sizes**: Balance thoroughness with test speed
4. **Test failure modes**: Verify error conditions with invalid inputs

**Example Strategy**:
```rust
fn evm_address_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("0x[0-9a-fA-F]{40}")
        .expect("regex should be valid")
}

fn amount_strategy() -> impl Strategy<Value = u128> {
    1u128..=1_000_000_000_000u128
}
```

### Benchmark Guidelines

1. **Use `black_box`**: Prevent compiler optimizations from skewing results
2. **Benchmark realistic scenarios**: Use typical input sizes and patterns
3. **Isolate what you're measuring**: Set up data outside the benchmark closure
4. **Establish baselines**: Save baselines for regression detection

**Example**:
```rust
fn bench_with_setup(c: &mut Criterion) {
    // Setup outside the benchmark
    let data = vec![0u8; 1024];

    c.bench_function("my_function", |b| {
        b.iter(|| {
            my_function(black_box(&data))
        })
    });
}
```

## Property-Based Testing

### When to Use Property-Based Tests

Use property-based tests when:
- Testing functions with many possible inputs
- Verifying invariants that should always hold
- Finding edge cases you might not have thought of
- Testing serialization/deserialization round-trips
- Validating parser or builder patterns

### Common Properties to Test

1. **Idempotence**: Applying operation twice yields same result
2. **Round-trip**: Serialize then deserialize yields original value
3. **Invariants**: Properties that never change (e.g., sorted list stays sorted)
4. **Relationship**: Output relates to input in predictable way
5. **Error conditions**: Invalid inputs always produce errors

### Example Properties

```rust
proptest! {
    // Round-trip property
    #[test]
    fn test_serialization_roundtrip(tx in any::<Transaction>()) {
        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(tx, deserialized);
    }

    // Invariant property
    #[test]
    fn test_amount_always_positive(
        from in evm_address_strategy(),
        to in evm_address_strategy(),
        amount in 1u128..=u128::MAX
    ) {
        let tx = TransactionBuilder::new()
            .from_evm_address(&from)
            .to_evm_address(&to)
            .amount(amount)
            .build()
            .unwrap();

        prop_assert!(tx.amount > 0);
    }
}
```

## Benchmark Testing

### Understanding Benchmark Output

Criterion provides detailed statistics:

```
build_evm_to_evm_transaction
                        time:   [1.2345 µs 1.2456 µs 1.2567 µs]
                        change: [-2.3% -1.5% -0.7%] (p = 0.00 < 0.05)
                        Performance has improved.
```

- **time**: Mean execution time with confidence interval
- **change**: Percentage change from previous run
- **p-value**: Statistical significance of the change

### Interpreting Results

- **Green "Performance has improved"**: Faster than baseline
- **Red "Performance has regressed"**: Slower than baseline
- **Yellow "No change"**: Within statistical noise

### Best Practices

1. **Run benchmarks on stable hardware**: Avoid running on battery or under load
2. **Use consistent conditions**: Same CPU governor, temperature, background processes
3. **Establish baselines**: Save baselines for each release
4. **Track trends**: Monitor performance over time
5. **Profile slow benchmarks**: Use `cargo flamegraph` or `perf` for detailed analysis

## Test Coverage

### Coverage Goals

- **Overall**: Target 80%+ code coverage
- **Critical paths**: 100% coverage for error handling and validation
- **Public API**: 100% coverage for all public functions
- **Internal implementation**: 70%+ coverage

### Viewing Coverage

Generate and view HTML coverage report:
```bash
cargo tarpaulin --out Html --output-dir coverage
open coverage/index.html
```

### Coverage in CI

Coverage is automatically tracked in CI using Codecov. View coverage reports at:
`https://codecov.io/gh/apex-protocol/apex-sdk`

## Best Practices

### General Testing Principles

1. **Write tests first** (TDD): Define behavior before implementation
2. **Test behavior, not implementation**: Focus on what, not how
3. **Keep tests independent**: Tests should not depend on each other
4. **Use descriptive names**: `test_transaction_builder_rejects_missing_sender`
5. **Test edge cases**: Empty, zero, maximum, negative values
6. **Test error paths**: Ensure errors are properly handled

### Code Organization

```rust
// 1. Module imports
use super::*;

// 2. Test helpers
fn create_test_transaction() -> Transaction { ... }

// 3. Unit tests
#[test]
fn test_basic_functionality() { ... }

// 4. Integration tests
#[tokio::test]
async fn test_end_to_end_flow() { ... }

// 5. Property tests
proptest! { ... }
```

### Async Testing

```rust
// Use tokio::test for async tests
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}

// Use timeout for potentially hanging operations
use tokio::time::timeout;
use std::time::Duration;

#[tokio::test]
async fn test_with_timeout() {
    let result = timeout(
        Duration::from_secs(5),
        potentially_slow_operation()
    ).await;
    assert!(result.is_ok());
}
```

### Error Testing

```rust
#[test]
fn test_error_handling() {
    let result = function_that_fails();

    // Method 1: Check for error
    assert!(result.is_err());

    // Method 2: Match specific error variant
    match result {
        Err(Error::Transaction(msg)) => {
            assert!(msg.contains("expected text"));
        }
        _ => panic!("Expected Transaction error"),
    }

    // Method 3: Use matches! macro (stable)
    assert!(matches!(result, Err(Error::Transaction(_))));

    // Alternatively, for assert_matches! on stable, add the `assert_matches` crate:
    // use assert_matches::assert_matches;
    // assert_matches!(result, Err(Error::Transaction(_)));
}
```

## Continuous Integration

### CI Pipeline

The CI pipeline runs automatically on:
- Every push to `main` and `dev` branches
- Every pull request
- Nightly builds

### CI Test Stages

1. **Format Check**: `cargo fmt --check`
2. **Linting**: `cargo clippy --all-targets`
3. **Unit Tests**: `cargo test --lib`
4. **Integration Tests**: `cargo test --test '*'`
5. **Doc Tests**: `cargo test --doc`
6. **Coverage**: `cargo tarpaulin`
7. **Benchmarks**: `cargo bench` (on main branch only)

### Platform Matrix

Tests run on:
- **Operating Systems**: Linux (Ubuntu), macOS, Windows
- **Rust Versions**: stable, beta, nightly, MSRV (1.75.0)

### Viewing CI Results

Check CI status:
- GitHub Actions: `.github/workflows/ci.yml`
- Coverage: Codecov dashboard
- Benchmarks: GitHub Pages benchmark history

## Troubleshooting

### Common Issues

**Problem**: Tests fail intermittently
- **Solution**: Check for race conditions, use proper async synchronization

**Problem**: Benchmarks show high variance
- **Solution**: Run on dedicated hardware, increase sample size

**Problem**: Property tests fail with specific input
- **Solution**: Add that input as a unit test, adjust strategy

**Problem**: Coverage is lower than expected
- **Solution**: Check for untested error paths and edge cases

### Debug Tips

```rust
// Print debug output in tests
#[test]
fn test_with_debug() {
    let value = calculate_something();
    dbg!(&value); // Prints debug info
    assert_eq!(value, expected);
}

// Run single test with output
// cargo test test_name -- --nocapture

// Run tests with backtrace
// RUST_BACKTRACE=1 cargo test
```

## Resources

### Documentation
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [proptest Documentation](https://docs.rs/proptest/)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/)

### Tools
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) - Code coverage
- [cargo-nextest](https://nexte.st/) - Faster test runner
- [cargo-watch](https://github.com/watchexec/cargo-watch) - Auto-run tests on change

### Internal Documentation
- `TESTING.md` - General testing guide
- `CONTRIBUTING.md` - Contribution guidelines
- `CI_CD.md` - CI/CD pipeline documentation

## Conclusion

The Apex SDK testing framework provides comprehensive coverage through multiple testing strategies:

- **Unit tests** ensure individual components work correctly
- **Integration tests** verify components work together
- **Property-based tests** find edge cases automatically
- **Benchmarks** track performance over time

By following these guidelines and best practices, you can maintain high code quality and catch issues early in the development cycle.

For questions or suggestions, please open an issue on GitHub.
