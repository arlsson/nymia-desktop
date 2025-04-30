# Product Requirements Document: Initial User Flow Redesign

## 1. Introduction

This document outlines the requirements for redesigning the initial user onboarding and authentication flow for the Chat DApp. The goal is to create a more intuitive, visually appealing, and informative experience, inspired by modern application design principles.

## 2. Goals

*   Improve the user experience for first-time and returning users during the initial setup.
*   Provide clear guidance on selecting the correct blockchain context.
*   Streamline the RPC credential entry process with better explanations and validation.
*   Clarify the VerusID selection step.
*   Establish a modern visual identity using a defined color palette and layout structure (left panel focus during onboarding).
*   Ensure the flow clearly communicates the saving of credentials for future use and provides a clear way to reset.

## 3. User Flow & Navigation

### 3.1 Onboarding Structure
*   The onboarding process consists of up to three sequential steps presented within a dedicated left-side panel.
*   Only the *current* step is visible to the user at any time to reduce cognitive load.
*   Users must be able to navigate back to previous steps (e.g., via a back button or indicator) if they need to make changes.
*   This left/right panel layout is *exclusively* for the onboarding flow.

### 3.2 Startup Logic
*   On application start, check if credentials for "Verus Testnet" are stored.
    *   **If Yes:** Skip Steps 1 & 2 and proceed directly to Step 3 (VerusID Selection).
    *   **If No:** Start at Step 1 (Blockchain Selection).

### 3.3 Onboarding Steps

1.  **Step 1: Blockchain Selection** (Entry point if no credentials stored)
    *   **UI:** Display a dropdown menu listing available blockchains (Verus Testnet, Verus, vDEX, vARRR, CHIPS).
    *   **Functionality:** Only "Verus Testnet" is selectable. Other options are visually disabled within the dropdown.
    *   **Action:** User selects "Verus Testnet" and clicks "Next" (or similar) to proceed to Step 2.

2.  **Step 2: Credential Entry**
    *   **UI:**
        *   Input fields for RPC Username and RPC Password.
        *   Explanatory text below the title or near inputs:
            *   Link ("How to find these?") to `#` (placeholder).
            *   Info text: "These credentials will be saved securely so you don't have to enter them again. You can clear them later."
        *   "Test Connection" button.
        *   Area to display connection status (e.g., "Connection successful! Block Height: XXXXXX" or "Error: Invalid credentials").
        *   "Continue" button (disabled initially).
        *   Back button/link to return to Step 1.
    *   **Functionality:**
        *   User enters credentials.
        *   User clicks "Test Connection". The app calls `connect_verus_daemon`.
        *   On success: Display block height, enable the "Continue" button.
        *   On failure: Display a clear error message (e.g., "Invalid credentials", "Daemon unreachable").
        *   User clicks "Continue" (only possible after successful test) to proceed to Step 3.

3.  **Step 3: VerusID Selection** (Entry point if credentials *were* stored)
    *   **UI:**
        *   Dropdown menu populated with available VerusIDs (formatted `Name@`) fetched via `get_formatted_login_identities`.
        *   "Login" button (disabled until an ID is selected).
        *   "Clear Authentication" button (always visible in this step).
        *   Back button/link to return to Step 2 (only if user arrived from Step 2).
    *   **Functionality:**
        *   App automatically fetches identities upon entering this step.
        *   User selects a VerusID from the dropdown, enabling the "Login" button.
        *   User clicks "Login": Proceed to the Post-Login Application View (Section 3.4).
        *   User clicks "Clear Authentication":
            *   Call `clear_credentials`.
            *   Navigate the user back to Step 1 (Blockchain Selection).

### 3.4 Post-Login Application View
*   Upon successful VerusID login, the left/right panel onboarding UI is completely replaced.
*   The application transitions to a full-window view.
*   **Initial Placeholder:** This view will initially be very basic, displaying:
    *   Selected VerusID Name (`Name@`)
    *   Selected VerusIDs PrivateAddress
    *   Current Block Height (which should continue to update periodically as currently implemented).
    *   A "Log Out" button (Returns to onboarding Step 3).

## 4. UI & Design Requirements

*   **Layout:** Onboarding: Fixed left panel for steps, right panel for decoration. Post-Login: Full window.
*   **Primary Color:** Use `#419A6A` for primary actions (buttons like "Test Connection", "Continue", "Login"), highlights, and potentially active state indicators.
*   **Right Panel Background:** Use a subtle, animated blurry/gradient effect with `#3165D4` as the dominant color.
*   **Error Handling:** Display errors inline within the current step, close to the element that caused the error (e.g., below the credential inputs if the test fails).
*   **Modern Aesthetics:** Ensure input fields, buttons, dropdowns, and text follow modern UI/UX practices, drawing inspiration from the reference image where appropriate (cleanliness, spacing, typography).

## 5. Non-Goals

*   Implementing logic for blockchains other than "Verus Testnet".
*   Building out the *full* main application features beyond the placeholder Post-Login view.
*   Complex credential management (e.g., editing stored credentials).
*   Advanced error recovery scenarios beyond displaying messages.

## 6. Future Considerations

*   Implementation of other blockchain connections.
*   More robust post-login "Log Out" functionality.
*   Handling edge cases like wallets with no VerusIDs or network interruptions during onboarding.
*   Accessibility improvements.

