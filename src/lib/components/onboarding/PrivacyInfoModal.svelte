<script lang="ts">
// Component: src/lib/components/onboarding/PrivacyInfoModal.svelte
// Description: Modal popup that explains Nymia's privacy features
// Displays detailed information about zero-knowledge cryptography, decentralization, and user control
// Features clean overlay design with proper accessibility
// Changes:
// - Prevents background scrolling when modal is open
// - Moved "Got it" button inline with content instead of separate footer section
// - Updated to use the reusable Button component for consistency

  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  import Button from '../Button.svelte';

  const dispatch = createEventDispatcher<{ close: void }>();

  function handleClose() {
    dispatch('close');
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleClose();
    }
  }

  // Prevent background scrolling when modal is open
  onMount(() => {
    document.body.style.overflow = 'hidden';
  });

  onDestroy(() => {
    document.body.style.overflow = '';
  });
</script>

<svelte:window on:keydown={handleKeyDown} />

<!-- Modal Backdrop -->
<div 
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  on:click={handleBackdropClick}
  transition:fade={{ duration: 200 }}
>
  <!-- Modal Content -->
  <div 
    class="bg-dark-bg-primary border border-dark-border-primary rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto"
    transition:scale={{ duration: 200, easing: quintOut }}
  >
   

    <!-- Content -->
    <div class="p-6 space-y-6 text-dark-text-primary select-none cursor-default">
      
      <!-- Zero central servers -->
      <div class="space-y-2">
        <h3 class="font-semibold text-sm text-dark-text-primary">
          Zero central servers = Zero data collection
        </h3>
        <p class="text-xs text-dark-text-secondary leading-relaxed">
          Unlike WhatsApp, Discord, or Telegram, Nymia doesn't route your messages through company servers. There's literally nowhere for your data to be stored, analyzed, or sold.
        </p>
      </div>

      <!-- Zero-knowledge cryptography -->
      <div class="space-y-2">
        <h3 class="font-semibold text-sm text-dark-text-primary">
          Zero-knowledge cryptography protects everything
        </h3>
        <p class="text-xs text-dark-text-secondary leading-relaxed">
          Your messages travel through shielded transactions (z-txs) using advanced cryptography called zk-SNARKs.
        </p>
        
        <div class="ml-4 space-y-1">
          <p class="text-xs text-dark-text-secondary">This means:</p>
          <ul class="ml-4 space-y-1 text-xs text-dark-text-secondary">
            <li class="flex items-start">
              <span class="text-brand-green text-xs mr-2 mt-1">•</span>
              <span>The blockchain sees that a valid transaction happened, but not who sent it, who received it, or what you said</span>
            </li>
            <li class="flex items-start">
              <span class="text-brand-green text-xs mr-2 mt-1">•</span>
              <span>Only you and your recipient can read the actual message content</span>
            </li>
            <li class="flex items-start">
              <span class="text-brand-green text-xs mr-2 mt-1">•</span>
              <span>Even the transaction amounts and addresses are hidden</span>
            </li>
          </ul>
        </div>
      </div>

      <!-- Identity control -->
      <div class="space-y-2">
        <h3 class="font-semibold text-sm text-dark-text-primary">
          You control your identity
        </h3>
        <p class="text-xs text-dark-text-secondary leading-relaxed">
          Your VerusID lives on the blockchain under your complete control — no company can suspend, ban, or impersonate you.
        </p>
      </div>

      <!-- Open source -->
      <div class="space-y-2">
        <h3 class="font-semibold text-sm text-dark-text-primary">
          Open source and auditable
        </h3>
        <p class="text-xs text-dark-text-secondary leading-relaxed">
          Every line of code can be inspected. No hidden backdoors, no secret data collection — just transparent, private, peer-to-peer communication. 
          <a 
            href="https://github.com/Meyse/nymia-desktop" 
            target="_blank" 
            rel="noopener noreferrer"
            class="inline-flex items-center text-white transition-colors duration-150"
          >
            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
            View on GitHub
          </a>
        </p>
      </div>

      <!-- Got it button -->
      <div class="flex justify-end mt-6">
        <Button
          variant="primary"
          on:click={handleClose}
        >
          Got It
        </Button>
      </div>

    </div>
  </div>
</div>

 