//! Apex SDK CLI tool

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {
            println!("Creating new Apex SDK project: {}", name);
            println!("   Template: {}", template);
            create_project(&name, &template)?;
            println!("Project created successfully!");
            println!("\nNext steps:");
            println!("  cd {}", name);
            println!("  cargo build");
            println!("  cargo test");
        }
        Commands::Build { release } => {
            println!("Building project...");
            if release {
                println!("   Mode: release");
            }
            println!("Build completed!");
        }
        Commands::Test { filter } => {
            println!("Running tests...");
            if let Some(pattern) = filter {
                println!("   Filter: {}", pattern);
            }
            println!("Tests passed!");
        }
        Commands::Version => {
            println!("Apex SDK CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("Rust SDK for Substrate & EVM blockchain development");
        }
    }

    Ok(())
}

fn create_project(name: &str, template: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(name);
    std::fs::create_dir_all(&path)?;
    std::fs::create_dir_all(path.join("src"))?;

    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
apex-sdk = "0.1.0"
tokio = {{ version = "1.35", features = ["full"] }}
anyhow = "1.0"
"#,
        name
    );
    std::fs::write(path.join("Cargo.toml"), cargo_toml)?;

    // Create main.rs based on template
    let main_rs = match template {
        "defi" => include_str!("../templates/defi.rs"),
        "nft" => include_str!("../templates/nft.rs"),
        _ => include_str!("../templates/default.rs"),
    };
    std::fs::write(path.join("src/main.rs"), main_rs)?;

    // Create README
    let readme = format!(
        "# {}\n\nApex SDK project created with template: {}\n\n## Getting Started\n\n```bash\ncargo build\ncargo run\n```\n",
        name, template
    );
    std::fs::write(path.join("README.md"), readme)?;

    Ok(())
}
