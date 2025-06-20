<script lang="ts">
// Component: src/lib/components/onboarding/CredentialsStep.svelte
// Description: Simplified blockchain connection step that automatically discovers and tests credentials with a single button.
// Changes: 
// - Removed manual credential entry form
// - Removed separate discovery UI section
// - Single "Test Connection" button that handles discovery + testing automatically
// - Streamlined UI to focus only on connection status
// - Added manual folder selection when automatic discovery fails

    import { createEventDispatcher } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { slide } from 'svelte/transition';
    
    // Import Shared Types
    import type { Credentials } from '$lib/types';

    // --- Types ---
    type ConnectionStatus = 'idle' | 'testing' | 'success' | 'error';
    
    interface DiscoveryResult {
        success: boolean;
        credentials: Credentials | null;
        config_path: string | null;
        error_message: string | null;
    }

    // --- Props ---
    export let initialRpcUser: string = '';
    export let initialRpcPassword: string = '';
    export let initialRpcPort: number = 27486; // Default Verus port
    export let selectedBlockchain: string = 'verus-testnet'; // Selected blockchain for discovery

    // --- State ---
    let connectionStatus: ConnectionStatus = 'idle';
    let connectionError: string | null = null;
    let connectionBlockHeight: number | null = null;
    let showManualSelection = false; // Show manual folder selection option
    
    // Internal credentials state (not exposed to user)
    let rpcUser = initialRpcUser;
    let rpcPassword = initialRpcPassword;
    let rpcPort = initialRpcPort;

    // --- Event Dispatcher ---
    const dispatch = createEventDispatcher<{
        testSuccess: { blockHeight: number; credentials: Credentials };
        testError: { error: string };
        credentialsChanged: Credentials;
    }>();

    // --- Logic ---
    async function testConnection() {
        connectionStatus = 'testing';
        connectionError = null;
        connectionBlockHeight = null;
        showManualSelection = false; // Hide manual selection during test

        try {
            // First, try to discover credentials automatically
            console.log(`CredentialsStep: Starting automatic discovery for blockchain: ${selectedBlockchain}`);
            const discoveryResult = await invoke<DiscoveryResult>('discover_blockchain_credentials', {
                blockchainId: selectedBlockchain
            });
            
            let credentialsToTest: Credentials;
            
            if (discoveryResult.success && discoveryResult.credentials) {
                console.log('CredentialsStep: Using discovered credentials');
                credentialsToTest = discoveryResult.credentials;
                // Update internal state
                rpcUser = discoveryResult.credentials.rpc_user;
                rpcPassword = discoveryResult.credentials.rpc_pass;
                rpcPort = discoveryResult.credentials.rpc_port;
            } else {
                console.log('CredentialsStep: Discovery failed, using provided credentials');
                // Fall back to any provided initial credentials
                credentialsToTest = { rpc_user: rpcUser, rpc_pass: rpcPassword, rpc_port: rpcPort };
            }

            // Test the connection with the credentials we have
            console.log(`CredentialsStep: Testing connection with user: ${credentialsToTest.rpc_user} port: ${credentialsToTest.rpc_port}`);
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                rpcUser: credentialsToTest.rpc_user,
                rpcPass: credentialsToTest.rpc_pass,
                rpcPort: credentialsToTest.rpc_port,
            });

            connectionBlockHeight = blockHeightResult;
            connectionStatus = 'success';
            console.log('CredentialsStep: Connection test successful, block height:', blockHeightResult);
            
            // Save credentials automatically on successful test
            await saveCredentials(credentialsToTest);

            // Notify parent that credentials have changed
            dispatch('credentialsChanged', credentialsToTest);

            // Dispatch success event AFTER saving
            dispatch('testSuccess', { 
                blockHeight: blockHeightResult, 
                credentials: credentialsToTest 
            });

        } catch (err: any) {
            connectionStatus = 'error';
            let errorMsg = String(err) || 'Failed to connect. Check that your blockchain daemon is running.';
            
            // If discovery failed and we don't have valid credentials, show manual selection option
            if (!rpcUser || !rpcPassword) {
                errorMsg = 'Could not find blockchain configuration automatically. You can locate it manually below.';
                showManualSelection = true;
            }
            
            connectionError = errorMsg;
            console.error('CredentialsStep: Connection test failed:', err);
            dispatch('testError', { error: errorMsg });
        }
    }

    async function selectConfigFolder() {
        try {
            // Use Tauri command to open folder selection dialog
            const selectedPath = await invoke<string | null>('select_folder_dialog', {
                title: 'Select blockchain configuration folder'
            });

            if (selectedPath) {
                console.log('CredentialsStep: User selected folder:', selectedPath);
                
                // Try to discover credentials from the selected folder
                connectionStatus = 'testing';
                connectionError = null;
                showManualSelection = false;

                const discoveryResult = await invoke<DiscoveryResult>('discover_blockchain_credentials_from_path', {
                    blockchainId: selectedBlockchain,
                    configPath: selectedPath
                });

                if (discoveryResult.success && discoveryResult.credentials) {
                    console.log('CredentialsStep: Found credentials in selected folder');
                    
                    // Update internal state
                    rpcUser = discoveryResult.credentials.rpc_user;
                    rpcPassword = discoveryResult.credentials.rpc_pass;
                    rpcPort = discoveryResult.credentials.rpc_port;

                    // Test the connection with discovered credentials
                    const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                        rpcUser: discoveryResult.credentials.rpc_user,
                        rpcPass: discoveryResult.credentials.rpc_pass,
                        rpcPort: discoveryResult.credentials.rpc_port,
                    });

                    connectionBlockHeight = blockHeightResult;
                    connectionStatus = 'success';
                    console.log('CredentialsStep: Connection test successful with manual path, block height:', blockHeightResult);
                    
                    // Save credentials automatically on successful test
                    await saveCredentials(discoveryResult.credentials);

                    // Notify parent that credentials have changed
                    dispatch('credentialsChanged', discoveryResult.credentials);

                    // Dispatch success event
                    dispatch('testSuccess', { 
                        blockHeight: blockHeightResult, 
                        credentials: discoveryResult.credentials 
                    });

                } else {
                    connectionStatus = 'error';
                    connectionError = 'No valid blockchain configuration found in the selected folder.';
                    showManualSelection = true;
                    dispatch('testError', { error: connectionError });
                }
            }
        } catch (err: any) {
            connectionStatus = 'error';
            connectionError = `Failed to read configuration from selected folder: ${String(err)}`;
            showManualSelection = true;
            console.error('CredentialsStep: Manual folder selection failed:', err);
            dispatch('testError', { error: connectionError });
        }
    }

    async function saveCredentials(credentials: Credentials) {
        try {
            await invoke('save_credentials', {
                rpcUser: credentials.rpc_user,
                rpcPass: credentials.rpc_pass,
                rpcPort: credentials.rpc_port
            });
            console.log('CredentialsStep: Credentials saved securely.');
        } catch (saveErr) {
            console.error('CredentialsStep: Failed to save credentials:', saveErr);
            // Update error state to inform user, but don't block flow
            connectionStatus = 'error';
            connectionError = connectionError 
                ? connectionError + ' (Also failed to save credentials)' 
                : 'Connected, but failed to save credentials.';
            // Dispatch error again to update parent if desired
            dispatch('testError', { error: connectionError }); 
        }
    }
