// File: src-tauri/src/credentials.rs
// Description: Handles storage and retrieval of RPC credentials using tauri-plugin-store.
// Note: This stores credentials in a plain JSON file, NOT encrypted.
// Changes:
// - Replaced insert() with set().
// - Corrected app.store() call (removed .into()).
// - Fixed proper method return handling (set, has, delete, etc.)
// - Added rpc_port field to Credentials struct for automatic credential discovery support.
// - Added blockchain configuration structures and automatic discovery functionality.
// - Removed ALL hardcoded default ports - ports must be discovered from config files or manually entered.
// - Modified credential migration to clear old format instead of assuming wrong port.
// - MAJOR: Added parallel blockchain detection system with enhanced error reporting
// - Added folder selection dialog for manual configuration discovery
// - Added detection result structures for comprehensive status reporting

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::{StoreExt, Error as StoreError};
use std::path::PathBuf;
use std::fs;
use tokio::task::JoinSet;
use std::time::Duration;


// Path for the store file relative to AppData directory
const STORE_PATH: &str = "store.json";

// Key used within the store file
const CREDENTIALS_KEY: &str = "verus_rpc_credentials";

// Detection timeout in seconds
const DETECTION_TIMEOUT_SECS: u64 = 8;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    pub rpc_user: String,
    pub rpc_pass: String,
    pub rpc_port: u16, // NEW: Port support for different blockchains
}

// NEW: Blockchain configuration structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainConfig {
    pub id: String,
    pub name: String,
    pub chain_string: Option<String>,
    pub config_file_name: String,
}

// NEW: Enhanced detection result for individual blockchains
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainDetectionResult {
    pub blockchain_id: String,
    pub blockchain_name: String,
    pub status: BlockchainStatus,
    pub credentials: Option<Credentials>,
    pub config_path: Option<String>,
    pub error_message: Option<String>,
    pub block_height: Option<u64>,
}

// NEW: Status enum for blockchain detection
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockchainStatus {
    Available,       // Config found, daemon responsive, ready to use
    Loading,         // Daemon is starting up (error code -28)
    Error,          // Config found but daemon error or connection failed  
    NotFound,       // No config file found in standard locations
    Timeout,        // Daemon not responding within timeout
    ParseError,     // Config file exists but couldn't be parsed
}

// NEW: Parallel detection result containing all blockchains
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParallelDetectionResult {
    pub blockchains: Vec<BlockchainDetectionResult>,
    pub total_detected: usize,
    pub detection_duration_ms: u64,
}

// NEW: Discovery result types (keep existing for compatibility)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveryResult {
    pub success: bool,
    pub credentials: Option<Credentials>,
    pub config_path: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, thiserror::Error, Serialize)]
pub enum DiscoveryError {
    #[error("Config file not found in standard locations")]
    NotFound,
    #[error("Failed to parse config file: {0}")]
    ParseError(String),
    #[error("Permission denied accessing config file")]
    PermissionDenied,
    #[error("IO error: {0}")]
    IoError(String),
}

// NEW: Get blockchain configurations in the specified order
pub fn get_blockchain_configs() -> Vec<BlockchainConfig> {
    vec![
        BlockchainConfig {
            id: "verus".to_string(),
            name: "Verus".to_string(),
            chain_string: None,
            config_file_name: "VRSC.conf".to_string(),
        },
        BlockchainConfig {
            id: "chips".to_string(),
            name: "CHIPS".to_string(),
            chain_string: Some("f315367528394674d45277e369629605a1c3ce9f".to_string()),
            config_file_name: "f315367528394674d45277e369629605a1c3ce9f.conf".to_string(),
        },
        BlockchainConfig {
            id: "vdex".to_string(),
            name: "vDEX".to_string(),
            chain_string: Some("53fe39eea8c06bba32f1a4e20db67e5524f0309d".to_string()),
            config_file_name: "53fe39eea8c06bba32f1a4e20db67e5524f0309d.conf".to_string(),
        },
        BlockchainConfig {
            id: "varrr".to_string(),
            name: "vARRR".to_string(),
            chain_string: Some("e9e10955b7d16031e3d6f55d9c908a038e3ae47d".to_string()),
            config_file_name: "e9e10955b7d16031e3d6f55d9c908a038e3ae47d.conf".to_string(),
        },
        BlockchainConfig {
            id: "verus-testnet".to_string(),
            name: "Verus Testnet".to_string(),
            chain_string: None,
            config_file_name: "vrsctest.conf".to_string(),
        },
    ]
}

