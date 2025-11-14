//! Performance benchmarks for Apex SDK transaction operations

use apex_sdk::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

/// Benchmark transaction builder creation
fn bench_transaction_builder_creation(c: &mut Criterion) {
    c.bench_function("transaction_builder_new", |b| {
        b.iter(|| black_box(TransactionBuilder::new()))
    });
}

/// Benchmark EVM to EVM transaction building
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

/// Benchmark Substrate to Substrate transaction building
fn bench_substrate_to_substrate_transaction(c: &mut Criterion) {
    c.bench_function("build_substrate_to_substrate_transaction", |b| {
        b.iter(|| {
            TransactionBuilder::new()
                .from_substrate_account(black_box(
                    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                ))
                .to_substrate_account(black_box(
                    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                ))
                .amount(black_box(1000))
                .build()
                .unwrap()
        })
    });
}

/// Benchmark cross-chain transaction building
fn bench_cross_chain_transaction(c: &mut Criterion) {
    c.bench_function("build_cross_chain_transaction", |b| {
        b.iter(|| {
            TransactionBuilder::new()
                .from_substrate_account(black_box(
                    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                ))
                .to_evm_address(black_box("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
                .amount(black_box(1000))
                .build()
                .unwrap()
        })
    });
}

/// Benchmark transaction building with data
fn bench_transaction_with_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_with_data");

    for size in [32, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                TransactionBuilder::new()
                    .from_evm_address(black_box("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
                    .to_evm_address(black_box("0x1234567890123456789012345678901234567890"))
                    .amount(black_box(1000))
                    .with_data(black_box(data.clone()))
                    .build()
                    .unwrap()
            })
        });
    }

    group.finish();
}

/// Benchmark transaction hash computation
fn bench_transaction_hash(c: &mut Criterion) {
    let tx = TransactionBuilder::new()
        .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .to_evm_address("0x1234567890123456789012345678901234567890")
        .amount(1000)
        .build()
        .unwrap();

    c.bench_function("transaction_hash", |b| b.iter(|| black_box(tx.hash())));
}

/// Benchmark is_cross_chain check
fn bench_is_cross_chain(c: &mut Criterion) {
    let tx_same_chain = TransactionBuilder::new()
        .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .to_evm_address("0x1234567890123456789012345678901234567890")
        .amount(1000)
        .build()
        .unwrap();

    let tx_cross_chain = TransactionBuilder::new()
        .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .amount(1000)
        .build()
        .unwrap();

    c.bench_function("is_cross_chain_same", |b| {
        b.iter(|| black_box(tx_same_chain.is_cross_chain()))
    });

    c.bench_function("is_cross_chain_different", |b| {
        b.iter(|| black_box(tx_cross_chain.is_cross_chain()))
    });
}

/// Benchmark transaction serialization
fn bench_transaction_serialization(c: &mut Criterion) {
    let tx = TransactionBuilder::new()
        .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .to_evm_address("0x1234567890123456789012345678901234567890")
        .amount(1000)
        .with_data(vec![1, 2, 3, 4, 5, 6, 7, 8])
        .with_gas_limit(21000)
        .build()
        .unwrap();

    c.bench_function("transaction_serialize", |b| {
        b.iter(|| black_box(serde_json::to_string(&tx).unwrap()))
    });

    let json = serde_json::to_string(&tx).unwrap();
    c.bench_function("transaction_deserialize", |b| {
        b.iter(|| black_box(serde_json::from_str::<Transaction>(&json).unwrap()))
    });
}

/// Benchmark builder with varying amounts
fn bench_transaction_amounts(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_amounts");

    for amount in [1u128, 1_000, 1_000_000, 1_000_000_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(amount), amount, |b, &amt| {
            b.iter(|| {
                TransactionBuilder::new()
                    .from_evm_address(black_box("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
                    .to_evm_address(black_box("0x1234567890123456789012345678901234567890"))
                    .amount(black_box(amt))
                    .build()
                    .unwrap()
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_transaction_builder_creation,
    bench_evm_to_evm_transaction,
    bench_substrate_to_substrate_transaction,
    bench_cross_chain_transaction,
    bench_transaction_with_data,
    bench_transaction_hash,
    bench_is_cross_chain,
    bench_transaction_serialization,
    bench_transaction_amounts,
);

criterion_main!(benches);
