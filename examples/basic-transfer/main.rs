//! Basic cross-chain transfer example
//!
//! This example demonstrates how to:
//! - Initialize the Apex SDK with both Substrate and EVM endpoints
//! - Create a cross-chain transaction
//! - Execute the transaction
//! - Monitor transaction status

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    println!("=== Basic Cross-Chain Transfer Example ===\n");

    // Note: This is a demonstration using mock endpoints
    // In production, replace with real endpoints
    println!("Initializing Apex SDK with test endpoints...");

    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot-test.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
        .build()
        .await?;

    println!("SDK initialized successfully");

    // Check which chains are supported
    println!("\nChecking chain support:");
    println!("  Polkadot: {}", sdk.is_chain_supported(&Chain::Polkadot));
    println!("  Ethereum: {}", sdk.is_chain_supported(&Chain::Ethereum));
    println!("  Moonbeam: {}", sdk.is_chain_supported(&Chain::Moonbeam));

    // Example 1: Same-chain transfer (EVM to EVM)
    println!("\nExample 1: EVM to EVM Transfer");
    let tx1 = sdk
        .transaction()
        .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .to_evm_address("0x1234567890123456789012345678901234567890")
        .amount(1_000_000_000_000_000_000u128) // 1 ETH in wei
        .with_gas_limit(21000)
        .build()?;

    println!("  Transaction built:");
    println!("    From: {}", tx1.from.as_str());
    println!("    To: {}", tx1.to.as_str());
    println!("    Amount: {} wei", tx1.amount);
    println!("    Cross-chain: {}", tx1.is_cross_chain());

    let result1 = sdk.execute(tx1).await?;
    println!("  Transaction executed:");
    println!("    Source TX: {}", result1.source_tx_hash);
    println!("    Status: {:?}", result1.status);

    // Example 2: Cross-chain transfer (Substrate to EVM)
    println!("\nExample 2: Cross-Chain Transfer (Substrate → EVM)");
    let tx2 = sdk
        .transaction()
        .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .amount(5_000_000_000_000) // 5 DOT (in Planck)
        .build()?;

    println!("  Transaction built:");
    println!("    From: {}", tx2.from.as_str());
    println!("    To: {}", tx2.to.as_str());
    println!("    Amount: {} Planck", tx2.amount);
    println!("    Cross-chain: {}", tx2.is_cross_chain());

    let result2 = sdk.execute(tx2).await?;
    println!("  Transaction executed:");
    println!("    Source TX: {}", result2.source_tx_hash);
    if let Some(dest_tx) = result2.destination_tx_hash {
        println!("    Destination TX: {}", dest_tx);
    }
    println!("    Status: {:?}", result2.status);

    // Example 3: Query transaction status
    println!("\nExample 3: Query Transaction Status");
    let status = sdk
        .get_transaction_status(&Chain::Ethereum, &result1.source_tx_hash)
        .await?;
    println!("  Transaction status: {:?}", status);

    println!("\nAll examples completed successfully!");
    println!("\nNext steps:");
    println!("  - Replace test endpoints with production endpoints");
    println!("  - Use real account addresses and private keys");
    println!("  - Implement proper error handling");
    println!("  - Add transaction signing");

    Ok(())
}
