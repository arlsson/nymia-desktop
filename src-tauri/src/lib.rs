// File: src-tauri/src/lib.rs
// Description: Main library file for the Tauri application backend.
// Changes:
// - Added credentials module and commands.
// - Added verus_rpc module.
// - Added thiserror dependency for error handling.
// - Added connect_verus_daemon command to interact with Verus RPC.
// - Registered tauri-plugin-secure-store.
// - Updated invoke_handler to include all commands.
// - Added send_private_message command.
// - Added settings module and persistence commands.
// - Added `pub mod rpc_client;` to declare the new module for RPC client logic.
// - Added `pub mod identity_rpc;` to declare the new module for identity RPC logic.
// - Added `pub mod message_rpc;` to declare the new module for message RPC logic.
// - Added `pub mod wallet_rpc;` to declare the new module for wallet RPC logic.
// - Corrected use statements and function call paths to reflect new module structure.
// - SECURITY: Integrated cryptographic message signing and verification system
// - Zero-trust messaging: Only verified messages are displayed, unverified messages are silently filtered

mod credentials; // Added credentials module
mod settings; // Added settings module
pub mod rpc_client;
pub mod identity_rpc;
pub mod message_rpc;
pub mod wallet_rpc;

use crate::rpc_client::VerusRpcError; // Corrected
use crate::credentials::CredentialError; // Import credential error
use crate::settings::SettingsError; // Import settings error
use crate::identity_rpc::FormattedIdentity; // Corrected
use crate::message_rpc::ChatMessage; // Corrected

// Custom error type serializable for Tauri
#[derive(Debug, serde::Serialize, thiserror::Error)]
enum CommandError {
    #[error("Verus RPC Error: {0}")]
    Rpc(String),
    #[error("Credential Error: {0}")] // Added Credential error variant
    Credentials(String),
    #[error("Settings Error: {0}")] // Added Settings error variant
    Settings(String),
    #[error("Verus RPC Error: {0}")] // Use the same variant, but handle specific RPC errors
    RpcSpecific(crate::rpc_client::VerusRpcError), // Corrected
}

// Convert VerusRpcError to CommandError
impl From<crate::rpc_client::VerusRpcError> for CommandError { // Corrected
    fn from(error: crate::rpc_client::VerusRpcError) -> Self { // Corrected
        log::error!("RPC call failed: {:?}", error);
        // Return the specific error type for frontend handling
        CommandError::RpcSpecific(error)
    }
}

// Convert CredentialError to CommandError
impl From<CredentialError> for CommandError {
    fn from(error: CredentialError) -> Self {
        log::error!("Credential operation failed: {:?}", error);
        // Avoid leaking potentially sensitive details from StoreError
        match error {
            CredentialError::Store(_) => CommandError::Credentials("Failed to access store.".to_string()),
            _ => CommandError::Credentials(error.to_string()),
        }
    }
}