// NEW: Get standard config paths for a blockchain
pub fn get_standard_config_paths(blockchain_config: &BlockchainConfig) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    if let Some(home_dir) = dirs::home_dir() {
        let config_file = &blockchain_config.config_file_name;
        
        match blockchain_config.id.as_str() {
            "verus-testnet" => {
                // Linux: ~/.komodo/vrsctest/
                // macOS: ~/Library/Application Support/Komodo/vrsctest/
                // Windows: %AppData%\Roaming\Komodo\vrsctest\
                if cfg!(target_os = "windows") {
                    if let Some(appdata) = std::env::var_os("APPDATA") {
                        let path = PathBuf::from(appdata).join("Komodo").join("vrsctest").join(config_file);
                        paths.push(path);
                    }
                } else if cfg!(target_os = "macos") {
                    let path = home_dir.join("Library").join("Application Support").join("Komodo").join("vrsctest").join(config_file);
                    paths.push(path);
                } else {
                    // Linux
                    let path = home_dir.join(".komodo").join("vrsctest").join(config_file);
                    paths.push(path);
                }
            },
            "verus" => {
                // Similar pattern for Verus Mainnet
                if cfg!(target_os = "windows") {
                    if let Some(appdata) = std::env::var_os("APPDATA") {
                        let path = PathBuf::from(appdata).join("Komodo").join("VRSC").join(config_file);
                        paths.push(path);
                    }
                } else if cfg!(target_os = "macos") {
                    let path = home_dir.join("Library").join("Application Support").join("Komodo").join("VRSC").join(config_file);
                    paths.push(path);
                } else {
                    let path = home_dir.join(".komodo").join("VRSC").join(config_file);
                    paths.push(path);
                }
            },
            "chips" | "varrr" | "vdex" => {
                // PBaaS chains use different paths
                if let Some(chain_string) = &blockchain_config.chain_string {
                    if cfg!(target_os = "windows") {
                        if let Some(appdata) = std::env::var_os("APPDATA") {
                            let path = PathBuf::from(appdata).join("Verus").join("pbaas").join(chain_string).join(config_file);
                            paths.push(path);
                        }
                    } else if cfg!(target_os = "macos") {
                        let path = home_dir.join("Library").join("Application Support").join("Verus").join("PBAAS").join(chain_string).join(config_file);
                        paths.push(path);
                    } else {
                        let path = home_dir.join(".verus").join("pbaas").join(chain_string).join(config_file);
                        paths.push(path);
                    }
                }
            },
            _ => {
                log::warn!("Unknown blockchain configuration: {}", blockchain_config.id);
            }
        }
    }
    
    paths
}

// NEW: Parse config file to extract credentials
pub fn parse_config_file(file_path: &PathBuf) -> Result<Credentials, DiscoveryError> {
    log::info!("Attempting to parse config file: {:?}", file_path);
    
    let content = fs::read_to_string(file_path)
        .map_err(|e| {
            log::error!("Failed to read config file {:?}: {}", file_path, e);
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => DiscoveryError::PermissionDenied,
                _ => DiscoveryError::IoError(e.to_string()),
            }
        })?;
    
    let mut rpc_user: Option<String> = None;
    let mut rpc_pass: Option<String> = None;
    let mut rpc_port: Option<u16> = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "rpcuser" => {
                    rpc_user = Some(value.to_string());
                    log::debug!("Found rpcuser in config");
                },
                "rpcpassword" => {
                    rpc_pass = Some(value.to_string());
                    log::debug!("Found rpcpassword in config");
                },
                "rpcport" => {
                    rpc_port = value.parse().ok();
                    log::debug!("Found rpcport in config: {:?}", rpc_port);
                },
                _ => {} // Ignore other config options
            }
        }
    }
    
    match (rpc_user, rpc_pass, rpc_port) {
        (Some(user), Some(pass), Some(port)) => {
            log::info!("Successfully parsed credentials from config file. Port: {}", port);
            Ok(Credentials {
                rpc_user: user,
                rpc_pass: pass,
                rpc_port: port,
            })
        },
        (Some(_), Some(_), None) => {
            log::error!("Config file missing required rpcport");
            Err(DiscoveryError::ParseError("Missing rpcport in config file".to_string()))
        },
        _ => {
            log::error!("Config file missing required rpcuser, rpcpassword, or rpcport");
            Err(DiscoveryError::ParseError("Missing rpcuser, rpcpassword, or rpcport".to_string()))
        }
    }
}

