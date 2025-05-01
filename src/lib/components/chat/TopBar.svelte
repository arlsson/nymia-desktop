<script lang="ts">
// Component: src/lib/components/chat/TopBar.svelte
// Description: Displays the top bar of the chat interface with user info and controls.
// Changes:
// - Switched from Tabler Icons to Lucide Icons.
// - Redesigned with a more subtle, modern style (white background with subtle border).
// - Reduced height and increased whitespace.
// - Added block height indicator.

  import { createEventDispatcher } from 'svelte';
  import { LogOut, RefreshCw, Settings, Layers } from 'lucide-svelte'; // Added Layers icon
  // Import the new type alias
  import type { PrivateBalance } from '$lib/types';

  // --- Props ---
  export let verusIdName: string = 'Unknown User';
  // Use the new type alias
  export let privateBalance: PrivateBalance = null;
  export let blockHeight: number | null = null; // Added block height prop

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      logout: void; 
      refresh: void;
      settings: void;
  }>();

  function handleLogout() {
    dispatch('logout');
  }
  function handleRefresh() {
    dispatch('refresh');
  }
  function handleSettings() {
    dispatch('settings');
  }

  // Format balance for display
  $: formattedBalance = privateBalance !== null ? `${privateBalance.toFixed(4)} VRSC` : 'Loading...';

</script>

<div class="flex items-center justify-between h-[50px] px-4 bg-gray-50 border-b border-gray-200 flex-shrink-0">
  <!-- Left: VerusID Name -->
  <div class="font-semibold text-base text-gray-800">
    {verusIdName}
  </div>

  <!-- Center: Balance (Placeholder) -->
  <div class="text-sm text-gray-600 flex items-center">
    <span class="mr-1 font-medium">Private Balance:</span> 
    <span class={privateBalance !== null ? 'text-[#419A6A]' : 'text-gray-500'}>{formattedBalance}</span>
  </div>

  <!-- Right: Action Buttons -->
  <div class="flex items-center space-x-2">
    <!-- Block Height Indicator -->
    {#if blockHeight !== null}
      <div class="flex items-center text-xs text-gray-500 bg-gray-100 px-2 py-1 rounded mr-1">
        <Layers size={12} class="mr-1" />
        <span title="Current block height">#{blockHeight}</span>
      </div>
    {/if}
    
    <button 
      on:click={handleRefresh} 
      title="Refresh Data"
      class="p-1.5 rounded text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-200 transition-colors duration-150"
    >
      <RefreshCw size={16} />
    </button>
    <button 
      on:click={handleSettings} 
      title="Settings"
      class="p-1.5 rounded text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-200 transition-colors duration-150"
    >
      <Settings size={16} />
    </button>
    <button 
      on:click={handleLogout} 
      title="Log Out"
      class="p-1.5 rounded text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-200 transition-colors duration-150"
    >
      <LogOut size={16} />
    </button>
  </div>
</div> 