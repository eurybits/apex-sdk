//! Basic cross-chain transfer example

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Basic Cross-Chain Transfer Example ===\n");

    // Note: This is a demonstration. In production, use real endpoints.
    println!("Initializing Apex SDK...");

    // Uncomment and configure with your endpoints:
    // let sdk = ApexSDK::builder()
    //     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    //     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    //     .build()
    //     .await?;

    // println!("SDK initialized");

    // Example: Check chain support
    // let supported = sdk.is_chain_supported(&Chain::Ethereum);
    // println!("Ethereum supported: {}", supported);

    println!("\nExample code structure:");
    println!("  1. Initialize SDK with endpoints");
    println!("  2. Build transaction");
    println!("  3. Execute cross-chain transfer");
    println!("  4. Monitor transaction status");

    Ok(())
}
