// File: src/lib/types.ts
// Description: Centralized type definitions for the application.
// Changes:
// - Added ChatMessage type for imported history.
// - Added optional 'status' field to ChatMessage.
// - Added Conversation type, including recipient_private_address.

// Credentials for Verus RPC connection
export interface Credentials {
    rpc_user: string;
    rpc_pass: string;
}

// Structure for Verus identity details returned from backend
export interface FormattedIdentity {
    formatted_name: string;
    i_address: string;
    private_address: string | null; 
}

// Generic structure for dropdown options
export interface DropdownOption {
    id: string | number | null;
    name: string;
    enabled: boolean;
}

// Payload for successful login event
export interface LoginPayload {
    identity: FormattedIdentity;
    blockHeight: number | null;
}

// Potential application states (can be expanded)
export type AppStatus = 'loading' | 'onboarding' | 'loggedIn' | 'error';

// Onboarding step names
export type OnboardingStep = 'welcome' | 'blockchain' | 'credentials' | 'verusid'; 

// Type alias for the private balance
export type PrivateBalance = number | null;

// Structure for chat messages (especially imported ones)
export interface ChatMessage {
    id: string; // txid or generated ID for sent messages
    sender: string | 'self'; // VerusID of the sender (@ format) or 'self'
    text: string;
    timestamp: number; // Placeholder for now, investigate source (blocktime?)
    amount: number;
    confirmations: number;
    direction: 'received' | 'sent'; // Initially only 'received' from history
    status?: 'sent' | 'delivered' | 'failed'; // Optional delivery status for sent messages
}

// Structure for conversation entries in the list
export type Conversation = {
    id: string;         // Unique ID, typically the recipient's VerusID name (e.g., user@)
    name: string;       // Display name (VerusID name)
    recipient_private_address: string; // The recipient's z-address needed for sending
    unread?: boolean;   // Optional flag for unread messages
  }; 