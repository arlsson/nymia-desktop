// File: src-tauri/src/credentials.rs
// Description: Handles storage and retrieval of RPC credentials using tauri-plugin-store.
// Note: This stores credentials in a plain JSON file, NOT encrypted.
// Changes:
// - Replaced insert() with set().
// - Corrected app.store() call (removed .into()).
// - Fixed proper method return handling (set, has, delete, etc.)

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::{StoreExt, Error as StoreError};

// Path for the store file relative to AppData directory
const STORE_PATH: &str = "store.json";

// Key used within the store file
const CREDENTIALS_KEY: &str = "verus_rpc_credentials";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    pub rpc_user: String,
    pub rpc_pass: String,
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
) -> Result<(), CredentialError> {
    log::info!("Attempting to save credentials to store...");
    let credentials = Credentials { rpc_user, rpc_pass };
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
            serde_json::from_value::<Credentials>(value.clone())
                 .map_err(|e| CredentialError::Deserialization(e.to_string()))
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