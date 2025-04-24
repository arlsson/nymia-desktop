// File: src-tauri/src/lib.rs
// Description: Main library file for the Tauri application backend.
// Changes:
// - Added verus_rpc module.
// - Added thiserror dependency for error handling.
// - Added connect_verus_daemon command to interact with Verus RPC.
// - Updated invoke_handler to include the new command.

mod verus_rpc;

use verus_rpc::VerusRpcError;

// Custom error type serializable for Tauri
#[derive(Debug, serde::Serialize, thiserror::Error)]
enum CommandError {
    #[error("Verus RPC Error: {0}")]
    Rpc(String),
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // TODO: Initialize logger here instead of in command
    env_logger::init(); // Basic logger initialization

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect_verus_daemon // Added command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
