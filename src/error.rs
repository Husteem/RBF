use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum CliError {
    #[error("Connection failure: Could not connect to the Bitcoin Core node. Check if it is running.\nDetails: {0}")]
    Connection(reqwest::Error),

    #[error("Authentication failed: Invalid RPC username or password.")]
    Unauthorized,

    #[error("RPC Error (code {code}): {message}")]
    Rpc {
        code: i32,
        message: String,
    },

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("JSON Serialization/Deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("HTTP Error: {status} - {body}")]
    Http {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            CliError::Connection(err)
        } else if let Some(status) = err.status() {
            if status == reqwest::StatusCode::UNAUTHORIZED {
                CliError::Unauthorized
            } else {
                CliError::Connection(err)
            }
        } else {
            CliError::Connection(err)
        }
    }
}
