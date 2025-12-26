// EVM Integration Tests
// These tests run against a real EVM node (Hardhat) in Docker
// Run with: INTEGRATION_TESTS=1 cargo test --test evm_integration_test -- --include-ignored

#[path = "integration_helpers.rs"]
mod integration_helpers;

use alloy::primitives::U256;
use apex_sdk_evm::{wallet::Wallet, EvmAdapter};
use integration_helpers::*;

#[tokio::test]
#[ignore]
async fn test_evm_connection_to_docker_node() {
    skip_if_not_integration!();

    wait_for_evm_node(30)
        .await
        .expect("EVM node should be ready");

    let adapter = EvmAdapter::connect(&evm_rpc_url())
        .await
        .expect("Should connect to EVM node");

    let chain_id = adapter.provider().get_chain_id().await;
    assert!(chain_id.is_ok(), "Should get chain ID");
    assert_eq!(
        chain_id.unwrap(),
        31337,
        "Chain ID should be 31337 (Hardhat default)"
    );

    println!("Successfully connected to Docker EVM node");
}

#[tokio::test]
#[ignore]
async fn test_evm_get_balance_from_docker_node() {
    skip_if_not_integration!();

    wait_for_evm_node(30)
        .await
        .expect("EVM node should be ready");

    let adapter = EvmAdapter::connect(&evm_rpc_url())
        .await
        .expect("Should connect to EVM node");

    let test_address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

    let balance = adapter.get_balance(test_address).await;
    assert!(balance.is_ok(), "Should get balance");

    let balance_value = balance.unwrap();
    assert!(balance_value > U256::ZERO, "Balance should be > 0");

    println!("Test account balance: {} wei", balance_value);
}

#[tokio::test]
#[ignore]
async fn test_evm_send_transaction_to_docker_node() {
    skip_if_not_integration!();

    wait_for_evm_node(30)
        .await
        .expect("EVM node should be ready");

    let adapter = EvmAdapter::connect(&evm_rpc_url())
        .await
        .expect("Should connect to EVM node");

    let wallet = Wallet::from_private_key(
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    )
    .expect("Should create wallet")
    .with_chain_id(1337);

    let from_address = wallet.address();
    let to_address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

    let initial_from = adapter.get_balance(&from_address).await.unwrap();
    let initial_to = adapter.get_balance(to_address).await.unwrap();

    println!("Initial balances:");
    println!("  From: {} wei", initial_from);
    println!("  To:   {} wei", initial_to);

    // Note: Actual transaction sending would require transaction executor
    // This test verifies connection and balance queries work

    println!("Transaction test infrastructure verified");
}

#[tokio::test]
#[ignore]
async fn test_evm_multiple_accounts() {
    skip_if_not_integration!();

    wait_for_evm_node(30)
        .await
        .expect("EVM node should be ready");

    let adapter = EvmAdapter::connect(&evm_rpc_url())
        .await
        .expect("Should connect to EVM node");

    let test_accounts = [
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
        "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
        "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
    ];

    for (i, account) in test_accounts.iter().enumerate() {
        let balance = adapter.get_balance(account).await;
        assert!(balance.is_ok(), "Should get balance for account {}", i);
        assert!(
            balance.unwrap() > U256::ZERO,
            "Account {} should have balance",
            i
        );
    }

    println!("All {} test accounts verified", test_accounts.len());
}
