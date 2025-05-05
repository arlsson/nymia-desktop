# PRD: UTXO Pending Change Indicator & Temporary Send Lock

**Version:** 1.0
**Date:** 2024-07-29

**1. Introduction & Goals**

*   **Problem:** Users sending messages/gifts on the UTXO-based Verus blockchain experience a temporary decrease in their displayed balance due to how change outputs are handled. The delay waiting for block confirmation (average 1 min, up to 10+ min) can cause confusion about the available balance and potentially lead to failed subsequent transactions if initiated before the change UTXO is confirmed and usable.
*   **Goals:**
    *   Clearly communicate to the user when their displayed balance is temporarily affected by pending change from a recent transaction.
    *   Prevent users from initiating new send transactions while a previous one is awaiting confirmation, avoiding potential UTXO availability issues ("double spending" unconfirmed change).
    *   Improve user confidence and provide clarity on balance fluctuations inherent to the UTXO model.

**2. Proposed Features**

*   **F1: Pending Balance Indicator:**
    *   When a user successfully sends a message/gift, display a visual indicator next to the "Private Balance" in the `TopBar`.
    *   This indicator signals that the displayed balance might not yet reflect incoming change from the last transaction and that a confirmation is pending.
*   **F2: Temporary Send Disable:**
    *   After a user successfully sends a message/gift, temporarily disable the main "Send" button in the `MessageInput`.
    *   This lock persists until the application detects that a new block has arrived.

**3. Functional Requirements**

*   **FR1: Triggering the Pending State:**
    *   The pending state (Indicator visible, Send button disabled) must activate immediately after the `send_private_message` command successfully returns a transaction ID (`txid`).
*   **FR2: Displaying the Indicator (F1):**
    *   A small spinner icon (e.g., ⏳ or a subtle animated spinner) shall appear directly next to the Private Balance value in the `TopBar`.
    *   The balance value itself should remain visible.
*   **FR3: Disabling Send Functionality (F2):**
    *   Only the final "Send" button in the `MessageInput` component shall be disabled.
    *   The message text area and gift amount input should remain active, allowing the user to prepare the next message.
    *   When the Send button is disabled due to a pending transaction, hovering over it shall display a tooltip explaining the reason (e.g., "Waiting for previous transaction to confirm").
*   **FR4: Clearing the Pending State:**
    *   The pending state (spinner icon removed, Send button re-enabled) must be cleared when the application detects an increase in the `blockHeight`.
    *   This requires the application to periodically check the *current* `blockHeight` and compare it against the height recorded (`pendingSinceBlock`) when the pending state was initiated (as per FR1).
    *   **Crucially, the pending state is cleared ONLY when `currentBlockHeight > pendingSinceBlock`. This handles scenarios where multiple blocks arrive between checks.**
*   **FR5: State Persistence & Concurrency:**
    *   The pending state does not need to persist across application restarts. If the app is closed and reopened while a transaction was pending, the indicator and lock will be lost, which is acceptable for this version.
    *   The system will assume only one pending transaction needs tracking at a time. If a send is initiated, the lock engages until the next block, regardless of whether the underlying wallet could technically handle more UTXOs.

**4. Non-Functional Requirements**

*   **Performance:** The `blockHeight` check should be implemented efficiently (e.g., as part of an existing polling mechanism if available, or with a reasonable interval like every 10-15 seconds) to avoid excessive RPC calls.
*   **UI:** The spinner icon and disabled button state should be clear but visually unobtrusive, integrating smoothly with the existing UI.

**5. Out of Scope (Future Considerations)**

*   Tracking specific transaction confirmations (`gettransaction` RPC).
*   Persisting the pending state across restarts.
*   Handling multiple concurrent pending transactions.
*   More sophisticated UTXO management/display. 