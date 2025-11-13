//! Main ApexSDK implementation

use crate::builder::ApexSDKBuilder;
use crate::error::{Error, Result};
use apex_sdk_evm::EvmAdapter;
use apex_sdk_substrate::SubstrateAdapter;
use apex_sdk_types::{Chain, TransactionStatus};

/// Main Apex SDK struct providing unified interface to blockchain operations
pub struct ApexSDK {
    pub(crate) substrate_adapter: Option<SubstrateAdapter>,
    pub(crate) evm_adapter: Option<EvmAdapter>,
}

impl ApexSDK {
    /// Create a new builder for configuring the SDK
    pub fn builder() -> ApexSDKBuilder {
        ApexSDKBuilder::new()
    }

    /// Create a new SDK instance with default configuration
    pub async fn new() -> Result<Self> {
        Err(Error::Config(
            "Use ApexSDK::builder() to configure the SDK".to_string(),
        ))
    }

    /// Get a reference to the Substrate adapter
    pub fn substrate(&self) -> Result<&SubstrateAdapter> {
        self.substrate_adapter
            .as_ref()
            .ok_or_else(|| Error::Config("Substrate adapter not configured".to_string()))
    }

    /// Get a reference to the EVM adapter
    pub fn evm(&self) -> Result<&EvmAdapter> {
        self.evm_adapter
            .as_ref()
            .ok_or_else(|| Error::Config("EVM adapter not configured".to_string()))
    }

    /// Check if a chain is supported
    pub fn is_chain_supported(&self, chain: &Chain) -> bool {
        match chain {
            Chain::Polkadot | Chain::Kusama => self.substrate_adapter.is_some(),
            Chain::Ethereum | Chain::Polygon | Chain::BinanceSmartChain | Chain::Avalanche => {
                self.evm_adapter.is_some()
            }
            Chain::Moonbeam | Chain::Astar => {
                self.substrate_adapter.is_some() && self.evm_adapter.is_some()
            }
        }
    }

    /// Get the status of a transaction
    pub async fn get_transaction_status(
        &self,
        chain: &Chain,
        tx_hash: &str,
    ) -> Result<TransactionStatus> {
        match chain {
            Chain::Polkadot | Chain::Kusama => self
                .substrate()?
                .get_transaction_status(tx_hash)
                .await
                .map_err(Error::Substrate),
            Chain::Ethereum | Chain::Polygon | Chain::BinanceSmartChain | Chain::Avalanche => self
                .evm()?
                .get_transaction_status(tx_hash)
                .await
                .map_err(Error::Evm),
            Chain::Moonbeam | Chain::Astar => {
                // Try EVM first for hybrid chains
                self.evm()?
                    .get_transaction_status(tx_hash)
                    .await
                    .map_err(Error::Evm)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_requires_adapter() {
        let result = ApexSDK::builder().build().await;
        assert!(result.is_err());
    }
}
