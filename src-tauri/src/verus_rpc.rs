// File: src-tauri/src/verus_rpc.rs
// Description: Handles RPC communication with the local Verus daemon.
// Changes:
// - Complete rewrite of identity handling to simplify and fix the private address issue.
// - FIX: Updated privateaddress location - it's inside the identity object, not at the top level.
// - Added `private_address` to FormattedIdentity and populate it.
// - FIX: Corrected listidentities parameters to (true, true, true).
// - FIX: Reverted privateaddress extraction logic to look inside the `identity` sub-object.
// - Implemented sub-ID formatting (name.parentname@) using getidentity.
// - Added check_identity_eligibility function for New Chat flow.
// - Added get_chat_history function for New Chat flow.
// - Added ChatMessage struct.
// - Added NotFoundOrIneligible and InvalidFormat error variants to VerusRpcError.
// - Added send_private_message function to send messages/gifts via z_sendmany.
// - Added hex crate import.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use hex; // Added for memo encoding

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
    NotFoundOrIneligible,
    #[error("Invalid VerusID format")]
    InvalidFormat,
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

// SIMPLIFIED: Use a simpler approach where we deserialize to a dynamic Value first
// This allows us to inspect the raw structure to figure out what's happening

// Struct to hold formatted identity name and addresses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormattedIdentity {
    pub formatted_name: String,
    pub i_address: String, // Include original i-address for reference
    pub private_address: Option<String>, // Added optional private address
}

// Struct for imported chat messages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: String, // txid
    pub sender: String, // target_identity_name (the sender in this context)
    pub text: String, // Parsed message content
    pub timestamp: u64, // Transaction timestamp (if available, else 0 or estimate) - Needs more investigation
    pub amount: f64, // Amount from the transaction
    pub confirmations: i64, // Confirmations from the transaction
    pub direction: String, // "received"
}

