<script lang="ts">
// File: src/routes/+page.svelte
// Description: Main application page. Handles auth state, VerusID selection, and auto-login.
// Changes:
// - Added VerusID fetching and selection logic.
// - Renamed logout button to "Cancel Authentication".
// - Added `id_selection` and `logged_in` states.

	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import AuthForm from '$lib/components/AuthForm.svelte';

    // Define types
    type AppState = 
        'loading' 
        | 'needs_auth' 
        | 'authenticating' 
        | 'auth_error' 
        | 'fetching_ids' 
        | 'id_selection' 
        | 'logging_in' 
        | 'logged_in';

    interface AuthPayload {
        rpc_user: string;
        rpc_pass: string;
        blockHeight?: number;
    }

    interface FormattedIdentity {
        formatted_name: string;
        i_address: string;
    }

    // State variables
	let appState: AppState = 'loading';
	let authErrorMessage: string | null = null;
    let currentBlockHeight: number | null = null;
    let blockCheckIntervalId: ReturnType<typeof setInterval> | null = null;
    let loginIdentities: FormattedIdentity[] = [];
    let selectedIdentityIAddress: string | null = null;
    let selectedIdentityFormattedName: string | null = null; 

    // --- Lifecycle Functions ---
	onMount(async () => {
		console.log('App mounted, checking for stored credentials...');
		try {
            const storedCreds = await invoke<AuthPayload>('load_credentials');
			console.log('Stored credentials found, attempting auto-login...');
			autoAuthenticate(storedCreds);
		} catch (error: any) {
             console.error("ERROR caught during load_credentials:", error);
             const errorStr = String(error);
             if (errorStr === 'NotFound' || errorStr.includes("not found")) {
                 console.log('No stored credentials found - normal first run or cleared credentials');
 			    appState = 'needs_auth';
             } else if (errorStr.includes("Failed to access store")) {
                  console.error('Store access error:', error);
                  authErrorMessage = "Could not access credential store.";
                  appState = 'auth_error';
             } else {
                  console.error('Unexpected error loading credentials:', error);
                  authErrorMessage = "An unexpected error occurred while loading credentials.";
                  appState = 'auth_error';
             }
		}
	});

    onDestroy(() => {
        stopBlockCheckTimer();
    });

    // --- Authentication and ID Selection Logic ---
    async function autoAuthenticate(credentials: AuthPayload) {
        appState = 'authenticating';
        authErrorMessage = null;
        currentBlockHeight = null;
        try {
            console.log(`Auto-authenticating with user: ${credentials.rpc_user}`);
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                rpcUser: credentials.rpc_user,
                rpcPass: credentials.rpc_pass,
            });
            console.log('Auto-authentication successful.');
            handleSuccessfulRpcAuth(blockHeightResult); // Proceed to fetch IDs
        } catch (error: any) {
            handleAuthFailure(error);
        }
    }

    function handleManualRpcAuth(event: CustomEvent<AuthPayload>) {
        console.log('Manual RPC authentication successful from AuthForm:', event.detail);
        handleSuccessfulRpcAuth(event.detail.blockHeight ?? null); // Proceed to fetch IDs
    }

    // Called after successful RPC auth (manual or auto)
    async function handleSuccessfulRpcAuth(blockHeight: number | null) {
        appState = 'fetching_ids';
        authErrorMessage = null;
        currentBlockHeight = blockHeight;
        startBlockCheckTimer(); // Start timer now

        try {
            console.log("Fetching login identities...");
            const ids = await invoke<FormattedIdentity[]>('get_formatted_login_identities');
            loginIdentities = ids;
            if (loginIdentities.length === 0) {
                authErrorMessage = "No VerusIDs with private addresses found in your wallet.";
                appState = 'auth_error'; // Or a different state like 'no_valid_ids'?
                stopBlockCheckTimer(); // Stop timer if no IDs found
            } else {
                console.log(`Found ${loginIdentities.length} login identities.`);
                appState = 'id_selection';
            }
        } catch (error: any) {
            console.error("Failed to fetch login identities:", error);
            authErrorMessage = `Error fetching identities: ${String(error)}`;
            appState = 'auth_error';
            stopBlockCheckTimer(); // Stop timer on error
        }
    }

    function handleIdSelectionLogin() {
        const selectedId = loginIdentities.find(id => id.i_address === selectedIdentityIAddress);
        if (selectedId) {
            console.log(`Logging in with VerusID: ${selectedId.formatted_name}`);
            selectedIdentityFormattedName = selectedId.formatted_name; 
            appState = 'logged_in'; 
            // Keep timer running in 'logged_in' state
        } else {
             console.error("Selected i-address not found in list?"); // Should not happen
             authErrorMessage = "An error occurred during ID selection.";
             appState = 'id_selection'; // Stay on selection screen
        }
    }

    function handleAuthFailure(error: any) {
        console.error('Authentication failed:', error);
        const errorStr = String(error);
         if (errorStr.includes("Authentication failed")) {
             authErrorMessage = "Stored credentials invalid. Please log in again.";
             clearStoredCredentials(); 
         } else {
             authErrorMessage = errorStr || "Login failed. Please try again.";
         }
         appState = 'needs_auth';
         stopBlockCheckTimer();
    }

    async function handleCancelAuthentication() {
        await clearStoredCredentials(); 
        appState = 'needs_auth'; 
        authErrorMessage = null;
        currentBlockHeight = null;
        loginIdentities = [];
        selectedIdentityIAddress = null;
        selectedIdentityFormattedName = null;
        stopBlockCheckTimer();
    }

    async function clearStoredCredentials() {
        try {
            await invoke('clear_credentials');
            console.log("Stored credentials cleared.");
        } catch (clearError) {
            console.error("Failed to clear stored credentials:", clearError);
        }
    }

    // --- Periodic Block Check Logic ---
    function startBlockCheckTimer() {
        stopBlockCheckTimer(); // Clear any existing timer first
        console.log("Starting periodic block height check (every 30s).");
        performBlockCheck(); // Perform initial check immediately
        blockCheckIntervalId = setInterval(performBlockCheck, 30000); // 30 seconds
    }

    function stopBlockCheckTimer() {
        if (blockCheckIntervalId) {
            console.log("Stopping periodic block height check.");
            clearInterval(blockCheckIntervalId);
            blockCheckIntervalId = null;
        }
    }

    async function performBlockCheck() {
        // Only run if we are authenticated or selecting ID
        if (!['id_selection', 'logged_in'].includes(appState)) return;

        console.log("Performing periodic block height check...");
        try {
            const creds = await invoke<AuthPayload>('load_credentials');
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                 rpcUser: creds.rpc_user,
                 rpcPass: creds.rpc_pass,
            });
            currentBlockHeight = blockHeightResult;
            console.log(`Block height updated: ${currentBlockHeight}`);
        } catch (error) {
            console.error("Periodic block check failed:", error);
            authErrorMessage = `Connection lost: ${String(error)}. Please log in again.`;
            appState = 'auth_error'; // Set error state
            stopBlockCheckTimer(); // Stop checking
        }
    }

