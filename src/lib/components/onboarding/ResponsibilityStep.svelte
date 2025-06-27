<script lang="ts">
// Component: src/lib/components/onboarding/ResponsibilityStep.svelte
// Description: Responsibility disclosure step for the onboarding flow.
// Explains that Nymia is a tool, not a service, and emphasizes user responsibility.
// Features consistent styling with other onboarding components and smooth animations.
// Changes:
// - Created new component to display responsibility message between welcome and blockchain detection steps
// - Uses consistent typography and spacing patterns from existing onboarding components
// - Implements smooth entrance animations using Svelte transitions

import { createEventDispatcher, onMount } from 'svelte';
import { fade, fly } from 'svelte/transition';
import { quintOut } from 'svelte/easing';

// --- Event Dispatcher ---
// Dispatches understood event when user clicks the "I Understand" button
const dispatch = createEventDispatcher<{ 
  understood: void;
}>();

// Control element visibility for animations
let visible = false;

// Set visible after component mounts to trigger animations
onMount(() => {
  setTimeout(() => {
    visible = true;
  }, 150);
});
</script>

<div class="step-content-area">
  <h1 class="text-2xl font-semibold text-dark-text-primary mb-8 select-none cursor-default">A powerful communication tool. Not a service.</h1>
 

  {#if visible}
    <div class="space-y-4" transition:fade={{ duration: 800, delay: 200 }}>
      
      <!-- Local Tool Section -->
      <div class="rounded-lg mb-8" transition:fly={{ y: 20, duration: 600, delay: 400, easing: quintOut }}>
        <div class="flex items-start space-x-4">
          <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-brand-green/10 flex items-center justify-center">
            <svg class="w-5 h-5 text-brand-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
            </svg>
          </div>
          <div class="flex-1 space-y-3">
            <p class="text-xs text-dark-text-secondary leading-relaxed select-none cursor-default">
              Nymia runs 100% locally and doesn't talk to any servers.
            </p>
            <p class="text-xs text-dark-text-secondary leading-relaxed select-none cursor-default">
                It connects only to your node, which handles all network communication.
            </p>
          </div>
        </div>
      </div>

      <!-- Your Control Section -->
      <div class="rounded-lg" transition:fly={{ y: 20, duration: 600, delay: 500, easing: quintOut }}>
        <div class="flex items-start space-x-4">
          <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-brand-green/10 flex items-center justify-center">
            <svg class="w-5 h-5 text-brand-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
            </svg>
          </div>
          <div class="flex-1 space-y-3">
            <p class="text-xs text-dark-text-secondary leading-relaxed select-none cursor-default">
              Everything stays with you — your keys, your messages, your responsibility.
            </p>
            <p class="text-xs text-dark-text-secondary leading-relaxed select-none cursor-default">
              Just make sure your wallet is running and synced.
            </p>
          </div>
        </div>
      </div>

    </div>
  {/if}
</div>

<style>
  .step-content-area {
    width: 100%;
    min-height: 300px;
  }
</style> 