// Struct for the z_listreceivedbyaddress RPC response item
#[derive(Deserialize, Debug)]
struct ReceivedByAddressEntry {
    txid: String,
    amount: f64,
    confirmations: i64,
    memostr: Option<String>, // Memo might be absent
    // memo: String, // We only need memostr
    // outindex: u32,
    // change: bool,
    // blocktime: Option<u64>, // Add blocktime if available and needed for timestamp
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
                        Err(e) => {
                           // Convert reqwest::Error into our VerusRpcError
                           let verus_error: VerusRpcError = e.into();
                           Err(verus_error)
                       }
                    }
                }
                Err(status_error) => Err(status_error.into()), // Convert reqwest::Error here too
            }
        }
        Err(e) => {
           // Convert reqwest::Error into our VerusRpcError
           let verus_error: VerusRpcError = e.into();
           Err(verus_error)
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

// Updated function to include private address and sub-ID formatting
pub async fn get_login_identities(
    rpc_user: String,
    rpc_pass: String,
) -> Result<Vec<FormattedIdentity>, VerusRpcError> {
    log::info!("Fetching identities for login selection...");

    let identities_raw: Vec<Value> = make_rpc_call(
        &rpc_user,
        &rpc_pass,
        "listidentities",
        vec![json!(true), json!(true), json!(true)],
    )
    .await?;

    log::info!("Received {} raw identity entries from listidentities.", identities_raw.len());

    let mut formatted_identities = Vec::new();

    for identity_obj in identities_raw {
        log::debug!("Raw identity: {}", serde_json::to_string(&identity_obj).unwrap_or_default());

        if let Some(identity_details) = identity_obj.get("identity") {
            let private_address_opt = identity_details.get("privateaddress")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(String::from);

            if let Some(private_address) = private_address_opt {
                if let (Some(name), Some(i_address), Some(parent_id), Some(system_id)) = (
                    identity_details.get("name").and_then(|v| v.as_str()),
                    identity_details.get("identityaddress").and_then(|v| v.as_str()),
                    identity_details.get("parent").and_then(|v| v.as_str()),
                    identity_details.get("systemid").and_then(|v| v.as_str()),
                ) {
                    log::debug!("Processing identity with private address: {} ({})", name, i_address);

                    let mut formatted_name = format!("{}@", name); // Default format

                    // Check if it's a sub-ID
                    if parent_id != system_id {
                        log::debug!("Identity '{}' is a sub-ID. Fetching parent '{}'...", name, parent_id);
                        // Make the second RPC call to get the parent identity details
                        match make_rpc_call::<Value>(&rpc_user, &rpc_pass, "getidentity", vec![json!(parent_id)]).await {
                            Ok(parent_identity_result) => {
                                // Extract parent name from its nested identity object
                                if let Some(parent_name) = parent_identity_result
                                    .get("identity")
                                    .and_then(|id_details| id_details.get("name"))
                                    .and_then(|n| n.as_str()) 
                                {
                                    log::debug!("Parent name found: {}", parent_name);
                                    formatted_name = format!("{}.{}@", name, parent_name);
                                } else {
                                    log::error!("Failed to extract parent name for ID '{}' from parent '{}'. Using default format.", name, parent_id);
                                    // Keep the default format as fallback
                                }
                            },
                            Err(e) => {
                                log::error!("RPC call failed for getidentity({}): {:?}. Using default format for '{}'.", parent_id, e, name);
                                // Keep the default format as fallback
                            }
                        }
                    }

                    formatted_identities.push(FormattedIdentity {
                        formatted_name, // Use the potentially updated name
                        i_address: i_address.to_string(),
                        private_address: Some(private_address),
                    });
                } else {
                    log::warn!("Identity has private address but missing required fields (name, i-address, parent, systemid) in identity details.");
                }
            } else {
                log::debug!("Skipping identity '{}' because no private address found.", identity_details.get("name").and_then(|v| v.as_str()).unwrap_or("unknown"));
            }
        } else {
            log::warn!("Skipping raw identity entry because 'identity' sub-object is missing.");
        }
    }

    log::info!("Found {} identities with private addresses for login.", formatted_identities.len());

    if formatted_identities.is_empty() {
        log::error!("No VerusIDs with private addresses found in the wallet.");
        return Err(VerusRpcError::Rpc {
            code: -1,
            message: "No VerusIDs with private addresses found in your wallet.".to_string(),
        });
    }

    Ok(formatted_identities)
}

// NEW function to get balance for a z-address
pub async fn get_private_balance(rpc_user: String, rpc_pass: String, address: String) -> Result<f64, VerusRpcError> {
    log::info!("Fetching private balance for address: {}", address);
    make_rpc_call(&rpc_user, &rpc_pass, "z_getbalance", vec![json!(address)]).await
}

// NEW function for New Chat: Check identity eligibility
pub async fn check_identity_eligibility(
    rpc_user: String,
    rpc_pass: String,
    target_identity_name: String,
) -> Result<FormattedIdentity, VerusRpcError> {
    log::info!("Checking eligibility for identity: {}", target_identity_name);

    // Basic format check
    if !target_identity_name.ends_with('@') || target_identity_name.len() <= 1 {
        log::warn!("Invalid identity format: {}", target_identity_name);
        return Err(VerusRpcError::InvalidFormat);
    }

    match make_rpc_call::<Value>(&rpc_user, &rpc_pass, "getidentity", vec![json!(target_identity_name)]).await {
        Ok(identity_result) => {
            log::debug!("getidentity result for {}: {:?}", target_identity_name, identity_result);
            if let Some(identity_details) = identity_result.get("identity") {
                let private_address_opt = identity_details.get("privateaddress")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(String::from);

                if private_address_opt.is_some() {
                    if let (Some(name), Some(i_address), Some(parent_id), Some(system_id)) = (
                        identity_details.get("name").and_then(|v| v.as_str()),
                        identity_details.get("identityaddress").and_then(|v| v.as_str()),
                        identity_details.get("parent").and_then(|v| v.as_str()),
                        identity_details.get("systemid").and_then(|v| v.as_str()),
                    ) {
                        // Start with default format
                        let mut formatted_name = format!("{}@", name);
                        
                        // Check if it's a sub-ID (parent is not the system ID)
                        if parent_id != system_id {
                            log::debug!("Identity '{}' is a sub-ID. Fetching parent '{}'...", name, parent_id);
                            // Get parent identity to format the name properly (name.parentname@)
                            match make_rpc_call::<Value>(&rpc_user, &rpc_pass, "getidentity", vec![json!(parent_id)]).await {
                                Ok(parent_identity_result) => {
                                    // Extract parent name from the parent identity details
                                    if let Some(parent_name) = parent_identity_result
                                        .get("identity")
                                        .and_then(|id_details| id_details.get("name"))
                                        .and_then(|n| n.as_str()) 
                                    {
                                        log::debug!("Parent name found: {}", parent_name);
                                        formatted_name = format!("{}.{}@", name, parent_name);
                                    } else {
                                        log::error!("Failed to extract parent name for sub-ID. Using default format.");
                                        // Keep default format as fallback
                                    }
                                },
                                Err(e) => {
                                    log::error!("Error fetching parent identity: {:?}. Using default format.", e);
                                    // Keep default format as fallback
                                }
                            }
                        }
                        
                        log::info!("Identity {} is eligible. Formatted as: {}", target_identity_name, formatted_name);
                        Ok(FormattedIdentity {
                            formatted_name,
                            i_address: i_address.to_string(),
                            private_address: private_address_opt,
                        })
                    } else {
                        log::warn!("Identity {} found but missing required fields.", target_identity_name);
                        Err(VerusRpcError::NotFoundOrIneligible)
                    }
                } else {
                    log::warn!("Identity {} found but has no private address.", target_identity_name);
                    Err(VerusRpcError::NotFoundOrIneligible)
                }
            } else {
                 log::warn!("'identity' object not found in getidentity result for {}.", target_identity_name);
                 Err(VerusRpcError::NotFoundOrIneligible)
            }
        }
        Err(e) => {
            // Handle specific error types that indicate "Not Found" for getidentity
            match e {
                VerusRpcError::Rpc { code, ref message } if code == -5 || code == -8 => {
                    // Code -5: Invalid address or key (Identity not found)
                    // Code -8: Invalid parameter (Could also indicate identity not found)
                    log::warn!("getidentity RPC error indicates not found for {}: code={}, message={}", target_identity_name, code, message);
                    Err(VerusRpcError::NotFoundOrIneligible)
                },
                VerusRpcError::ParseError(ref msg) if msg.contains("500 Internal Server Error") => {
                     // Treat 500 error specifically for getidentity as likely not found
                    log::warn!("getidentity received 500 error, treating as not found for {}: {}", target_identity_name, msg);
                    Err(VerusRpcError::NotFoundOrIneligible)
                }
                _ => {
                    // Propagate other errors (network, timeout, different RPC errors, etc.)
                    log::error!("RPC call failed for getidentity({}): {:?}", target_identity_name, e);
                    Err(e)
                }
            }
        }
    }
}

// NEW function for New Chat: Get chat history from received memos
pub async fn get_chat_history(
    rpc_user: String,
    rpc_pass: String,
    target_identity_name: String, // The user we want history *from*
    own_private_address: String, // The logged-in user's z-addr
) -> Result<Vec<ChatMessage>, VerusRpcError> {
    log::info!("Fetching chat history from {} for owner {}", target_identity_name, own_private_address);

    let params = vec![json!(own_private_address)];
    let received_txs: Vec<ReceivedByAddressEntry> = make_rpc_call(
        &rpc_user,
        &rpc_pass,
        "z_listreceivedbyaddress",
        params,
    )
    .await?;

    log::debug!("Received {} transactions for address {}", received_txs.len(), own_private_address);

    let mut chat_messages = Vec::new();
    let history_marker = format!("//f//{}", target_identity_name);

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            if let Some(message_text) = memostr.strip_suffix(&history_marker) {
                log::debug!("Found history message in tx {}: '{}'", tx.txid, message_text);
                chat_messages.push(ChatMessage {
                    id: tx.txid,
                    sender: target_identity_name.clone(), // The sender is the target ID
                    text: message_text.trim().to_string(),
                    // TODO: Determine best way to get timestamp. blocktime might be available.
                    // For now, using 0 as placeholder.
                    timestamp: 0, // Placeholder - investigate using blocktime or other tx details
                    amount: tx.amount,
                    confirmations: tx.confirmations,
                    direction: "received".to_string(),
                });
            }
        }
    }

    log::info!("Found {} historical messages from {}", chat_messages.len(), target_identity_name);
    // Sort by confirmations? Or timestamp if available?
    chat_messages.sort_by_key(|m| m.confirmations); // Example: sort by confirmations ascending

    Ok(chat_messages)
}

