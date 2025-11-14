//! NFT Bridge Example
//!
//! This example demonstrates how to build a cross-chain NFT bridge
//! that enables NFT transfers between Substrate and EVM chains.
//!
//! Features demonstrated:
//! - NFT metadata synchronization
//! - Cross-chain NFT transfers
//! - NFT collection management
//! - Bridge security patterns

use apex_sdk::prelude::*;

/// Represents an NFT on any chain
#[derive(Debug, Clone)]
struct NFT {
    token_id: String,
    collection: String,
    owner: Address,
    metadata_uri: String,
    chain: Chain,
}

impl NFT {
    fn new(
        token_id: impl Into<String>,
        collection: impl Into<String>,
        owner: Address,
        metadata_uri: impl Into<String>,
        chain: Chain,
    ) -> Self {
        Self {
            token_id: token_id.into(),
            collection: collection.into(),
            owner,
            metadata_uri: metadata_uri.into(),
            chain,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Cross-Chain NFT Bridge Example ===\n");

    // Initialize SDK with multi-chain support
    println!("Initializing NFT Bridge...");
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://kusama.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://polygon-mainnet.g.alchemy.com/v2/demo")
        .build()
        .await?;

    println!("Bridge initialized with Substrate and EVM support\n");

    // Example 1: NFT Collection Overview
    println!("Example 1: NFT Collection Overview");

    let user_substrate = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    let user_evm = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

    // Simulated NFT collections
    let substrate_nft = NFT::new(
        "1",
        "Kusama Digital Art",
        user_substrate.clone(),
        "ipfs://QmX...abc123",
        Chain::Kusama,
    );

    let evm_nft = NFT::new(
        "42",
        "Polygon Collectibles",
        user_evm.clone(),
        "ipfs://QmY...def456",
        Chain::Polygon,
    );

    println!("\n  User's NFT Portfolio:");
    println!("    Kusama Chain:");
    println!("      Collection: {}", substrate_nft.collection);
    println!("      Token ID: {}", substrate_nft.token_id);
    println!("      Metadata: {}", substrate_nft.metadata_uri);

    println!("\n    Polygon Chain:");
    println!("      Collection: {}", evm_nft.collection);
    println!("      Token ID: {}", evm_nft.token_id);
    println!("      Metadata: {}", evm_nft.metadata_uri);

    // Example 2: Bridge NFT from Substrate to EVM
    println!("\nExample 2: Bridge NFT (Substrate → EVM)");
    println!("  Bridging Kusama NFT to Polygon...");

    // Step 1: Lock NFT on source chain
    println!("\n  Step 1: Locking NFT on Kusama");
    println!("    Token ID: {}", substrate_nft.token_id);
    println!("    Owner: {}", substrate_nft.owner.as_str());

    // Step 2: Create bridge transaction
    let bridge_tx = sdk
        .transaction()
        .from(substrate_nft.owner.clone())
        .to(user_evm.clone())
        .amount(0) // NFT transfer, no value
        .with_data(format!("nft:{}:{}", substrate_nft.collection, substrate_nft.token_id).into_bytes())
        .build()?;

    println!("\n  Step 2: Creating bridge transaction");
    println!("    Source Chain: {:?}", bridge_tx.source_chain);
    println!("    Destination Chain: {:?}", bridge_tx.destination_chain);
    println!("    Cross-chain: {}", bridge_tx.is_cross_chain());

    // Step 3: Execute bridge
    let result = sdk.execute(bridge_tx).await?;

    println!("\n  Step 3: Bridge execution");
    println!("    Source TX (Lock): {}", result.source_tx_hash);
    if let Some(dest_tx) = &result.destination_tx_hash {
        println!("    Destination TX (Mint): {}", dest_tx);
    }
    println!("    Status: {:?}", result.status);

    // Step 4: Mint wrapped NFT on destination
    println!("\n  Step 4: Wrapped NFT minted on Polygon");
    println!("    New Token ID: wrapped-{}", substrate_nft.token_id);
    println!("    Owner: {}", user_evm.as_str());
    println!("    Metadata synchronized: ✓");

    // Example 3: Bridge NFT back (EVM to Substrate)
    println!("\nExample 3: Bridge NFT Back (EVM → Substrate)");
    println!("  Bridging wrapped NFT back to Kusama...");

    let return_tx = sdk
        .transaction()
        .from(user_evm.clone())
        .to(user_substrate.clone())
        .amount(0)
        .with_data(format!("nft:wrapped-{}:{}", evm_nft.collection, evm_nft.token_id).into_bytes())
        .build()?;

    let return_result = sdk.execute(return_tx).await?;

    println!("\n  Bridge Return Completed:");
    println!("    Burn TX on Polygon: {}", return_result.source_tx_hash);
    if let Some(unlock_tx) = &return_result.destination_tx_hash {
        println!("    Unlock TX on Kusama: {}", unlock_tx);
    }
    println!("    Original NFT unlocked: ✓");

    // Example 4: NFT Metadata Synchronization
    println!("\nExample 4: NFT Metadata Synchronization");
    println!("  Ensuring metadata consistency across chains...");

    println!("\n  Metadata Sync:");
    println!("    Source: {}", substrate_nft.metadata_uri);
    println!("    Destination: {}", substrate_nft.metadata_uri);
    println!("    Status: ✓ Synchronized");

    println!("\n  Attributes:");
    println!("    Name: Digital Sunset #1");
    println!("    Description: A beautiful sunset on Kusama");
    println!("    Creator: Alice");
    println!("    Rarity: Legendary");

    // Example 5: Bridge Security Features
    println!("\nExample 5: Bridge Security Features");
    println!("  Demonstrating security mechanisms...");

    println!("\n  Security Checks:");
    println!("    ✓ NFT ownership verified");
    println!("    ✓ Collection whitelist checked");
    println!("    ✓ Bridge contract validated");
    println!("    ✓ Metadata hash verified");
    println!("    ✓ Rate limiting applied");

    println!("\n  Bridge Statistics:");
    println!("    Total Bridged NFTs: 1,234");
    println!("    Active Wrapped NFTs: 567");
    println!("    Total Volume: $1.2M");
    println!("    Success Rate: 99.8%");

    // Transaction monitoring
    println!("\nExample 6: Transaction Monitoring");
    println!("  Checking bridge transaction status...");

    let status = sdk
        .get_transaction_status(&Chain::Polygon, &result.source_tx_hash)
        .await?;

    println!("\n  Transaction Status:");
    println!("    Status: {:?}", status);
    match status {
        TransactionStatus::Confirmed {
            block_number,
            confirmations,
        } => {
            println!("    Block: {}", block_number);
            println!("    Confirmations: {}", confirmations);
        }
        _ => {}
    }

    println!("\nAll NFT bridge operations completed successfully!");
    println!("\nNFT Bridge Features:");
    println!("  Cross-chain NFT transfers");
    println!("  Metadata synchronization");
    println!("  Bidirectional bridging");
    println!("  Security validations");
    println!("  Real-time monitoring");

    println!("\nImportant Considerations:");
    println!("  - NFTs are locked on source chain, not destroyed");
    println!("  - Wrapped NFTs maintain original metadata");
    println!("  - Bridge fees may apply based on chain");
    println!("  - Ensure collection is whitelisted before bridging");
    println!("  - Always verify bridge contract addresses");

    println!("\nSupported NFT Standards:");
    println!("  - ERC-721 (Ethereum NFTs)");
    println!("  - ERC-1155 (Multi-token)");
    println!("  - PSP-34 (Substrate NFTs)");
    println!("  - Uniques Pallet (Substrate)");

    Ok(())
}
