<script lang="ts">
// File: src/routes/+page.svelte
// Description: Main application page. Handles overall state (loading, onboarding, logged in) 
//              and orchestrates the display of OnboardingFlow or the ChatInterface.
// Changes:
// - Replaced LoggedInView with ChatInterface component.
// - Updated imports and component usage in the template.
// - Ensured logout event from ChatInterface is handled.
// - Added fetching and passing of private balance.
// - Added dynamic currency symbol support by tracking selected blockchain ID and passing to ChatInterface.
// - Modified to start at blockchain detection step (instead of VerusID step) when stored credentials are found.
// - Fixed first launch to start at welcome step instead of blockchain detection step.

	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
    import OnboardingFlow from '$lib/components/OnboardingFlow.svelte';
    // import LoggedInView from '$lib/components/LoggedInView.svelte'; // Removed
    import ChatInterface from '$lib/components/ChatInterface.svelte'; // Added
    import type { Credentials, FormattedIdentity, LoginPayload, PrivateBalance } from '$lib/types';

    // --- Types --- 
    type AppStatus = 'loading' | 'onboarding' | 'loggedIn' | 'error';

    // Removed local type definitions, assuming they exist in $lib/types
    // interface Credentials { ... }
    // interface FormattedIdentity { ... }
    // interface LoginPayload { ... }

    // --- State Variables --- 
	let appStatus: AppStatus = 'loading';
	let startupError: string | null = null;
    let storedCredentials: Credentials | null = null;
    let initialOnboardingStep: 'welcome' | 'blockchain' | 'verusid' = 'welcome';

    let loggedInIdentity: FormattedIdentity | null = null;
    let currentBlockHeight: number | null = null;
    let currentPrivateBalance: PrivateBalance = null;
    let selectedBlockchainId: string | null = null; // Track selected blockchain for currency symbol
    let blockCheckIntervalId: ReturnType<typeof setInterval> | null = null;

    // --- Lifecycle --- 
	onMount(async () => {
		console.log('App mounted, checking for stored credentials...');
		try {
            storedCredentials = await invoke<Credentials>('load_credentials');
			console.log('Stored credentials found.');
            // If creds found, we start onboarding at the blockchain detection step
            initialOnboardingStep = 'blockchain';
            appStatus = 'onboarding'; // Move to onboarding state
            // Start timer only if we potentially skip straight to ID selection or login
            // Note: OnboardingFlow also tries to fetch IDs which implicitly needs creds
            startBlockCheckTimer(); 
		} catch (error: any) {
            const errorStr = String(error);
            if (errorStr === 'NotFound' || errorStr.includes("not found") || errorStr.includes("Deserialization")) {
                console.log('No stored credentials found or credentials incompatible (missing rpc_port field), starting onboarding from beginning.');
                // Clear any incompatible stored credentials
                try {
                    await invoke('clear_credentials');
                    console.log('Cleared incompatible stored credentials.');
                } catch (clearError) {
                    console.warn('Failed to clear stored credentials:', clearError);
                }
                storedCredentials = null;
                initialOnboardingStep = 'welcome';
                appStatus = 'onboarding'; // Normal start
            } else {
                console.error('Unexpected error loading credentials:', error);
                startupError = `Failed to load credentials: ${errorStr}`;
                appStatus = 'error';
            }
		}
	});

    onDestroy(() => {
        stopBlockCheckTimer();
    });

    // --- Event Handlers from Components ---
    async function handleLoginSuccess(event: CustomEvent<LoginPayload>) {
        console.log("Login successful from OnboardingFlow:", event.detail);
        loggedInIdentity = event.detail.identity;
        currentBlockHeight = event.detail.blockHeight;
        selectedBlockchainId = event.detail.blockchainId; // Capture selected blockchain
        appStatus = 'loggedIn';
        currentPrivateBalance = null; // Reset balance before fetching

        // Fetch balance immediately after login
        if (loggedInIdentity?.private_address) {
            try {
                currentPrivateBalance = await invoke<number>('get_private_balance', {
                     address: loggedInIdentity.private_address 
                });
                console.log("Fetched private balance successfully:", currentPrivateBalance);
            } catch (balanceError) {
                 console.error("Failed to fetch initial private balance:", balanceError);
                 currentPrivateBalance = null; // Keep it null on error
                 // TODO: Show non-critical error to user?
            }
        } else {
            console.warn("Logged in, but no private address found for identity:", loggedInIdentity?.formatted_name);
            currentPrivateBalance = null; // Ensure balance is null if no address
        }
        
        // Start periodic check (or ensure it continues if already started by onMount)
        startBlockCheckTimer(); 
    }

    function handleAuthenticationCleared() {
        console.log("Authentication cleared event received from OnboardingFlow.");
        storedCredentials = null;
        loggedInIdentity = null; // Ensure identity is cleared
        currentPrivateBalance = null; // Clear balance
        stopBlockCheckTimer();
        appStatus = 'onboarding';
    }

    async function handleLogout() {
        console.log("Logout event received from ChatInterface.");
        console.log("Logout: storedCredentials before logout:", storedCredentials?.rpc_port);
        
        // Reload credentials from store to ensure we have the latest ones
        try {
            storedCredentials = await invoke<Credentials>('load_credentials');
            console.log("Logout: Refreshed credentials from store:", storedCredentials?.rpc_port);
            // If we have stored credentials, skip to VerusID step, otherwise start from welcome
            initialOnboardingStep = 'verusid'; 
        } catch (error) {
            console.warn("Logout: Could not reload credentials:", error);
            // If no stored credentials, start from welcome
            storedCredentials = null;
            initialOnboardingStep = 'welcome';
        }
        
        loggedInIdentity = null;
        currentPrivateBalance = null; // Clear balance on logout
        selectedBlockchainId = null; // Clear blockchain selection on logout
        appStatus = 'onboarding';
        // Restart timer as we are back in an authenticated state (RPC creds still valid)
        startBlockCheckTimer();
    }

    // --- Periodic Block & Balance Check Logic ---
    function startBlockCheckTimer() {
        // Check if we have credentials or are logged in (which implies credentials were valid)
        if (!storedCredentials && appStatus !== 'loggedIn') {
            console.log("Not starting periodic check: No credentials and not logged in.");
            return; 
        }
        
        stopBlockCheckTimer(); // Clear any existing timer first
        console.log("Starting periodic block/balance check (every 15s).");
        performChecks(); // Perform initial check immediately
        blockCheckIntervalId = setInterval(performChecks, 15000); // Check every 15 seconds
    }

    function stopBlockCheckTimer() {
        if (blockCheckIntervalId) {
            console.log("Stopping periodic block/balance check.");
            clearInterval(blockCheckIntervalId);
            blockCheckIntervalId = null;
        }
    }

    async function performChecks() {
         // Only run if onboarding (and likely at VerusID step) or logged in
        if (appStatus !== 'onboarding' && appStatus !== 'loggedIn') {
            console.log("Skipping periodic check: App status is", appStatus);
            stopBlockCheckTimer(); // Stop if state changed unexpectedly
            return;
        }
        
        // We need credentials to check
        let credsToCheck = storedCredentials;
        if (!credsToCheck) {
            try {
                 credsToCheck = await invoke<Credentials>('load_credentials');
            } catch {
                console.warn("Periodic check: Credentials not available, stopping checks.");
                stopBlockCheckTimer();
                return;
            }
        }

        console.log("Performing periodic block & balance checks...");
        try {
            // Fetch block height
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                 rpcUser: credsToCheck.rpc_user,
                 rpcPass: credsToCheck.rpc_pass,
                 rpcPort: credsToCheck.rpc_port,
            });
            currentBlockHeight = blockHeightResult;
            console.log(`Block height updated: ${currentBlockHeight}`);

            // Fetch balance only if logged in and private address exists
            if (appStatus === 'loggedIn' && loggedInIdentity?.private_address) {
                 try {
                    const balanceResult = await invoke<number>('get_private_balance', {
                         address: loggedInIdentity.private_address 
                    });
                    currentPrivateBalance = balanceResult;
                    console.log(`Private balance updated: ${currentPrivateBalance}`);
                 } catch (balanceError) {
                    console.error("Periodic balance check failed:", balanceError);
                    // Don't stop timer, maybe it recovers, but maybe set balance to null?
                    // currentPrivateBalance = null;
                 }
            }
        } catch (error) { // If core check (getblockcount) fails, stop the timer
            console.error("Periodic block check failed, stopping timer:", error);
            stopBlockCheckTimer();
            // Optionally set blockHeight/balance to null?
            // currentBlockHeight = null;
            // currentPrivateBalance = null;
        }
    }

