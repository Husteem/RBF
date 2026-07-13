use crate::config::Config;
use crate::error::CliError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: &'static str,
    method: String,
    params: Vec<Value>,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcErrorPayload>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RpcErrorPayload {
    pub code: i32,
    pub message: String,
}

pub struct BitcoinRpcClient {
    client: Client,
    url: String,
    user: Option<String>,
    pass: Option<String>,
}

impl BitcoinRpcClient {
    /// Creates a new instance of the client using resolved config parameters.
    pub fn new(config: &Config) -> Self {
        let mut url = config.rpc_url.clone();
        if let Some(ref wallet_name) = config.wallet {
            if url.ends_with('/') {
                url.pop();
            }
            url = format!("{}/wallet/{}", url, wallet_name);
        }

        BitcoinRpcClient {
            client: Client::new(),
            url,
            user: config.rpc_user.clone(),
            pass: config.rpc_pass.clone(),
        }
    }

    /// Performs an async JSON-RPC call.
    pub async fn call<T>(&self, method: &str, params: Vec<Value>) -> Result<T, CliError>
    where
        T: serde::de::DeserializeOwned,
    {
        let payload = RpcRequest {
            jsonrpc: "1.0",
            id: "rfb-cli",
            method: method.to_string(),
            params,
        };

        let mut req = self.client.post(&self.url);
        if let (Some(u), Some(p)) = (&self.user, &self.pass) {
            req = req.basic_auth(u, Some(p));
        }

        let resp = match self.client.execute(req.json(&payload).build()?).await {
            Ok(r) => r,
            Err(e) => return Err(CliError::from(e)),
        };

        let status = resp.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CliError::Unauthorized);
        }

        let body_text = resp.text().await?;

        // Try to parse the standard JSON-RPC envelope first
        let rpc_resp: RpcResponse<Value> = match serde_json::from_str(&body_text) {
            Ok(r) => r,
            Err(_) => {
                if !status.is_success() {
                    return Err(CliError::Http { status, body: body_text });
                } else {
                    return Err(CliError::Config(format!("Invalid response format: {}", body_text)));
                }
            }
        };

        // Handle error payloads returned by Bitcoin Core JSON-RPC
        if let Some(err_payload) = rpc_resp.error {
            // Code -18: Requested wallet does not exist or is not loaded.
            if err_payload.code == -18 {
                return Err(CliError::Wallet(err_payload.message));
            }
            return Err(CliError::Rpc {
                code: err_payload.code,
                message: err_payload.message,
            });
        }

        // Deserialize the result field into the expected type T
        if let Some(res) = rpc_resp.result {
            serde_json::from_value(res).map_err(CliError::Serialization)
        } else {
            // If result is null/None, check if T can be deserialized from null (e.g. Unit or Option)
            serde_json::from_value(Value::Null).map_err(CliError::Serialization)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_url_resolution() {
        let config = Config {
            rpc_url: "http://localhost:18443/".to_string(),
            rpc_user: Some("user".to_string()),
            rpc_pass: Some("pass".to_string()),
            wallet: Some("testwallet".to_string()),
        };
        let client = BitcoinRpcClient::new(&config);
        assert_eq!(client.url, "http://localhost:18443/wallet/testwallet");
    }
}
