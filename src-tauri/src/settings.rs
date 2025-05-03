// File: src-tauri/src/settings.rs
// Description: Handles storage and retrieval of user settings and chat data persistence.
// Changes:
// - Created file to manage settings persistence.
// - Added Conversation and ChatMessage structs (mirroring frontend types).
// - Added Tauri commands for saving/loading persistence preference.
// - Added Tauri commands for saving/loading conversations.
// - Added Tauri commands for saving/loading messages per conversation.
// - Added Tauri command for deleting chat data.

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::{StoreExt, Error as StoreError};
use std::collections::HashMap; // Needed if using HashMap approach later
use serde_json::json; // Import serde_json macro for json!() usage

// Use the same store path as credentials for simplicity, just different keys
const STORE_PATH: &str = "store.json";

// --- Structs mirroring frontend types ---

// Mirror src/lib/types.ts Conversation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub id: String,         // Unique ID, typically the recipient's VerusID name (e.g., user@)
    pub name: String,       // Display name (VerusID name)
    pub recipient_private_address: String, // The recipient's z-address needed for sending
    #[serde(default)] // Handle optional field during deserialization
    pub unread: Option<bool>,   // Optional flag for unread messages
}

// Mirror src/lib/types.ts ChatMessage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: String, // txid or generated ID for sent messages
    pub sender: String, // VerusID of the sender (@ format) or 'self'
    pub text: String,
    pub timestamp: u64, // Use u64 to match frontend Date.now() possibility
    pub amount: f64,
    pub confirmations: i64,
    pub direction: String, // "received" | "sent"
    #[serde(default)] // Handle optional field during deserialization
    pub status: Option<String>, // Optional delivery status for sent messages "sent" | "delivered" | "failed"
}

// Custom error type (can be expanded)
#[derive(Debug, thiserror::Error, Serialize)]
pub enum SettingsError {
    #[error("Store plugin error: {0}")]
    Store(String),
    #[error("Data not found for key: {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl From<StoreError> for SettingsError {
    fn from(error: StoreError) -> Self {
        SettingsError::Store(error.to_string())
    }
}

// --- Helper Functions ---

fn get_preference_key(identity_i_address: &str) -> String {
    format!("persist_pref_{}", identity_i_address)
}

fn get_conversations_key(identity_i_address: &str) -> String {
    format!("conversations_{}", identity_i_address)
}

fn get_messages_key(identity_i_address: &str, conversation_id: &str) -> String {
    format!("messages_{}_{}", identity_i_address, conversation_id)
}

// --- Tauri Commands ---

#[tauri::command]
pub async fn save_persistence_setting<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
    save_preference: bool,
) -> Result<(), SettingsError> {
    log::info!("Saving persistence setting for {}: {}", identity_i_address, save_preference);
    let store = app.store(STORE_PATH)?;
    let key = get_preference_key(&identity_i_address);
    store.set(key, json!(save_preference)); // Use serde_json::json macro
    store.save()?;
    log::info!("Persistence setting saved successfully.");
    Ok(())
}

#[tauri::command]
pub async fn load_persistence_setting<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
) -> Result<Option<bool>, SettingsError> {
    log::info!("Loading persistence setting for {}", identity_i_address);
    let store = app.store(STORE_PATH)?;
    let key = get_preference_key(&identity_i_address);
    match store.get(&key) {
        Some(value) => {
            log::debug!("Found persistence setting value for {}: {:?}", identity_i_address, value);
            // Directly deserialize as bool
             let pref = serde_json::from_value::<bool>(value.clone())
                 .map_err(|e| SettingsError::Deserialization(format!("Failed to parse preference bool: {}", e)))?;
             Ok(Some(pref))
        }
        None => {
            log::info!("Persistence setting not found for {}", identity_i_address);
            Ok(None) // Return Ok(None) when the key doesn't exist
        }
    }
}

#[tauri::command]
pub async fn save_conversations<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
    conversations: Vec<Conversation>,
) -> Result<(), SettingsError> {
    log::info!("Saving {} conversations for {}", conversations.len(), identity_i_address);
    let store = app.store(STORE_PATH)?;
    let key = get_conversations_key(&identity_i_address);
    let conversations_json = serde_json::to_value(conversations)
        .map_err(|e| SettingsError::Serialization(e.to_string()))?;
    store.set(key, conversations_json);
    store.save()?;
    log::info!("Conversations saved successfully.");
    Ok(())
}

