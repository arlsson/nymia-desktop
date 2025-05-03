# PRD: Optional Chat History Persistence

**Version:** 0.1
**Date:** 2024-07-26
**Author:** Gemini Assistant

## 1. Introduction & Goal

This document outlines the requirements for implementing an optional local chat history and conversation list persistence feature in the Verus Chat Dapp. The goal is to allow users to choose whether their chat data (messages and conversation list) is saved locally on their device for each VerusID they use to log in.

## 2. Background & Problem

Currently, the application does not persist chat messages or the list of active conversations between sessions. When a user logs out and logs back in (even with the same VerusID), their previous chats and conversation history are lost. This provides privacy by default but lacks convenience for users who want to retain their chat history.

## 3. Proposed Solution

Implement a system where, upon first logging in with a specific VerusID, the user is prompted to choose whether they want to save their chat data locally for that ID.

- **User Prompt:** A modal dialog will ask the user if they want to enable local chat saving for the currently logged-in VerusID.
- **Per-ID Setting:** The user's choice (Yes/No) will be stored securely and associated *only* with that specific VerusID.
- **Persistence Mechanism:** Use the existing `tauri-plugin-store` (similar to how RPC credentials are handled) to store both the user's preference and the chat data itself (if enabled).
- **Conditional Loading/Saving:**
    - If saving is enabled for the VerusID, load saved conversations and messages upon login. Save new messages and conversation updates during the session.
    - If saving is disabled, the application behaves as it currently does (ephemeral chats).
- **Future Configuration:** Users should be able to change their persistence preference and clear their locally stored chat data later via application settings.

## 4. User Flow

1.  **Login:** User completes the onboarding/login flow using `OnboardingFlow.svelte` and selects a VerusID.
2.  **Chat Interface Load:** The application transitions to `ChatInterface.svelte`.
3.  **Check Preference:** The application checks if a persistence preference (Yes/No) has already been stored for the logged-in VerusID's `i_address`.
4.  **First Time Prompt (If No Preference Stored):**
    - A modal dialog appears: "Save Chat History Locally for [VerusID Name]? Saving chats locally allows you to see your conversation history when you log back in with this ID. You can change this setting later."
    - Options: "Yes, Save Locally", "No, Keep Chats Ephemeral".
    - **On "Yes":** Store preference (`true`) for this VerusID. Initialize chat state (likely empty initially). Start saving subsequent chat activity. Close modal.
    - **On "No":** Store preference (`false`) for this VerusID. Initialize chat state (empty). Do not save chat activity. Close modal.
5.  **Subsequent Logins (Preference Stored):**
    - **If Preference is "Yes":** Load saved conversations and messages from local storage for this VerusID into the application state (`conversations` and `messages` in `ChatInterface.svelte`). Start polling for new messages and save incoming/outgoing activity.
    - **If Preference is "No":** Initialize chat state as empty. Start polling for new messages but do not save any activity.

## 5. Technical Design

### 5.1 Frontend Changes (`src/lib/...`)

- **New Modal Component (`PersistencePromptModal.svelte`):**
    - Displays the Yes/No question.
    - Emits events based on user choice ("save", "dont-save").
- **`ChatInterface.svelte`:**
    - **State:** Add state variable `persistenceSetting: boolean | null = null;`, `showPersistencePrompt: boolean = false;`.
    - **`onMount` Logic:**
        - Call new Tauri command `load_persistence_setting(identity_i_address)`
        - If setting exists (`true`/`false`), update `persistenceSetting`.
        - If setting *doesn't* exist, set `showPersistencePrompt = true;`.
        - Based on `persistenceSetting` (once determined):
            - If `true`, call Tauri commands to load saved conversations and messages.
            - If `false` or `null` initially, initialize empty state.
    - **Event Handlers:**
        - Handle "save"/"dont-save" events from the modal: Call Tauri command `save_persistence_setting(identity_i_address, choice)`. Update `persistenceSetting`. Load data if "save" was chosen and data exists.
    - **Data Saving Logic (if `persistenceSetting === true`):**
        - **Conversations:** When a new conversation is added (`handleStartChat`) or potentially removed, call Tauri command `save_conversations(identity_i_address, current_conversations_array)`.
        - **Messages:** When a new message is received (`processNewMessages`) or sent (`handleSendMessage`), update the relevant conversation's message list and call Tauri command `save_messages_for_conversation(identity_i_address, conversation_id, current_messages_array)`. (Debouncing or batching saves might be needed for performance).