</script>

<!-- Connect with Blockchain step content -->
<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-dark-text-primary mb-1">Connect with Blockchain</h1>
    <p class="text-dark-text-secondary text-normal mb-6">Connect to your blockchain daemon automatically.</p>
    
    <!-- Connection Test Section -->
    <div class="mb-6">
        <div class="flex justify-center">
            <button
                type="button"
                on:click={testConnection}
                disabled={connectionStatus === 'testing'}
                class="inline-flex items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-brand-green hover:bg-brand-green/80 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand-green disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150"
            >
                {#if connectionStatus === 'testing'}
                    <svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Testing Connection...
                {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                    Test Connection
                {/if}
            </button>
        </div>

        <!-- Manual Folder Selection (shown when auto-discovery fails) -->
        {#if showManualSelection}
            <div class="mt-4 p-4 bg-dark-bg-secondary border border-dark-border-secondary rounded-md" transition:slide={{ duration: 200 }}>
                <div class="text-center">
                    <h3 class="text-sm font-medium text-dark-text-primary mb-2">Locate Configuration Folder</h3>
                    <p class="text-xs text-dark-text-secondary mb-3">
                        Select the folder containing your blockchain daemon's configuration file (.conf)
                    </p>
                    <button
                        type="button"
                        on:click={selectConfigFolder}
                        disabled={connectionStatus === 'testing'}
                        class="inline-flex items-center px-4 py-2 border border-dark-border-primary rounded-md shadow-sm text-xs font-medium text-dark-text-primary bg-dark-bg-tertiary hover:bg-dark-bg-primary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand-green disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        </svg>
                        Browse for Folder
                    </button>
                </div>
            </div>
        {/if}

        <!-- Status Messages -->
        {#if connectionStatus === 'success' && connectionBlockHeight !== null}
            <div class="mt-4 bg-brand-green/20 border border-brand-green/30 rounded-md overflow-hidden" transition:slide={{ duration: 150 }}>
                <div class="bg-brand-green/30 py-2 px-4">
                    <div class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-green-300 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <p class="text-base font-medium text-green-200">Connection successful!</p>
                    </div>
                </div>
                <div class="px-4 py-3 bg-dark-bg-secondary">
                    <div class="flex items-baseline">
                        <span class="text-sm text-dark-text-secondary mr-2">Current Block Height:</span>
                        <span class="text-lg font-semibold text-green-300">{connectionBlockHeight}</span>
                    </div>
                </div>
            </div>
        {:else if connectionStatus === 'error' && connectionError}
            <div class="mt-4 bg-red-800/30 border border-red-600/40 rounded-md overflow-hidden" transition:slide={{ duration: 150 }}>
                <div class="bg-red-700/40 py-2 px-4">
                    <div class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-red-300 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13-732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                        </svg>
                        <p class="text-base font-medium text-red-200">Connection Failed</p>
                    </div>
                </div>
                <div class="px-4 py-3 bg-dark-bg-secondary">
                    <p class="text-sm text-red-300">{connectionError}</p>
                </div>
            </div>
        {/if}
    </div>

    <!-- Helper Text -->
    {#if connectionStatus === 'idle'}
        <div class="text-center">
            <p class="text-sm text-dark-text-secondary">
                We'll automatically discover and test your blockchain daemon configuration.
            </p>
        </div>
    {/if}
</div>

<style>
    .step-content-area {
        width: 100%;
    }
</style> 