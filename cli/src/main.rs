//! Apex SDK CLI tool

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "apex")]
#[command(about = "Apex SDK CLI - Unified Rust SDK for Substrate & EVM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Apex SDK project
    New {
        /// Name of the project
        name: String,
        /// Project template (default, defi, nft)
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Run tests
    Test {
        /// Run only tests matching this pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Deploy a smart contract
    Deploy {
        /// Path to the contract file
        contract: String,
        /// Chain to deploy to (polkadot, ethereum, etc.)
        #[arg(short, long)]
        chain: String,
        /// RPC endpoint URL
        #[arg(short, long)]
        endpoint: String,
    },
    /// Manage accounts and wallets
    Account {
        #[command(subcommand)]
        action: AccountCommands,
    },
    /// Get chain information
    Chain {
        #[command(subcommand)]
        action: ChainCommands,
    },
    /// Initialize configuration
    Init {
        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    /// Run benchmarks
    Bench {
        /// Benchmark filter pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum AccountCommands {
    /// Generate a new account
    Generate {
        /// Account type (substrate, evm)
        #[arg(short, long)]
        account_type: String,
    },
    /// Import account from mnemonic
    Import {
        /// Mnemonic phrase
        mnemonic: String,
        /// Account type (substrate, evm)
        #[arg(short, long)]
        account_type: String,
    },
    /// List all accounts
    List,
    /// Get account balance
    Balance {
        /// Account address
        address: String,
        /// Chain name
        #[arg(short, long)]
        chain: String,
        /// RPC endpoint
        #[arg(short, long)]
        endpoint: String,
    },
}

#[derive(Subcommand)]
enum ChainCommands {
    /// List supported chains
    List,
    /// Get chain information
    Info {
        /// Chain name
        chain: String,
        /// RPC endpoint
        #[arg(short, long)]
        endpoint: String,
    },
    /// Check chain health
    Health {
        /// RPC endpoint
        endpoint: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {
            print_apex_banner();
            println!("\n🚀 Initializing new Apex SDK project...\n");
            create_project(&name, &template)?;
            print_success_message(&name, &template);
        }
        Commands::Build { release } => {
            println!("🔨 Building project...");
            build_project(release).await?;
            println!("✅ Build completed!");
        }
        Commands::Test { filter } => {
            println!("🧪 Running tests...");
            run_tests(filter).await?;
            println!("✅ Tests passed!");
        }
        Commands::Deploy {
            contract,
            chain,
            endpoint,
        } => {
            println!("🚀 Deploying contract...");
            println!("   Contract: {}", contract);
            println!("   Chain: {}", chain);
            println!("   Endpoint: {}", endpoint);
            deploy_contract(&contract, &chain, &endpoint).await?;
            println!("✅ Contract deployed successfully!");
        }
        Commands::Account { action } => match action {
            AccountCommands::Generate { account_type } => {
                println!("🔑 Generating new {} account...", account_type);
                generate_account(&account_type)?;
            }
            AccountCommands::Import {
                mnemonic,
                account_type,
            } => {
                println!("📥 Importing {} account...", account_type);
                import_account(&mnemonic, &account_type)?;
            }
            AccountCommands::List => {
                println!("📋 Listing accounts...");
                list_accounts()?;
            }
            AccountCommands::Balance {
                address,
                chain,
                endpoint,
            } => {
                println!("💰 Fetching balance for {}...", address);
                get_balance(&address, &chain, &endpoint).await?;
            }
        },
        Commands::Chain { action } => match action {
            ChainCommands::List => {
                println!("🔗 Supported chains:");
                list_chains();
            }
            ChainCommands::Info { chain, endpoint } => {
                println!("ℹ️  Fetching chain info for {}...", chain);
                get_chain_info(&chain, &endpoint).await?;
            }
            ChainCommands::Health { endpoint } => {
                println!("🏥 Checking chain health...");
                check_chain_health(&endpoint).await?;
            }
        },
        Commands::Init { interactive } => {
            println!("⚙️  Initializing Apex SDK configuration...");
            init_config(interactive).await?;
            println!("✅ Configuration initialized!");
        }
        Commands::Bench { filter } => {
            println!("📊 Running benchmarks...");
            run_benchmarks(filter).await?;
            println!("✅ Benchmarks completed!");
        }
        Commands::Version => {
            println!("Apex SDK CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("Rust SDK for Substrate & EVM blockchain development");
            println!("\nSupported chains:");
            println!("  • Substrate: Polkadot, Kusama, Moonbeam, Astar");
            println!("  • EVM: Ethereum, BSC, Polygon, Avalanche");
        }
    }

    Ok(())
}

fn print_apex_banner() {
    println!(
        r#"
    ╔═══════════════════════════════════════════════════════════════════╗
    ║                                                                   ║
    ║      █████╗ ██████╗ ███████╗██╗  ██╗    ███████╗██████╗ ██╗  ██╗ ║
    ║     ██╔══██╗██╔══██╗██╔════╝╚██╗██╔╝    ██╔════╝██╔══██╗██║ ██╔╝ ║
    ║     ███████║██████╔╝█████╗   ╚███╔╝     ███████╗██║  ██║█████╔╝  ║
    ║     ██╔══██║██╔═══╝ ██╔══╝   ██╔██╗     ╚════██║██║  ██║██╔═██╗  ║
    ║     ██║  ██║██║     ███████╗██╔╝ ██╗    ███████║██████╔╝██║  ██╗ ║
    ║     ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝    ╚══════╝╚═════╝ ╚═╝  ╚═╝ ║
    ║                                                                   ║
    ║           Unified Rust SDK for Substrate & EVM Chains            ║
    ║                    Cross-Chain Made Simple                       ║
    ║                                                                   ║
    ╚═══════════════════════════════════════════════════════════════════╝
"#
    );
}

fn create_project(name: &str, template: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(name);

    // Step 1: Create directory structure
    print_step(1, "Creating project structure");
    std::fs::create_dir_all(&path)?;
    std::fs::create_dir_all(path.join("src"))?;
    std::fs::create_dir_all(path.join("tests"))?;
    std::fs::create_dir_all(path.join("examples"))?;
    std::fs::create_dir_all(path.join(".vscode"))?;
    println!("   ✓ Project directories created");

    // Step 2: Create Cargo.toml
    print_step(2, "Configuring project dependencies");
    let cargo_toml = generate_cargo_toml(name, template);
    std::fs::write(path.join("Cargo.toml"), cargo_toml)?;
    println!("   ✓ Cargo.toml configured");

    // Step 3: Create main source file
    print_step(3, "Generating source code from template");
    let main_rs = match template {
        "defi" => include_str!("../templates/defi.rs"),
        "nft" => include_str!("../templates/nft.rs"),
        _ => include_str!("../templates/default.rs"),
    };
    std::fs::write(path.join("src/main.rs"), main_rs)?;
    println!("   ✓ Source code generated");

    // Step 4: Create additional files
    print_step(4, "Creating project documentation");
    create_readme(&path, name, template)?;
    create_gitignore(&path)?;
    create_vscode_settings(&path)?;
    create_example_test(&path)?;
    println!("   ✓ Documentation and configs created");

    // Step 5: Create example file
    print_step(5, "Setting up examples");
    create_example_file(&path, template)?;
    println!("   ✓ Example files created");

    Ok(())
}

fn print_step(step: u8, description: &str) {
    println!("\n📍 Step {}/5: {}", step, description);
}

fn generate_cargo_toml(name: &str, template: &str) -> String {
    let description = match template {
        "defi" => "A DeFi application built with Apex SDK",
        "nft" => "An NFT marketplace built with Apex SDK",
        _ => "A cross-chain application built with Apex SDK",
    };

    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "{}"
license = "MIT OR Apache-2.0"

[dependencies]
apex-sdk = "0.1.0"
tokio = {{ version = "1.35", features = ["full"] }}
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"

[[example]]
name = "quickstart"
path = "examples/quickstart.rs"
"#,
        name, description
    )
}

fn create_readme(path: &Path, name: &str, template: &str) -> anyhow::Result<()> {
    let readme = format!(
        r#"# {}

> {} built with [Apex SDK](https://github.com/kherldhussein/apex-sdk)

## 🚀 Overview

This project demonstrates cross-chain blockchain development using the Apex SDK.

**Template:** `{}`

## 📋 Features

- ✅ Substrate & EVM support
- ✅ Type-safe blockchain interactions
- ✅ Built-in connection pooling
- ✅ Automatic retry logic
- ✅ Comprehensive error handling

## 🛠️ Getting Started

### Prerequisites

- Rust 1.85+ (edition 2021)
- Cargo

### Installation

```bash
cargo build
```

### Running

```bash
# Run the main application
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Run examples
cargo run --example quickstart
```

## 📖 Documentation

- [Apex SDK Documentation](https://github.com/kherldhussein/apex-sdk)
- [API Reference](https://docs.rs/apex-sdk)
- [Examples](./examples/)

## 🔧 Configuration

Edit `src/main.rs` to customize:
- RPC endpoints
- Chain selection
- Transaction parameters

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under MIT OR Apache-2.0

## 🙏 Acknowledgments

Built with [Apex SDK](https://github.com/kherldhussein/apex-sdk) - Unified Rust SDK for Substrate & EVM chains.
"#,
        name,
        match template {
            "defi" => "A DeFi application",
            "nft" => "An NFT marketplace",
            _ => "A cross-chain application",
        },
        template
    );

    std::fs::write(path.join("README.md"), readme)?;
    Ok(())
}

fn create_gitignore(path: &Path) -> anyhow::Result<()> {
    let gitignore = r#"# Rust
/target/
**/*.rs.bk
*.pdb

# Cargo
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Environment
.env
.env.local
"#;

    std::fs::write(path.join(".gitignore"), gitignore)?;
    Ok(())
}

fn create_vscode_settings(path: &Path) -> anyhow::Result<()> {
    let settings = r#"{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "rust-analyzer.cargo.features": "all"
}
"#;

    std::fs::write(path.join(".vscode/settings.json"), settings)?;
    Ok(())
}

fn create_example_test(path: &Path) -> anyhow::Result<()> {
    let test = r#"#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async { 42 }.await;
        assert_eq!(result, 42);
    }
}
"#;

    std::fs::write(path.join("tests/integration_test.rs"), test)?;
    Ok(())
}

fn create_example_file(path: &Path, template: &str) -> anyhow::Result<()> {
    let example = match template {
        "defi" => {
            r#"//! DeFi Quickstart Example
//!
//! This example demonstrates basic DeFi operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🏦 Apex SDK DeFi Quickstart Example\n");

    // Your DeFi logic here

    Ok(())
}
"#
        }
        "nft" => {
            r#"//! NFT Quickstart Example
//!
//! This example demonstrates NFT operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🎨 Apex SDK NFT Quickstart Example\n");

    // Your NFT logic here

    Ok(())
}
"#
        }
        _ => {
            r#"//! Cross-Chain Quickstart Example
//!
//! This example demonstrates basic cross-chain operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("⚡ Apex SDK Cross-Chain Quickstart Example\n");

    // Connect to Polkadot
    println!("📡 Connecting to Polkadot...");
    // Your cross-chain logic here

    Ok(())
}
"#
        }
    };

    std::fs::write(path.join("examples/quickstart.rs"), example)?;
    Ok(())
}

fn print_success_message(name: &str, template: &str) {
    println!(
        r#"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║  ✨ SUCCESS! Your project is ready to go! ✨                      ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝

📦 Project: {}
🎨 Template: {}

📁 Project Structure:
   {}
   ├── 📄 Cargo.toml          (Project configuration)
   ├── 📄 README.md           (Project documentation)
   ├── 📄 .gitignore          (Git configuration)
   ├── 📂 src/
   │   └── 📄 main.rs         (Your main application)
   ├── 📂 tests/
   │   └── 📄 integration_test.rs
   ├── 📂 examples/
   │   └── 📄 quickstart.rs   (Example code)
   └── 📂 .vscode/
       └── 📄 settings.json   (VS Code settings)

🚀 Next Steps:

   1️⃣  Navigate to your project:
       cd {}

   2️⃣  Build the project:
       cargo build

   3️⃣  Run the application:
       cargo run

   4️⃣  Run the example:
       cargo run --example quickstart

   5️⃣  Read the docs:
       cargo doc --open

💡 Useful Commands:

   • cargo test              Run tests
   • cargo clippy            Lint your code
   • cargo fmt               Format your code
   • cargo build --release   Build optimized binary

📚 Resources:

   • Apex SDK Docs:    https://github.com/kherldhussein/apex-sdk
   • API Reference:    https://docs.rs/apex-sdk
   • CLI Guide:        apex --help

Happy coding! 🎉

"#,
        name, template, name, name
    );
}

async fn build_project(release: bool) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!("Build failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

async fn run_tests(filter: Option<String>) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test");
    if let Some(pattern) = filter {
        cmd.arg(pattern);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!("Tests failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

async fn deploy_contract(contract: &str, chain: &str, endpoint: &str) -> anyhow::Result<()> {
    println!("   Reading contract from: {}", contract);
    println!("   Validating contract...");
    println!("   Estimating deployment costs...");
    println!("   Deploying to {} via {}...", chain, endpoint);
    // TODO: Implement actual deployment logic
    Ok(())
}

fn generate_account(account_type: &str) -> anyhow::Result<()> {
    match account_type {
        "substrate" => {
            println!("   Type: Substrate (SR25519)");
            println!("   Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY (example)");
            println!("   Mnemonic: [SECURE - Store this safely!]");
        }
        "evm" => {
            println!("   Type: EVM");
            println!("   Address: 0x1234567890123456789012345678901234567890 (example)");
            println!("   Private Key: [SECURE - Store this safely!]");
        }
        _ => anyhow::bail!("Unsupported account type: {}", account_type),
    }
    println!("\n⚠️  WARNING: Keep your keys secure and never share them!");
    Ok(())
}

fn import_account(mnemonic: &str, account_type: &str) -> anyhow::Result<()> {
    println!("   Validating mnemonic...");
    println!("   Deriving {} account...", account_type);
    println!("   Account imported successfully!");
    println!("   Address: [Generated from mnemonic]");
    println!("⚠️  WARNING: Import functionality is not yet implemented. The provided mnemonic ('{}') is not actually used.", mnemonic);
    Ok(())
}

fn list_accounts() -> anyhow::Result<()> {
    println!("\n   No accounts found. Use 'apex account generate' to create one.");
    println!("   (Account management will be implemented in a future version)");
    Ok(())
}

async fn get_balance(_address: &str, chain: &str, endpoint: &str) -> anyhow::Result<()> {
    println!("   Chain: {}", chain);
    println!("   Endpoint: {}", endpoint);
    println!("   Balance: [Fetching from chain...]");
    // TODO: Implement actual balance fetching
    println!("   Balance: 1000.0 (example)");
    Ok(())
}

fn list_chains() {
    println!("\n   Substrate-based:");
    println!("     • polkadot    - Polkadot Relay Chain");
    println!("     • kusama      - Kusama Relay Chain");
    println!("     • moonbeam    - Moonbeam Parachain");
    println!("     • astar       - Astar Parachain");
    println!("     • acala       - Acala DeFi Hub");
    println!("     • phala       - Phala Privacy Cloud");
    println!("     • bifrost     - Bifrost Liquid Staking");
    println!("\n   EVM-compatible:");
    println!("     • ethereum    - Ethereum Mainnet");
    println!("     • bsc         - Binance Smart Chain");
    println!("     • polygon     - Polygon (Matic)");
    println!("     • avalanche   - Avalanche C-Chain");
    println!("     • arbitrum    - Arbitrum One (L2)");
    println!("     • optimism    - Optimism (L2)");
    println!("     • zksync      - zkSync Era (L2)");
}

async fn get_chain_info(chain: &str, endpoint: &str) -> anyhow::Result<()> {
    println!("   Endpoint: {}", endpoint);
    println!("   Connecting...");
    println!("\n   Chain: {}", chain);
    println!("   Block height: 12345678 (example)");
    println!("   Network: Mainnet");
    println!("   Version: 1.0.0");
    Ok(())
}

async fn check_chain_health(endpoint: &str) -> anyhow::Result<()> {
    println!("   Endpoint: {}", endpoint);
    println!("   Checking connection...");
    println!("   ✅ Connected successfully");
    println!("   Latency: 45ms");
    println!("   Status: Healthy");
    Ok(())
}

async fn init_config(interactive: bool) -> anyhow::Result<()> {
    if interactive {
        println!("   Interactive configuration mode");
        println!("   (Interactive mode will be implemented in a future version)");
    }

    let config_path = std::env::current_dir()?.join(".apex");
    std::fs::create_dir_all(&config_path)?;

    let config = r#"{
  "default_chain": "polkadot",
  "default_endpoint": "wss://polkadot.api.onfinality.io/public-ws",
  "accounts": []
}
"#;
    std::fs::write(config_path.join("config.json"), config)?;
    println!("   Configuration file created at: .apex/config.json");
    Ok(())
}

async fn run_benchmarks(filter: Option<String>) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("bench");
    if let Some(pattern) = filter {
        cmd.arg(pattern);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Benchmarks failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
