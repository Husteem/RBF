use crate::error::CliError;
use crate::rpc::BitcoinRpcClient;
use colored::*;

pub async fn run(client: &BitcoinRpcClient) -> Result<(), CliError> {
    println!("{}", "Generating a new receiving address...".dimmed());

    let address: String = client.call("getnewaddress", vec![]).await?;

    println!("{} {}", "New Address:".green().bold(), address.cyan().bold());

    Ok(())
}