- **`+page.svelte` (Potentially):** Might need minor adjustments depending on how loading states are handled.

### 5.2 Backend Changes (`src-tauri/src/...`)

- **New Module (`settings.rs` or similar):** Create a dedicated module for managing settings and chat data persistence.
- **Tauri Commands:**
    - `save_persistence_setting(app: AppHandle<R>, identity_i_address: String, save_preference: bool) -> Result<(), StoreError>`
    - `load_persistence_setting(app: AppHandle<R>, identity_i_address: String) -> Result<Option<bool>, StoreError>`
    - `save_conversations(app: AppHandle<R>, identity_i_address: String, conversations: Vec<Conversation>) -> Result<(), StoreError>`
    - `load_conversations(app: AppHandle<R>, identity_i_address: String) -> Result<Vec<Conversation>, StoreError>`
    - `save_messages_for_conversation(app: AppHandle<R>, identity_i_address: String, conversation_id: String, messages: Vec<ChatMessage>) -> Result<(), StoreError>`
    - `load_messages_for_conversation(app: AppHandle<R>, identity_i_address: String, conversation_id: String) -> Result<Vec<ChatMessage>, StoreError>`
    - `load_all_messages(app: AppHandle<R>, identity_i_address: String) -> Result<HashMap<String, Vec<ChatMessage>>, StoreError>` (Alternative if storing all messages together)
    - `delete_chat_data(app: AppHandle<R>, identity_i_address: String) -> Result<(), StoreError>` (For future settings)
- **`tauri-plugin-store` Usage:**
    - Use a separate store file (e.g., `chats.json`) or prefix keys within `store.json`.
    - **Keys:**
        - `persist_pref_{identity_i_address}`: Stores `true` or `false`.
        - `conversations_{identity_i_address}`: Stores `Vec<Conversation>`.
        - `messages_{identity_i_address}_{conversation_id}`: Stores `Vec<ChatMessage>` (if using per-conversation storage).
        *or*
        - `messages_{identity_i_address}`: Stores `HashMap<String, Vec<ChatMessage>>` (if storing all messages together).

### 5.3 Data Structures (Ensure consistency)

- Reuse `Conversation` and `ChatMessage` types from `src/lib/types.ts` in the Rust backend (derive `Serialize`, `Deserialize`).

## 6. Future Considerations / Settings

- **Settings UI:** Implement UI elements (likely within a dedicated Settings section/modal) to:
    - Allow users to toggle the `persistenceSetting` for the current VerusID.
    - Provide a button to "Clear Local Chat History" for the current VerusID (calls `delete_chat_data`).
- **Data Migration:** If storage format changes later, consider migration strategies.
- **Performance:** Monitor performance, especially when saving/loading large amounts of message data. Consider optimizations like debouncing saves, pagination for loading, or using a more performant storage solution if `tauri-plugin-store` becomes a bottleneck (though unlikely for typical chat volumes).
- **Encryption:** The current plan uses plain JSON via `tauri-plugin-store`. While credentials are also stored this way, consider if local chat data warrants additional client-side encryption (adds complexity).

## 7. Open Questions

1.  **Timing of the Prompt:** Confirm if the prompt should appear in `ChatInterface.svelte` on first load or as part of `OnboardingFlow.svelte`. (Current Assumption: `ChatInterface.svelte`)
2.  **Initial Save:** Confirm if only *new* activity is saved after opting in, or if an attempt should be made to save already-loaded data. (Current Assumption: Save new activity only)
3.  **Message Storage Structure:** Decide between storing messages per-conversation or all together per-user. (Need User Input)
4.  **Future Settings Location:** Confirm the planned location for changing the setting/deleting data. (Need User Input - Assumed: TopBar Settings)
5.  **Error Handling:** Define user feedback for errors during saving/loading chat data. 