#[tauri::command]
pub async fn load_conversations<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
) -> Result<Vec<Conversation>, SettingsError> {
    log::info!("Loading conversations for {}", identity_i_address);
    let store = app.store(STORE_PATH)?;
    let key = get_conversations_key(&identity_i_address);
    match store.get(&key) {
        Some(value) => {
            log::debug!("Found conversations value for {}", identity_i_address);
             serde_json::from_value::<Vec<Conversation>>(value.clone())
                 .map_err(|e| SettingsError::Deserialization(format!("Failed to parse conversations Vec: {}", e)))
        }
        None => {
            log::info!("No conversations found in store for {}", identity_i_address);
            Ok(Vec::new()) // Return empty Vec if not found
        }
    }
}

#[tauri::command]
pub async fn save_messages_for_conversation<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
    conversation_id: String,
    messages: Vec<ChatMessage>,
) -> Result<(), SettingsError> {
    log::info!("Saving {} messages for conversation {} (user {})", messages.len(), conversation_id, identity_i_address);
    let store = app.store(STORE_PATH)?;
    let key = get_messages_key(&identity_i_address, &conversation_id);
     let messages_json = serde_json::to_value(messages)
        .map_err(|e| SettingsError::Serialization(e.to_string()))?;
    store.set(key, messages_json);
    store.save()?;
    log::info!("Messages for conversation {} saved successfully.", conversation_id);
    Ok(())
}

#[tauri::command]
pub async fn load_messages_for_conversation<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
    conversation_id: String,
) -> Result<Vec<ChatMessage>, SettingsError> {
    log::info!("Loading messages for conversation {} (user {})", conversation_id, identity_i_address);
     let store = app.store(STORE_PATH)?;
    let key = get_messages_key(&identity_i_address, &conversation_id);
    match store.get(&key) {
        Some(value) => {
            log::debug!("Found messages value for conversation {}", conversation_id);
             serde_json::from_value::<Vec<ChatMessage>>(value.clone())
                 .map_err(|e| SettingsError::Deserialization(format!("Failed to parse messages Vec for {}: {}", conversation_id, e)))
        }
        None => {
            log::info!("No messages found in store for conversation {}", conversation_id);
            Ok(Vec::new()) // Return empty Vec if not found
        }
    }
}

#[tauri::command]
pub async fn delete_chat_data<R: Runtime>(
    app: AppHandle<R>,
    identity_i_address: String,
) -> Result<(), SettingsError> {
     log::warn!("Attempting to delete ALL chat data (preference, conversations, messages) for identity: {}", identity_i_address);
    let store = app.store(STORE_PATH)?;

    let pref_key = get_preference_key(&identity_i_address);
    let convos_key = get_conversations_key(&identity_i_address);

    // 1. Load conversations to find message keys
    let conversations_to_delete: Vec<Conversation> = match store.get(&convos_key) {
        Some(value) => serde_json::from_value(value.clone()).unwrap_or_else(|_| Vec::new()),
        None => Vec::new(),
    };

    // 2. Delete preference
    if store.has(&pref_key) {
        if store.delete(&pref_key) {
             log::info!("Deleted preference key: {}", pref_key);
        } else {
            log::warn!("Failed to delete preference key (though it existed): {}", pref_key);
        }
    }

    // 3. Delete conversations list
    if store.has(&convos_key) {
         if store.delete(&convos_key) {
             log::info!("Deleted conversations key: {}", convos_key);
         } else {
             log::warn!("Failed to delete conversations key (though it existed): {}", convos_key);
         }
    }

    // 4. Delete messages for each conversation
    let mut messages_deleted = 0;
    for convo in conversations_to_delete {
         let msg_key = get_messages_key(&identity_i_address, &convo.id);
         if store.has(&msg_key) {
            if store.delete(&msg_key) {
                 log::debug!("Deleted messages key: {}", msg_key);
                 messages_deleted += 1;
            } else {
                log::warn!("Failed to delete messages key (though it existed): {}", msg_key);
            }
         }
    }
    log::info!("Deleted message data for {} conversations.", messages_deleted);

    // 5. Save changes to the store file
    store.save()?;
    log::warn!("Completed deletion of chat data for identity: {}. Store saved.", identity_i_address);

    Ok(())
} 