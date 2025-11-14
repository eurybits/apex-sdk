//! Multi-Chain DAO Governance Example
//!
//! This example demonstrates how to build a cross-chain DAO governance system
//! that enables coordination across Substrate and EVM chains.
//!
//! Features demonstrated:
//! - Multi-chain voting
//! - Cross-chain proposal execution
//! - Treasury management across chains
//! - Delegation and vote aggregation

use apex_sdk::prelude::*;

/// Represents a governance proposal
#[derive(Debug, Clone)]
struct Proposal {
    id: u64,
    title: String,
    description: String,
    proposer: Address,
    chain: Chain,
    votes_for: u128,
    votes_against: u128,
    status: ProposalStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

/// Represents a vote from a member
#[derive(Debug, Clone)]
struct Vote {
    voter: Address,
    proposal_id: u64,
    support: bool,
    voting_power: u128,
    chain: Chain,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Multi-Chain DAO Governance Example ===\n");

    // Initialize SDK for multi-chain governance
    println!("Initializing Multi-Chain DAO...");
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/demo")
        .build()
        .await?;

    println!("DAO initialized with cross-chain support\n");

    // Example 1: DAO Overview
    println!("Example 1: DAO Overview");

    println!("\n  DAO Statistics:");
    println!("    Total Members: 1,523");
    println!("    Polkadot Members: 892");
    println!("    Ethereum Members: 631");
    println!("    Total Treasury: $5.2M");
    println!("    Active Proposals: 3");

    // Example 2: Create Cross-Chain Proposal
    println!("\nExample 2: Create Cross-Chain Proposal");

    let proposer = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

    let proposal = Proposal {
        id: 1,
        title: "Allocate Treasury Funds for Development".to_string(),
        description: "Proposal to allocate 100 ETH from treasury for protocol development".to_string(),
        proposer: proposer.clone(),
        chain: Chain::Ethereum,
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Active,
    };

    println!("\n  New Proposal Created:");
    println!("    ID: {}", proposal.id);
    println!("    Title: {}", proposal.title);
    println!("    Proposer: {}", proposal.proposer.as_str());
    println!("    Chain: {:?}", proposal.chain);
    println!("    Status: {:?}", proposal.status);

    // Create proposal transaction
    let proposal_tx = sdk
        .transaction()
        .from(proposer.clone())
        .to_evm_address("0x1111111111111111111111111111111111111111") // DAO contract
        .amount(0)
        .with_data(format!("proposal:{}", proposal.id).into_bytes())
        .build()?;

    let result = sdk.execute(proposal_tx).await?;
    println!("\n  Proposal Transaction:");
    println!("    TX Hash: {}", result.source_tx_hash);
    println!("    Status: {:?}", result.status);

    // Example 3: Multi-Chain Voting
    println!("\nExample 3: Multi-Chain Voting");
    println!("  Aggregating votes from multiple chains...");

    // Votes from Ethereum
    let eth_votes = vec![
        Vote {
            voter: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            proposal_id: 1,
            support: true,
            voting_power: 1000_u128,
            chain: Chain::Ethereum,
        },
        Vote {
            voter: Address::evm("0x1234567890123456789012345678901234567890"),
            proposal_id: 1,
            support: true,
            voting_power: 500_u128,
            chain: Chain::Ethereum,
        },
    ];

    // Votes from Polkadot
    let dot_votes = vec![
        Vote {
            voter: Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
            proposal_id: 1,
            support: true,
            voting_power: 2000_u128,
            chain: Chain::Polkadot,
        },
        Vote {
            voter: Address::substrate("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"),
            proposal_id: 1,
            support: false,
            voting_power: 300_u128,
            chain: Chain::Polkadot,
        },
    ];

    println!("\n  Ethereum Votes:");
    for vote in &eth_votes {
        println!(
            "    {} - {} votes ({})",
            vote.voter.as_str(),
            vote.voting_power,
            if vote.support { "FOR" } else { "AGAINST" }
        );
    }

    println!("\n  Polkadot Votes:");
    for vote in &dot_votes {
        println!(
            "    {} - {} votes ({})",
            vote.voter.as_str(),
            vote.voting_power,
            if vote.support { "FOR" } else { "AGAINST" }
        );
    }

    // Calculate totals
    let total_for: u128 = eth_votes
        .iter()
        .chain(dot_votes.iter())
        .filter(|v| v.support)
        .map(|v| v.voting_power)
        .sum();

    let total_against: u128 = eth_votes
        .iter()
        .chain(dot_votes.iter())
        .filter(|v| !v.support)
        .map(|v| v.voting_power)
        .sum();

