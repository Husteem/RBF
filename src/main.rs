mod cli;
mod config;
mod error;
mod rpc;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use rpc::BitcoinRpcClient;
use colored::*;

#[tokio::main]
async fn main() {
    // 1. Try to load variables from a .env file (if it exists)
    dotenvy::dotenv().ok();

    // 2. Parse command-line parameters using clap
    let args = Cli::parse();

    // 3. Resolve configuration parameters
    let config = match Config::resolve(args.rpc_url, args.rpc_user, args.rpc_pass, args.wallet) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}", "Configuration Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // 4. Initialize the custom Bitcoin RPC Client
    let client = BitcoinRpcClient::new(&config);

    // 5. Dispatch subcommands to corresponding modules
    let result = match args.command {
        Commands::BlockchainInfo => commands::blockchain::run(&client).await,
        Commands::WalletInfo => commands::wallet::run_info(&client).await,
        Commands::Balance => commands::wallet::run_balance(&client).await,
        Commands::NewAddress => commands::address::run(&client).await,
        Commands::Rpc { method, params } => {
            commands::run_generic_rpc(&client, &method, &params).await
        }
    };

    // 6. Handle errors gracefully and exit with non-zero code on failures
    if let Err(e) = result {
        eprintln!("\n{} {}", "Execution Error:".red().bold(), e.to_string().red());
        std::process::exit(1);
    }
}
