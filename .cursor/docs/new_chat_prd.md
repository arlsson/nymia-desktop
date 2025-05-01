# PRD: Start New Chat Feature

**Version:** 0.2
**Date:** 2024-07-27
**Author:** AI Assistant

## 1. Introduction

This document outlines the requirements for the "Start New Chat" feature in the Nymia chat application. This feature allows users to initiate a new private conversation with another VerusID user, check the eligibility of the recipient, and optionally import existing chat history derived from blockchain transaction memos.

## 2. Goals

-   Allow users to initiate a new chat conversation with any VerusID.
-   Ensure the recipient VerusID is capable of receiving private messages (has a private address associated).
-   Detect if prior communication history exists between the users via transaction memos.
-   Provide an option to import this existing history into the chat view.
-   Provide clear feedback to the user during the process (validation, errors, success).

## 3. User Flow

1.  **Trigger:** User clicks the "New Chat" button in the `ConversationsList` component.
2.  **Popup Display:** A modal popup appears.
3.  **Input:** User enters the target VerusID (e.g., `friend@`) into an input field within the popup.
4.  **Initial Frontend Check:** Before calling the backend, the frontend checks if a conversation with the entered VerusID already exists in the `ConversationsList`.
    *   **If Yes:** Display a message in the popup's status area (e.g., "You already have a conversation with this user."). Disable the "Find User" button. The flow stops here unless the user enters a different ID.
    *   **If No:** Proceed to the next step.
5.  **Initiate Check:** User clicks a button (e.g., "Find User" or "Next"). The status area shows "Checking...".
6.  **Backend Check 1 (Eligibility):**
    *   The application backend (Tauri/Rust) receives the target VerusID.
    *   It calls the `getidentity` RPC command for the target VerusID.
    *   It checks if the identity exists and has a non-empty `privateaddress` field within the `identity` object.
    *   **If Ineligible (No private address/Not Found):** An error message is displayed in the popup's status area (e.g., "User not found or cannot receive private messages."). The flow stops here unless the user enters a different ID.
    *   **If Eligible:** Proceed to the next step. The status area might briefly show "User found, checking history...".
7.  **Backend Check 2 (History):**
    *   The backend uses the *logged-in user's* private address (obtained during login/onboarding).
    *   It calls the `z_listreceivedbyaddress` RPC command with the logged-in user's private address. *All* returned transactions will be checked.
    *   It iterates through the returned transactions. For each transaction, it examines the `memostr` field.
    *   It checks if `memostr` strictly matches the pattern `(.*)//f//<target_verus_id>`.
    *   **If History Found:** Proceed to Step 8a.
    *   **If No History Found:** Proceed to Step 8b.
8.  **Popup Update:**
    *   **8a. (History Found):** The popup status area shows "User found." It displays an option (e.g., a checkbox, checked by default) labelled "Import existing chat history". A "Start Chat" button is enabled, replacing the "Find User" button.
    *   **8b. (No History Found):** The popup status area shows "User found." A "Start Chat" button is enabled, replacing the "Find User" button. The import option is not shown.
9.  **Final Action (Start Chat):**
    *   User clicks the "Start Chat" button.
    *   The popup closes.
    *   The target VerusID (name and i-address confirmed by eligibility check) is added to the `ConversationsList`.
    *   The new conversation is selected.
    *   **If History Import Selected (from 8a):**
        *   The backend extracts the relevant messages (text part of `memostr` before `//f//`) and associated metadata (`txid`, `amount`, `confirmations`, timestamp from transaction if available) from the transactions identified in step 7.
        *   These historical messages are passed to the frontend.
        *   The `ConversationView` displays these imported messages (potentially marked distinctly or integrated chronologically).
    *   **If History Import Not Selected / No History:**
        *   The `ConversationView` for the target VerusID is displayed (empty if no history was imported or found).

## 4. UI Description

-   **Trigger:** A button labelled "New Chat" with a plus icon (+) in the header of the `ConversationsList.svelte` component.
-   **Popup (`NewChatModal.svelte`):**
    *   Title: "Start New Chat"
    *   Input Field: Labelled "Enter VerusID" (e.g., `user@` or `user.parent@`).
    *   Button 1: "Find User" / "Next" (Triggers backend checks). Initially enabled. Gets disabled/replaced after checks.
    *   **Status/Error Area:** A designated text area within the popup to display dynamic feedback messages, including:
        *   *Informational:* "Checking...", "User found.", "User found, checking history..."
        *   *Validation:* "You already have a conversation with this user."
        *   *Errors:* "User not found or cannot receive private messages.", "Invalid VerusID format.", "Could not connect to Verus daemon.", "Failed to check history.", "An unexpected error occurred."
    *   (Conditional) Checkbox: "Import existing chat history" - Appears only if history is found. Checked by default.
    *   (Conditional) Button 2: "Start Chat" - Enabled once a valid, eligible VerusID is confirmed. Replaces/Disables Button 1.
    *   Button 3: "Cancel" / Close icon (X) - Closes the popup without action.

