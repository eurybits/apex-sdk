//! Wallet management for EVM chains
//!
//! This module provides secure wallet management including:
//! - Wallet creation and recovery
//! - Private key management
//! - Transaction signing
//! - Message signing (EIP-191, EIP-712)

use crate::Error;
use ethers::prelude::*;
use ethers::signers::{coins_bip39::English, LocalWallet, Signer};
use ethers::types::{transaction::eip2718::TypedTransaction, transaction::eip712::Eip712, Address as EthAddress, Signature};
use std::str::FromStr;

/// Wallet for managing EVM accounts and signing transactions
#[derive(Clone)]
pub struct Wallet {
    /// The underlying ethers wallet
    inner: LocalWallet,
    /// The address of this wallet
    address: EthAddress,
}

impl Wallet {
    /// Create a new random wallet
    ///
    /// # Example
    /// ```no_run
    /// use apex_sdk_evm::wallet::Wallet;
    ///
    /// let wallet = Wallet::new_random();
    /// println!("Address: {}", wallet.address());
    /// ```
    pub fn new_random() -> Self {
        let inner = LocalWallet::new(&mut rand::thread_rng());
        let address = inner.address();

        tracing::info!("Created new random wallet: {}", address);

        Self { inner, address }
    }

    /// Create a wallet from a private key (hex string with or without 0x prefix)
    ///
    /// # Arguments
    /// * `private_key` - The private key as a hex string
    ///
    /// # Example
    /// ```no_run
    /// use apex_sdk_evm::wallet::Wallet;
    ///
    /// let wallet = Wallet::from_private_key(
    ///     "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    /// ).unwrap();
    /// ```
    pub fn from_private_key(private_key: &str) -> Result<Self, Error> {
        let key = private_key.trim_start_matches("0x");

        let inner = LocalWallet::from_str(key)
            .map_err(|e| Error::Other(format!("Invalid private key: {}", e)))?;

        let address = inner.address();

        tracing::info!("Loaded wallet from private key: {}", address);

        Ok(Self { inner, address })
    }

    /// Create a wallet from a mnemonic phrase
    ///
    /// # Arguments
    /// * `mnemonic` - The BIP-39 mnemonic phrase
    /// * `index` - The account index (default 0 for first account)
    ///
    /// # Example
    /// ```no_run
    /// use apex_sdk_evm::wallet::Wallet;
    ///
    /// let wallet = Wallet::from_mnemonic(
    ///     "test test test test test test test test test test test junk",
    ///     0
    /// ).unwrap();
    /// ```
    pub fn from_mnemonic(mnemonic: &str, index: u32) -> Result<Self, Error> {
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic)
            .index(index)
            .map_err(|e| Error::Other(format!("Invalid index: {}", e)))?
            .build()
            .map_err(|e| Error::Other(format!("Failed to build wallet from mnemonic: {}", e)))?;

        let address = wallet.address();

        tracing::info!("Loaded wallet from mnemonic at index {}: {}", index, address);

        Ok(Self {
            inner: wallet,
            address,
        })
    }

    /// Create a wallet with a specific chain ID
    ///
    /// This is important for EIP-155 replay protection
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.inner = self.inner.with_chain_id(chain_id);
        tracing::debug!("Set wallet chain ID to {}", chain_id);
        self
    }

    /// Get the wallet's address
    pub fn address(&self) -> String {
        format!("{:?}", self.address)
    }

    /// Get the wallet's address as EthAddress
    pub fn eth_address(&self) -> EthAddress {
        self.address
    }

    /// Sign a transaction
    ///
    /// # Arguments
    /// * `tx` - The transaction to sign
    ///
    /// # Returns
    /// The signature as bytes
    pub async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, Error> {
        let signature = self.inner
            .sign_transaction(tx)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to sign transaction: {}", e)))?;

        tracing::debug!("Signed transaction");

        Ok(signature)
    }

    /// Sign a message (EIP-191)
    ///
    /// # Arguments
    /// * `message` - The message to sign
    ///
    /// # Returns
    /// The signature as bytes
    pub async fn sign_message<S: AsRef<[u8]> + Send + Sync>(&self, message: S) -> Result<Signature, Error> {
        let signature = self.inner
            .sign_message(message)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to sign message: {}", e)))?;

        tracing::debug!("Signed message");

        Ok(signature)
    }

    /// Sign typed data (EIP-712)
    ///
    /// # Arguments
    /// * `data` - The typed data to sign
    ///
    /// # Returns
    /// The signature as bytes
    pub async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        data: &T,
    ) -> Result<Signature, Error> {
        let signature = self.inner
            .sign_typed_data(data)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to sign typed data: {}", e)))?;

        tracing::debug!("Signed typed data");

        Ok(signature)
    }

    /// Get the chain ID configured for this wallet
    pub fn chain_id(&self) -> Option<u64> {
        Some(self.inner.chain_id())
    }

    /// Export private key (WARNING: Handle with extreme care!)
    ///
    /// # Security Warning
    /// This exposes the private key. Only use in secure contexts.
    pub fn export_private_key(&self) -> String {
        tracing::warn!("Private key exported - ensure secure handling!");
        format!("0x{}", hex::encode(self.inner.signer().to_bytes()))
    }
}

