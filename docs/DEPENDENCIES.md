# Dependencies & Technology Stack

Complete overview of Apex SDK's technology stack, dependencies, and build requirements.

## Core Technologies

| Technology | Purpose | Version | Documentation |
|------------|---------|---------|---------------|
| **Rust** | Primary programming language | 1.85+ | [rust-lang.org](https://www.rust-lang.org/) |
| **Substrate** | Polkadot ecosystem framework | Latest | [substrate.io](https://substrate.io/) |
| **EVM** | Ethereum Virtual Machine | - | [ethereum.org](https://ethereum.org/) |
| **WebAssembly** | Runtime execution environment | - | [webassembly.org](https://webassembly.org/) |

## Dependency Categories

### Blockchain Integration

| Crate | Version | Purpose |
|-------|---------|---------|
| `subxt` | 0.44.0 | Substrate client library for runtime metadata and chain interaction |
| `ethers` | 2.0 | Ethereum library for EVM chain interaction and smart contracts |
| `sp-core` | 42.0.0 | Substrate primitives for cryptography and key management |
| `sp-keyring` | 42.0.0 | Account management and key derivation |
| `subxt-signer` | 0.44.0 | Transaction signing for Substrate chains |

**Migration Note:** `ethers` is being replaced with `alloy` in future versions. See [Migration Guide](./ETHERS_TO_ALLOY_MIGRATION.md).

### Async Runtime

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.43.0 | Asynchronous runtime with multi-threading support |

**Features Used:** `rt-multi-thread`, `macros`, `sync`, `time`

### Serialization & Encoding

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1.0 | Serialization/deserialization framework |
| `serde_json` | 1.0 | JSON format support |
| `scale-codec` | 3.7.0 | SCALE encoding for Substrate types |
| `parity-scale-codec-derive` | 3.7.0 | Derive macros for SCALE codec |

### Cryptography

| Crate | Version | Purpose |
|-------|---------|---------|
| `sha3` | 0.10 | SHA3 and Keccak hashing algorithms |
| `hex` | 0.4 | Hexadecimal encoding/decoding |
| `bs58` | 0.5 | Base58 encoding for blockchain addresses |
| `bip39` | 2.0 | BIP39 mnemonic phrase generation |

### Error Handling & Logging

| Crate | Version | Purpose |
|-------|---------|---------|
| `anyhow` | 1.0 | Flexible error handling and context |
| `thiserror` | 2.0.9 | Procedural macro for custom error types |
| `tracing` | 0.1 | Application-level tracing framework |
| `tracing-subscriber` | 0.3 | Tracing subscribers for logging |

### Testing & Benchmarking

| Crate | Version | Purpose |
|-------|---------|---------|
| `criterion` | 0.7.0 | Statistical benchmarking framework |
| `mockall` | 0.13.1 | Mock object generation for testing |
| `proptest` | 1.4 | Property-based testing |
| `tempfile` | 3.8 | Temporary file creation for tests |

### CLI-Specific Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.4 | Command-line argument parsing |
| `clap_complete` | 4.4 | Shell completion generation |
| `indicatif` | 0.18.3 | Progress bars and spinners |
| `dialoguer` | 0.12.0 | Interactive CLI prompts |
| `colored` | 3.0.0 | Terminal color support |
| `rpassword` | 7.3 | Secure password input |

## Build Requirements

### System Requirements

- **Rust Toolchain:** 1.85 or higher
- **Cargo:** Latest stable version
- **Git:** For version control and dependency fetching

### Platform-Specific Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    clang \
    curl
```

#### macOS
```bash
brew install openssl pkg-config
```

#### Fedora/RHEL
```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    openssl-devel \
    pkg-config \
    git
```

### Environment Variables

```bash
# macOS with Homebrew OpenSSL
export OPENSSL_DIR=/opt/homebrew/opt/openssl
export PKG_CONFIG_PATH=/opt/homebrew/opt/openssl/lib/pkgconfig

# Custom OpenSSL installation
export OPENSSL_DIR=/path/to/openssl
export OPENSSL_LIB_DIR=/path/to/openssl/lib
export OPENSSL_INCLUDE_DIR=/path/to/openssl/include
```

## Optional Development Tools

### Recommended Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| `cargo-deny` | License and security auditing | `cargo install cargo-deny` |
| `cargo-audit` | Security vulnerability scanning | `cargo install cargo-audit` |
| `cargo-outdated` | Check for outdated dependencies | `cargo install cargo-outdated` |
| `cargo-udeps` | Detect unused dependencies | `cargo install cargo-udeps --locked` |
| `cargo-watch` | Auto-rebuild on file changes | `cargo install cargo-watch` |
| `cargo-expand` | Expand macros for debugging | `cargo install cargo-expand` |

### Substrate-Specific Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| `subxt-cli` | Generate typed metadata | `cargo install subxt-cli` |
| `substrate-contracts-node` | Local contract testing | See [docs](https://github.com/paritytech/substrate-contracts-node) |

### EVM-Specific Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| `foundry` | Smart contract development | See [book.getfoundry.sh](https://book.getfoundry.sh/) |
| `hardhat` | Contract testing framework | `npm install --save-dev hardhat` |

## Workspace Structure

```
apex-sdk/
├── apex-sdk/           # Main unified SDK
├── apex-sdk-core/      # Core traits and types
├── apex-sdk-substrate/ # Substrate implementation
├── apex-sdk-evm/       # EVM implementation
├── apex-sdk-types/     # Shared type definitions
└── cli/                # Command-line interface
```

### Workspace Dependencies

Dependencies are managed at the workspace level in the root `Cargo.toml`:

```toml
[workspace.dependencies]
tokio = { version = "1.43.0", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }
# ... (see root Cargo.toml for complete list)
```

## Security Considerations

### Dependency Auditing

We regularly audit dependencies using:

- **cargo-deny:** License compliance and security advisories
- **cargo-audit:** Known security vulnerabilities
- **Dependabot:** Automated dependency updates

### Critical Dependencies

Special attention is paid to cryptographic and blockchain interaction libraries:

- `sp-core` - Substrate cryptography primitives
- `ethers` - Ethereum signing and transactions
- `sha3` - Hashing algorithms
- `bip39` - Mnemonic generation

### Version Pinning

Production deployments should pin exact versions:

```toml
[dependencies]
apex-sdk = "=0.1.3"  # Pin exact version
```

## Performance Optimization

### Compile-Time Optimizations

```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit for better optimization
strip = true            # Strip symbols
panic = "abort"         # Smaller binary size
```

### Runtime Performance

- **Async Runtime:** Tokio provides efficient multi-threaded async execution
- **Zero-Copy:** Extensive use of references to avoid allocations
- **Type-Level Optimization:** Compile-time metadata resolution

## License Compliance

All dependencies are vetted for license compatibility with Apache 2.0:

- **Permissive:** MIT, Apache-2.0, BSD
- **Copyleft:** None (we avoid GPL dependencies)

Run license audit:
```bash
cargo deny check licenses
```

## Updating Dependencies

### Checking for Updates

```bash
# Check for outdated dependencies
cargo outdated

# Check security advisories
cargo audit

# Full audit
cargo deny check
```

### Update Process

1. Review changelog and breaking changes
2. Update version in `Cargo.toml`
3. Run full test suite: `cargo test --all-features`
4. Run benchmarks: `cargo bench`
5. Update documentation if API changed

## Related Documentation

- [Installation Guide](./INSTALLATION.md)
- [Development Guide](./DEVELOPMENT.md)
- [Architecture Overview](./ARCHITECTURE.md)
- [Security Policy](./SECURITY.md)

---

**Last Updated:** 2025-11-21
**Rust Version:** 1.85+
**SDK Version:** 0.1.3