## 5. Backend Requirements (Tauri/Rust - `verus_rpc.rs`)

-   **New Tauri Command 1: `check_identity_eligibility`**
    *   Input: `target_identity_name: String`
    *   Action: Calls `getidentity` RPC for the input name.
    *   Output:
        *   `Ok(FormattedIdentity)`: If found and has a `privateaddress`. The `FormattedIdentity` struct includes `formatted_name`, `i_address`, and `private_address`.
        *   `Err(VerusRpcError)`: If not found, no private address, RPC error, etc. Specific error types/messages should distinguish:
            *   `NotFoundOrIneligible`: Identity doesn't exist or lacks a private address.
            *   `RpcError`: Communication error with the daemon.
            *   `InvalidFormat`: Input string is not a valid identity name format.
-   **New Tauri Command 2: `get_chat_history`**
    *   Input: `target_identity_name: String`, `own_private_address: String` (logged-in user's z-addr)
    *   Action:
        *   Calls `z_listreceivedbyaddress` RPC using `own_private_address`. Will process all returned transactions.
        *   Filters the results based on `memostr` strictly containing `//f//<target_identity_name>`.
        *   Parses the message content (part before `//f//`) from the `memostr`.
        *   Retrieves relevant metadata: `amount`, `confirmations`.
    *   Output:
        *   `Ok(Vec<ChatMessage>)`: A list of chat message objects. `ChatMessage` struct definition:
            ```rust
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct ChatMessage {
                pub id: String, // txid
                pub sender: String, // target_identity_name (the sender in this context)
                pub text: String, // Parsed message content
                pub timestamp: u64, // Transaction timestamp (if available, else 0 or estimate)
                pub amount: f64, // Amount from the transaction
                pub confirmations: i64, // Confirmations from the transaction
                pub direction: String, // "received" (since we use z_listreceivedbyaddress)
            }
            ```
            Returns an empty vector if no history is found.
        *   `Err(VerusRpcError)`: If RPC call fails.
-   **Memo Parsing Logic:** Implement strict parsing for the `message//f//sender@` format within the `get_chat_history` function. Memos not matching this exact structure are ignored for history import.

## 6. Open Questions / Considerations

-   **Q1 (Popup Component):** *Resolved.* Yes, create `src/lib/components/chat/NewChatModal.svelte`.
-   **Q2 (Error Handling):** *Resolved.* Specific user-friendly error messages (examples provided in UI section) will be displayed in the popup's status area. Backend should return distinct error types/codes to enable this.
-   **Q3 (History Check Scope):** *Resolved.* All transactions returned by `z_listreceivedbyaddress` will be checked for relevant memos. Performance implications to be monitored.
-   **Q4 (Memo Format):** *Resolved.* Parsing will be strict, only matching `message//f//sender@`. Other memos ignored.
-   **Q5 (Existing Conversations):** *Resolved.* If the entered VerusID is already in the `ConversationsList`, the popup will display a message and prevent proceeding, rather than selecting the existing chat.
-   **Q6 (History Import Details):** *Resolved.* Imported message data (`ChatMessage` struct) will include `txid` (as `id`), `sender`, `text`, `timestamp` (from tx time if possible), `amount`, and `confirmations`. A `direction` field is added.
-   **Import Checkbox Default:** *Resolved.* The "Import history" checkbox should be checked by default when it appears.
-   **Combined Backend Call?:** Could `check_identity_eligibility` and `get_chat_history` be combined into a single backend call for efficiency? (e.g., `initiate_chat_check(target_id, own_zaddr) -> Result<{identity: FormattedIdentity, history: Vec<ChatMessage>}, Error>`) - *Decision: Keep separate for now for clarity, can combine later if performance requires.*

## 7. Future Considerations

-   Handling sent messages history (currently only checks received messages - would require scanning sent transactions).
-   Support for other memo formats or encryption schemes.
-   Real-time updates if history changes while popup is open (likely out of scope for v1).
-   Searching/filtering contacts within the popup.
-   Using transaction timestamps more reliably.
-   Displaying `amount` and `confirmations` in the chat UI for imported messages. 