// NEW function for polling new received messages (for ANY sender)
pub async fn get_new_received_messages(
    rpc_user: String,
    rpc_pass: String,
    own_private_address: String, // The logged-in user's z-addr
) -> Result<Vec<ChatMessage>, VerusRpcError> {
    log::info!("Polling for new received messages for owner {}", own_private_address);

    // Call with 0 confirmations to include unconfirmed messages
    let params = vec![json!(own_private_address), json!(0)]; 
    let received_txs: Vec<ReceivedByAddressEntry> = match make_rpc_call(
        &rpc_user,
        &rpc_pass,
        "z_listreceivedbyaddress",
        params,
    ).await {
        Ok(txs) => txs,
        Err(VerusRpcError::Rpc { code, message }) if code == -8 => {
            // Handle potential error if address has never received anything
            log::warn!("z_listreceivedbyaddress RPC error (potentially no transactions yet) for {}: code={}, message={}", own_private_address, code, message);
            Vec::new() // Return empty list if address is unused/error indicates no transactions
        },
        Err(e) => return Err(e), // Propagate other errors
    };

    log::debug!("Received {} total transactions (including unconfirmed) for address {}", received_txs.len(), own_private_address);

    let mut chat_messages = Vec::new();
    let marker = "//f//"; // General marker to find sender

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            if let Some(marker_pos) = memostr.find(marker) {
                let message_text = memostr[..marker_pos].trim();
                let sender_id = memostr[marker_pos + marker.len()..].trim();

                // FIX: Updated validation to allow empty message text if amount > 0
                let is_valid_sender = sender_id.ends_with('@') && sender_id.len() > 1;
                let has_message_content = !message_text.is_empty();
                let has_gift_amount = tx.amount > 0.0;
                
                if is_valid_sender && (has_message_content || has_gift_amount) {
                    log::debug!(
                        "Found valid message/gift in tx {}: '{}' from sender '{}', amount: {}",
                        tx.txid,
                        message_text, // Will be empty string if no text
                        sender_id,
                        tx.amount
                    );
                    chat_messages.push(ChatMessage {
                        id: tx.txid,
                        sender: sender_id.to_string(), // Sender identified from memo
                        text: message_text.to_string(), // Correctly handles empty string
                        timestamp: 0, // Placeholder - confirmations are primary
                        amount: tx.amount,
                        confirmations: tx.confirmations,
                        direction: "received".to_string(),
                    });
                } else {
                    log::trace!("Skipping memo in tx {} due to invalid format or no content/gift: {}", tx.txid, memostr);
                }
            } else {
                log::trace!("Skipping memo in tx {} (no valid marker): {}", tx.txid, memostr);
            }
        } // Ignore transactions without memos
    }

    log::info!("Parsed {} potential messages from polling.", chat_messages.len());
    // No sorting needed here, frontend will handle merging and sorting

    Ok(chat_messages)
}

