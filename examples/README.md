# Apex SDK Examples

This directory contains comprehensive examples demonstrating how to use the Apex SDK for cross-chain blockchain development.

## 🚀 Quick Start

All examples work out-of-the-box with **testnet endpoints** (no setup required):

```bash
# Run any example
cargo run --example account-manager
cargo run --example evm-contract-call
cargo run --example contract-orchestration
cargo run --example price-oracle
cargo run --example parachain-assets
```

## 📋 Available Examples

### 1. Account Manager
**File**: `account-manager/main.rs`

Demonstrates unified multi-chain account management across Substrate and EVM.

**Features**:
- Generate keys for both ecosystems from single seed
- Derive addresses in both SS58 and 0x formats
- Query balances across multiple chains
- Track nonces and manage accounts

**Run**:
```bash
cargo run --example account-manager
```

### 2. EVM Contract Call
**File**: `evm-contract-call/main.rs`

Shows real EVM contract interactions using Alloy's `sol!` macro for type-safe ABI encoding.

**Features**:
- Query ERC20 token metadata (name, symbol, decimals)
- Check token balances
- Demonstrate transfer encoding
- Real contract calls on Sepolia testnet

**Run**:
```bash
cargo run --example evm-contract-call
```

**Output**:
```
=== Real EVM Contract Interaction Example ===

✓ SDK initialized with Sepolia testnet

Contract Addresses:
  WETH: 0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14
  USDC: 0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238

Step 1: Query WETH Token Metadata
─────────────────────────────────────
  Token Name: Wrapped Ether
  Token Symbol: WETH
  Token Decimals: 18

Step 2: Query Token Balance
─────────────────────────────────────
  Balance: 0.5 WETH (500000000000000000 wei)
```

### 3. Contract Orchestration
**File**: `contract-orchestration/main.rs`

Demonstrates orchestrating smart contract calls across both Substrate (ink!) and EVM chains.

**Features**:
- Type-safe Uniswap V2 Router interface
- ERC20 token approvals and transfers
- Cross-chain asset bridging concepts
- ink! contract encoding with SCALE codec

**Run**:
```bash
cargo run --example contract-orchestration
```

### 4. Price Oracle
**File**: `price-oracle/main.rs`

Shows building a cross-chain price oracle that aggregates data from multiple blockchains.

**Features**:
- Query prices from Substrate DEXs
- Query prices from EVM DEXs (Uniswap, Sushiswap)
- Calculate median and VWAP from cross-chain sources
- Detect price manipulation attempts

**Run**:
```bash
cargo run --example price-oracle
```

### 5. Parachain Assets
**File**: `parachain-assets/main.rs`

Demonstrates managing assets across Substrate parachains.

**Features**:
- Query parachain asset metadata
- Check asset balances
- Cross-parachain transfers
- Asset creation and management

**Run**:
```bash
cargo run --example parachain-assets
```

## 🔧 Configuration

### Default Endpoints

All examples use **testnets** by default:
- **Substrate**: Westend (`wss://westend-rpc.polkadot.io`)
- **EVM**: Sepolia (`https://eth-sepolia.g.alchemy.com/v2/demo`)

### Custom Endpoints

Use environment variables to customize endpoints:

```bash
# Use different networks
export SUBSTRATE_ENDPOINT="wss://polkadot.api.onfinality.io/public-ws"
export EVM_ENDPOINT="https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY"

cargo run --example account-manager
```

### With Wallet (For Transactions)

To execute actual transactions, provide wallet credentials:

**EVM (Private Key)**:
```bash
export PRIVATE_KEY=0x1234567890abcdef...

cargo run --example evm-contract-call
```

**Substrate (Seed Phrase)**:
```bash
export SUBSTRATE_SEED="your twelve word seed phrase here"

cargo run --example account-manager
```

## 📦 Dependencies

Examples use these Apex SDK features:

```toml
[dependencies]
apex-sdk = "0.1.4"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

# For EVM contract interactions
alloy = { version = "1.2.1", features = ["sol-types"] }
alloy-primitives = "1.5.2"
alloy-sol-types = "1.2.1"

# For Substrate contract interactions
parity-scale-codec = "3.6"
```

## 🧪 Testing Examples

### Integration Tests

Run examples against local test networks:

```bash
# Start Docker test infrastructure
docker-compose up -d

# Run integration tests
INTEGRATION_TESTS=1 cargo test --test evm_integration_test -- --include-ignored
INTEGRATION_TESTS=1 cargo test --test substrate_integration_test -- --include-ignored
```

### Real Testnet Tests

Execute transactions on actual testnets (requires testnet funds):

