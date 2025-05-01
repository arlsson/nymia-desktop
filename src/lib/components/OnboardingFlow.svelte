<script lang="ts">
// Component: src/lib/components/OnboardingFlow.svelte
// Description: Orchestrates the multi-step user onboarding process.
// Manages current step, shared state (credentials), renders step components,
// controls navigation buttons, and communicates with the parent page.
// Changes:
// - Added new welcome step as the first step in the flow
// - Updated step navigation to include the welcome step
// - Modified button logic to handle welcome step
// - Added WelcomeStep component import and rendering

  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { slide } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  // Import Step Components
  import WelcomeStep from './onboarding/WelcomeStep.svelte';
  import BlockchainStep from './onboarding/BlockchainStep.svelte';
  import CredentialsStep from './onboarding/CredentialsStep.svelte';
  import VerusIdStep from './onboarding/VerusIdStep.svelte';

  // Import Shared Types
  import type { 
      Credentials, 
      FormattedIdentity, 
      LoginPayload, 
      DropdownOption, 
      OnboardingStep 
  } from '$lib/types';

  // --- Types ---
  type Step = 'welcome' | 'blockchain' | 'credentials' | 'verusid';

  // --- Props ---
  export let initialStep: OnboardingStep = 'welcome';
  export let initialCredentials: Credentials | null = null;

  // --- State ---
  let currentStep: OnboardingStep = initialStep;
  
  // Shared state between steps
  let rpcUser = initialCredentials?.rpc_user || '';
  let rpcPassword = initialCredentials?.rpc_pass || '';
  let connectionBlockHeight: number | null = null; // Updated by CredentialsStep
  let selectedIdentity: FormattedIdentity | null = null; // Store the full selected identity
  let currentCredentialsFromChild: Credentials | null = initialCredentials; // Track latest from CredentialsStep
  let connectionTestedSuccessfully = false; // Track if connection test passed in CredentialsStep

  // Blockchain Step specific state
  let selectedBlockchain = 'verus-testnet'; // Only option for now
  const blockchainOptions: DropdownOption[] = [
    { id: 'verus-testnet', name: 'Verus Testnet', enabled: true },
    { id: 'verus', name: 'Verus', enabled: false },
    { id: 'vdex', name: 'vDEX', enabled: false },
    { id: 'varrr', name: 'vARRR', enabled: false },
    { id: 'chips', name: 'CHIPS', enabled: false },
  ];

  // --- Event Dispatcher (to parent: +page.svelte) ---
  const dispatch = createEventDispatcher<{
    'login-success': LoginPayload;
    'authentication-cleared': void;
  }>();

  // --- Lifecycle & State Management ---
  // Update internal credentials if initialCredentials prop changes (e.g., on logout/restart)
  $: if (initialCredentials && (initialCredentials.rpc_user !== rpcUser || initialCredentials.rpc_pass !== rpcPassword)) {
      console.log("OnboardingFlow: initialCredentials prop changed, updating internal state.");
      rpcUser = initialCredentials.rpc_user;
      rpcPassword = initialCredentials.rpc_pass;
      currentCredentialsFromChild = initialCredentials;
      // Also reset identity selection if creds change during onboarding
      selectedIdentity = null; 
  }
  
  // Reset connection success flag when moving away from credentials step
  $: if (currentStep !== 'credentials') {
      connectionTestedSuccessfully = false;
  }

  // --- Step Navigation ---
  function goToStep(step: OnboardingStep) {
    currentStep = step;
    // Reset relevant state when changing steps
    if (step !== 'verusid') {
      selectedIdentity = null;
    }
  }

  function nextStep() {
    if (currentStep === 'welcome') {
      goToStep('blockchain');
    } else if (currentStep === 'blockchain') {
      goToStep('credentials');
    } else if (currentStep === 'credentials' && connectionTestedSuccessfully) {
      goToStep('verusid');
    }
  }

  function prevStep() {
    if (currentStep === 'blockchain') {
      goToStep('welcome');
    } else if (currentStep === 'credentials') {
      goToStep('blockchain');
    } else if (currentStep === 'verusid') {
      // Reset VerusID selection when going back
      selectedIdentity = null;
      goToStep('credentials');
    }
  }

  // --- Event Handlers from Step Components ---
  function handleGetStarted() {
    goToStep('blockchain');
  }

  function handleBlockchainChange(event: CustomEvent<string | number | null>) {
      // Update local state, parent component handles enabling/disabling next button
      const value = event.detail;
      if (typeof value === 'string') { // Ensure it's a string for blockchain ID
        selectedBlockchain = value;
      }
  }

  function handleCredentialsChanged(event: CustomEvent<Credentials>) {
      // Keep parent state synced with CredentialsStep inputs
      currentCredentialsFromChild = event.detail;
      // Reset test success flag if credentials change
      connectionTestedSuccessfully = false; 
  }

  function handleTestSuccess(event: CustomEvent<{ blockHeight: number; credentials: Credentials }>) {
      console.log("OnboardingFlow: Received testSuccess event");
      connectionBlockHeight = event.detail.blockHeight;
      // Update shared credentials state with the ones that passed the test
      rpcUser = event.detail.credentials.rpc_user;
      rpcPassword = event.detail.credentials.rpc_pass;
      currentCredentialsFromChild = event.detail.credentials;
      connectionTestedSuccessfully = true; // Enable moving to next step
  }

  function handleTestError(event: CustomEvent<{ error: string }>) {
      console.log("OnboardingFlow: Received testError event");
      connectionBlockHeight = null;
      connectionTestedSuccessfully = false; // Disable moving to next step
  }

  function handleIdSelected(event: CustomEvent<{ identity: FormattedIdentity | null }>) {
      console.log("OnboardingFlow: Received idSelected event with identity:", event.detail.identity);
      selectedIdentity = event.detail.identity; 
      // Parent component (this one) controls enabling the login button via `isPrimaryButtonDisabled`
  }

  async function handleClearAuthenticationRequest() {
      console.log("OnboardingFlow: Received clearAuthentication request, handling...");
      try {
          await invoke('clear_credentials');
          console.log("OnboardingFlow: Stored credentials cleared via Tauri.");
          // Reset local state
          rpcUser = '';
          rpcPassword = '';
          connectionBlockHeight = null;
          selectedIdentity = null; 
          currentCredentialsFromChild = null;
          connectionTestedSuccessfully = false;
          // Dispatch event up to +page.svelte
          dispatch('authentication-cleared');
          // Go back to the first step
          goToStep('welcome'); 
      } catch (clearError) {
          console.error("OnboardingFlow: Failed to clear stored credentials:", clearError);
          // TODO: Show error to the user within the VerusIdStep? Or here?
          // For now, just log it and stay on the current step.
      }
  }

  async function handleLogin() {
      if (!selectedIdentity || !currentCredentialsFromChild) {
          console.error("OnboardingFlow: Cannot login - missing selected ID or credentials.");
          return;
      }
      
      console.log(`OnboardingFlow: Login initiated for ${selectedIdentity.formatted_name} (${selectedIdentity.i_address})`);
      dispatch('login-success', {
         identity: selectedIdentity,
         blockHeight: connectionBlockHeight 
      });
  }

  // --- Dynamic Button Logic (Remains Here) ---
  $: primaryButtonLabel = 
      currentStep === 'welcome' ? 'Get Started' :
      currentStep === 'blockchain' ? 'Next' : 
      currentStep === 'credentials' ? 'Continue' : 'Login';

  $: isPrimaryButtonDisabled = 
      currentStep === 'welcome' ? false :
      (currentStep === 'blockchain' && selectedBlockchain !== 'verus-testnet') ||
      (currentStep === 'credentials' && !connectionTestedSuccessfully) || // Use the flag
      (currentStep === 'verusid' && !selectedIdentity); // Check the full object

  $: primaryButtonAction = 
      currentStep === 'welcome' ? handleGetStarted :
      currentStep === 'blockchain' ? nextStep :
      currentStep === 'credentials' ? nextStep : handleLogin;

