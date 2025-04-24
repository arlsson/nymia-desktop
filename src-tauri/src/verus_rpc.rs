// File: src-tauri/src/verus_rpc.rs
// Description: Handles RPC communication with the local Verus daemon.
// Changes:
// - Refactored HTTP status code handling using `response.error_for_status()`.
// - Explicitly handle 401 Unauthorized separately.

use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

// Define structs for the JSON-RPC request and response
#[derive(Deserialize, Debug)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
    // id: Option<serde_json::Value>, // We don't strictly need the id for this case
}

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

// Custom error type for our function
#[derive(Debug, thiserror::Error)]
pub enum VerusRpcError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("RPC error {code}: {message}")]
    Rpc { code: i32, message: String },
    #[error("Failed to parse response: {0}")]
    Parse(#[source] reqwest::Error),
    #[error("RPC call timed out")]
    Timeout,
    #[error("RPC response format error: missing result and error fields")]
    Format,
}

// Function to connect and get block height
// Exposed as a Tauri command
pub async fn connect_and_get_block_height(
    rpc_user: String,
    rpc_pass: String,
) -> Result<u64, VerusRpcError> {
    let client = reqwest::Client::new();
    let rpc_url = "http://localhost:18843"; // As per PRD for VRSCTEST

    let request_body = json!({
        "jsonrpc": "1.0",
        "id":"chat-dapp-connect",
        "method": "getblockcount",
        "params": []
    });

    log::info!("Attempting to connect to Verus daemon at {}", rpc_url);

    let request = client
        .post(rpc_url)
        .basic_auth(rpc_user, Some(rpc_pass))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(5)); // 5-second timeout as per PRD

    match request.send().await {
        Ok(response) => {
            // Handle 401 Unauthorized specifically
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                log::error!("Authentication failed (HTTP 401).");
                return Err(VerusRpcError::Rpc {
                    code: 401,
                    message: "Authentication failed. Check RPC username/password.".to_string(),
                });
            }

            // Check for other client/server errors (4xx, 5xx)
            match response.error_for_status() {
                Ok(successful_response) => {
                    // Status was 2xx, proceed to parse JSON
                    match successful_response.json::<RpcResponse<u64>>().await {
                        Ok(rpc_response) => {
                            if let Some(block_height) = rpc_response.result {
                                log::info!("Successfully connected. Block height: {}", block_height);
                                Ok(block_height)
                            } else if let Some(err) = rpc_response.error {
                                log::error!("RPC error from daemon: code={}, message={}", err.code, err.message);
                                Err(VerusRpcError::Rpc { code: err.code, message: err.message })
                            } else {
                                log::error!("RPC response format error: missing result and error fields");
                                Err(VerusRpcError::Format)
                            }
                        }
                        Err(e) => {
                            // JSON parsing failed on a 2xx response
                            log::error!("Failed to parse successful RPC response: {}", e);
                            Err(VerusRpcError::Parse(e))
                        }
                    }
                }
                Err(status_error) => {
                    // Status was 4xx/5xx (but not 401, handled above)
                    log::error!("HTTP error: {}", status_error);
                    Err(VerusRpcError::Network(status_error))
                }
            }
        }
        Err(e) => {
            // Original request sending failed (network issue, timeout, etc.)
            if e.is_timeout() {
                log::error!("Connection timed out after 5 seconds.");
                Err(VerusRpcError::Timeout)
            } else {
                 log::error!("Network request failed: {}", e);
                 Err(VerusRpcError::Network(e))
            }
        }
    }
} 