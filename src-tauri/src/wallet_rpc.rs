// File: src-tauri/src/wallet_rpc.rs
// Description: Handles general wallet-related RPC calls.
// Changes:
// - Moved connect_and_get_block_height and get_private_balance functions from verus_rpc.rs.
// - Added necessary use statements for rpc_client and serde_json.
// - Added UtxoInfo struct and get_utxo_info function for Fast Messages feature
// - Implemented z_listunspent RPC call with UTXO filtering and processing

use serde_json::{json, Value};
use super::rpc_client::{make_rpc_call, VerusRpcError};
use serde::{Deserialize, Serialize};

// UTXO information structure for Fast Messages feature
#[derive(Debug, Serialize, Deserialize)]
pub struct UtxoInfo {
    pub total_utxos: u32,           // Total count including dust
    pub usable_utxos: u32,          // Count with amount >= 0.0001 (Fast Messages count)
    pub total_spendable_value: f64, // Sum of usable UTXOs only
    pub largest_utxo: f64,          // Largest single UTXO amount
    pub smallest_utxo: f64,         // Smallest usable UTXO amount (>= 0.0001)
}

// Function to connect and get block height
// Exposed as a Tauri command
pub async fn connect_and_get_block_height(
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
) -> Result<u64, VerusRpcError> {
    log::info!("Attempting to connect to Verus daemon...");
    make_rpc_call(&rpc_user, &rpc_pass, rpc_port, "getblockcount", vec![]).await
}

// Function to get balance for a z-address
pub async fn get_private_balance(rpc_user: String, rpc_pass: String, rpc_port: u16, address: String) -> Result<f64, VerusRpcError> {
    log::info!("Fetching private balance for address: {}", address);
    make_rpc_call(&rpc_user, &rpc_pass, rpc_port, "z_getbalance", vec![json!(address)]).await
}

// Function to get pending balance for a z-address (0 confirmations)
pub async fn get_pending_balance(rpc_user: String, rpc_pass: String, rpc_port: u16, address: String) -> Result<f64, VerusRpcError> {
    log::info!("Fetching pending balance for address: {}", address);
    make_rpc_call(&rpc_user, &rpc_pass, rpc_port, "z_getbalance", vec![json!(address), json!(0)]).await
}

// NEW function to get UTXO information for Fast Messages
pub async fn get_utxo_info(
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
    address: String,
) -> Result<UtxoInfo, VerusRpcError> {
    log::info!("Fetching UTXO info for address: {}", address);
    
    // Call z_listunspent with specific parameters:
    // minconf=1: Only confirmed UTXOs
    // maxconf=9999999: All confirmed UTXOs  
    // watchonly=false: Only spendable UTXOs
    // addresses=[address]: Only for this specific address
    let utxo_list: Value = make_rpc_call(
        &rpc_user,
        &rpc_pass,
        rpc_port,
        "z_listunspent",
        vec![json!(1), json!(9999999), json!(false), json!([address])],
    ).await?;

    log::debug!("Raw UTXO response: {:?}", utxo_list);

    // Process the UTXO list
    let utxos = utxo_list.as_array().ok_or(VerusRpcError::ParseError(
        "Expected array of UTXOs".to_string(),
    ))?;

    let mut total_utxos = 0u32;
    let mut usable_utxos = 0u32;
    let mut total_spendable_value = 0.0f64;
    let mut largest_utxo = 0.0f64;
    let mut smallest_utxo = f64::MAX;

    for utxo in utxos {
        let amount = utxo["amount"].as_f64().unwrap_or(0.0);
        total_utxos += 1;

        // Track largest UTXO regardless of usability
        if amount > largest_utxo {
            largest_utxo = amount;
        }

        // Filter for usable UTXOs (amount >= 0.0001)
        if amount >= 0.0001 {
            usable_utxos += 1;
            total_spendable_value += amount;
            
            // Track smallest usable UTXO
            if amount < smallest_utxo {
                smallest_utxo = amount;
            }
        }
    }

    // If no usable UTXOs, set smallest to 0
    if usable_utxos == 0 {
        smallest_utxo = 0.0;
    }

    let utxo_info = UtxoInfo {
        total_utxos,
        usable_utxos,
        total_spendable_value,
        largest_utxo,
        smallest_utxo,
    };

    log::info!(
        "UTXO analysis complete: {} total UTXOs, {} usable UTXOs, {:.4} total spendable, largest: {:.4}, smallest: {:.4}",
        utxo_info.total_utxos,
        utxo_info.usable_utxos,
        utxo_info.total_spendable_value,
        utxo_info.largest_utxo,
        utxo_info.smallest_utxo
    );

    Ok(utxo_info)
} 