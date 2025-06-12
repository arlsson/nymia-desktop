// File: src-tauri/src/rpc_client.rs
// Description: Handles generic RPC communication logic.
// Changes:
// - Moved RpcResponse, RpcError, VerusRpcError, and make_rpc_call from verus_rpc.rs.
// - Added SignatureResponse struct for signmessage API response
// - Added signature verification specific error handling

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

// Signature response structure for signmessage API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignatureResponse {
    pub hash: String,
    pub signature: String,
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
    #[error("Message signing failed")]
    SigningFailed,
    #[error("Message verification failed")]
    VerificationFailed,
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

// Sign message using Verus signmessage RPC
pub async fn sign_message(
    rpc_user: &str,
    rpc_pass: &str,
    verusid: &str,
    message: &str,
) -> Result<SignatureResponse, VerusRpcError> {
    log::info!("Signing message with VerusID: {}", verusid);
    log::debug!("Message to sign: '{}'", message);

    let params = vec![json!(verusid), json!(message)];
    
    match make_rpc_call::<SignatureResponse>(rpc_user, rpc_pass, "signmessage", params).await {
        Ok(signature_response) => {
            log::info!("Message signed successfully. Hash: {}", signature_response.hash);
            Ok(signature_response)
        }
        Err(e) => {
            log::error!("Failed to sign message: {:?}", e);
            Err(VerusRpcError::SigningFailed)
        }
    }
}

// Verify message using Verus verifymessage RPC
pub async fn verify_message(
    rpc_user: &str,
    rpc_pass: &str,
    verusid: &str,
    signature: &str,
    message: &str,
) -> Result<bool, VerusRpcError> {
    log::debug!("Verifying message signature for VerusID: {}", verusid);
    log::debug!("Original message: '{}'", message);
    log::debug!("Signature: {}", signature);

    let params = vec![json!(verusid), json!(signature), json!(message)];
    
    match make_rpc_call::<bool>(rpc_user, rpc_pass, "verifymessage", params).await {
        Ok(is_valid) => {
            if is_valid {
                log::debug!("Message signature verified successfully for {}", verusid);
            } else {
                log::warn!("Message signature verification failed for {}", verusid);
            }
            Ok(is_valid)
        }
        Err(e) => {
            log::error!("Failed to verify message signature: {:?}", e);
            // Return false for verification failures rather than propagating the error
            // This ensures failed verifications are treated as invalid signatures
            Ok(false)
        }
    }
} 