```bash
# EVM on Sepolia (get test ETH from https://sepoliafaucet.com)
PRIVATE_KEY=0x... REAL_TX_TESTS=1 \\
  cargo test --test real_transaction_test test_evm_real_transfer -- --ignored --nocapture

# Substrate on Westend (get test tokens from https://faucet.polkadot.io)
SUBSTRATE_SEED="your seed" REAL_TX_TESTS=1 \\
  cargo test --test real_transaction_test test_substrate_real_transfer -- --ignored --nocapture
```

## 📚 Learning Path

**Beginner** → **Intermediate** → **Advanced**

### 1. Start with Account Manager
Learn the basics of multi-chain account management.

### 2. Try EVM Contract Call
Understand contract interactions with type-safe ABI encoding.

### 3. Explore Contract Orchestration
See how to coordinate contracts across multiple chains.

### 4. Build Price Oracle
Learn cross-chain data aggregation patterns.

### 5. Study Parachain Assets
Understand Substrate-specific features.

## 🎯 Example Use Cases

### DeFi Application

```rust
use apex_sdk::prelude::*;
use alloy::sol;

sol! {
    interface IUniswapV2Router {
        function swapExactTokensForTokens(...) external returns (uint[] memory);
    }
}

// Build cross-chain DeFi app
let sdk = ApexSDK::builder()
    .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
    .with_evm_wallet(wallet)
    .build()
    .await?;

// Execute swap
let swap_call = IUniswapV2Router::swapExactTokensForTokensCall { ... };
let tx = sdk.transaction()
    .to_evm_address(router_address)
    .with_data(swap_call.abi_encode())
    .build()?;

sdk.execute(tx).await?;
```

### NFT Marketplace

```rust
// Query NFTs across both EVM and Substrate
let evm_nfts = query_evm_nfts(&sdk, collection_address).await?;
let substrate_nfts = query_substrate_nfts(&sdk, collection_id).await?;

// Unified marketplace
let all_nfts = [evm_nfts, substrate_nfts].concat();
```

### Cross-Chain Bridge

```rust
// Lock assets on EVM
let lock_tx = lock_on_ethereum(&sdk, amount).await?;

// Mint wrapped assets on Substrate
let mint_tx = mint_on_polkadot(&sdk, amount, proof).await?;
```

## 🐛 Troubleshooting

### Connection Issues

**Error**: "Failed to connect to endpoint"

**Solution**:
```bash
# Check endpoint is accessible
curl https://eth-sepolia.g.alchemy.com/v2/demo
wscat -c wss://westend-rpc.polkadot.io

# Try alternative endpoints
export EVM_ENDPOINT="https://ethereum.publicnode.com"
export SUBSTRATE_ENDPOINT="wss://westend-rpc.polkadot.io"
```

### Transaction Failures

**Error**: "Insufficient funds for gas"

**Solution**:
```bash
# Get testnet funds
# Sepolia: https://sepoliafaucet.com
# Westend: https://faucet.polkadot.io

# Check balance before transaction
let balance = sdk.evm()?.get_balance(address).await?;
println!("Balance: {}", balance);
```

### Build Errors

**Error**: "Cannot find crate alloy"

**Solution**:
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --example evm-contract-call
```

## 🔗 Resources

### Documentation
- [Main Documentation](../README.md)
- [Transaction Execution Guide](../TRANSACTION_EXECUTION_GUIDE.md)
- [API Reference](https://docs.rs/apex-sdk)

### Testnets
- **Sepolia Faucet**: https://sepoliafaucet.com
- **Westend Faucet**: https://faucet.polkadot.io
- **Westend Explorer**: https://westend.subscan.io
- **Sepolia Explorer**: https://sepolia.etherscan.io

### Community
- **GitHub**: https://github.com/carbobit/apex-sdk
- **Issues**: https://github.com/carbobit/apex-sdk/issues
- **Discussions**: https://github.com/carbobit/apex-sdk/discussions

## 📝 Contributing

Want to add an example? Great! Please follow these guidelines:

1. **Create a new directory**: `examples/your-example/`
2. **Add Cargo.toml**: With necessary dependencies
3. **Write main.rs**: With comprehensive comments
4. **Test it**: Ensure it runs successfully
5. **Document it**: Add section to this README
6. **Submit PR**: With description of what it demonstrates

Example structure:
```
examples/your-example/
├── Cargo.toml
├── main.rs
└── README.md (optional, for complex examples)
```

## 🎓 Next Steps

After exploring these examples:

1. **Read the Guides**: See `docs/guides/` for in-depth documentation
2. **Try Integration Tests**: Run `INTEGRATION_TESTS=1 cargo test`
3. **Build Your App**: Use the SDK in your own project
4. **Join Community**: Share your experience and get help

Happy building with Apex SDK! 🚀
