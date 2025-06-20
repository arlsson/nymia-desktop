// File: src-tauri/src/message_rpc.rs
// Description: Handles Verus private messaging and chat history RPC calls.
// Changes:
// - Moved ChatMessage, ReceivedByAddressEntry, get_chat_history, get_new_received_messages, and send_private_message from verus_rpc.rs.
// - Added necessary use statements for rpc_client, serde, serde_json, and hex.
// - BREAKING: Implemented timestamp-based messaging system with new memo format: {message_text}//f//{sender_identity}//t//{unix_timestamp}
// - Updated send_private_message to include UTC timestamps when sending messages.
// - Updated message parsing logic with strict validation - messages without valid timestamps are rejected.
// - Implemented chronological timestamp-based sorting (oldest first) for consistent message ordering.
// - SECURITY: Added mandatory cryptographic message signing and verification
// - BREAKING: Extended message format to {message_text}//f//{sender_identity}//t//{unix_timestamp}//{signature}
// - Zero-trust approach: Only verified messages are displayed, unverified messages are silently filtered
// - Message sending fails if signing fails (no fallback to unsigned messages)

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use hex;
use super::rpc_client::{make_rpc_call, sign_message, verify_message, VerusRpcError};

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
pub struct ReceivedByAddressEntry {
    txid: String,
    amount: f64,
    confirmations: i64,
    memostr: Option<String>, // Memo might be absent
    // memo: String, // We only need memostr
    // outindex: u32,
    // change: bool,
    // blocktime: Option<u64>, // Add blocktime if available and needed for timestamp
}

// Helper function to parse message with signature verification
async fn parse_and_verify_message(
    rpc_user: &str,
    rpc_pass: &str,
    rpc_port: u16,
    memo: &str,
    txid: &str,
) -> Option<(String, String, u64, String)> { // Returns (message_text, sender_id, timestamp, signature) if valid
    // Parse new signature format: {message_text}//f//{sender_identity}//t//{timestamp}//{signature}
    if let Some(sender_marker_pos) = memo.find("//f//") {
        let message_text = memo[..sender_marker_pos].trim();
        let after_sender_marker = &memo[sender_marker_pos + 5..]; // 5 = "//f//".len()
        
        if let Some(time_marker_pos) = after_sender_marker.find("//t//") {
            let sender_id = after_sender_marker[..time_marker_pos].trim();
            let after_time_marker = &after_sender_marker[time_marker_pos + 5..]; // 5 = "//t//".len()
            
            if let Some(sig_marker_pos) = after_time_marker.find("//") {
                let timestamp_str = after_time_marker[..sig_marker_pos].trim();
                let signature = after_time_marker[sig_marker_pos + 2..].trim(); // 2 = "//".len()
                
                // Parse timestamp - reject message if invalid (strict parsing)
                if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                    // Reconstruct the original message for verification (without signature)
                    let original_message = format!("{}//f//{}//t//{}", message_text, sender_id, timestamp);
                    
                    // Verify the signature
                    match verify_message(rpc_user, rpc_pass, rpc_port, sender_id, signature, &original_message).await {
                        Ok(true) => {
                            log::debug!("Message verification successful for tx {}: '{}' from {} at timestamp {}", 
                                txid, message_text, sender_id, timestamp);
                            return Some((message_text.to_string(), sender_id.to_string(), timestamp, signature.to_string()));
                        }
                        Ok(false) => {
                            log::warn!("Message verification failed for tx {} - signature invalid. Message silently filtered.", txid);
                            return None;
                        }
                        Err(e) => {
                            log::error!("Message verification error for tx {}: {:?}. Message silently filtered.", txid, e);
                            return None;
                        }
                    }
                } else {
                    log::warn!("Skipping message in tx {} due to invalid timestamp format: '{}'", txid, timestamp_str);
                    return None;
                }
            } else {
                // Legacy format without signature - silently filter out
                log::debug!("Skipping legacy unsigned message in tx {} (no signature marker)", txid);
                return None;
            }
        } else {
            log::trace!("Skipping memo in tx {} (no timestamp marker): {}", txid, memo);
            return None;
        }
    } else {
        log::trace!("Skipping memo in tx {} (no sender marker): {}", txid, memo);
        return None;
    }
}

