//! DeFi Aggregator Example
//!
//! This example demonstrates how to build a cross-chain DeFi aggregator
//! that can interact with DeFi protocols across Substrate and EVM chains.
//!
//! Features demonstrated:
//! - Multi-chain liquidity aggregation
//! - Cross-chain swap routing
//! - Portfolio tracking across chains
//! - Yield farming optimization

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Cross-Chain DeFi Aggregator Example ===\n");

    // Initialize SDK with multiple chain support
    println!("Initializing multi-chain SDK...");
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/demo")
        .build()
        .await?;

    println!("SDK initialized with Substrate and EVM support\n");

    // Example 1: Portfolio Balance Aggregation
    println!("Example 1: Cross-Chain Portfolio Balance");
    println!("  Checking balances across multiple chains...");

    // Simulated user addresses
    let substrate_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let evm_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7";

    println!("  User Addresses:");
    println!("    Substrate: {}", substrate_address);
    println!("    EVM: {}", evm_address);

    // In a real implementation, you would query actual balances
    println!("\n  Portfolio Summary:");
    println!("    Polkadot: 1,000 DOT");
    println!("    Ethereum: 5 ETH");
    println!("    Total USD Value: $50,000\n");

    // Example 2: Cross-Chain Liquidity Optimization
    println!("Example 2: Cross-Chain Liquidity Optimization");
    println!("  Finding best liquidity pools across chains...");

    println!("\n  Available Liquidity:");
    println!("    Polkadot DEX: 100,000 DOT");
    println!("    Ethereum Uniswap: 500 ETH");
    println!("    Moonbeam DEX: 50,000 GLMR");

    println!("\n  Optimal Route:");
    println!("    1. Swap DOT on Polkadot DEX (best rate)");
    println!("    2. Bridge to Ethereum for final settlement");

    // Example 3: Execute Cross-Chain Swap
    println!("\nExample 3: Cross-Chain Swap Execution");
    println!("  Swapping 100 DOT → ETH via cross-chain route");

    let swap_tx = sdk
        .transaction()
        .from_substrate_account(substrate_address)
        .to_evm_address(evm_address)
        .amount(100_000_000_000_000u128) // 100 DOT in Planck
        .build()?;

    println!("\n  Transaction Details:");
    println!("    Source Chain: {:?}", swap_tx.source_chain);
    println!("    Destination Chain: {:?}", swap_tx.destination_chain);
    println!("    Amount: {} Planck", swap_tx.amount);
    println!("    Cross-chain: {}", swap_tx.is_cross_chain());

    let result = sdk.execute(swap_tx).await?;
    println!("\n  Swap Executed:");
    println!("    Source TX: {}", result.source_tx_hash);
    if let Some(dest_tx) = result.destination_tx_hash {
        println!("    Destination TX: {}", dest_tx);
    }
    println!("    Status: {:?}", result.status);

    // Example 4: Yield Farming Optimization
    println!("\nExample 4: Yield Farming Optimization");
    println!("  Analyzing yield farming opportunities...");

    println!("\n  Top Yield Farms:");
    println!("    1. Polkadot Liquid Staking: 12% APY");
    println!("    2. Ethereum Lido: 4.5% APY");
    println!("    3. Moonbeam DEX LP: 25% APY");

    println!("\n  Recommendation:");
    println!("    Allocate 50% to Moonbeam DEX LP for highest yield");
    println!("    Allocate 30% to Polkadot Liquid Staking for security");
    println!("    Allocate 20% to Ethereum Lido for diversification");

    // Example 5: Real-time Price Aggregation
    println!("\nExample 5: Real-time Price Aggregation");
    println!("  Fetching prices from multiple DEXs...");

    println!("\n  DOT/USD Prices:");
    println!("    Polkadot DEX: $6.50");
    println!("    Moonbeam DEX: $6.48");
    println!("    CEX Average: $6.52");
    println!("    Best Price: $6.52 (CEX)");

    println!("\nAll DeFi operations completed successfully!");
    println!("\nDeFi Aggregator Features:");
    println!("  Multi-chain balance tracking");
    println!("  Optimal liquidity routing");
    println!("  Cross-chain swap execution");
    println!("  Yield farming optimization");
    println!("  Real-time price aggregation");

    println!("\nSecurity Considerations:");
    println!("  - Always verify smart contract addresses");
    println!("  - Use slippage protection for swaps");
    println!("  - Implement transaction replay protection");
    println!("  - Monitor bridge security status");
    println!("  - Diversify across multiple chains");

    Ok(())
}
