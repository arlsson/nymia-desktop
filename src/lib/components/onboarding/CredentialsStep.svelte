<script lang="ts">
// Component: src/lib/components/onboarding/CredentialsStep.svelte
// Description: Handles RPC credential input, testing, and status display.

    import { createEventDispatcher } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { slide } from 'svelte/transition';
    
    // Import Shared Types
    import type { Credentials } from '$lib/types';

    // --- Type --- (Removed local definition)
    type ConnectionStatus = 'idle' | 'testing' | 'success' | 'error';

    // --- Props ---
    export let initialRpcUser: string = '';
    export let initialRpcPassword: string = '';

    // --- State ---
    let rpcUser = initialRpcUser;
    let rpcPassword = initialRpcPassword;
    let connectionStatus: ConnectionStatus = 'idle';
    let connectionError: string | null = null;
    let connectionBlockHeight: number | null = null;

    // --- Event Dispatcher ---
    const dispatch = createEventDispatcher<{
        testSuccess: { blockHeight: number; credentials: Credentials }; // Use imported type
        testError: { error: string };
        credentialsChanged: Credentials; // Use imported type
    }>();

    // Propagate input changes up to the parent immediately
    $: dispatch('credentialsChanged', { rpc_user: rpcUser, rpc_pass: rpcPassword });

    // --- Logic ---
    async function testConnection() {
        connectionStatus = 'testing';
        connectionError = null;
        connectionBlockHeight = null;

        try {
            console.log(`CredentialsStep: Testing connection with user: ${rpcUser}`);
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                rpcUser: rpcUser,
                rpcPass: rpcPassword,
            });
            connectionBlockHeight = blockHeightResult;
            connectionStatus = 'success';
            console.log('CredentialsStep: Connection test successful, block height:', blockHeightResult);
            
            // Save credentials automatically on successful test
            saveCredentials();

            // Dispatch success event AFTER saving
            dispatch('testSuccess', { 
                blockHeight: blockHeightResult, 
                credentials: { rpc_user: rpcUser, rpc_pass: rpcPassword } 
            });

        } catch (err: any) {
            connectionStatus = 'error';
            const errorMsg = String(err) || 'Failed to connect. Check credentials or daemon status.';
            connectionError = errorMsg;
            console.error('CredentialsStep: Connection test failed:', err);
            dispatch('testError', { error: errorMsg });
        }
    }

    async function saveCredentials() {
        try {
            await invoke('save_credentials', {
                rpcUser: rpcUser,
                rpcPass: rpcPassword
            });
            console.log('CredentialsStep: Credentials saved securely.');
        } catch (saveErr) {
            console.error('CredentialsStep: Failed to save credentials:', saveErr);
            // Update error state to inform user, but don't block flow
            connectionStatus = 'error'; // Or keep success but show a warning?
            connectionError = connectionError 
                ? connectionError + ' (Also failed to save credentials)' 
                : 'Connected, but failed to save credentials.';
            // Dispatch error again to update parent if desired
            dispatch('testError', { error: connectionError }); 
        }
    }
</script>

<!-- Credentials step content - Optimized for 700px height -->
<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-gray-800 mb-1">Enter Credentials</h1>
    <p class="text-gray-600 text-normal mb-3">Enter the RPC credentials for your Verus daemon.</p>
    
    <!-- Redesigned help link - more clearly external -->
    <a href="#" target="_blank" rel="noopener noreferrer" class="inline-flex items-center text-sm bg-gray-50 hover:bg-gray-100 text-blue-600 hover:text-blue-700 px-3 py-1.5 rounded-md mb-3 transition-colors duration-150 border border-gray-200">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>How to find these credentials</span>
        <!-- External link icon -->
        <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 ml-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
        </svg>
    </a>
    
    <div class="flex items-center py-1.5 px-3 rounded-md bg-gray-50 mb-4 border-l-3 border-green-600">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-600 mr-2 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
        <p class="text-xs text-gray-500">These credentials will be saved securely for next time.</p>
    </div>

    <form on:submit|preventDefault={testConnection} class="space-y-3">
        <div>
            <label for="cs-rpcUser" class="block text-sm font-medium text-gray-700 mb-1">RPC Username</label> <!-- Unique ID -->
            <input
                type="text"
                id="cs-rpcUser" 
                bind:value={rpcUser}
                required
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent sm:text-sm transition-all duration-150"
                placeholder="Your RPC username"
            />
        </div>
        <div class="mb-1">
            <label for="cs-rpcPassword" class="block text-sm font-medium text-gray-700 mb-1">RPC Password</label> <!-- Unique ID -->
            <input
                type="password"
                id="cs-rpcPassword"
                bind:value={rpcPassword}
                required
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent sm:text-sm transition-all duration-150"
                placeholder="Your RPC password"
            />
        </div>
        
        <!-- Redesigned test connection button - smaller and secondary -->
        <div class="flex justify-end">
            <button
                type="submit" 
                disabled={connectionStatus === 'testing' || !rpcUser || !rpcPassword}
                class="flex justify-center items-center py-1.5 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-600 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-60 disabled:cursor-not-allowed transition-all duration-150 ease-in-out"
            >
                {#if connectionStatus === 'testing'}
                    <svg class="animate-spin mr-1.5 h-3.5 w-3.5 text-gray-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Testing...
                {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                    Test Connection
                {/if}
            </button>
        </div>
    </form>

    <!-- Connection Status/Error - Compact design -->
    {#if connectionStatus === 'success' && connectionBlockHeight !== null}
        <div class="mt-3 bg-green-50 border border-green-100 rounded-md overflow-hidden" transition:slide={{ duration: 150 }}>
            <div class="bg-green-100 py-1.5 px-3">
                <div class="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-700 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <p class="text-sm font-medium text-green-800">Connection successful!</p>
                </div>
            </div>
            <div class="px-3 py-2 bg-white">
                <div class="flex items-baseline">
                    <span class="text-sm text-gray-500 mr-2">Current Block Height:</span>
                    <span class="text-lg font-semibold text-green-700">{connectionBlockHeight}</span>
                </div>
            </div>
        </div>
    {:else if connectionStatus === 'error' && connectionError}
        <div class="mt-3 bg-red-50 border border-red-100 rounded-md overflow-hidden" transition:slide={{ duration: 150 }}>
            <div class="bg-red-100 py-1.5 px-3">
                <div class="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-red-700 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                    <p class="text-sm font-medium text-red-800">Connection Failed</p>
                </div>
            </div>
            <div class="px-3 py-2 bg-white">
                <p class="text-xs text-red-700 line-clamp-2">{connectionError}</p>
            </div>
        </div>
    {/if}
</div>

<style>
    .step-content-area {
        /* Add any specific styles for this step if needed */
         width: 100%;
    }
    /* Add border-left utility if not default in Tailwind */
    .border-l-3 {
        border-left-width: 3px;
    }
</style> 