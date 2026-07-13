use crate::error::CliError;
use crate::rpc::BitcoinRpcClient;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
pub struct GetBlockchainInfoResult {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

pub async fn run(client: &BitcoinRpcClient) -> Result<(), CliError> {
    println!("{}", "Fetching blockchain information...".dimmed());
    
    let info: GetBlockchainInfoResult = client.call("getblockchaininfo", vec![]).await?;

    println!("\n{}", "=== Blockchain Info ===".green().bold());
    println!("{:<24} : {}", "Chain", info.chain.cyan().bold());
    println!("{:<24} : {}", "Blocks", info.blocks.to_string().yellow());
    println!("{:<24} : {}", "Headers", info.headers.to_string().yellow());
    println!("{:<24} : {}", "Difficulty", format!("{:.8}", info.difficulty).magenta());
    
    // Format verification progress as percentage
    let progress_percentage = info.verificationprogress * 100.0;
    println!(
        "{:<24} : {}", 
        "Verification Progress", 
        format!("{:.4}%", progress_percentage).blue()
    );
    println!("{}", "=======================".green().bold());

    Ok(())
}
