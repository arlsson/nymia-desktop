// File: src-tauri/src/wallet_rpc.rs
// Description: Handles general wallet-related RPC calls.
// Changes:
// - Moved connect_and_get_block_height and get_private_balance functions from verus_rpc.rs.
// - Added necessary use statements for rpc_client and serde_json.

use serde_json::{json, Value};
use super::rpc_client::{make_rpc_call, VerusRpcError};

// Function to connect and get block height
// Exposed as a Tauri command
pub async fn connect_and_get_block_height(
    rpc_user: String,
    rpc_pass: String,
) -> Result<u64, VerusRpcError> {
    log::info!("Attempting to connect to Verus daemon...");
    make_rpc_call(&rpc_user, &rpc_pass, "getblockcount", vec![]).await
}

// NEW function to get balance for a z-address
pub async fn get_private_balance(rpc_user: String, rpc_pass: String, address: String) -> Result<f64, VerusRpcError> {
    log::info!("Fetching private balance for address: {}", address);
    make_rpc_call(&rpc_user, &rpc_pass, "z_getbalance", vec![json!(address)]).await
} 