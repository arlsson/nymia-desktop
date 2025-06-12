<script lang="ts">
// Component: src/lib/components/chat/TopBar.svelte
// Description: Discord-style top bar with user avatar, balance, and controls
// Changes:
// - Added user Avatar integration
// - Updated color scheme to black/dark gray with minimal brand green
// - Enhanced visual design with better spacing and layout
// - Improved button styling and hover states
// - Better visual hierarchy for user information

  import { createEventDispatcher } from 'svelte';
  import { LogOut, Settings, Layers, Loader } from 'lucide-svelte';
  import Avatar from '../Avatar.svelte';
  import type { PrivateBalance } from '$lib/types';

  // --- Props ---
  export let verusIdName: string = 'Unknown User';
  export let privateBalance: PrivateBalance = null;
  export let blockHeight: number | null = null;
  export let isTransactionPending: boolean = false;

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

<div class="flex items-center justify-between h-12 px-4 bg-dark-bg-secondary border-b border-dark-border-primary flex-shrink-0">
  <!-- Left: User Info with Avatar -->
  <div class="flex items-center min-w-0">
    <Avatar userId={verusIdName} size="small" showHover={true} />
    <div class="ml-3 min-w-0">
      <div class="font-semibold text-dark-text-primary text-sm truncate">
        {verusIdName}
      </div>
    </div>
  </div>

  <!-- Center: Balance -->
  <div class="flex items-center text-sm text-dark-text-secondary space-x-2">
    <span class="font-medium text-white/60">Private Balance:</span> 
    <span class={`font-semibold ${privateBalance !== null ? 'text-brand-green' : 'text-dark-text-disabled'}`}>
      {formattedBalance}
    </span>
    {#if isTransactionPending}
      <div class="animate-spin text-brand-green" title="Waiting for transaction confirmation">
        <Loader size={14} />
      </div>
    {/if}
  </div>

  <!-- Right: Action Buttons -->
  <div class="flex items-center space-x-1">
    <!-- Block Height Indicator -->
    {#if blockHeight !== null}
      <div class="flex items-center text-xs text-dark-text-disabled bg-dark-bg-tertiary px-2 py-1 rounded-md mr-2 border border-dark-border-secondary">
        <Layers size={12} class="mr-1.5" />
        <span title="Current block height">#{blockHeight}</span>
      </div>
    {/if}
    
    <button 
      on:click={handleSettings} 
      title="Settings"
      class="p-2 rounded-md text-dark-text-secondary hover:bg-dark-bg-tertiary hover:text-dark-text-primary focus:outline-none focus:ring-2 focus:ring-brand-green focus:ring-offset-2 focus:ring-offset-dark-bg-secondary transition-all duration-150"
    >
      <Settings size={16} />
    </button>
    
    <button 
      on:click={handleLogout} 
      title="Log Out"
      class="p-2 rounded-md text-dark-text-secondary hover:bg-dark-bg-tertiary hover:text-dark-text-primary focus:outline-none focus:ring-2 focus:ring-brand-green focus:ring-offset-2 focus:ring-offset-dark-bg-secondary transition-all duration-150"
    >
      <LogOut size={16} />
    </button>
  </div>
</div> 