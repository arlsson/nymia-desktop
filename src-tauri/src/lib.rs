// File: src-tauri/src/lib.rs
// Description: Main library file for the Tauri application backend.
// Changes:
// - Added credentials module and commands.
// - Added verus_rpc module.
// - Added thiserror dependency for error handling.
// - Added connect_verus_daemon command to interact with Verus RPC.
// - Registered tauri-plugin-secure-store.
// - Updated invoke_handler to include all commands.

mod verus_rpc;
mod credentials; // Added credentials module

use verus_rpc::VerusRpcError;
use credentials::CredentialError; // Import credential error
use verus_rpc::FormattedIdentity;

// Custom error type serializable for Tauri
#[derive(Debug, serde::Serialize, thiserror::Error)]
enum CommandError {
    #[error("Verus RPC Error: {0}")]
    Rpc(String),
    #[error("Credential Error: {0}")] // Added Credential error variant
    Credentials(String),
}

// Convert VerusRpcError to CommandError
impl From<VerusRpcError> for CommandError {
    fn from(error: VerusRpcError) -> Self {
        // Log the detailed error on the backend
        log::error!("RPC call failed: {:?}", error);
        // Return a user-friendly string representation to the frontend
        CommandError::Rpc(error.to_string())
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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn connect_verus_daemon(rpc_user: String, rpc_pass: String) -> Result<u64, CommandError> {
    // Ensure logging is initialized (can be done once at startup too)
    // TODO: Initialize logger properly in main/run function
    let _ = env_logger::try_init();

    log::info!("connect_verus_daemon command received");
    verus_rpc::connect_and_get_block_height(rpc_user, rpc_pass)
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
    let creds = credentials::load_credentials(app).await?;
    // Then call the RPC function
    verus_rpc::get_login_identities(creds.rpc_user, creds.rpc_pass)
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
    let creds = credentials::load_credentials(app).await?;
    verus_rpc::get_private_balance(creds.rpc_user, creds.rpc_pass, address)
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
            credentials::save_credentials, // Add credential commands
            credentials::load_credentials,
            credentials::clear_credentials,
            get_login_identities, // Correct name used here
            get_private_balance // Add the new balance command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