// NEW: Parallel blockchain detection with timeout and error handling
#[tauri::command]
pub async fn detect_all_blockchains() -> Result<ParallelDetectionResult, DiscoveryError> {
    let start_time = std::time::Instant::now();
    log::info!("Starting parallel blockchain detection for all supported chains");
    
    let configs = get_blockchain_configs();
    let mut join_set = JoinSet::new();
    
    // Spawn detection tasks for all blockchains in parallel
    for config in configs {
        join_set.spawn(async move {
            detect_single_blockchain(config).await
        });
    }
    
    // Collect results as they complete
    let mut results = Vec::new();
    while let Some(task_result) = join_set.join_next().await {
        match task_result {
            Ok(detection_result) => {
                results.push(detection_result);
            }
            Err(e) => {
                log::error!("Detection task failed: {}", e);
                // Create error result for failed task
                results.push(BlockchainDetectionResult {
                    blockchain_id: "unknown".to_string(),
                    blockchain_name: "Unknown".to_string(),
                    status: BlockchainStatus::Error,
                    credentials: None,
                    config_path: None,
                    error_message: Some(format!("Task execution failed: {}", e)),
                    block_height: None,
                });
            }
        }
    }
    
    // Sort results by the original order (Verus, CHIPS, vDEX, vARRR, Testnet)
    let order = ["verus", "chips", "vdex", "varrr", "verus-testnet"];
    results.sort_by_key(|r| {
        order.iter().position(|&id| id == r.blockchain_id).unwrap_or(999)
    });
    
    let available_count = results.iter().filter(|r| matches!(r.status, BlockchainStatus::Available)).count();
    let duration = start_time.elapsed();
    
    log::info!("Parallel detection completed: {} available out of {} total in {:?}", 
               available_count, results.len(), duration);
    
    Ok(ParallelDetectionResult {
        blockchains: results,
        total_detected: available_count,
        detection_duration_ms: duration.as_millis() as u64,
    })
}

