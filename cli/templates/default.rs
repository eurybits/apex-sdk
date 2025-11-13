//! Default Apex SDK project template

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to Apex SDK!");
    
    // Initialize the SDK (configure with your endpoints)
    // let sdk = ApexSDK::builder()
    //     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    //     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    //     .build()
    //     .await?;
    
    println!("Ready to build cross-chain applications!");
    
    Ok(())
}
