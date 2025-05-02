<script lang="ts">
// Component: src/lib/components/onboarding/VerusIdStep.svelte
// Description: Handles fetching and selecting a VerusID.
// Changes:
// - Modified 'idSelected' event to dispatch the full FormattedIdentity object.

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
        clearAuthentication: void; // Request parent to clear auth
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
            fetchError = `Error fetching identities: ${String(error)}`;
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

    function requestClearAuthentication() {
        // Ask the parent component (OnboardingFlow) to handle clearing
        dispatch('clearAuthentication');
    }

</script>

<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-gray-800 mb-2">Select VerusID</h1>
    <p class="text-gray-600 text-normal mb-1">Choose the VerusID you want to log in with.</p>
    <p class="text-xs text-gray-500 mb-6">(Only identities with a private address work with Nymia)</p>

    {#if fetchStatus === 'fetching'}
        <div class="flex items-center justify-center text-gray-500 space-x-2 py-4">
            <svg class="animate-spin h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>Loading Identities...</span>
        </div>
    {:else if fetchStatus === 'error' && fetchError}
        <div class="mt-4 p-3 bg-red-100 border border-red-300 rounded-md text-center">
            <p class="text-sm font-medium text-red-800">Error Loading Identities</p>
            <p class="text-xs text-red-700 mb-2">{fetchError}</p>
            <!-- Show Clear Auth button even on error -->
            <button 
                type="button"
                on:click={requestClearAuthentication} 
                class="w-full mt-2 flex justify-center py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out"
            >
                 Clear Authentication & Start Over
            </button>
        </div>
    {:else if fetchStatus === 'success' && loginIdentities.length > 0}
        <CustomDropdown
            label="Select Identity"
            options={loginIdentityOptions}
            bind:selectedId={selectedIdentityIAddress}
            placeholder="-- Please choose an ID --"
            on:change={handleIdSelection} 
        />
         <!-- Clear Auth button shown separately in parent's button bar now -->
    {:else if fetchStatus === 'success' && loginIdentities.length === 0}
        <div class="mt-4 p-3 bg-yellow-100 border border-yellow-300 rounded-md text-center">
            <p class="text-sm font-medium text-yellow-800">No Login IDs Found</p>
            <p class="text-xs text-yellow-700">No VerusIDs with private addresses were found in your wallet.</p>
        </div>
        <!-- Show Clear Auth button if no IDs -->
        <button 
            type="button"
            on:click={requestClearAuthentication} 
            class="w-full mt-4 flex justify-center py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out"
        >
            Clear Authentication & Start Over
        </button>
    {/if}
</div>

<style>
 .step-content-area {
        /* Add any specific styles for this step if needed */
         width: 100%;
    }
</style> 