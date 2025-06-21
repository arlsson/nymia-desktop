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
// - Added video to the right panel filling the complete panel  
// - Video plays automatically on loop and is muted for autoplay compatibility
// - Removed hardcoded port fallback - ports must come from credentials or be undefined
// - MAJOR: Replaced manual blockchain + credentials steps with automatic BlockchainDetectionStep
// - Simplified onboarding to Welcome → Detection → VerusID (3 steps instead of 4)
// - Credential saving moved to login step for better separation of concerns
// - Fixed Continue button logic to only enable when Available blockchain is selected (not just Loading)

  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { slide } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  // Import Step Components
  import WelcomeStep from './onboarding/WelcomeStep.svelte';
  import BlockchainDetectionStep from './onboarding/BlockchainDetectionStep.svelte';
  import VerusIdStep from './onboarding/VerusIdStep.svelte';

  // Import Shared Types
  import type { 
      Credentials, 
      FormattedIdentity, 
      LoginPayload, 
      OnboardingStep 
  } from '$lib/types';

  // --- Types ---
  type Step = 'welcome' | 'blockchain' | 'verusid';

  // --- Props ---
  export let initialStep: OnboardingStep = 'welcome';
  export let initialCredentials: Credentials | null = null;

  // --- State ---
  let currentStep: OnboardingStep = initialStep;
  
  // Shared state between steps
  let selectedIdentity: FormattedIdentity | null = null; // Store the full selected identity
  let currentCredentials: Credentials | null = initialCredentials; // Track latest credentials from detection
  let connectionBlockHeight: number | null = null; // Updated by BlockchainDetectionStep
  let selectedBlockchainId: string | null = null; // Track selected blockchain
  let detectionCompleted = false; // Track if detection step is completed
  let availableBlockchainsCount = 0; // Track how many blockchains are available
  let blockchainSelected = false; // Track if an Available blockchain has been selected

  // --- Event Dispatcher (to parent: +page.svelte) ---
  const dispatch = createEventDispatcher<{
    'login-success': LoginPayload;
    'authentication-cleared': void;
  }>();

  // --- Lifecycle & State Management ---

  // Update internal state if initialCredentials prop changes (e.g., on logout/restart)
  $: if (initialCredentials && initialCredentials !== currentCredentials) {
      console.log("OnboardingFlow: initialCredentials prop changed, updating internal state. Port:", initialCredentials.rpc_port);
      currentCredentials = initialCredentials;
      // Also reset identity selection if creds change during onboarding
      selectedIdentity = null; 
  }
  
  // Reset relevant state when moving away from detection step
  $: if (currentStep !== 'blockchain') {
      // Keep the detection state for navigation logic but reset selection
      blockchainSelected = false;
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
    } else if (currentStep === 'blockchain' && detectionCompleted && currentCredentials) {
      goToStep('verusid');
    }
  }

  function prevStep() {
    if (currentStep === 'blockchain') {
      goToStep('welcome');
    } else if (currentStep === 'verusid') {
      // Reset VerusID selection when going back
      selectedIdentity = null;
      blockchainSelected = false; // Reset blockchain selection too
      selectedBlockchainId = null; // Reset blockchain visual selection
      goToStep('blockchain');
    }
  }

  // --- Event Handlers from Step Components ---
  function handleGetStarted() {
    goToStep('blockchain');
  }

  function handleBlockchainSelected(event: CustomEvent<{ 
    blockchainId: string; 
    credentials: Credentials; 
    blockHeight: number;
  }>) {
    console.log("OnboardingFlow: Blockchain selected event received");
    const { blockchainId, credentials, blockHeight } = event.detail;
    
    // Update shared state
    selectedBlockchainId = blockchainId;
    currentCredentials = credentials;
    connectionBlockHeight = blockHeight;
    detectionCompleted = true;
    blockchainSelected = true; // Mark that an Available blockchain has been selected
    
    console.log(`OnboardingFlow: Selected ${blockchainId} with block height ${blockHeight}`);
  }

  function handleDetectionCompleted(event: CustomEvent<{ availableCount: number }>) {
    console.log("OnboardingFlow: Detection completed event received");
    availableBlockchainsCount = event.detail.availableCount;
    detectionCompleted = true;
  }

  function handleIdSelected(event: CustomEvent<{ identity: FormattedIdentity | null }>) {
      console.log("OnboardingFlow: Received idSelected event with identity:", event.detail.identity);
      selectedIdentity = event.detail.identity; 
      // Parent component (this one) controls enabling the login button via `isPrimaryButtonDisabled`
  }

  async function handleLogin() {
      if (!selectedIdentity || !currentCredentials) {
          console.error("OnboardingFlow: Cannot login - missing selected ID or credentials.");
          return;
      }
      
      try {
          // Save credentials to store after successful identity selection
          await invoke('save_credentials', {
              rpcUser: currentCredentials.rpc_user,
              rpcPass: currentCredentials.rpc_pass,
              rpcPort: currentCredentials.rpc_port
          });
          
          console.log(`OnboardingFlow: Login initiated for ${selectedIdentity.formatted_name} (${selectedIdentity.i_address})`);
          dispatch('login-success', {
             identity: selectedIdentity,
             blockHeight: connectionBlockHeight,
             blockchainId: selectedBlockchainId
          });
      } catch (error) {
          console.error("OnboardingFlow: Failed to save credentials during login:", error);
          // Could show error to user, but for now we'll still proceed with login
          dispatch('login-success', {
             identity: selectedIdentity,
             blockHeight: connectionBlockHeight,
             blockchainId: selectedBlockchainId
          });
      }
  }

  // --- Dynamic Button Logic ---
  $: primaryButtonLabel = 
      currentStep === 'welcome' ? 'Get Started' :
      currentStep === 'blockchain' ? 'Continue' : 'Login';

  $: isPrimaryButtonDisabled = 
      currentStep === 'welcome' ? false :
      (currentStep === 'blockchain' && (!detectionCompleted || !blockchainSelected)) ||
      (currentStep === 'verusid' && !selectedIdentity); // Check the full object

  $: primaryButtonAction = 
      currentStep === 'welcome' ? handleGetStarted :
      currentStep === 'blockchain' ? nextStep : handleLogin;

