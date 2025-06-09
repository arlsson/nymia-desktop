<script lang="ts">
// Component: src/lib/components/chat/TopBar.svelte
// Description: Displays the top bar of the chat interface with user info and controls.
// Changes:
// - Switched from Tabler Icons to Lucide Icons.
// - Redesigned with a more subtle, modern style (white background with subtle border).
// - Reduced height and increased whitespace.
// - Added block height indicator.
// - Added spinner icon when transaction is pending.

  import { createEventDispatcher } from 'svelte';
  import { LogOut, Settings, Layers, Loader } from 'lucide-svelte'; // Removed RefreshCw
  // Import the new type alias
  import type { PrivateBalance } from '$lib/types';

  // --- Props ---
  export let verusIdName: string = 'Unknown User';
  // Use the new type alias
  export let privateBalance: PrivateBalance = null;
  export let blockHeight: number | null = null; // Added block height prop
  export let isTransactionPending: boolean = false; // Added pending state prop

  // --- Events ---
  const dispatch = createEventDispatcher<{
    logout: void;
    settings: void;
  }>();

  function handleLogout() {
    dispatch('logout');
  }
  function handleSettings() {
    dispatch('settings');
  }

  // Format balance for display
  $: formattedBalance = privateBalance !== null ? `${privateBalance.toFixed(4)} VRSC` : 'Loading...';

</script>

<div class="flex items-center justify-between h-[50px] px-4 bg-dark-bg-secondary border-b border-dark-border-primary flex-shrink-0">
  <!-- Left: VerusID Name -->
  <div class="font-semibold text-base text-dark-text-primary">
    {verusIdName}
  </div>

  <!-- Center: Balance -->
  <div class="text-sm text-dark-text-secondary flex items-center space-x-1.5">
    <span class="mr-1 font-medium">Private Balance:</span> 
    <span class={privateBalance !== null ? 'text-brand-green' : 'text-dark-text-disabled'}>{formattedBalance}</span>
    {#if isTransactionPending}
      <div class="animate-spin text-dark-text-secondary" title="Waiting for transaction confirmation">
        <Loader size={14} />
      </div>
    {/if}
  </div>

  <!-- Right: Action Buttons -->
  <div class="flex items-center space-x-2">
    <!-- Block Height Indicator -->
    {#if blockHeight !== null}
      <div class="flex items-center text-xs text-dark-text-secondary bg-dark-bg-tertiary px-2 py-1 rounded mr-1">
        <Layers size={12} class="mr-1" />
        <span title="Current block height">#{blockHeight}</span>
      </div>
    {/if}
    
    <button 
      on:click={handleLogout} 
      title="Log Out"
      class="p-1.5 rounded text-dark-text-secondary hover:bg-dark-bg-tertiary hover:text-dark-text-primary focus:outline-none focus:ring-1 focus:ring-dark-border-secondary transition-colors duration-150"
    >
      <LogOut size={16} />
    </button>
    <button 
      on:click={handleSettings} 
      title="Settings"
      class="p-1.5 rounded text-dark-text-secondary hover:bg-dark-bg-tertiary hover:text-dark-text-primary focus:outline-none focus:ring-1 focus:ring-dark-border-secondary transition-colors duration-150"
    >
      <Settings size={16} />
    </button>
  </div>
</div> 