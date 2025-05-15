// File: src-tauri/src/rpc_client.rs
// Description: Handles generic RPC communication logic.
// Changes:
// - Moved RpcResponse, RpcError, VerusRpcError, and make_rpc_call from verus_rpc.rs.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

// Define structs for the JSON-RPC request and response
#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<RpcError>,
    // id: Option<serde_json::Value>, // We don't strictly need the id for this case
}

#[derive(Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

// Custom error type for our function
#[derive(Debug, thiserror::Error, Serialize, Clone)]
pub enum VerusRpcError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("RPC error {code}: {message}")]
    Rpc { code: i32, message: String },
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("RPC call timed out")]
    Timeout,
    #[error("RPC response format error: missing result and error fields")]
    Format,
    #[error("Identity not found or cannot receive private messages")]
    NotFoundOrIneligible, // Keep this here as it's a general RPC outcome
    #[error("Invalid VerusID format")]
    InvalidFormat, // Keep this here as it's a general RPC outcome
}

// Convert reqwest::Error to String for serialization
impl From<reqwest::Error> for VerusRpcError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            VerusRpcError::Timeout
        } else if err.is_connect() || err.is_request() {
            VerusRpcError::NetworkError(err.to_string())
        } else {
            VerusRpcError::ParseError(err.to_string())
        }
    }
}

// Helper function for generic RPC calls
pub async fn make_rpc_call<T: for<'de> Deserialize<'de>>(
    rpc_user: &str,
    rpc_pass: &str,
    method: &str,
    params: Vec<Value>,
) -> Result<T, VerusRpcError> {
    let client = reqwest::Client::new();
    let rpc_url = "http://localhost:18843";

    let request_body = json!({
        "jsonrpc": "1.0",
        "id": format!("chat-dapp-{}", method),
        "method": method,
        "params": params
    });

    log::debug!("Making RPC call: method={}, params={:?}", method, params);

    let request = client
        .post(rpc_url)
        .basic_auth(rpc_user, Some(rpc_pass))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(10));

    match request.send().await {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(VerusRpcError::Rpc { code: 401, message: "Authentication failed.".to_string() });
            }
            match response.error_for_status() {
                Ok(successful_response) => {
                    match successful_response.json::<RpcResponse<T>>().await {
                        Ok(rpc_response) => {
                            if let Some(result) = rpc_response.result {
                                Ok(result)
                            } else if let Some(err) = rpc_response.error {
                                Err(VerusRpcError::Rpc { code: err.code, message: err.message })
                            } else {
                                Err(VerusRpcError::Format)
                            }
                        }
                        Err(e) => {
                           let verus_error: VerusRpcError = e.into();
                           Err(verus_error)
                       }
                    }
                }
                Err(status_error) => Err(status_error.into()),
            }
        }
        Err(e) => {
           let verus_error: VerusRpcError = e.into();
           Err(verus_error)
        }
    }
} 