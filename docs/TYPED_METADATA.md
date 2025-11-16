# Typed Metadata Generation for Substrate

This document explains how to generate and use typed metadata for type-safe Substrate transactions.

## Benefits of Typed Metadata

- **Type Safety**: Compile-time checking of transaction parameters
- **Auto-completion**: IDE support for exploring available calls and types
- **Performance**: Slightly better performance than dynamic API
- **Documentation**: Generated types include chain-specific documentation

## Quick Start

### 1. Install subxt CLI

```bash
cargo install subxt-cli
```

### 2. Generate Metadata from a Chain

#### For Polkadot:
```bash
subxt metadata --url wss://rpc.polkadot.io > polkadot-metadata.scale
```

#### For Kusama:
```bash
subxt metadata --url wss://kusama-rpc.polkadot.io > kusama-metadata.scale
```

#### For Local Development Node:
```bash
subxt metadata --url ws://localhost:9944 > local-metadata.scale
```

### 3. Generate Rust Code from Metadata

```bash
subxt codegen --file polkadot-metadata.scale | rustfmt --edition 2021 > src/generated/polkadot.rs
```

Or for multiple chains:

```bash
mkdir -p src/generated
subxt codegen --file polkadot-metadata.scale | rustfmt --edition 2021 > src/generated/polkadot.rs
subxt codegen --file kusama-metadata.scale | rustfmt --edition 2021 > src/generated/kusama.rs
```

### 4. Include Generated Code in Your Project

Add to `src/lib.rs` or create a new module:

```rust
#[cfg(feature = "polkadot-metadata")]
pub mod polkadot {
    include!("generated/polkadot.rs");
}

#[cfg(feature = "kusama-metadata")]
pub mod kusama {
    include!("generated/kusama.rs");
}
```

## Using Typed Transactions

### Balance Transfer Example

**Dynamic API (current)**:
```rust
use subxt::dynamic;

let tx = dynamic::tx(
    "Balances",
    "transfer_keep_alive",
    vec![
        dynamic::Value::from_bytes(&recipient_bytes),
        dynamic::Value::u128(amount),
    ],
);
```

**Typed API (with generated metadata)**:
```rust
use crate::polkadot;

let tx = polkadot::tx().balances().transfer_keep_alive(
    recipient_account,
    amount,
);
```

### Custom Call Example

**Dynamic API**:
```rust
let tx = dynamic::tx(
    "Staking",
    "bond",
    vec![
        dynamic::Value::from_bytes(&controller),
        dynamic::Value::u128(value),
        dynamic::Value::variant("Staked"),
    ],
);
```

**Typed API**:
```rust
let tx = polkadot::tx().staking().bond(
    controller,
    value,
    RewardDestination::Staked,
);
```

## Recommended Workflow

### For Production Applications

1. Generate metadata from testnet first
2. Test thoroughly with typed transactions
3. Generate production metadata from mainnet
4. Commit generated files to version control
5. Update metadata periodically after runtime upgrades

### For Development

1. Use dynamic API for rapid prototyping
2. Generate typed metadata once stable
3. Switch to typed API for production code

## Keeping Metadata Up-to-Date

Substrate chains upgrade their runtime periodically. After a runtime upgrade:

1. Fetch new metadata from the chain
2. Regenerate Rust code
3. Fix any breaking changes in your code
4. Test thoroughly before deploying

## Feature Flags

Add feature flags to your `Cargo.toml` to enable specific chain support:

```toml
[features]
default = []
polkadot-metadata = []
kusama-metadata = []
westend-metadata = []
```

Build with specific features:

```bash
cargo build --features polkadot-metadata
```

## Troubleshooting

### Metadata Fetch Fails

**Problem**: Unable to connect to RPC endpoint

**Solutions**:
- Check endpoint URL is correct
- Verify network connectivity
- Try alternative RPC endpoints
- Use archived metadata file if available

### Code Generation Fails

**Problem**: `subxt codegen` produces errors

**Solutions**:
- Update `subxt-cli` to latest version
- Verify metadata file is valid SCALE-encoded data
- Check for incompatible runtime versions

### Generated Code Doesn't Compile

**Problem**: Type errors in generated code

**Solutions**:
- Ensure `subxt` version matches the version used by `subxt-cli`
- Update all dependencies to compatible versions
- Check for missing re-exports

## Performance Comparison

| Metric | Dynamic API | Typed API |
|--------|-------------|-----------|
| Compilation Time | Faster | Slower (more code to compile) |
| Runtime Performance | ~same | ~same (minor improvements) |
| Type Safety | Runtime only | Compile-time |
| Code Maintainability | Lower | Higher |
| Binary Size | Smaller | Larger |

## Best Practices

1. **Version Control**: Commit generated files to track changes across runtime upgrades
2. **Documentation**: Document which runtime version the metadata was generated from
3. **Testing**: Maintain tests for both dynamic and typed APIs during transition
4. **Gradual Migration**: Migrate module-by-module rather than all at once
5. **Fallback**: Keep dynamic API as fallback for new/unsupported calls

## Additional Resources

- [subxt Documentation](https://docs.rs/subxt/)
- [subxt GitHub](https://github.com/paritytech/subxt)
- [Polkadot Documentation](https://wiki.polkadot.com/)
- [Substrate Documentation](https://docs.polkadot.com/)
