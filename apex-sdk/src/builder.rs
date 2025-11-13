//! Apex SDK builder for configuration

use crate::error::{Error, Result};
use crate::sdk::ApexSDK;

/// Builder for constructing an ApexSDK instance
#[derive(Default)]
pub struct ApexSDKBuilder {
    substrate_endpoint: Option<String>,
    evm_endpoint: Option<String>,
    timeout_seconds: Option<u64>,
}

impl ApexSDKBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Substrate endpoint URL
    pub fn with_substrate_endpoint(mut self, url: impl Into<String>) -> Self {
        self.substrate_endpoint = Some(url.into());
        self
    }

    /// Set the EVM endpoint URL
    pub fn with_evm_endpoint(mut self, url: impl Into<String>) -> Self {
        self.evm_endpoint = Some(url.into());
        self
    }

    /// Set the connection timeout in seconds
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Build the ApexSDK instance
    pub async fn build(self) -> Result<ApexSDK> {
        let substrate_adapter = if let Some(endpoint) = self.substrate_endpoint {
            Some(
                apex_sdk_substrate::SubstrateAdapter::connect(&endpoint)
                    .await
                    .map_err(Error::Substrate)?,
            )
        } else {
            None
        };

        let evm_adapter = if let Some(endpoint) = self.evm_endpoint {
            Some(
                apex_sdk_evm::EvmAdapter::connect(&endpoint)
                    .await
                    .map_err(Error::Evm)?,
            )
        } else {
            None
        };

        if substrate_adapter.is_none() && evm_adapter.is_none() {
            return Err(Error::Config(
                "At least one adapter (Substrate or EVM) must be configured".to_string(),
            ));
        }

        Ok(ApexSDK {
            substrate_adapter,
            evm_adapter,
        })
    }
}
