use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rfb-cli",
    version = "0.1.0",
    about = "A command-line tool in Rust to communicate with Bitcoin Core JSON-RPC interface"
)]
pub struct Cli {
    /// RPC server URL (e.g., http://127.0.0.1:18443)
    #[arg(long, global = true)]
    pub rpc_url: Option<String>,

    /// RPC username
    #[arg(long, global = true)]
    pub rpc_user: Option<String>,

    /// RPC password
    #[arg(long, global = true)]
    pub rpc_pass: Option<String>,

    /// Target wallet name (e.g., Miner, Trader)
    #[arg(long, global = true)]
    pub wallet: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Display blockchain metrics (Chain, Blocks, Headers, Difficulty, verification progress)
    BlockchainInfo,

    /// Display wallet metrics (Wallet name, Balance, Unconfirmed balance, transaction count)
    WalletInfo,

    /// Print the current wallet balance directly
    Balance,

    /// Generate a new address for receiving payments
    NewAddress,

    /// Run an arbitrary Bitcoin Core RPC command with dynamic arguments
    Rpc {
        /// The RPC method name (e.g. getblockhash)
        method: String,

        /// Positional parameters to send with the RPC call
        params: Vec<String>,
    },
}
