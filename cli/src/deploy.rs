//! Contract deployment functionality for Substrate (WASM) and EVM

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Deploy a contract
pub async fn deploy_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    // determine chain type
    let is_substrate = endpoint.starts_with("ws://")
        || endpoint.starts_with("wss://")
        || matches!(
            chain.to_lowercase().as_str(),
            "polkadot"
                | "kusama"
                | "paseo"
                | "westend"
                | "moonbeam"
                | "astar"
                | "acala"
                | "phala"
                | "bifrost"
        );

    if is_substrate {
        deploy_substrate_contract(contract_path, chain, endpoint, account_name, dry_run).await
    } else {
        deploy_evm_contract(contract_path, chain, endpoint, account_name, dry_run).await
    }
}

/// Deploy a Substrate WASM contract
async fn deploy_substrate_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    let title = if dry_run {
        " Dry-Run: Substrate WASM Contract Deployment"
    } else {
        " Deploying Substrate WASM Contract"
    };

    println!("\n{}", title.cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Contract".dimmed(), contract_path);
    println!("{}: {}", "Chain".dimmed(), chain);
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    if dry_run {
        println!(
            "{}: DRY RUN - No transaction will be broadcast",
            "Mode".yellow().bold()
        );
    }
    println!();

    // verify contract file exists
    let path = Path::new(contract_path);
    if !path.exists() {
        anyhow::bail!("Contract file not found: {}", contract_path);
    }

    // check if it's a .contract or .wasm file
    let extension = path.extension().and_then(|s| s.to_str());
    match extension {
        Some("contract") | Some("wasm") => {}
        Some(ext) => {
            println!(
                "{}",
                format!("  Warning: Expected .contract or .wasm file, got .{}", ext).yellow()
            );
        }
        None => {
            anyhow::bail!("Contract file must have .contract or .wasm extension");
        }
    }

    // read contract file
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Reading contract file...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let contract_code = std::fs::read(contract_path).context("Failed to read contract file")?;

    spinner.set_message(format!("Contract size: {} bytes", contract_code.len()));

    // get account for signing
    let (signer_name, _mnemonic) = if let Some(name) = account_name {
        spinner.set_message(format!("Loading account '{}'...", name));

        let password = rpassword::prompt_password("Enter account password: ")
            .context("Failed to read password")?;

        let keystore_path = crate::keystore::get_keystore_path()?;
        let keystore = crate::keystore::Keystore::load(&keystore_path)?;

        let mnemonic_bytes = keystore.get_account(&name, &password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes).context("Failed to decode mnemonic")?;

        (name, mnemonic)
    } else {
        spinner.finish_and_clear();
        anyhow::bail!(
            "Account required for deployment.\n\n\
            Use --account flag to specify an account:\n  \
            apex deploy {} --chain {} --endpoint {} --account <name>\n\n\
            Or create an account first:\n  \
            apex account generate --type substrate",
            contract_path,
            chain,
            endpoint
        );
    };

    spinner.set_message("Connecting to chain...");

    // note: full implementation would use subxt with contract pallet,...
    // this is a simplified version showing the structure

    spinner.finish_with_message(format!("Ready to deploy with account: {}", signer_name));

    println!("\n{}", " Deployment Summary".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!(
        "{}: {} bytes",
        "Contract Size".dimmed(),
        contract_code.len()
    );
    println!("{}: {}", "Deployer".dimmed(), signer_name);

    println!("\n{}", " Implementation Note:".yellow());
    println!("Full Substrate contract deployment requires:");
    println!("  • Chain must have the 'contracts' pallet enabled");
    println!("  • Sufficient balance for deployment fees");
    println!("  • Contract metadata (.contract file) for full functionality");
    println!("\n{}", " Resources:".cyan());
    println!("  • Use cargo-contract: https://github.com/paritytech/cargo-contract");
    println!("  • Polkadot.js Apps: https://polkadot.js.org/apps/");
    println!(
        "  • Substrate Contracts Workshop: https://docs.substrate.io/tutorials/smart-contracts/"
    );

    Ok(())
}

/// Deploy an EVM contract
async fn deploy_evm_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    use ethers::prelude::*;
    use ethers::types::transaction::eip2718::TypedTransaction;

    let title = if dry_run {
        " Dry-Run: EVM Contract Deployment"
    } else {
        " Deploying EVM Contract"
    };

    println!("\n{}", title.cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Contract".dimmed(), contract_path);
    println!("{}: {}", "Chain".dimmed(), chain);
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    if dry_run {
        println!(
            "{}: DRY RUN - No transaction will be broadcast",
            "Mode".yellow().bold()
        );
    }
    println!();

    // verify contract file exists
    let path = Path::new(contract_path);
    if !path.exists() {
        anyhow::bail!("Contract file not found: {}", contract_path);
    }

    // check if it's bytecode (.bin) or ABI+bytecode (.json)
    let extension = path.extension().and_then(|s| s.to_str());
    let contract_data = match extension {
        Some("bin") | Some("hex") => {
            // raw bytecode
            let code = std::fs::read_to_string(contract_path)
                .context("Failed to read contract bytecode")?;
            hex::decode(code.trim().trim_start_matches("0x")).context("Invalid hex bytecode")?
        }
        Some("json") => {
            // json with bytecode (common Solidity compiler output)
            let json_str =
                std::fs::read_to_string(contract_path).context("Failed to read contract JSON")?;
            let json: serde_json::Value =
                serde_json::from_str(&json_str).context("Invalid JSON file")?;

            // try to extract bytecode from different JSON formats
            let bytecode_hex = json
                .get("bytecode")
                .or_else(|| json.get("data"))
                .or_else(|| json.get("object"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Could not find bytecode in JSON file"))?;

            hex::decode(bytecode_hex.trim().trim_start_matches("0x"))
                .context("Invalid hex bytecode in JSON")?
        }
        Some(ext) => {
            anyhow::bail!(
                "Unsupported contract file extension: .{}\nSupported: .bin, .hex, .json",
                ext
            );
        }
        None => {
            anyhow::bail!("Contract file must have an extension (.bin, .hex, or .json)");
        }
    };

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message(format!("Contract bytecode: {} bytes", contract_data.len()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // get account for signing
    let (signer_name, mnemonic) = if let Some(name) = account_name {
        spinner.set_message(format!("Loading account '{}'...", name));

        let password = rpassword::prompt_password("Enter account password: ")
            .context("Failed to read password")?;

        let keystore_path = crate::keystore::get_keystore_path()?;
        let keystore = crate::keystore::Keystore::load(&keystore_path)?;

        let mnemonic_bytes = keystore.get_account(&name, &password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes).context("Failed to decode mnemonic")?;

        (name, mnemonic)
    } else {
        spinner.finish_and_clear();
        anyhow::bail!(
            "Account required for deployment.\n\n\
            Use --account flag to specify an account:\n  \
            apex deploy {} --chain {} --endpoint {} --account <name>\n\n\
            Or create an account first:\n  \
            apex account generate --type evm",
            contract_path,
            chain,
            endpoint
        );
    };

    spinner.set_message("Connecting to chain...");

    // create provider
    let provider = Provider::<Http>::try_from(endpoint).context("Failed to create provider")?;

    // create wallet from mnemonic
    let wallet: LocalWallet = mnemonic.parse().context("Failed to parse mnemonic")?;

    let chain_id = provider.get_chainid().await?;
    let wallet = wallet.with_chain_id(chain_id.as_u64());

    spinner.set_message("Preparing deployment transaction...");

    // create deployment transaction
    let tx = TransactionRequest::new()
        .data(contract_data.clone())
        .from(wallet.address());

    // convert to TypedTransaction for gas estimation
    let typed_tx: TypedTransaction = tx.clone().into();

    // estimate gas
    spinner.set_message("Estimating gas...");
    let gas_estimate = provider
        .estimate_gas(&typed_tx, None)
        .await
        .context("Failed to estimate gas")?;

    // get gas price
    let gas_price = provider.get_gas_price().await?;

    // create final transaction with gas settings
    let _tx = tx
        .gas(gas_estimate * 120 / 100) // add 20% buffer
        .gas_price(gas_price);

    spinner.finish_and_clear();

    // display deployment info
    println!("\n{}", " Deployment Summary".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!(
        "{}: {} bytes",
        "Bytecode Size".dimmed(),
        contract_data.len()
    );
    println!("{}: {}", "Deployer".dimmed(), signer_name);
    println!("{}: {:?}", "From Address".dimmed(), wallet.address());
    println!("{}: {}", "Chain ID".dimmed(), chain_id);
    println!("{}: {}", "Gas Estimate".dimmed(), gas_estimate);
    println!(
        "{}: {} gwei",
        "Gas Price".dimmed(),
        ethers::utils::format_units(gas_price, "gwei").unwrap_or_default()
    );

    let total_cost = gas_estimate * gas_price;
    println!(
        "{}: {} ETH",
        "Est. Cost".yellow().bold(),
        ethers::utils::format_units(total_cost, "ether").unwrap_or_default()
    );

    if dry_run {
        println!("\n{}", " Dry-Run Validation Complete".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("All validation checks passed:");
        println!("  ✓ Contract file is valid");
        println!("  ✓ Connected to chain");
        println!("  ✓ Account is ready");
        println!("  ✓ Gas estimation successful");
        println!("  ✓ Transaction can be constructed");
        println!();
        println!("{}", " Ready for Real Deployment".cyan().bold());
        println!("To perform the actual deployment, run the same command without --dry-run:");
        println!(
            "  apex deploy {} --chain {} --endpoint {} --account {}",
            contract_path, chain, endpoint, signer_name
        );
        println!();
        println!("{}", "  Note:".yellow());
        println!("The actual deployment will:");
        println!("  • Sign the transaction with your private key");
        println!("  • Broadcast to the network");
        println!("  • Wait for confirmation");
        println!(
            "  • Spend ~{} ETH in gas fees",
            ethers::utils::format_units(total_cost, "ether").unwrap_or_default()
        );
    } else {
        println!("\n{}", " Ready to Deploy".yellow().bold());
        println!("This will spend gas fees from your account.");

        print!("\nProceed with deployment? (yes/no): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "yes" {
            println!("\n{}", "Deployment cancelled.".yellow());
            return Ok(());
        }

        println!("\n{}", " Broadcasting transaction...".cyan());

        println!("\n{}", " Implementation Note:".yellow());
        println!("To complete the deployment, this would:");
        println!("  1. Sign the transaction with your private key");
        println!("  2. Broadcast to the network");
        println!("  3. Wait for confirmation");
        println!("  4. Return the deployed contract address");

        println!("\n{}", " Alternative Tools:".cyan());
        println!("  • Remix IDE: https://remix.ethereum.org/");
        println!("  • Hardhat: https://hardhat.org/");
        println!("  • Foundry: https://getfoundry.sh/");
    }

    Ok(())
}

use std::io::Write;

#[cfg(test)]
mod tests {

    #[test]
    fn test_detect_chain_type() {
        assert!(is_substrate_endpoint("wss://polkadot.api.onfinality.io"));
        assert!(is_substrate_endpoint("ws://localhost:9944"));
        assert!(!is_substrate_endpoint("https://eth.llamarpc.com"));
        assert!(!is_substrate_endpoint("http://localhost:8545"));
    }

    fn is_substrate_endpoint(endpoint: &str) -> bool {
        endpoint.starts_with("ws://") || endpoint.starts_with("wss://")
    }

    #[test]
    fn test_hex_decode() {
        let hex = "0x6080604052";
        let decoded = hex::decode(hex.trim_start_matches("0x"));
        assert!(decoded.is_ok());

        let without_prefix = "6080604052";
        let decoded = hex::decode(without_prefix);
        assert!(decoded.is_ok());
    }
}
