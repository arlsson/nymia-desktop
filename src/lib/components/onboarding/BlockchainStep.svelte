<script lang="ts">
// Component: src/lib/components/onboarding/BlockchainStep.svelte
// Description: Handles the selection of the blockchain network.

  import { createEventDispatcher } from 'svelte';
  import CustomDropdown from '../CustomDropdown.svelte';
  
  // Import Shared Types
  import type { DropdownOption } from '$lib/types';

  // --- Props ---
  export let options: DropdownOption[] = []; // Use imported type
  export let selectedId: string | number | null = null;

  // --- Event Dispatcher ---
  const dispatch = createEventDispatcher<{ change: string | number | null }>();

  function handleSelection(event: CustomEvent<string | number | null>) {
      // Propagate the change event upwards
      dispatch('change', event.detail);
  }

</script>

<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-dark-text-primary mb-2">Select Blockchain</h1>
    <p class="text-dark-text-secondary text-normal mb-6">Choose the network you want to connect to.</p>
    
    <CustomDropdown 
        label="Blockchain"
        options={options}
        bind:selectedId={selectedId}
        placeholder="Select network..."
        on:change={handleSelection}
    />


</div>

<style>
    .step-content-area {
        /* Add any specific styles for this step if needed */
        width: 100%;
    }
</style> 