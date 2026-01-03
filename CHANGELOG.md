# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.5] - 2025-11-25

### Added
- **Core Transaction Pipeline**: Implemented the complete transaction lifecycle (build -> sign -> submit -> track -> receipt) for both EVM and Substrate.
- **Connection Pooling**: Added robust connection pooling with health checks and failover for both EVM (HTTP) and Substrate (WebSocket) adapters.
- **Metrics Collection**: Introduced a metrics system to track RPC calls, transaction success rates, and errors.
- **Unified Signer**: Implemented a unified `Signer` trait and concrete implementations for EVM (PrivateKey) and Substrate (SR25519/ED25519).
- **Fee Estimation**: Added fee estimation logic, including EIP-1559 support for EVM.
- **Contract Interaction**: Added `contract.rs` to the EVM adapter for easier smart contract interaction (including ERC-20).
- **Integration Tests**: Added comprehensive integration tests running against local Docker nodes (Hardhat/Westend).

### Changed
- **Architecture**: Refactored the internal architecture to use the new `Provider`, `Signer`, `Broadcaster`, and `ReceiptWatcher` traits.
- **Error Handling**: Unified error handling across the SDK using the `SdkError` enum.
- **Documentation**: Updated the landing page and documentation to reflect the new features and professional branding.

## [0.1.5] - 2025-11-24

### Added
- Initial release of the Apex SDK.
- Basic support for EVM and Substrate chains.
- CLI tool for basic operations.
