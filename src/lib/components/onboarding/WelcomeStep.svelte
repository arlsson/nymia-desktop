<script lang="ts">
// Component: src/lib/components/onboarding/WelcomeStep.svelte
// Description: Initial welcome screen for the onboarding flow.
// Features Nymia icon, tagline image, and welcome text.
// Changes:
// - Replaced old logo/text with new icon, tagline image, and welcome message.
// - Centered content vertically and horizontally.
// - Adjusted spacing.
// - Added smooth entrance animations using Svelte transitions.
// - Enhanced icon animation for smoother motion.
// - Moved image assets to static directory and updated paths
// - Made welcome text non-selectable with default cursor for native desktop app feel
// - Integrated privacy info link naturally into welcome message as "learn why" text

import { createEventDispatcher, onMount } from 'svelte';
import { fade, fly } from 'svelte/transition';
import { quintOut } from 'svelte/easing';

// --- Event Dispatcher ---
// Dispatches getStarted event when user clicks the main button
// and showPrivacyInfo event when user clicks the privacy info link
const dispatch = createEventDispatcher<{ 
  getStarted: void;
  showPrivacyInfo: void;
}>();

// Control element visibility for animations
let visible = false;

// Set visible after component mounts to trigger animations
onMount(() => {
  // Slightly longer delay to ensure DOM is fully ready
  setTimeout(() => {
    visible = true;
  }, 150);
});
</script>

<div class="step-content-area flex flex-col items-center justify-center text-center h-full pt-10 pb-10">
  
  {#if visible}
    <!-- Nymia Icon -->
    <div class="transform-gpu"> <!-- Hardware acceleration wrapper -->
      <img 
        src="/nymia-icon.webp" 
        alt="Nymia Icon" 
        class="w-12 h-12 mb-6" 
        transition:fly={{ y: -20, duration: 1200, delay: 0, easing: quintOut }}
      />
    </div>
  {/if}
  
  {#if visible}
    <!-- Tagline Image -->
    <img 
      src="/tagline.webp" 
      alt="Messages you can't see. Payments you can't trace. Freedom I can feel." 
      class="w-72 mb-16" 
      transition:fly={{ x: -20, duration: 1200, delay: 400, easing: quintOut }}
    />
  {/if}

  {#if visible}
    <!-- Welcome Text -->
    <div 
      class="text-sm text-dark-text-primary tracking-tight select-none cursor-default"
      transition:fade={{ duration: 800, delay: 700 }}
    >
      <p class="mb-1">Welcome, full&#8209;node runner.</p>
      <p>
        Your privacy starts now — 
        <button 
          class="text-dark-text-secondary hover:text-dark-text-primary cursor-pointer"
          on:click={() => dispatch('showPrivacyInfo')}
        >
          learn why.
        </button>
      </p>
    </div>
  {/if}
</div>

<style>
  .step-content-area {
    width: 100%;
    min-height: 300px;
  }
  
  /* Add hardware acceleration for smoother animations */
  .transform-gpu {
    will-change: transform;
    transform: translateZ(0);
  }
</style> 