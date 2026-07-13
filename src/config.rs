use std::env;
use crate::error::CliError;

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_user: Option<String>,
    pub rpc_pass: Option<String>,
    pub wallet: Option<String>,
}

impl Config {
    /// Resolves configuration parameters by combining CLI options and environment variables.
    /// Priority: CLI Flags > Environment Variables > Defaults.
    pub fn resolve(
        cli_url: Option<String>,
        cli_user: Option<String>,
        cli_pass: Option<String>,
        cli_wallet: Option<String>,
    ) -> Result<Self, CliError> {
        // Resolve URL: CLI -> env(RPC_URL) -> Default to localhost Regtest port 18443
        let rpc_url = cli_url
            .or_else(|| env::var("RPC_URL").ok())
            .unwrap_or_else(|| "http://127.0.0.1:18443".to_string());

        // Resolve RPC User: CLI -> env(RPC_USER)
        let rpc_user = cli_user.or_else(|| env::var("RPC_USER").ok());

        // Resolve RPC Pass: CLI -> env(RPC_PASS)
        let rpc_pass = cli_pass.or_else(|| env::var("RPC_PASS").ok());

        // Resolve Wallet: CLI -> env(RPC_WALLET)
        let wallet = cli_wallet.or_else(|| env::var("RPC_WALLET").ok());

        Ok(Config {
            rpc_url,
            rpc_user,
            rpc_pass,
            wallet,
        })
    }
}
