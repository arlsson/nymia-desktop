<script lang="ts">
// Component: src/lib/components/onboarding/WelcomeStep.svelte
// Description: Initial welcome screen for the onboarding flow.
// Features a static text message and the Nymia logo.
// Changes:
// - Made logo smaller
// - Made text smaller
// - Aligned text to the left

import { createEventDispatcher } from 'svelte';

// --- Event Dispatcher ---
const dispatch = createEventDispatcher<{ getStarted: void }>();
</script>

<div class="step-content-area flex flex-col items-center justify-center">
  <img src="/src/lib/assets/nymia-logo.png" alt="Nymia Logo" class="w-40 mb-8" />
  
  <div class="welcome-text text-left max-w-md">
    <p class="text-base font-medium text-gray-800 mb-2">
      No one can see a thing—not even that you're here.
    </p>
    <p class="text-base text-gray-700">
      Your messages and payments leave zero trace. It's like you were never on the network at all.
    </p>
  </div>
</div>

<style>
  .step-content-area {
    width: 100%;
    height: 100%;
  }
  
  .welcome-text {
    margin-bottom: 2rem;
    line-height: 1.5;
  }
</style> 