// Convert SettingsError to CommandError
impl From<SettingsError> for CommandError {
    fn from(error: SettingsError) -> Self {
        log::error!("Settings operation failed: {:?}", error);
        // Avoid leaking potentially sensitive details from StoreError
        match error {
            SettingsError::Store(_) => CommandError::Settings("Failed to access settings store.".to_string()),
            _ => CommandError::Settings(error.to_string()),
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn connect_verus_daemon(rpc_user: String, rpc_pass: String) -> Result<u64, CommandError> {
    // Ensure logging is initialized (can be done once at startup too)
    // TODO: Initialize logger properly in main/run function
    let _ = env_logger::try_init();

    log::info!("connect_verus_daemon command received");
    crate::wallet_rpc::connect_and_get_block_height(rpc_user, rpc_pass) // Corrected path
        .await
        .map_err(CommandError::from)
}

// New command to get formatted identities
#[tauri::command]
async fn get_login_identities(
    app: tauri::AppHandle, // Need AppHandle to get stored credentials
) -> Result<Vec<FormattedIdentity>, CommandError> {
    log::info!("get_login_identities command received");
    // Load credentials first
    let creds = crate::credentials::load_credentials(app).await?;
    // Then call the RPC function
    crate::identity_rpc::get_login_identities(creds.rpc_user, creds.rpc_pass) // Corrected path
        .await
        .map_err(CommandError::from)
}

// NEW command to get private balance
#[tauri::command]
async fn get_private_balance(
    app: tauri::AppHandle, // Need AppHandle for credentials
    address: String,
) -> Result<f64, CommandError> {
    log::info!("get_private_balance command received for address: {}", address);
    let creds = crate::credentials::load_credentials(app).await?;
    crate::wallet_rpc::get_private_balance(creds.rpc_user, creds.rpc_pass, address) // Correct path
        .await
        .map_err(CommandError::from)
}

// NEW Command: Check Identity Eligibility
#[tauri::command]
async fn check_identity_eligibility(
    app: tauri::AppHandle,
    target_identity_name: String,
) -> Result<FormattedIdentity, CommandError> {
    log::info!("check_identity_eligibility command received for: {}", target_identity_name);
    let creds = crate::credentials::load_credentials(app).await?;
    crate::identity_rpc::check_identity_eligibility(creds.rpc_user, creds.rpc_pass, target_identity_name) // Corrected path
        .await
        .map_err(CommandError::from) // Uses the updated From implementation
}

// NEW Command: Get Chat History (with automatic signature verification)
#[tauri::command]
async fn get_chat_history(
    app: tauri::AppHandle,
    target_identity_name: String,
    own_private_address: String,
) -> Result<Vec<ChatMessage>, CommandError> {
    log::info!("get_chat_history command received from: {} for owner: {}", target_identity_name, own_private_address);
    let creds = crate::credentials::load_credentials(app).await?;
    crate::message_rpc::get_chat_history(creds.rpc_user, creds.rpc_pass, target_identity_name, own_private_address) // Corrected path
        .await
        .map_err(CommandError::from)
}

// NEW Command: Get New Received Messages (Polling) (with automatic signature verification)
#[tauri::command]
async fn get_new_received_messages(
    app: tauri::AppHandle,
    own_private_address: String,
) -> Result<Vec<ChatMessage>, CommandError> {
    log::info!("get_new_received_messages command received for owner: {}", own_private_address);
    let creds = crate::credentials::load_credentials(app).await?;
    crate::message_rpc::get_new_received_messages(creds.rpc_user, creds.rpc_pass, own_private_address) // Corrected path
        .await
        .map_err(CommandError::from)
}

// NEW Command: Send Private Message/Gift (with mandatory signature)
#[tauri::command]
async fn send_private_message(
    app: tauri::AppHandle,
    sender_z_address: String,
    recipient_z_address: String,
    memo_text: String,
    sender_identity: String,
    amount: f64,
) -> Result<String, CommandError> { // Returns txid
    log::info!(
        "send_private_message command received: to={}, amount={}, sender_id={}",
        recipient_z_address,
        amount,
        sender_identity
    );
    let creds = crate::credentials::load_credentials(app).await?;
    crate::message_rpc::send_private_message( // Corrected path
        creds.rpc_user,
        creds.rpc_pass,
        sender_z_address,
        recipient_z_address,
        memo_text,
        sender_identity,
        amount,
    )
    .await
    .map_err(CommandError::from)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // TODO: Initialize logger here instead of in command
    env_logger::init(); // Basic logger initialization

    let store_plugin = tauri_plugin_store::Builder::default().build(); // Build the store plugin instance

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(store_plugin) // Register the store plugin instance
        .invoke_handler(tauri::generate_handler![
            connect_verus_daemon,
            crate::credentials::save_credentials, // Add credential commands
            crate::credentials::load_credentials,
            crate::credentials::clear_credentials,
            get_login_identities, // Correct name used here
            get_private_balance, // Add the new balance command
            check_identity_eligibility,
            get_chat_history,
            get_new_received_messages,
            send_private_message, // Added send message command
            // New Settings Commands
            crate::settings::save_persistence_setting,
            crate::settings::load_persistence_setting,
            crate::settings::save_conversations,
            crate::settings::load_conversations,
            crate::settings::save_messages_for_conversation,
            crate::settings::load_messages_for_conversation,
            crate::settings::delete_chat_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
