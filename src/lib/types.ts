// File: src/lib/types.ts
// Description: Centralized type definitions for the application.

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
export type OnboardingStep = 'blockchain' | 'credentials' | 'verusid'; 