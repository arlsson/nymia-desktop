<script lang="ts">
// File: src/routes/+page.svelte
// Description: Main application page. Handles overall state (loading, onboarding, logged in) 
//              and orchestrates the display of OnboardingFlow or LoggedInView.
// Changes:
// - Complete refactor to use OnboardingFlow and LoggedInView components.
// - Manages application state: loading, onboarding, loggedIn, error.
// - Handles initial credential loading and determines starting state.
// - Listens for events from child components (login-success, authentication-cleared, logout).
// - Continues managing the periodic block check timer.

	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
    import OnboardingFlow from '$lib/components/OnboardingFlow.svelte';
    import LoggedInView from '$lib/components/LoggedInView.svelte';

    // --- Types --- 
    type AppStatus = 'loading' | 'onboarding' | 'loggedIn' | 'error';

    interface Credentials {
        rpc_user: string;
        rpc_pass: string;
    }

     interface FormattedIdentity {
        formatted_name: string;
        i_address: string;
        private_address: string | null; 
    }

    interface LoginPayload {
        identity: FormattedIdentity;
        blockHeight: number | null;
    }

    // --- State Variables --- 
	let appStatus: AppStatus = 'loading';
	let startupError: string | null = null;
    let storedCredentials: Credentials | null = null;
    let initialOnboardingStep: 'blockchain' | 'verusid' = 'blockchain';

    let loggedInIdentity: FormattedIdentity | null = null;
    let currentBlockHeight: number | null = null;
    let blockCheckIntervalId: ReturnType<typeof setInterval> | null = null;

    // --- Lifecycle --- 
	onMount(async () => {
		console.log('App mounted, checking for stored credentials...');
		try {
            storedCredentials = await invoke<Credentials>('load_credentials');
			console.log('Stored credentials found.');
            // If creds found, we start onboarding at the ID selection step
            initialOnboardingStep = 'verusid';
            appStatus = 'onboarding'; // Move to onboarding state
            // Start timer only if we potentially skip straight to ID selection or login
            // Note: OnboardingFlow also tries to fetch IDs which implicitly needs creds
            startBlockCheckTimer(); 
		} catch (error: any) {
            const errorStr = String(error);
            if (errorStr === 'NotFound' || errorStr.includes("not found")) {
                console.log('No stored credentials found, starting onboarding at step 1.');
                initialOnboardingStep = 'blockchain';
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
    function handleLoginSuccess(event: CustomEvent<LoginPayload>) {
        console.log("Login successful from OnboardingFlow:", event.detail);
        loggedInIdentity = event.detail.identity;
        currentBlockHeight = event.detail.blockHeight; // Use blockheight from onboarding test
        appStatus = 'loggedIn';
        // Keep block timer running
    }

    function handleAuthenticationCleared() {
        console.log("Authentication cleared event received from OnboardingFlow.");
        storedCredentials = null;
        // The OnboardingFlow component itself resets its internal state and goes to step 1.
        // We just need to ensure the timer stops if it was running.
        stopBlockCheckTimer();
        // Ensure app state is onboarding (it should be already, but just in case)
        appStatus = 'onboarding';
    }

    function handleLogout() {
        console.log("Logout event received from LoggedInView.");
        loggedInIdentity = null;
        // Per PRD, logout returns to Step 3 (VerusID selection) of onboarding
        initialOnboardingStep = 'verusid'; 
        appStatus = 'onboarding';
        // Restart timer as we are back in an authenticated state (RPC creds still valid)
        startBlockCheckTimer();
    }

    // --- Periodic Block Check Logic (Remains similar) ---
    function startBlockCheckTimer() {
        // Only start if we have credentials (either stored or just entered)
        if (!storedCredentials && !loggedInIdentity) return; 
        
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
         // Only run if onboarding (and likely at VerusID step) or logged in
        if (appStatus !== 'onboarding' && appStatus !== 'loggedIn') return;
        
        // We need credentials to check
        let credsToCheck = storedCredentials;
        if (!credsToCheck) {
            try {
                // If not in memory maybe they were just saved? Try loading.
                 credsToCheck = await invoke<Credentials>('load_credentials');
            } catch {
                console.warn("Periodic check: Credentials not available, skipping check.");
                // Might indicate user cleared creds, timer should have stopped but double check
                stopBlockCheckTimer();
                return;
            }
        }

        console.log("Performing periodic block height check...");
        try {
            const blockHeightResult = await invoke<number>('connect_verus_daemon', {
                 rpcUser: credsToCheck.rpc_user,
                 rpcPass: credsToCheck.rpc_pass,
            });
            currentBlockHeight = blockHeightResult;
            console.log(`Block height updated: ${currentBlockHeight}`);
        } catch (error) { // If the check fails, log error, but don't necessarily force logout
            console.error("Periodic block check failed:", error);
            // Potentially show a non-blocking connection status indicator?
            // For now, just log it. User actions will trigger re-auth if needed.
            // Stop the timer to prevent repeated errors if daemon is down.
            stopBlockCheckTimer();
            // Optionally set blockHeight to null? 
            // currentBlockHeight = null;
        }
    }

</script>

<svelte:head>
	<title>Nymia - {appStatus}</title>
</svelte:head>

<div class="app-container">
    <!-- Loading State -->
    {#if appStatus === 'loading'}
        <div class="flex items-center justify-center h-screen bg-gray-100">
            <div class="text-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-teal-500 mx-auto mb-4"></div>
                <p class="text-gray-600">Loading application...</p>
            </div>
        </div>

    <!-- Startup Error State -->
    {:else if appStatus === 'error'}
         <div class="flex items-center justify-center h-screen bg-red-50">
            <div class="text-center p-8 border border-red-300 rounded bg-white shadow-lg">
                <h1 class="text-2xl font-bold text-red-700 mb-4">Application Error</h1>
                <p class="text-red-600">Could not start the application due to an error:</p>
                <p class="mt-2 p-2 bg-red-100 text-red-800 rounded font-mono text-sm">{startupError || 'An unknown error occurred.'}</p>
                 <p class="mt-4 text-sm text-gray-600">Please check your setup or try restarting the application.</p>
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
        <LoggedInView 
            identityName={loggedInIdentity.formatted_name}
            identityIAddress={loggedInIdentity.i_address}
            identityPrivateAddress={loggedInIdentity.private_address}
            bind:blockHeight={currentBlockHeight} 
            on:logout={handleLogout}
        />
    
    <!-- Fallback/Unexpected State -->
     {:else}
        <div class="flex items-center justify-center h-screen bg-gray-100">
            <p class="text-red-500">Unexpected application state. Please restart.</p>
        </div>
    {/if}
</div>

<style>
    /* Keep global styles minimal, rely on Tailwind and component styles */
    .app-container {
        min-height: 100vh; /* Ensure container takes full height */
    }
</style>
