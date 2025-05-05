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

import { createEventDispatcher, onMount } from 'svelte';
import { fade, fly } from 'svelte/transition';
import { quintOut } from 'svelte/easing';

// --- Event Dispatcher ---
// Although not used in this static welcome step, kept for consistency with previous versions
// and potential future use (e.g., animating entry before enabling 'Get Started').
const dispatch = createEventDispatcher<{ getStarted: void }>();

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
    <p 
      class="text-sm text-gray-600 tracking-tight"
      transition:fade={{ duration: 800, delay: 700 }}
    >
      Welcome, full&#8209;node runner. Your privacy starts now.
    </p>
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