</script>

<!-- Main container: Two-panel layout -->
<div class="flex h-screen font-sans">

  <!-- Left Panel: Onboarding Steps -->
  <div class="w-1/2 flex flex-col bg-dark-bg-primary">
  
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
                     <BlockchainDetectionStep 
                        bind:selectedBlockchainId={selectedBlockchainId}
                        on:blockchainSelected={handleBlockchainSelected}
                        on:detectionCompleted={handleDetectionCompleted}
                     />
                 </div>
              {:else if currentStep === 'verusid'}
                 <div transition:slide|local={{ duration: 300, easing: quintOut }}>
                     <VerusIdStep 
                        credentials={currentCredentials} 
                        on:idSelected={handleIdSelected}
                     />
                 </div>
              {/if}
          </div>
      </div>

      <!-- Bottom Button Bar -->
      <div class="px-10 py-4 border-t border-dark-border-primary bg-dark-bg-primary mt-auto">
          <div class="flex justify-end space-x-3">
               <!-- Back Button (Conditional) -->
              {#if currentStep !== 'welcome'}
                <button 
                    type="button"
                    on:click={prevStep} 
                    class="py-2 px-3 border border-dark-border-primary rounded-md shadow-sm text-xs font-medium text-dark-text-primary bg-dark-bg-secondary hover:bg-dark-bg-tertiary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand-green transition duration-150 ease-in-out"
                >
                     Back
                </button>
              {/if}

             <!-- Primary Action Button -->
              <button 
                type="button"
                on:click={primaryButtonAction} 
                disabled={isPrimaryButtonDisabled} 
                class={`py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand-green disabled:opacity-60 disabled:cursor-not-allowed transition duration-150 ease-in-out ${!isPrimaryButtonDisabled ? 'hover:bg-brand-green-hover' : ''}`}
                style={`background-color: ${isPrimaryButtonDisabled ? '#9fcfb8' : '#419A6A'};`} 
             >
                {primaryButtonLabel}
            </button>

          </div>
      </div>
  </div>

   <!-- Right Panel: Decorative Background -->
   <div class="w-1/2 relative overflow-hidden bg-[#419A6A]">
       <!-- Background elements... -->
        <div 
            class="absolute inset-0 opacity-15 mix-blend-soft-light bg-grid-pattern animate-pulse-slow"
            style="background-image: radial-gradient(#5ab88a 1px, transparent 1px); background-size: 25px 25px;"
        >
        </div>
     
        <!-- Onboarding video filling the panel -->
        <div class="absolute inset-0">
            <video 
                src="/onboarding-1.mp4" 
                autoplay 
                muted 
                loop 
                playsinline
                class="onboarding-video"
            >
                Your browser does not support the video tag.
            </video>
        </div>
         
        
   </div>
</div>

<style>
   /* Background animation keyframes */
  @keyframes pulse-slow {
    0%, 100% { opacity: 0.15; }
    50% { opacity: 0.3; }
  }
  
  .animate-pulse-slow {
    animation: pulse-slow 10s ease-in-out infinite;
  }
  
  @keyframes float-slow {
    0% { 
      transform: translate(0, 0) scale(1); 
    }
    25% { 
      transform: translate(var(--float-x, 40px), var(--float-y, 60px)) scale(1.08); 
    }
    50% { 
      transform: translate(var(--float-x2, -35px), var(--float-y2, 40px)) scale(1.04); 
    }
    75% { 
      transform: translate(var(--float-x3, -45px), var(--float-y3, -40px)) scale(0.96); 
    }
    100% { 
      transform: translate(0, 0) scale(1); 
    }
  }
  
  .animate-float-slow {
    animation-name: float-slow;
    animation-duration: var(--float-duration, 20s);
    animation-delay: var(--float-delay, 0s);
    animation-iteration-count: infinite;
    animation-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    will-change: transform;
  }
  
  .bg-gradient-radial {
    background-image: radial-gradient(circle, var(--tw-gradient-from) 0%, transparent 70%);
  }

  /* Onboarding video styling to fill the complete panel */
  .onboarding-video {
    width: 100%;
    height: 100%;
    object-fit: cover; /* Fill container, crop if needed to maintain aspect ratio */
    object-position: center center; /* Center the video */
  }

  /* Other styles */
  .step-container {
      width: 100%;
      /* max-width and margin removed, handled by parent div */
  }

</style>