<script lang="ts">
// Component: src/lib/components/onboarding/VerusIdStep.svelte
// Description: Handles fetching and selecting a VerusID.
// Changes:
// - Modified 'idSelected' event to dispatch the full FormattedIdentity object.
// - Removed "Clear Authentication" buttons as they're no longer needed with automatic blockchain detection.
// - Improved error handling with user-friendly messages for credential and connection issues
// - Added specific error message for "Credentials not found" to guide user back to blockchain step

    import { createEventDispatcher, onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import CustomDropdown from '../CustomDropdown.svelte';

    // Import Shared Types
    import type { Credentials, FormattedIdentity, DropdownOption } from '$lib/types';

    // --- Types --- (Removed local definitions)
    type FetchStatus = 'idle' | 'fetching' | 'success' | 'error';

    // --- Props ---
    export let credentials: Credentials | null = null; // Needed to fetch IDs

    // --- State ---
    let loginIdentities: FormattedIdentity[] = [];
    let loginIdentityOptions: DropdownOption[] = []; 
    let selectedIdentityIAddress: string | null = null; // Keep this for dropdown binding
    let fetchStatus: FetchStatus = 'idle';
    let fetchError: string | null = null;

    // --- Event Dispatcher ---
    const dispatch = createEventDispatcher<{
        idSelected: { identity: FormattedIdentity | null }; // Changed event payload
    }>();

    // --- Lifecycle ---
    onMount(() => {
        // Fetch identities immediately when component mounts if credentials provided
        if (credentials) {
            fetchIdentities();
        } else {
            fetchStatus = 'error';
            fetchError = 'Credentials not provided. Cannot fetch identities.';
        }
    });

    // --- Logic ---
    async function fetchIdentities() {
        if (!credentials) { // Double check credentials exist
             console.error("VerusIdStep: Credentials missing, cannot fetch.");
             fetchStatus = 'error';
             fetchError = 'Internal Error: Credentials missing.';
             return;
        }

        fetchStatus = 'fetching';
        fetchError = null;
        loginIdentities = [];
        loginIdentityOptions = [];
        selectedIdentityIAddress = null;
        // Dispatch null initially or when re-fetching
        dispatch('idSelected', { identity: null }); 

        try {
            console.log("VerusIdStep: Fetching login identities...");
            // Note: We use the credentials passed via prop, not internal state
            // TODO: Replace 'get_formatted_login_identities' with the actual correct Tauri command name if different
            const ids = await invoke<FormattedIdentity[]>('get_login_identities'); 
            loginIdentities = ids;
            loginIdentityOptions = ids.map(id => ({ 
                id: id.i_address, 
                name: id.formatted_name, 
                enabled: true 
            }));
            fetchStatus = 'success';
            if (ids.length === 0) {
                fetchError = "No VerusIDs with private addresses found in your wallet.";
                dispatch('idSelected', { identity: null }); // Ensure parent knows none are selected
            }
        } catch (error: any) {
            console.error("VerusIdStep: Failed to fetch login identities:", error);
            fetchStatus = 'error';
            
            // Better error message handling with user-friendly messages
            let errorMessage = 'Unknown error occurred';
            if (error && typeof error === 'object') {
                if (error.message) {
                    errorMessage = error.message;
                } else if (error.error) {
                    errorMessage = error.error;
                } else {
                    errorMessage = JSON.stringify(error);
                }
            } else {
                errorMessage = String(error);
            }
            
            // Check for specific credential-related errors and provide better messaging
            if (errorMessage.includes('Credentials not found') || errorMessage.includes('NotFound')) {
                fetchError = 'Connection to blockchain lost. Please go back and reconnect to your blockchain.';
            } else if (errorMessage.includes('No VerusIDs with private addresses found')) {
                fetchError = 'No VerusIDs with private addresses found in your wallet.';
            } else {
                fetchError = `Unable to load identities: ${errorMessage}`;
            }
            
            dispatch('idSelected', { identity: null }); // Ensure parent knows none are selected on error
        }
    }

    function handleIdSelection(event: CustomEvent<string | number | null>) {
        const newIAddress = typeof event.detail === 'string' ? event.detail : null;
        selectedIdentityIAddress = newIAddress; // Keep local state for dropdown sync
        
        // Find the full identity object based on the selected iAddress
        const selectedFullIdentity = newIAddress
            ? loginIdentities.find(id => id.i_address === newIAddress) || null
            : null;

        console.log("VerusIdStep: Dispatching idSelected with:", selectedFullIdentity);
        // Dispatch the full identity object (or null if none selected)
        dispatch('idSelected', { identity: selectedFullIdentity }); 
    }

</script>

<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-dark-text-primary mb-2 select-none cursor-default">Select VerusID</h1>
    <p class="text-dark-text-secondary text-normal mb-1 select-none cursor-default">Choose the VerusID you want to log in with.</p>
    
    <div class="flex items-center bg-blue-900/30 border-blue-700/50 rounded-md px-3 py-2 mb-6 border-l-2">
        <svg class="w-4 h-4 text-blue-400 mr-2 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
        </svg>
        <span class="text-xs text-blue-300 select-none cursor-default">Only identities with a private address work with Nymia</span>
    </div>

    {#if fetchStatus === 'fetching'}
        <div class="flex items-center justify-center text-dark-text-secondary space-x-2 py-4">
            <svg class="animate-spin h-5 w-5 text-dark-text-secondary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>Loading Identities...</span>
        </div>
    {:else if fetchStatus === 'error' && fetchError}
        <div class="mt-4 p-3 bg-red-900/40 border border-red-700/50 rounded-md text-center">
            <p class="text-sm font-medium text-red-300 select-none cursor-default">Error Loading Identities</p>
            <p class="text-xs text-red-400 select-none cursor-default">{fetchError}</p>
            {#if fetchError.includes('Connection to blockchain lost')}
                <button
                    type="button"
                    on:click={() => {
                        // Retry fetching identities
                        if (credentials) {
                            fetchIdentities();
                        }
                    }}
                    class="mt-3 inline-flex items-center px-3 py-1 border border-red-600/50 rounded text-xs font-medium text-red-200 bg-red-800/30 hover:bg-red-700/40 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-colors duration-150"
                >
                    <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                    </svg>
                    Retry
                </button>
            {/if}
        </div>
    {:else if fetchStatus === 'success' && loginIdentities.length > 0}
        <CustomDropdown
            label="Select Identity"
            options={loginIdentityOptions}
            bind:selectedId={selectedIdentityIAddress}
            placeholder="-- Please choose an ID --"
            on:change={handleIdSelection} 
        />
    {:else if fetchStatus === 'success' && loginIdentities.length === 0}
        <div class="mt-4 p-3 bg-yellow-900/40 border border-yellow-700/50 rounded-md text-center">
            <p class="text-sm font-medium text-yellow-300 select-none cursor-default">No Login IDs Found</p>
            <p class="text-xs text-yellow-400 select-none cursor-default">No VerusIDs with private addresses were found in your wallet.</p>
        </div>
    {/if}
</div>

<style>
 .step-content-area {
        /* Add any specific styles for this step if needed */
         width: 100%;
    }
</style> 