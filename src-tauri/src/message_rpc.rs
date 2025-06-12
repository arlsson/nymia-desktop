// File: src-tauri/src/message_rpc.rs
// Description: Handles Verus private messaging and chat history RPC calls.
// Changes:
// - Moved ChatMessage, ReceivedByAddressEntry, get_chat_history, get_new_received_messages, and send_private_message from verus_rpc.rs.
// - Added necessary use statements for rpc_client, serde, serde_json, and hex.
// - BREAKING: Implemented timestamp-based messaging system with new memo format: {message_text}//f//{sender_identity}//t//{unix_timestamp}
// - Updated send_private_message to include UTC timestamps when sending messages.
// - Updated message parsing logic with strict validation - messages without valid timestamps are rejected.
// - Implemented chronological timestamp-based sorting (oldest first) for consistent message ordering.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use hex;
use super::rpc_client::{make_rpc_call, VerusRpcError};

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

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            // Parse new timestamp format: {message_text}//f//{sender_identity}//t//{timestamp}
            if let Some(sender_marker_pos) = memostr.find("//f//") {
                let message_text = memostr[..sender_marker_pos].trim();
                let after_sender_marker = &memostr[sender_marker_pos + 5..]; // 5 = "//f//".len()
                
                if let Some(time_marker_pos) = after_sender_marker.find("//t//") {
                    let sender_id = after_sender_marker[..time_marker_pos].trim();
                    let timestamp_str = after_sender_marker[time_marker_pos + 5..].trim(); // 5 = "//t//".len()
                    
                    // Only process if this message is from the target identity
                    if sender_id == target_identity_name {
                        // Parse timestamp - reject message if invalid (strict parsing)
                        if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                            log::debug!("Found history message in tx {}: '{}' from {} at timestamp {}", 
                                tx.txid, message_text, sender_id, timestamp);
                            chat_messages.push(ChatMessage {
                                id: tx.txid,
                                sender: target_identity_name.clone(),
                                text: message_text.to_string(),
                                timestamp: timestamp, // Use parsed timestamp
                                amount: tx.amount,
                                confirmations: tx.confirmations,
                                direction: "received".to_string(),
                            });
                        } else {
                            log::warn!("Skipping history message in tx {} due to invalid timestamp format: '{}'", tx.txid, timestamp_str);
                        }
                    }
                } else {
                    log::trace!("Skipping memo in tx {} (no timestamp marker): {}", tx.txid, memostr);
                }
            } else {
                log::trace!("Skipping memo in tx {} (no sender marker): {}", tx.txid, memostr);
            }
        }
    }

    log::info!("Found {} historical messages from {}", chat_messages.len(), target_identity_name);
    // Sort by timestamp ascending (oldest first)
    chat_messages.sort_by_key(|m| m.timestamp);

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

    for tx in received_txs {
        if let Some(memostr) = tx.memostr {
            // Parse new timestamp format: {message_text}//f//{sender_identity}//t//{timestamp}
            if let Some(sender_marker_pos) = memostr.find("//f//") {
                let message_text = memostr[..sender_marker_pos].trim();
                let after_sender_marker = &memostr[sender_marker_pos + 5..]; // 5 = "//f//".len()
                
                if let Some(time_marker_pos) = after_sender_marker.find("//t//") {
                    let sender_id = after_sender_marker[..time_marker_pos].trim();
                    let timestamp_str = after_sender_marker[time_marker_pos + 5..].trim(); // 5 = "//t//".len()
                    
                    // Parse timestamp - reject message if invalid (strict parsing)
                    if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                        // Validate sender format
                        let is_valid_sender = sender_id.ends_with('@') && sender_id.len() > 1;
                        let has_message_content = !message_text.is_empty();
                        let has_gift_amount = tx.amount > 0.0;
                        
                        if is_valid_sender && (has_message_content || has_gift_amount) {
                            log::debug!(
                                "Found valid message/gift in tx {}: '{}' from sender '{}', amount: {}, timestamp: {}",
                                tx.txid,
                                message_text,
                                sender_id,
                                tx.amount,
                                timestamp
                            );
                            chat_messages.push(ChatMessage {
                                id: tx.txid,
                                sender: sender_id.to_string(),
                                text: message_text.to_string(),
                                timestamp: timestamp, // Use parsed timestamp
                                amount: tx.amount,
                                confirmations: tx.confirmations,
                                direction: "received".to_string(),
                            });
                        } else {
                            log::trace!("Skipping memo in tx {} due to invalid format or no content/gift: {}", tx.txid, memostr);
                        }
                    } else {
                        log::warn!("Skipping message in tx {} due to invalid timestamp format: '{}'", tx.txid, timestamp_str);
                    }
                } else {
                    log::trace!("Skipping memo in tx {} (no timestamp marker): {}", tx.txid, memostr);
                }
            } else {
                log::trace!("Skipping memo in tx {} (no sender marker): {}", tx.txid, memostr);
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

    // 1. Generate UTC timestamp when sending to blockchain
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 2. Construct the full memo string with timestamp
    let full_memo = format!("{}//f//{}//t//{}", memo_text, sender_identity, timestamp);
    log::debug!("Constructed memo string with timestamp: \"{}\" (timestamp: {})", full_memo, timestamp);

    // 3. Convert the memo string to its hexadecimal representation
    // Ensure the memo is not too long - z_sendmany memo limit is typically 512 bytes.
    // Hex encoding doubles the length, so the original memo should be < 256 bytes.
    // The frontend already limits input to 412 characters, which is safe.
    let memo_hex = hex::encode(full_memo.as_bytes());
    log::debug!("Hex encoded memo: {}", memo_hex);

    // 4. Construct the parameters for the z_sendmany RPC call
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

    // 5. Make the RPC call
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