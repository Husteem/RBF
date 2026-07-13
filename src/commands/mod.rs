pub mod blockchain;
pub mod wallet;
pub mod address;

use crate::error::CliError;
use crate::rpc::BitcoinRpcClient;
use colored::*;

/// Runner for the generic RPC command.
/// Parses string arguments dynamically (coercing numbers, booleans, arrays, objects, and strings)
/// and prints the formatted JSON result returned by the Bitcoin Core node.
pub async fn run_generic_rpc(
    client: &BitcoinRpcClient,
    method: &str,
    raw_params: &[String],
) -> Result<(), CliError> {
    println!(
        "{} {}{}...",
        "Executing RPC:".dimmed(),
        method.cyan().bold(),
        format!(" with {} parameter(s)", raw_params.len()).dimmed()
    );

    // Convert raw string arguments to typed JSON-RPC parameters
    let params: Vec<serde_json::Value> = raw_params.iter().map(|p| parse_param(p)).collect();

    // Call the generic RPC endpoint
    let result: serde_json::Value = client.call(method, params).await?;

    // Print the result pretty-printed
    let pretty_json = serde_json::to_string_pretty(&result)
        .unwrap_or_else(|_| result.to_string());
    
    println!("\n{}", "=== RPC Response ===".green().bold());
    println!("{}", pretty_json.yellow());
    println!("{}", "====================".green().bold());

    Ok(())
}

/// Helper function to parse CLI inputs into typed JSON-RPC values.
/// This matches `bitcoin-cli` behavior:
/// - If a string parses as valid JSON (like numbers, booleans, arrays, objects), use that.
/// - Otherwise, fall back to treating it as a raw JSON string.
fn parse_param(param: &str) -> serde_json::Value {
    match serde_json::from_str::<serde_json::Value>(param) {
        Ok(val) => val,
        Err(_) => serde_json::Value::String(param.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_param_types() {
        // Integer
        assert_eq!(parse_param("200"), json!(200));
        
        // Float
        assert_eq!(parse_param("1.25"), json!(1.25));

        // Boolean
        assert_eq!(parse_param("true"), json!(true));
        assert_eq!(parse_param("false"), json!(false));

        // Null
        assert_eq!(parse_param("null"), json!(null));

        // Array
        assert_eq!(parse_param("[1, 2, 3]"), json!([1, 2, 3]));

        // Object
        assert_eq!(parse_param("{\"a\": 1}"), json!({"a": 1}));

        // String fallback (since plain word is not valid JSON string without quotes)
        assert_eq!(parse_param("genesis"), json!("genesis"));
        assert_eq!(
            parse_param("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"),
            json!("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
        );
    }
}