</script>

<!-- Main container: Two-panel layout -->
<div class="flex h-screen font-sans">

  <!-- Left Panel: Onboarding Steps (Light Mode) -->
  <div class="w-1/2 flex flex-col bg-gray-50">
  
      <!-- Top padding -->
      <div class="pt-12 px-10"></div>

      <!-- Main Content Area (scrollable if needed) -->
      <div class="flex-grow px-10 pt-8 overflow-y-auto">
          <div class="step-container mx-auto" style="max-width: 450px;">
              {#if currentStep === 'welcome'}
                 <div transition:slide|local={{ duration: 300, easing: quintOut }}>
                     <WelcomeStep 
                        on:getStarted={handleGetStarted}
                     />
                 </div>
              {:else if currentStep === 'blockchain'}
                 <div transition:slide|local={{ duration: 300, easing: quintOut }}>
                     <BlockchainStep 
                        options={blockchainOptions} 
                        bind:selectedId={selectedBlockchain}
                        on:change={handleBlockchainChange}
                     />
                 </div>
              {:else if currentStep === 'credentials'}
                 <div transition:slide|local={{ duration: 300, easing: quintOut }}>
                     <CredentialsStep 
                        initialRpcUser={rpcUser} 
                        initialRpcPassword={rpcPassword} 
                        on:testSuccess={handleTestSuccess}
                        on:testError={handleTestError}
                        on:credentialsChanged={handleCredentialsChanged}
                     />
                 </div>
              {:else if currentStep === 'verusid'}
                 <div transition:slide|local={{ duration: 300, easing: quintOut }}>
                     <VerusIdStep 
                        credentials={currentCredentialsFromChild} 
                        on:idSelected={handleIdSelected}
                        on:clearAuthentication={handleClearAuthenticationRequest}
                     />
                 </div>
              {/if}
          </div>
      </div>

      <!-- Bottom Button Bar (Remains Here) -->
      <div class="px-10 py-4 border-t border-gray-200 bg-gray-50 mt-auto">
          <div class="flex justify-end space-x-3">
               <!-- Back Button (Conditional) -->
              {#if currentStep !== 'welcome'}
                <button 
                    type="button"
                    on:click={prevStep} 
                    class="py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out"
                >
                     Back
                </button>
              {/if}

              <!-- Clear Authentication Button (Only on VerusID step, before Login) -->
             {#if currentStep === 'verusid'}
                 <button 
                    type="button"
                    on:click={handleClearAuthenticationRequest} 
                    class="py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out"
                >
                     Clear Authentication
                </button>
            {/if}

             <!-- Primary Action Button -->
              <button 
                type="button"
                on:click={primaryButtonAction} 
                disabled={isPrimaryButtonDisabled} 
                class={`py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-60 disabled:cursor-not-allowed transition duration-150 ease-in-out ${!isPrimaryButtonDisabled ? 'hover:bg-green-700' : ''}`}
                style={`background-color: ${isPrimaryButtonDisabled ? '#9fcfb8' : '#419A6A'};`} 
             >
                {primaryButtonLabel}
            </button>

          </div>
      </div>
  </div>

   <!-- Right Panel: Decorative Background -->
   <div class="w-1/2 relative overflow-hidden bg-gradient-to-br from-blue-900 via-blue-700 to-indigo-900">
       <!-- Background elements... -->
        <div 
            class="absolute inset-0 opacity-30 mix-blend-soft-light bg-grid-pattern animate-pulse-slow"
            style="background-image: radial-gradient(#3165D4 1px, transparent 1px); background-size: 20px 20px;"
        >
        </div>
        <div 
            class="absolute -bottom-1/4 -left-1/4 w-1/2 h-1/2 bg-gradient-radial from-blue-500 to-transparent opacity-20 rounded-full filter blur-3xl animate-float-slow"
            style="--float-delay: 0s; --float-duration: 15s; background-color: #3165D4;"
        ></div>
         <div 
            class="absolute -top-1/4 -right-1/4 w-2/3 h-2/3 bg-gradient-radial from-purple-500 to-transparent opacity-15 rounded-full filter blur-3xl animate-float-slow"
            style="--float-delay: 5s; --float-duration: 20s;"
        ></div>
        <div 
            class="absolute bottom-1/4 right-0 w-1/3 h-1/3 bg-gradient-radial from-teal-400 to-transparent opacity-25 rounded-full filter blur-2xl animate-float-slow"
            style="--float-delay: 10s; --float-duration: 18s;"
        ></div>
   </div>
</div>

<style>
   /* Background animation keyframes */
  @keyframes pulse-slow { /* ... */ }
  .animate-pulse-slow { /* ... */ }
  @keyframes float-slow { /* ... */ }
  .animate-float-slow { /* ... */ }
  .bg-gradient-radial { /* ... */ }

  /* Other styles */
  .step-container {
      width: 100%;
      /* max-width and margin removed, handled by parent div */
  }

  /* Input styles - removed as inputs are in CredentialsStep.svelte now */

</style>