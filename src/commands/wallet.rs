use crate::error::CliError;
use crate::rpc::BitcoinRpcClient;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
pub struct GetWalletInfoResult {
    pub walletname: String,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub txcount: u64,
}

pub async fn run_info(client: &BitcoinRpcClient) -> Result<(), CliError> {
    println!("{}", "Fetching wallet information...".dimmed());

    let info: GetWalletInfoResult = client.call("getwalletinfo", vec![]).await?;

    println!("\n{}", "=== Wallet Info ===".green().bold());
    println!("{:<24} : {}", "Wallet Name", info.walletname.cyan().bold());
    println!("{:<24} : {} BTC", "Balance", format!("{:.8}", info.balance).yellow().bold());
    println!("{:<24} : {} BTC", "Unconfirmed Balance", format!("{:.8}", info.unconfirmed_balance).yellow());
    println!("{:<24} : {}", "Transaction Count", info.txcount.to_string().magenta());
    println!("{}", "===================".green().bold());

    Ok(())
}

pub async fn run_balance(client: &BitcoinRpcClient) -> Result<(), CliError> {
    // We can call the specific `getbalance` RPC command
    let balance: f64 = client.call("getbalance", vec![]).await?;
    println!("{} {} BTC", "Balance:".green().bold(), format!("{:.8}", balance).yellow().bold());
    Ok(())
}
