//! # Apex SDK
//!
//! Unified Rust SDK for Substrate & EVM blockchain development.
//!
//! ## Features
//!
//! - **Unified Interface**: Single API for both Substrate and EVM blockchains
//! - **Compile-Time Type Safety**: Catch errors before deployment
//! - **Native Performance**: Rust-based implementation
//! - **Cross-Chain Ready**: Built-in cross-chain communication support
//!
//! ## Example
//!
//! ```rust,no_run
//! use apex_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let sdk = ApexSDK::builder()
//!         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
//!         .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
//!         .build()
//!         .await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod error;
pub mod sdk;

pub use apex_sdk_core as core;
pub use apex_sdk_evm as evm;
pub use apex_sdk_substrate as substrate;
pub use apex_sdk_types as types;

pub use builder::ApexSDKBuilder;
pub use error::{Error, Result};
pub use sdk::ApexSDK;

/// Common imports for convenience
pub mod prelude {
    pub use crate::builder::ApexSDKBuilder;
    pub use crate::error::{Error, Result};
    pub use crate::sdk::ApexSDK;
    pub use apex_sdk_types::{Address, Chain, ChainType, TransactionStatus};
}
