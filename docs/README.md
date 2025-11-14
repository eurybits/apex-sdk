# Apex SDK Documentation

Welcome to the Apex SDK documentation! This site provides comprehensive guides and API references for building cross-chain applications with Apex SDK.

## Navigation

- **[Quick Start Guide](QUICK_START.md)** - Get started in under 5 minutes
- **[API Reference](API.md)** - Complete API documentation
- **[Contributing Guide](CONTRIBUTING.md)** - Learn how to contribute
- **[Security Policy](SECURITY.md)** - Security reporting and policies

## What is Apex SDK?

Apex SDK is the industry's first unified Rust SDK for Substrate and EVM blockchain development. It provides:

- **Unified Interface** - Single API for both Substrate and EVM blockchains
- **Compile-Time Safety** - Catch errors before deployment with Rust's type system
- **Native Performance** - Up to 6x faster than JavaScript alternatives
- **Cross-Chain Ready** - Built-in support for cross-chain communication

## Quick Example

```rust
use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
        .build()
        .await?;

    let tx = sdk
        .transaction()
        .from_substrate_account("5GrwvaEF...")
        .to_evm_address("0x742d35Cc...")
        .amount(1000)
        .build()?;

    let result = sdk.execute(tx).await?;
    println!("Transaction: {:?}", result);

    Ok(())
}
```

## Supported Chains

### Substrate
- Polkadot
- Kusama

### EVM
- Ethereum
- Binance Smart Chain
- Polygon
- Avalanche

### Hybrid
- Moonbeam
- Astar

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk = "0.1.0"
tokio = { version = "1.35", features = ["full"] }
```

Or use the CLI:

```bash
cargo install apex-sdk-cli
apex new my-project
```

## Examples

Check out our comprehensive examples:

- [Basic Transfer](https://github.com/kherldhussein/apex-sdk/tree/main/examples/basic-transfer) - Simple cross-chain transfers
- [DeFi Aggregator](https://github.com/kherldhussein/apex-sdk/tree/main/examples/defi-aggregator) - Multi-chain DeFi aggregation
- [NFT Bridge](https://github.com/kherldhussein/apex-sdk/tree/main/examples/nft-bridge) - Cross-chain NFT bridging
- [DAO Governance](https://github.com/kherldhussein/apex-sdk/tree/main/examples/dao-governance) - Multi-chain DAO implementation

## Community

- **GitHub**: [kherldhussein/apex-sdk](https://github.com/kherldhussein/apex-sdk)
- **Issues**: [Report bugs](https://github.com/kherldhussein/apex-sdk/issues)
- **Discussions**: [Join the conversation](https://github.com/kherldhussein/apex-sdk/discussions)

## License

[Apache 2.0](https://github.com/kherldhussein/apex-sdk/blob/main/LICENSE)