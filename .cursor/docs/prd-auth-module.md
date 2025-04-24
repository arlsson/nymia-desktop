# Product Requirements Document (PRD)
## Chat-dApp - Verus Daemon Authentication Module

### Overview
This document outlines the requirements for the authentication module of a chat-dApp built on the Verus blockchain. This module will handle secure authentication with a locally running Verus daemon (verusd) and verify connection success through basic blockchain data retrieval.

### Technology Stack
- **Frontend**: Svelte with Tailwind CSS
- **Backend/Desktop Integration**: Tauri
- **Blockchain Interaction**: RPC calls to local Verus daemon

### Scope
This PRD covers only the authentication functionality and connection verification, which will serve as the foundation for the broader chat-dApp.

### Key Requirements

#### 1. Daemon Authentication
- **1.1** The application must provide a form to input Verus daemon RPC credentials (rpcuser and rpcpassword).
- **1.2** The application must store credentials securely between sessions.
- **1.3** The application must connect to the Verus daemon on the specified port (18843).
- **1.4** The application must verify successful connection by retrieving the current block height.
- **1.5** The application must handle authentication errors with appropriate user feedback.

#### 2. Connection Verification
- **2.1** Upon submission of credentials, the application must make an RPC call to `getblockcount` to verify connection.
- **2.2** The application must display the retrieved block height as confirmation of successful connection.
- **2.3** On successful connection, the application must store credentials securely for future sessions.
- **2.4** The application should verify connection status by checking the block height approximately every 30 seconds.

#### 3. Security Requirements
- **3.1** RPC credentials must be encrypted when stored.
- **3.2** RPC credentials must never be exposed in logs or error messages.
- **3.3** All network traffic must be confined to localhost.

### User Interface Requirements

#### Authentication Screen
- Clean, minimal interface with no specific branding
- Form with labeled inputs for RPC username and password
- "Connect" button to initiate authentication
- Visual indication of connection status (connecting, connected, error)
- Display area for block height upon successful connection

### User Flows

#### First-Time Authentication
1. User opens application
2. User is presented with authentication form
3. User enters RPC username and password
4. User clicks "Connect" button
5. Application attempts to connect to daemon with provided credentials
6. Application retrieves block height to verify connection
7. On success, the application displays the current block height and stores credentials
8. On failure, the application displays an appropriate error message

#### Returning User
1. User opens application
2. Application automatically attempts to connect using stored credentials
3. On success, user proceeds to the main application (not in scope of this PRD)
4. On failure, user is presented with the authentication form to re-enter credentials

### Technical Implementation Notes

- RPC endpoint should be configured as `http://localhost:18843` for the VRSCTEST network
- Use Tauri's secure storage API for storing encrypted credentials
- Implement RPC calls as asynchronous operations with appropriate error handling
- Include timeout handling for daemon connection attempts (recommend 5-second timeout)
- Create a service layer to abstract RPC communication from the UI components

### Success Criteria
- Users can successfully authenticate with the Verus daemon using RPC credentials
- The application correctly retrieves and displays the block height after successful authentication
- Credentials are securely stored and reused in subsequent sessions
- Users receive clear feedback on connection status and any errors

### Not In Scope
- The chat functionality of the application
- Network selection or configuration
- Any wallet-related functionality
- Any other RPC commands beyond `getblockcount`
