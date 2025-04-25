# Product Requirements Document (PRD)
## Chat-dApp - VerusID Login Module

### Overview
This document outlines the requirements for the VerusID login module of the chat-dApp. This module enables users to select a VerusID with a private address from their wallet after successful RPC authentication.

### Scope
This PRD covers only the VerusID selection functionality, which follows the RPC authentication module and precedes the main chat functionality.

### Key Requirements

#### 1. VerusID Retrieval
- **1.1** The application must retrieve all identities from the Verus daemon using the `listidentities` RPC call.
- **1.2** The application must filter the retrieved identities to show only those with a non-empty `privateaddress` field.

#### 2. VerusID Selection
- **2.1** The application must present filtered VerusIDs in a dropdown menu.
- **2.2** The application must display VerusID names with proper formatting:
  - Regular IDs (where parent equals systemid): "name@"
  - Sub-IDs (where parent differs from systemid): "name.parentname@"
    - Parent name must be retrieved using `getidentity [parent_id]` RPC call when needed.
- **2.3** The application must allow users to select a VerusID from the dropdown.
- **2.4** Upon selection, the application must pass the formatted VerusID name to the next screen.

#### 3. Error Handling
- **3.1** If no VerusIDs with private addresses are found, the dropdown should be empty.
- **3.2** The application must handle RPC errors gracefully with appropriate user feedback.

### User Interface Requirements

#### VerusID Selection Screen
- Clean, minimal interface
- Dropdown menu showing formatted VerusID names
- "Login" button to proceed with selected VerusID
- Visual indication if RPC call is in progress

### User Flow

1. User successfully authenticates with Verus daemon
2. Application retrieves identities via `listidentities` RPC call
3. Application filters identities to those with private addresses
4. Application formats VerusID names correctly
5. User selects a VerusID from the dropdown
6. User clicks "Login" button
7. Application proceeds to next screen with the selected VerusID name

### Technical Implementation Notes

- Use the same RPC service layer created for authentication
- Process identities on the backend to avoid exposing sensitive data to the frontend

### Success Criteria
- Application correctly retrieves and filters VerusIDs with private addresses
- VerusID names are displayed with proper formatting
- Selected VerusID name is successfully passed to the next screen

### Not In Scope
- Storage of selected VerusID for future sessions
- Additional validation of VerusIDs beyond checking for private addresses
- The chat functionality of the application