    println!("\n  Vote Summary:");
    println!("    Total FOR: {}", total_for);
    println!("    Total AGAINST: {}", total_against);
    println!("    Result: {}", if total_for > total_against { "PASSING" } else { "FAILING" });

    // Example 4: Vote Delegation
    println!("\nExample 4: Cross-Chain Vote Delegation");
    println!("  Member delegates voting power to another member...");

    let delegator = Address::evm("0x9999999999999999999999999999999999999999");
    let delegate = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

    println!("\n  Delegation Details:");
    println!("    From: {} (Ethereum)", delegator.as_str());
    println!("    To: {} (Polkadot)", delegate.as_str());
    println!("    Voting Power: 1,500");

    let delegation_tx = sdk
        .transaction()
        .from(delegator.clone())
        .to(delegate.clone())
        .amount(0)
        .with_data(b"delegate:1500".to_vec())
        .build()?;

    let delegation_result = sdk.execute(delegation_tx).await?;
    println!("\n  Delegation Transaction:");
    println!("    TX Hash: {}", delegation_result.source_tx_hash);
    if let Some(dest_tx) = &delegation_result.destination_tx_hash {
        println!("    Cross-chain TX: {}", dest_tx);
    }

    // Example 5: Proposal Execution
    println!("\nExample 5: Cross-Chain Proposal Execution");
    println!("  Executing passed proposal...");

    println!("\n  Execution Steps:");
    println!("    1. Verify quorum reached");
    println!("    2. Check voting period ended");
    println!("    3. Execute on source chain");
    println!("    4. Bridge execution to destination chain");

    let execution_tx = sdk
        .transaction()
        .from_evm_address("0x1111111111111111111111111111111111111111") // DAO contract
        .to_evm_address("0x2222222222222222222222222222222222222222") // Treasury
        .amount(100_000_000_000_000_000_000u128) // 100 ETH
        .with_data(b"execute:proposal:1".to_vec())
        .build()?;

    let exec_result = sdk.execute(execution_tx).await?;
    println!("\n  Execution Result:");
    println!("    TX Hash: {}", exec_result.source_tx_hash);
    println!("    Status: {:?}", exec_result.status);
    println!("    Funds transferred: ✓");

    // Example 6: Treasury Management
    println!("\nExample 6: Multi-Chain Treasury Management");
    println!("  Managing treasury across multiple chains...");

    println!("\n  Treasury Balances:");
    println!("    Ethereum: 500 ETH ($1.5M)");
    println!("    Polkadot: 50,000 DOT ($350K)");
    println!("    Polygon: 100,000 MATIC ($100K)");
    println!("    Total: $1.95M");

    println!("\n  Recent Treasury Transactions:");
    println!("    1. Received: 10 ETH from grants");
    println!("    2. Spent: 5,000 DOT for development");
    println!("    3. Bridged: 20 ETH → Polygon");

    // Example 7: Governance Analytics
    println!("\nExample 7: Governance Analytics");

    println!("\n  Participation Metrics:");
    println!("    Average Turnout: 67%");
    println!("    Ethereum Participation: 72%");
    println!("    Polkadot Participation: 64%");

    println!("\n  Proposal Statistics:");
    println!("    Total Proposals: 45");
    println!("    Passed: 32 (71%)");
    println!("    Rejected: 10 (22%)");
    println!("    Pending: 3 (7%)");

    println!("\n  Top Contributors:");
    println!("    1. Alice (Polkadot): 15 proposals");
    println!("    2. Bob (Ethereum): 12 proposals");
    println!("    3. Charlie (Multi-chain): 8 proposals");

    println!("\nAll DAO governance operations completed successfully!");
    println!("\nDAO Features:");
    println!("  Multi-chain proposal creation");
    println!("  Cross-chain voting aggregation");
    println!("  Vote delegation across chains");
    println!("  Automated proposal execution");
    println!("  Multi-chain treasury management");
    println!("  Comprehensive analytics");

    println!("\nGovernance Best Practices:");
    println!("  - Implement time-locks for critical proposals");
    println!("  - Require minimum quorum for valid votes");
    println!("  - Use quadratic voting to prevent whale dominance");
    println!("  - Enable emergency pause mechanisms");
    println!("  - Conduct regular treasury audits");
    println!("  - Maintain transparent voting records");

    println!("\nSecurity Considerations:");
    println!("  - Multi-signature requirements for treasury");
    println!("  - Rate limiting on proposal creation");
    println!("  - Sybil attack resistance mechanisms");
    println!("  - Vote buying prevention");
    println!("  - Cross-chain message verification");

    Ok(())
}