// NEW: Detect a single blockchain with full error handling
async fn detect_single_blockchain(config: BlockchainConfig) -> BlockchainDetectionResult {
    log::debug!("Detecting blockchain: {}", config.name);
    
    // Step 1: Look for config file
    let standard_paths = get_standard_config_paths(&config);
    let mut found_config_path: Option<PathBuf> = None;
    let mut credentials: Option<Credentials> = None;
    
    for path in standard_paths {
        if path.exists() {
            log::debug!("Found config file for {} at: {:?}", config.name, path);
            match parse_config_file(&path) {
                Ok(parsed_creds) => {
                    found_config_path = Some(path);
                    credentials = Some(parsed_creds);
                    break;
                }
                Err(e) => {
                    log::warn!("Failed to parse config for {}: {}", config.name, e);
                    return BlockchainDetectionResult {
                        blockchain_id: config.id,
                        blockchain_name: config.name,
                        status: BlockchainStatus::ParseError,
                        credentials: None,
                        config_path: Some(path.to_string_lossy().to_string()),
                        error_message: Some(e.to_string()),
                        block_height: None,
                    };
                }
            }
        }
    }
    
    // Step 2: If no config found, return NotFound
    let creds = match credentials {
        Some(creds) => creds,
        None => {
            log::debug!("No config file found for {}", config.name);
            return BlockchainDetectionResult {
                blockchain_id: config.id,
                blockchain_name: config.name,
                status: BlockchainStatus::NotFound,
                credentials: None,
                config_path: None,
                error_message: Some("No configuration file found in standard locations".to_string()),
                block_height: None,
            };
        }
    };
    
    // Step 3: Test daemon connection with timeout
    log::debug!("Testing daemon connection for {}", config.name);
    match tokio::time::timeout(
        Duration::from_secs(DETECTION_TIMEOUT_SECS),
        test_daemon_connection(&creds)
    ).await {
        Ok(Ok(block_height)) => {
            log::info!("Successfully detected {}: block height {}", config.name, block_height);
            BlockchainDetectionResult {
                blockchain_id: config.id,
                blockchain_name: config.name,
                status: BlockchainStatus::Available,
                credentials: Some(creds),
                config_path: found_config_path.map(|p| p.to_string_lossy().to_string()),
                error_message: None,
                block_height: Some(block_height),
            }
        }
        Ok(Err(e)) => {
            // Check if this is a loading error (daemon starting up)
            if e.starts_with("LOADING:") {
                let loading_message = e.strip_prefix("LOADING:").unwrap_or("Daemon is starting up...");
                log::info!("Daemon is loading for {}: {}", config.name, loading_message);
                BlockchainDetectionResult {
                    blockchain_id: config.id,
                    blockchain_name: config.name,
                    status: BlockchainStatus::Loading,
                    credentials: Some(creds),
                    config_path: found_config_path.map(|p| p.to_string_lossy().to_string()),
                    error_message: Some(loading_message.to_string()),
                    block_height: None,
                }
            } else {
                log::warn!("Daemon connection failed for {}: {}", config.name, e);
                BlockchainDetectionResult {
                    blockchain_id: config.id,
                    blockchain_name: config.name,
                    status: BlockchainStatus::Error,
                    credentials: Some(creds),
                    config_path: found_config_path.map(|p| p.to_string_lossy().to_string()),
                    error_message: Some(format!("Connection failed: {}", e)),
                    block_height: None,
                }
            }
        }
        Err(_) => {
            log::warn!("Daemon connection timeout for {}", config.name);
            BlockchainDetectionResult {
                blockchain_id: config.id,
                blockchain_name: config.name,
                status: BlockchainStatus::Timeout,
                credentials: Some(creds),
                config_path: found_config_path.map(|p| p.to_string_lossy().to_string()),
                error_message: Some("Connection timeout - daemon may not be running".to_string()),
                block_height: None,
            }
        }
    }
}

// NEW: Test daemon connection (simplified version for detection)
async fn test_daemon_connection(credentials: &Credentials) -> Result<u64, String> {
    use reqwest::{Client, StatusCode};
    use serde_json::json;
    
    let client = Client::new();
    let url = format!("http://127.0.0.1:{}", credentials.rpc_port);
    
    log::info!("Testing connection to {} with user: {} (pass length: {})", 
               url, credentials.rpc_user, credentials.rpc_pass.len());
    
    let request_body = json!({
        "method": "getblockcount",
        "params": [],
        "id": 1
    });
    
    log::info!("Sending request body: {}", request_body);
    
    let response = client
        .post(&url)
        .basic_auth(&credentials.rpc_user, Some(&credentials.rpc_pass))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            log::error!("HTTP request failed for {}: {}", url, e);
            format!("HTTP request failed: {}", e)
        })?;
    
    let status = response.status();
    log::info!("HTTP response status: {}", status);
    
    // Don't fail immediately on 500 errors - the daemon might be returning JSON error info
    if !status.is_success() && status != StatusCode::INTERNAL_SERVER_ERROR {
        log::warn!("Non-success status (non-500): {}", status);
        return Err(format!("HTTP {} - Daemon may not be running", status));
    }
    
    let response_text = response.text().await
        .map_err(|e| format!("Failed to read response text: {}", e))?;
    
    log::info!("Raw response: {}", response_text);
    
    let json_response: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| {
            log::error!("Failed to parse JSON: {} from response: {}", e, response_text);
            format!("Failed to parse JSON response: {}", e)
        })?;
    
    if let Some(error) = json_response.get("error") {
        if !error.is_null() {
            log::info!("RPC error response: {}", error);
            
            // Check for error code -28 (daemon starting up)
            if let Some(error_code) = error.get("code").and_then(|c| c.as_i64()) {
                log::info!("RPC error code: {}", error_code);
                if error_code == -28 {
                    let error_message = error.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Daemon is starting up...");
                    log::info!("DETECTED LOADING STATE: {}", error_message);
                    return Err(format!("LOADING:{}", error_message));
                }
            }
            log::warn!("Non-loading RPC error: {}", error);
            return Err(format!("RPC error: {}", error));
        }
    }
    
    // If we got a 500 error but no RPC error in JSON, treat it as a generic error
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        log::warn!("HTTP 500 with no RPC error information");
        return Err(format!("HTTP 500 - Daemon may not be running properly"));
    }
    
    let block_count = json_response
        .get("result")
        .and_then(|r| r.as_u64())
        .ok_or_else(|| "Invalid block count in response".to_string())?;
    
    log::info!("Successfully got block count: {}", block_count);
    Ok(block_count)
}