impl std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wallet")
            .field("address", &self.address())
            .field("chain_id", &self.chain_id())
            .finish()
    }
}

/// Wallet manager for handling multiple accounts
pub struct WalletManager {
    wallets: Vec<Wallet>,
    active_index: usize,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new() -> Self {
        Self {
            wallets: Vec::new(),
            active_index: 0,
        }
    }

    /// Add a wallet to the manager
    pub fn add_wallet(&mut self, wallet: Wallet) -> usize {
        self.wallets.push(wallet);
        self.wallets.len() - 1
    }

    /// Get the active wallet
    pub fn active_wallet(&self) -> Option<&Wallet> {
        self.wallets.get(self.active_index)
    }

    /// Get a wallet by index
    pub fn wallet(&self, index: usize) -> Option<&Wallet> {
        self.wallets.get(index)
    }

    /// Set the active wallet
    pub fn set_active(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.wallets.len() {
            return Err(Error::Other("Invalid wallet index".to_string()));
        }
        self.active_index = index;
        Ok(())
    }

    /// Get the number of wallets
    pub fn wallet_count(&self) -> usize {
        self.wallets.len()
    }

    /// List all wallet addresses
    pub fn list_addresses(&self) -> Vec<String> {
        self.wallets.iter().map(|w| w.address()).collect()
    }

    /// Create and add a new random wallet
    pub fn create_wallet(&mut self) -> usize {
        let wallet = Wallet::new_random();
        self.add_wallet(wallet)
    }

    /// Import a wallet from private key and add it
    pub fn import_wallet(&mut self, private_key: &str) -> Result<usize, Error> {
        let wallet = Wallet::from_private_key(private_key)?;
        Ok(self.add_wallet(wallet))
    }

    /// Import a wallet from mnemonic and add it
    pub fn import_from_mnemonic(&mut self, mnemonic: &str, index: u32) -> Result<usize, Error> {
        let wallet = Wallet::from_mnemonic(mnemonic, index)?;
        Ok(self.add_wallet(wallet))
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_random_wallet() {
        let wallet = Wallet::new_random();
        assert!(wallet.address().starts_with("0x"));
        assert_eq!(wallet.address().len(), 42);
    }

    #[test]
    fn test_from_private_key() {
        // Test private key (from hardhat default)
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let wallet = Wallet::from_private_key(private_key).unwrap();

        // Expected address for this key
        let expected_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert_eq!(wallet.address().to_lowercase(), expected_address);
    }

    #[test]
    fn test_from_mnemonic() {
        // Test mnemonic (common test phrase)
        let mnemonic = "test test test test test test test test test test test junk";
        let wallet = Wallet::from_mnemonic(mnemonic, 0).unwrap();

        // Should create a valid address
        assert!(wallet.address().starts_with("0x"));
        assert_eq!(wallet.address().len(), 42);
    }

    #[test]
    fn test_wallet_with_chain_id() {
        let wallet = Wallet::new_random().with_chain_id(1);
        assert_eq!(wallet.chain_id(), Some(1));
    }

    #[tokio::test]
    async fn test_sign_message() {
        let wallet = Wallet::new_random();
        let message = "Hello, Ethereum!";

        let signature = wallet.sign_message(message).await.unwrap();

        // Signature should be 65 bytes (r: 32, s: 32, v: 1)
        let sig_bytes = signature.to_vec();
        assert_eq!(sig_bytes.len(), 65);
    }

    #[test]
    fn test_wallet_manager() {
        let mut manager = WalletManager::new();

        // Create wallets
        let idx1 = manager.create_wallet();
        let idx2 = manager.create_wallet();

        assert_eq!(manager.wallet_count(), 2);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);

        // Test active wallet
        assert!(manager.active_wallet().is_some());

        // Change active wallet
        manager.set_active(1).unwrap();
        assert!(manager.active_wallet().is_some());

        // List addresses
        let addresses = manager.list_addresses();
        assert_eq!(addresses.len(), 2);
    }

    #[test]
    fn test_wallet_manager_import() {
        let mut manager = WalletManager::new();

        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let idx = manager.import_wallet(private_key).unwrap();

        assert_eq!(idx, 0);
        assert_eq!(manager.wallet_count(), 1);

        let wallet = manager.wallet(0).unwrap();
        assert_eq!(
            wallet.address().to_lowercase(),
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        );
    }
}