// NEW function for sending a message/gift
pub async fn send_private_message(
    rpc_user: String,
    rpc_pass: String,
    sender_z_address: String,      // Logged-in user's private address
    recipient_z_address: String, // Target user's private address
    memo_text: String,             // The actual message content (optional)
    sender_identity: String,       // Logged-in user's VerusID (e.g., user@)
    amount: f64                    // Amount to send (0 if just a message)
) -> Result<String, VerusRpcError> // Returns the txid on success
{
    log::info!("send_private_message received memo_text: >>>{}<<<", memo_text); 
    
    log::info!(
        "Attempting to send message/gift: from_addr={}, to_addr={}, amount={}, sender_id={}",
        sender_z_address,
        recipient_z_address,
        amount,
        sender_identity
    );
    log::debug!("Original memo text: \"{}\"", memo_text);

    // 1. Construct the full memo string
    let full_memo = format!("{}//f//{}", memo_text, sender_identity);
    log::debug!("Constructed memo string: \"{}\"", full_memo);

    // 2. Convert the memo string to its hexadecimal representation
    // Ensure the memo is not too long - z_sendmany memo limit is typically 512 bytes.
    // Hex encoding doubles the length, so the original memo should be < 256 bytes.
    // The frontend already limits input to 412 characters, which is safe.
    let memo_hex = hex::encode(full_memo.as_bytes());
    log::debug!("Hex encoded memo: {}", memo_hex);

    // 3. Construct the parameters for the z_sendmany RPC call
    let amounts_param = json!([
        {
            "address": recipient_z_address,
            "amount": amount,
            "memo": memo_hex
        }
    ]);

    let params = vec![
        json!(sender_z_address),
        amounts_param,
        json!(1), // minconf (optional, default 1)
        // fee (optional, default 0.0001) - Daemon handles this
    ];

    // 4. Make the RPC call
    log::info!("Executing z_sendmany...");
    match make_rpc_call::<String>(&rpc_user, &rpc_pass, "z_sendmany", params).await {
        Ok(txid) => {
            log::info!("z_sendmany successful, txid: {}", txid);
            Ok(txid)
        }
        Err(e) => {
            log::error!("z_sendmany failed: {:?}", e);
            Err(e)
        }
    }
} 