// NEW: Folder selection dialog command
#[tauri::command]
pub async fn select_folder_dialog<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>, String> {
    log::info!("Opening folder selection dialog");
    
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
    use tokio::sync::oneshot;
    
    let (tx, rx) = oneshot::channel();
    
    FileDialogBuilder::new(app.dialog().clone())
        .set_title("Select blockchain configuration folder")
        .pick_folder(move |folder_path| {
            let result = folder_path.map(|path| path.to_string());
            let _ = tx.send(result);
        });
    
    match rx.await {
        Ok(result) => Ok(result),
        Err(_) => Err("Dialog was cancelled or failed".to_string()),
    }
}

// NEW: Detect blockchain from custom path
#[tauri::command]
pub async fn detect_blockchain_from_path(path: String) -> Result<ParallelDetectionResult, DiscoveryError> {
    log::info!("Detecting blockchains from custom path: {}", path);
    
    let start_time = std::time::Instant::now();
    let configs = get_blockchain_configs();
    let custom_path = PathBuf::from(path);
    let mut results = Vec::new();
    
    // Check for each blockchain config in the custom path
    for config in configs {
        let config_file_path = custom_path.join(&config.config_file_name);
        
        let result = if config_file_path.exists() {
            log::debug!("Found {} config in custom path", config.name);
            match parse_config_file(&config_file_path) {
                Ok(credentials) => {
                    // Test the connection
                    match test_daemon_connection(&credentials).await {
                        Ok(block_height) => {
                            log::info!("Successfully connected to {} from custom path", config.name);
                            BlockchainDetectionResult {
                                blockchain_id: config.id,
                                blockchain_name: config.name,
                                status: BlockchainStatus::Available,
                                credentials: Some(credentials),
                                config_path: Some(config_file_path.to_string_lossy().to_string()),
                                error_message: None,
                                block_height: Some(block_height),
                            }
                        }
                        Err(e) => {
                            BlockchainDetectionResult {
                                blockchain_id: config.id,
                                blockchain_name: config.name,
                                status: BlockchainStatus::Error,
                                credentials: Some(credentials),
                                config_path: Some(config_file_path.to_string_lossy().to_string()),
                                error_message: Some(e),
                                block_height: None,
                            }
                        }
                    }
                }
                Err(e) => {
                    BlockchainDetectionResult {
                        blockchain_id: config.id,
                        blockchain_name: config.name,
                        status: BlockchainStatus::ParseError,
                        credentials: None,
                        config_path: Some(config_file_path.to_string_lossy().to_string()),
                        error_message: Some(e.to_string()),
                        block_height: None,
                    }
                }
            }
        } else {
            BlockchainDetectionResult {
                blockchain_id: config.id,
                blockchain_name: config.name,
                status: BlockchainStatus::NotFound,
                credentials: None,
                config_path: None,
                error_message: Some("Config file not found in selected folder".to_string()),
                block_height: None,
            }
        };
        
        results.push(result);
    }
    
    let available_count = results.iter().filter(|r| matches!(r.status, BlockchainStatus::Available)).count();
    let duration = start_time.elapsed();
    
    Ok(ParallelDetectionResult {
        blockchains: results,
        total_detected: available_count,
        detection_duration_ms: duration.as_millis() as u64,
    })
}

