// File: src-tauri/src/verus_rpc.rs
// Description: Handles RPC communication with the local Verus daemon.
// Changes:
// - Complete rewrite of identity handling to simplify and fix the private address issue.
// - FIX: Updated privateaddress location - it's inside the identity object, not at the top level.
// - Added `private_address` to FormattedIdentity and populate it.
// - FIX: Corrected listidentities parameters to (true, true, true).
// - FIX: Reverted privateaddress extraction logic to look inside the `identity` sub-object.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

// SIMPLIFIED: Use a simpler approach where we deserialize to a dynamic Value first
// This allows us to inspect the raw structure to figure out what's happening

// Struct to hold formatted identity name and addresses
#[derive(Serialize, Debug, Clone)]
pub struct FormattedIdentity {
    pub formatted_name: String,
    pub i_address: String, // Include original i-address for reference
    pub private_address: Option<String>, // Added optional private address
}

// Helper function for generic RPC calls
async fn make_rpc_call<T: for<'de> Deserialize<'de>>(
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
        .timeout(Duration::from_secs(10)); // Increased timeout slightly for potentially slower calls

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
                        Err(e) => Err(VerusRpcError::Parse(e)),
                    }
                }
                Err(status_error) => Err(VerusRpcError::Network(status_error)),
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Err(VerusRpcError::Timeout)
            } else {
                 Err(VerusRpcError::Network(e))
            }
        }
    }
}

// Function to connect and get block height
// Exposed as a Tauri command
pub async fn connect_and_get_block_height(
    rpc_user: String,
    rpc_pass: String,
) -> Result<u64, VerusRpcError> {
    log::info!("Attempting to connect to Verus daemon...");
    make_rpc_call(&rpc_user, &rpc_pass, "getblockcount", vec![]).await
}

// Updated function to include private address
pub async fn get_login_identities(
    rpc_user: String,
    rpc_pass: String,
) -> Result<Vec<FormattedIdentity>, VerusRpcError> {
    log::info!("Fetching identities for login selection...");

    // Use a dynamic Value to first see what the raw structure looks like
    let identities_raw: Vec<serde_json::Value> = make_rpc_call(
        &rpc_user,
        &rpc_pass,
        "listidentities",
        // FIX: Pass (verbose=true, include_watchonly=true, include_private_data=true)
        vec![json!(true), json!(true), json!(true)] 
    ).await?;

    log::info!("Received {} raw identity entries from listidentities.", identities_raw.len());

    let mut formatted_identities = Vec::new();

    // Process each identity
    for identity_obj in identities_raw {
        // Log the entire structure to help debugging
        log::debug!("Raw identity: {}", serde_json::to_string(&identity_obj).unwrap_or_default());

        // FIX: Look for privateaddress inside the `identity` sub-object, as it worked before.
        if let Some(identity_details) = identity_obj.get("identity") {
            // Extract privateaddress FROM WITHIN the identity object
            let private_address_opt = identity_details.get("privateaddress")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(String::from); // Convert to Option<String>

            // Only process identities where we found a private address
            if let Some(private_address) = private_address_opt {
                // Also get name and i-address from the identity_details or identity_obj
                if let (Some(name), Some(i_address)) = (
                    identity_details.get("name").and_then(|v| v.as_str()),
                    identity_details.get("identityaddress").and_then(|v| v.as_str())
                ) {
                    log::debug!("Found identity with private address: {} ({})", name, i_address);

                    // Format the name with @ suffix to indicate it's a VerusID
                    let formatted_name = format!("{}@", name);

                    formatted_identities.push(FormattedIdentity {
                        formatted_name,
                        i_address: i_address.to_string(),
                        private_address: Some(private_address), // Store the found private address
                    });
                } else {
                    log::warn!("Identity has private address but missing name or i-address in identity details.");
                }
            } else {
                 log::debug!("Skipping identity '{}' because no private address found in identity object.", identity_details.get("name").and_then(|v| v.as_str()).unwrap_or("unknown"));
            }
        } else {
             log::warn!("Skipping raw identity entry because 'identity' sub-object is missing.");
        }
    }

    log::info!("Found {} identities with private addresses for login.", formatted_identities.len());

    // If no identities with private addresses, return an error
    if formatted_identities.is_empty() {
        log::error!("No VerusIDs with private addresses found in the wallet.");
        return Err(VerusRpcError::Rpc {
            code: -1,
            message: "No VerusIDs with private addresses found in your wallet.".to_string()
        });
    }

    Ok(formatted_identities)
} 