</script>

<svelte:head>
	<title>Chat-dApp - {appState}</title>
</svelte:head>

<div class="app-container p-4">
    <!-- Loading State -->
    {#if appState === 'loading'}
        <div class="flex items-center justify-center h-screen">
            <div class="text-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-teal-500 mx-auto mb-4"></div>
                <p class="text-gray-600">Loading application...</p>
            </div>
        </div>

    <!-- RPC Auth / ID Fetching / Error States -->
    {:else if ['needs_auth', 'authenticating', 'fetching_ids', 'auth_error'].includes(appState)}
        <div class="flex flex-col items-center justify-center min-h-screen">
             {#if appState === 'authenticating'}
                <div class="bg-blue-100 text-blue-700 p-4 mb-4 rounded text-center w-full max-w-md">
                    <p>Authenticating with stored credentials...</p>
                </div>
             {:else if appState === 'fetching_ids'}
                <div class="bg-blue-100 text-blue-700 p-4 mb-4 rounded text-center w-full max-w-md">
                    <p>Fetching VerusIDs...</p>
                 </div>
             {/if}

             {#if authErrorMessage}
                <div class="p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400 text-center w-full max-w-md" role="alert">
                    <span class="font-medium">Error:</span> {authErrorMessage}
                </div>
             {/if}

             {#if appState === 'needs_auth' || appState === 'auth_error'}
                 <AuthForm on:authenticated={handleManualRpcAuth} />
             {/if}
        </div>

    <!-- VerusID Selection State -->
     {:else if appState === 'id_selection'}
         <div class="flex flex-col items-center justify-center min-h-screen">
             <div class="w-full max-w-md p-8 space-y-6 bg-white rounded-lg shadow-md">
                <h2 class="text-2xl font-semibold text-center text-gray-700">Select VerusID</h2>
                
                 {#if currentBlockHeight !== null}
                    <p class="text-sm text-gray-600 text-center">
                        Block Height: <span class="font-medium text-teal-600">{currentBlockHeight}</span>
                    </p>
                {/if}

                 <div class="space-y-4">
                    <div>
                         <label for="verusIdSelect" class="block text-sm font-medium text-gray-600 mb-1">Select Identity</label>
                         <select 
                            id="verusIdSelect" 
                            bind:value={selectedIdentityIAddress} 
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-teal-500 focus:border-teal-500 sm:text-sm rounded-md border"
                         >
                             <option value={null} disabled selected>-- Please choose an ID --</option>
                             {#each loginIdentities as identity (identity.i_address)}
                                <option value={identity.i_address}>{identity.formatted_name}</option>
                             {/each}
                         </select>
                    </div>

                    <button
                        type="button"
                        on:click={handleIdSelectionLogin}
                        disabled={!selectedIdentityIAddress}
                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-teal-600 hover:bg-teal-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        Login
                    </button>

                    <button 
                        type="button"
                        on:click={handleCancelAuthentication} 
                        class="w-full mt-2 flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                         Cancel Authentication
                    </button>
                 </div>
             </div>
        </div>

    <!-- Logged In State -->
    {:else if appState === 'logged_in'}
        <div class="p-8 text-center">
            <h1 class="text-2xl font-bold text-green-700 mb-2">Logged In!</h1>
             {#if currentBlockHeight !== null}
                <p class="text-sm text-gray-600 mb-4">
                    Block Height: <span class="font-medium text-teal-600">{currentBlockHeight}</span>
                </p>
            {/if}
            <p class="mb-6 text-gray-600">Logged in as: <span class="font-semibold">{selectedIdentityFormattedName}</span></p>
             <button 
                on:click={handleCancelAuthentication} 
                class="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
             >
                Log Out
             </button>
        </div>
    {/if}
</div>