// Custom error type for credential operations
#[derive(Debug, thiserror::Error, Serialize)]
pub enum CredentialError {
    #[error("Store plugin error: {0}")]
    Store(String),
    #[error("Credentials not found in store")]
    NotFound,
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

// Convert StoreError to CredentialError
impl From<StoreError> for CredentialError {
    fn from(error: StoreError) -> Self {
        CredentialError::Store(error.to_string())
    }
}

// Tauri command to save credentials
#[tauri::command]
pub async fn save_credentials<R: Runtime>(
    app: AppHandle<R>,
    rpc_user: String,
    rpc_pass: String,
    rpc_port: u16,
) -> Result<(), CredentialError> {
    log::info!("Attempting to save credentials to store...");
    let credentials = Credentials { rpc_user, rpc_pass, rpc_port };
    let credentials_json = serde_json::to_value(credentials)
        .map_err(|e| CredentialError::Serialization(e.to_string()))?;

    // Get the store instance using the StoreExt trait
    let store = app.store(STORE_PATH)?;

    // set() returns () (unit type)
    store.set(CREDENTIALS_KEY.to_string(), credentials_json);
    
    // save() returns Result so we keep the ?
    store.save()?;

    log::info!("Credentials saved successfully to store.");
    Ok(())
}

// Tauri command to load credentials
#[tauri::command]
pub async fn load_credentials<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Credentials, CredentialError> {
    log::info!("Attempting to load credentials from store...");

    // Get the store instance
    let store = app.store(STORE_PATH)?;

    match store.get(CREDENTIALS_KEY) {
        Some(value) => {
            log::info!("Credentials JSON retrieved from store.");
            
            // Try to deserialize into the new format first
            match serde_json::from_value::<Credentials>(value.clone()) {
                Ok(credentials) => {
                    log::info!("Successfully loaded credentials with port: {}", credentials.rpc_port);
                    Ok(credentials)
                }
                Err(e) => {
                    log::warn!("Failed to deserialize as new format: {}", e);
                    
                    // Try to migrate from old format (without rpc_port)
                    #[derive(Deserialize)]
                    struct OldCredentials {
                        rpc_user: String,
                        rpc_pass: String,
                    }
                    
                    match serde_json::from_value::<OldCredentials>(value) {
                        Ok(_old_creds) => {
                            log::warn!("Found old credentials format without port information. Cannot migrate safely as port is blockchain-specific. Clearing old credentials to force fresh setup.");
                            
                            // Clear the old credentials instead of migrating with wrong port
                            if store.delete(CREDENTIALS_KEY) {
                                store.save()?;
                                log::info!("Cleared old credentials. User will need to set up credentials again with proper port.");
                            }
                            
                            Err(CredentialError::NotFound)
                        }
                        Err(migration_error) => {
                            log::error!("Failed to parse credentials in any known format: {}", migration_error);
                            Err(CredentialError::Deserialization(format!(
                                "Could not deserialize credentials. New format error: {}. Old format error: {}",
                                e, migration_error
                            )))
                        }
                    }
                }
            }
        }
        None => {
            log::info!("Key '{}' not found in store.", CREDENTIALS_KEY);
            Err(CredentialError::NotFound)
        }
    }
}

// Tauri command to clear credentials
#[tauri::command]
pub async fn clear_credentials<R: Runtime>(app: AppHandle<R>) -> Result<(), CredentialError> {
    log::info!("Attempting to clear credentials from store...");

    // Get the store instance
    let store = app.store(STORE_PATH)?;

    // has() returns bool
    if store.has(CREDENTIALS_KEY) {
        // delete() returns bool indicating whether the key was found and deleted
        let deleted = store.delete(CREDENTIALS_KEY);
        
        if deleted {
            // Only need to save if we actually deleted something
            store.save()?;
            log::info!("Credentials cleared successfully from store.");
        } else {
            log::info!("Key '{}' not found during delete attempt.", CREDENTIALS_KEY);
        }
    } else {
        log::info!("Key '{}' not found, nothing to clear.", CREDENTIALS_KEY);
    }
    
    Ok(())
} 