</script>

<svelte:head>
	<title>Nymia - {appStatus}</title>
</svelte:head>

<div class="app-container">
    <!-- Loading State -->
    {#if appStatus === 'loading'}
        <div class="flex items-center justify-center h-screen">
            <div class="text-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-brand-green mx-auto mb-4"></div>
                <p class="text-dark-text-secondary">Loading application...</p>
            </div>
        </div>

    <!-- Startup Error State -->
    {:else if appStatus === 'error'}
         <div class="flex items-center justify-center h-screen bg-red-900">
            <div class="text-center p-8 border border-red-500 rounded bg-dark-bg-secondary shadow-lg max-w-lg">
                <h1 class="text-2xl font-bold text-red-400 mb-4">Application Error</h1>
                <p class="text-red-300">Could not start the application due to an error:</p>
                <p class="mt-2 p-2 bg-red-800 text-red-200 rounded font-mono text-sm">{startupError || 'An unknown error occurred.'}</p>
                <p class="mt-4 text-sm text-dark-text-secondary">Please check your setup or try restarting the application.</p>
                
                <!-- Add a button to clear stored data and restart -->
                <button 
                    class="mt-4 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded font-medium text-sm transition-colors"
                    on:click={async () => {
                        try {
                            await invoke('clear_credentials');
                            console.log('Cleared stored credentials from error state.');
                            // Reload the page to restart the app
                            location.reload();
                        } catch (e) {
                            console.error('Failed to clear credentials:', e);
                        }
                    }}
                >
                    Clear Stored Data & Restart
                </button>
            </div>
        </div>

    <!-- Onboarding State -->
    {:else if appStatus === 'onboarding'}
        <OnboardingFlow 
            initialStep={initialOnboardingStep}
            initialCredentials={storedCredentials} 
            on:login-success={handleLoginSuccess}
            on:authentication-cleared={handleAuthenticationCleared}
        />

    <!-- Logged In State -->
    {:else if appStatus === 'loggedIn' && loggedInIdentity}
        <!-- Replaced LoggedInView with ChatInterface -->
        <ChatInterface 
            loggedInIdentity={loggedInIdentity}
            bind:blockHeight={currentBlockHeight}
            privateBalance={currentPrivateBalance}
            blockchainId={selectedBlockchainId}
            on:logout={handleLogout} 
        />
    
    <!-- Fallback/Unexpected State -->
     {:else}
        <div class="flex items-center justify-center h-screen">
            <p class="text-red-400">Unexpected application state. Please restart.</p>
        </div>
    {/if}
</div>

<style>
    /* Keep global styles minimal, rely on Tailwind and component styles */
    .app-container {
        min-height: 100vh; /* Ensure container takes full height */
    }
</style>