// NEW function for New Chat: Get chat history from received memos
pub async fn get_chat_history(
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
    target_identity_name: String, // The user we want history *from*
    own_private_address: String, // The logged-in user's z-addr
) -> Result<Vec<ChatMessage>, VerusRpcError> {
    log::info!("Fetching chat history from {} for owner {}", target_identity_name, own_private_address);

    let params = vec![json!(own_private_address)];
    let received_txs: Vec<ReceivedByAddressEntry> = make_rpc_call(
        &rpc_user,
        &rpc_pass,
        rpc_port,
        "z_listreceivedbyaddress",
        params,
    )
    .await?;

    log::debug!("Received {} transactions for address {}", received_txs.len(), own_private_address);

    let mut chat_messages = Vec::new();

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            // Parse and verify message - only verified messages are processed
            if let Some((message_text, sender_id, timestamp, _signature)) = 
                parse_and_verify_message(&rpc_user, &rpc_pass, rpc_port, &memostr, &tx.txid).await {
                
                // Only process if this message is from the target identity
                if sender_id == target_identity_name {
                    chat_messages.push(ChatMessage {
                        id: tx.txid,
                        sender: target_identity_name.clone(),
                        text: message_text,
                        timestamp: timestamp,
                        amount: tx.amount,
                        confirmations: tx.confirmations,
                        direction: "received".to_string(),
                    });
                }
            }
            // Note: Unverified messages are silently filtered out - no logging needed per zero-trust requirement
        }
    }

    log::info!("Found {} verified messages from {}", chat_messages.len(), target_identity_name);
    // Sort by timestamp ascending (oldest first)
    chat_messages.sort_by_key(|m| m.timestamp);

    Ok(chat_messages)
}

// NEW function for polling new received messages (for ANY sender)
pub async fn get_new_received_messages(
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
    own_private_address: String, // The logged-in user's z-addr
) -> Result<Vec<ChatMessage>, VerusRpcError> {
    log::info!("Polling for new received messages for owner {}", own_private_address);

    // Call with 0 confirmations to include unconfirmed messages
    let params = vec![json!(own_private_address), json!(0)]; 
    let received_txs: Vec<ReceivedByAddressEntry> = match make_rpc_call(
        &rpc_user,
        &rpc_pass,
        rpc_port,
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

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            // Parse and verify message - only verified messages are processed
            if let Some((message_text, sender_id, timestamp, _signature)) = 
                parse_and_verify_message(&rpc_user, &rpc_pass, rpc_port, &memostr, &tx.txid).await {
                
                // Validate sender format
                let is_valid_sender = sender_id.ends_with('@') && sender_id.len() > 1;
                let has_message_content = !message_text.is_empty();
                let has_gift_amount = tx.amount > 0.0;
                
                if is_valid_sender && (has_message_content || has_gift_amount) {
                    log::debug!(
                        "Found valid verified message/gift in tx {}: '{}' from sender '{}', amount: {}, timestamp: {}",
                        tx.txid,
                        message_text,
                        sender_id,
                        tx.amount,
                        timestamp
                    );
                    chat_messages.push(ChatMessage {
                        id: tx.txid,
                        sender: sender_id,
                        text: message_text,
                        timestamp: timestamp,
                        amount: tx.amount,
                        confirmations: tx.confirmations,
                        direction: "received".to_string(),
                    });
                } else {
                    log::trace!("Skipping verified memo in tx {} due to invalid format or no content/gift: {}", tx.txid, memostr);
                }
            }
            // Note: Unverified messages are silently filtered out - no logging needed per zero-trust requirement
        } // Ignore transactions without memos
    }

    log::info!("Parsed {} verified messages from polling.", chat_messages.len());
    // No sorting needed here, frontend will handle merging and sorting

    Ok(chat_messages)
}

// NEW function for sending a message/gift with mandatory signature
pub async fn send_private_message(
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
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

    // 1. Generate UTC timestamp when sending to blockchain
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 2. Construct the base message for signing (without signature)
    let base_message = format!("{}//f//{}//t//{}", memo_text, sender_identity, timestamp);
    log::debug!("Base message for signing: \"{}\" (timestamp: {})", base_message, timestamp);

    // 3. MANDATORY SIGNING: Sign the base message
    let signature_response = match sign_message(&rpc_user, &rpc_pass, rpc_port, &sender_identity, &base_message).await {
        Ok(sig) => {
            log::info!("Message signed successfully. Hash: {}", sig.hash);
            sig
        }
        Err(e) => {
            log::error!("CRITICAL: Message signing failed: {:?}. Message will NOT be sent.", e);
            return Err(VerusRpcError::SigningFailed);
        }
    };

    // 4. Construct the full memo string with signature
    let full_memo = format!("{}//f//{}//t//{}//{}", memo_text, sender_identity, timestamp, signature_response.signature);
    log::debug!("Constructed signed memo string: \"{}\"", full_memo);

    // 5. Convert the memo string to its hexadecimal representation
    // Ensure the memo is not too long - z_sendmany memo limit is typically 512 bytes.
    // Hex encoding doubles the length, so the original memo should be < 256 bytes.
    // The frontend already limits input to 412 characters, which is safe.
    let memo_hex = hex::encode(full_memo.as_bytes());
    log::debug!("Hex encoded memo: {}", memo_hex);

    // 6. Construct the parameters for the z_sendmany RPC call
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

    // 7. Make the RPC call
    log::info!("Executing z_sendmany with signed message...");
    match make_rpc_call::<String>(&rpc_user, &rpc_pass, rpc_port, "z_sendmany", params).await {
        Ok(txid) => {
            log::info!("z_sendmany successful with signed message, txid: {}", txid);
            Ok(txid)
        }
        Err(e) => {
            log::error!("z_sendmany failed even with valid signature: {:?}", e);
            Err(e)
        }
    }
} 