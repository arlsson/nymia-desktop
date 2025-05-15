// File: src-tauri/src/identity_rpc.rs
// Description: Handles VerusID identity-related RPC calls.
// Changes:
// - Moved FormattedIdentity struct, get_login_identities, and check_identity_eligibility functions from verus_rpc.rs.
// - Added necessary use statements for rpc_client, serde, and serde_json.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use super::rpc_client::{make_rpc_call, VerusRpcError};

// Struct to hold formatted identity name and addresses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormattedIdentity {
    pub formatted_name: String,
    pub i_address: String, // Include original i-address for reference
    pub private_address: Option<String>, // Added optional private address
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