# PRD: Send Private Message/Gift

**Version:** 1.0
**Date:** 2024-07-26
**Author:** AI Assistant

## 1. Overview

This document outlines the requirements for sending a private message, optionally accompanied by a VRSC gift, from one VerusID user to another within the Chat DApp.

## 2. Goals

-   Enable users to securely send end-to-end encrypted messages via Verus memos.
-   Allow users to optionally attach a VRSC gift amount to their message.
-   Provide immediate feedback in the UI upon initiating the send action.

## 3. User Flow (Post-Confirmation)

1.  The user has typed a message and/or entered a gift amount in the `MessageInput.svelte` component.
2.  The user clicks the "Send" button, triggering the confirmation dialog.
3.  The user reviews the message preview (if any), gift amount (if any), and total cost (gift + fee) in the confirmation dialog.
4.  The user clicks the "Confirm" button in the dialog.
5.  **Frontend (Optimistic Update):**
    *   The confirmation dialog closes.
    *   The message input field and amount input are cleared.
    *   A new message entry representing the sent message/gift is immediately added to the `ConversationView.svelte` for the current chat. This entry shows the message text and/or gift amount. No specific "sending" or "pending" status indicator is displayed at this stage.
6.  **Backend (Asynchronous):**
    *   The frontend invokes a Tauri command (`send_private_message`) in the Rust backend.
    *   The backend function constructs the necessary parameters for the `z_sendmany` RPC call.
    *   The backend executes the `z_sendmany` call via the Verus daemon.
    *   The backend receives the transaction ID (txid) upon successful submission to the daemon. (Handling of this txid for status updates is deferred).

## 4. Technical Implementation Details

### 4.1 Frontend (`ChatInterface.svelte`, `ConversationView.svelte`, `MessageInput.svelte`)

-   **Get Recipient Address:** The `ChatInterface.svelte` component must store the recipient's `private_address` (obtained during the "New Chat" flow via `check_identity_eligibility` and passed in the `start-chat` event) associated with each conversation ID.
-   **Invoke Backend:** When the user confirms sending in `MessageInput.svelte`, an event is dispatched upwards. `ChatInterface.svelte` will handle this, retrieve the necessary data (sender identity, sender private address, recipient private address, message text, amount), and invoke the `send_private_message` Tauri command.
-   **Optimistic UI Update:** Upon invoking the backend command, `ChatInterface.svelte` immediately updates the `messages` state for the relevant `selectedConversationId`, adding a new `ChatMessage` object with `direction: 'sent'`, the entered text/amount, and a temporary ID/timestamp.

### 4.2 Backend (`src-tauri/src/verus_rpc.rs`)

-   **New Function:** Create a new public async function `send_private_message`:
    ```rust
    pub async fn send_private_message(
        rpc_user: String,
        rpc_pass: String,
        sender_z_address: String,
        recipient_z_address: String,
        memo_text: String,
        sender_identity: String, // e.g., user@
        amount: f64
    ) -> Result<String, VerusRpcError> // Returns txid
    ```
-   **Memo Construction:**
    -   Combine `memo_text` and the sender marker: `format!("{}//f//{}", memo_text, sender_identity)`
    -   Handle cases where `memo_text` might be empty (if only sending a gift).
-   **Hex Encoding:** Convert the full memo string into its hexadecimal representation. Standard libraries or crates like `hex` can be used.
-   **RPC Call (`z_sendmany`):**
    -   Use the `make_rpc_call` helper.
    -   **Method:** `z_sendmany`
    -   **Parameters:**
        1.  `sender_z_address` (string): The logged-in user's private z-address.
        2.  `amounts` (array of objects): A JSON array containing *one* object:
            ```json
            [
              {
                "address": "recipient_z_address",
                "amount": amount, // The gift amount (can be 0)
                "memo": "hex_encoded_memo_string"
              }
            ]
            ```
        3.  `minconf` (integer, optional): Default is usually 1. Can be omitted or set to 1.
        4.  `fee` (number, optional): Default is 0.0001. Can be omitted.
-   **Return Value:** Return the `txid` (string) contained in the successful RPC response `result`. Propagate errors using `VerusRpcError`.

## 5. Error Handling

-   **Frontend:**
    -   Basic validation (insufficient balance, message length) is already handled in `MessageInput.svelte`.
    -   If the backend `send_private_message` call returns an error, log the error to the console. Displaying specific errors to the user is deferred.
-   **Backend:**
    -   The `send_private_message` function should handle potential errors from `make_rpc_call` (network, RPC, parse errors) and return them as `VerusRpcError`.
    -   Specific `z_sendmany` errors (e.g., insufficient funds after fee, invalid address) will be returned within the `VerusRpcError::Rpc` variant.

## 6. Open Questions / Future Considerations

-   **Persistence of Sent Messages:** How and where will sent messages be stored locally so they persist across application restarts? (Currently only optimistic).
-   **Sent Message Status Tracking:** How will the frontend use the returned `txid` to track the confirmation status of sent messages (e.g., update UI from "sent" to "confirmed")? This likely involves periodic polling or checking transaction details.
-   **Detailed Error Display:** Should specific `z_sendmany` errors (like insufficient funds) be shown to the user in the UI after a failed send attempt?
-   **Handling Large Memos:** Verify Verus memo size limits and ensure the hex encoding handles potential multi-byte characters correctly if needed (UTF-8 to hex is standard). The current frontend limit (412 chars) should be well within typical limits. 