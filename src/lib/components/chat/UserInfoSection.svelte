<script lang="ts">
// Component: src/lib/components/chat/UserInfoSection.svelte
// Description: Discord-style user info section for the bottom of the sidebar
// Displays user avatar, name, balance, block height, and action buttons (settings, logout)
// Changes:
// - Created new component for sidebar user info section
// - Discord-style layout with avatar, name, and action buttons
// - Balance and block height on separate lines
// - Fixed positioning at bottom of sidebar
// - Added dynamic currency symbol support based on selected blockchain
// - Updated background color to specific hex color #121214
// - Improved private balance display with better layout and monospace font

  import { createEventDispatcher } from 'svelte';
  import { LogOut, Settings, Layers, Loader } from 'lucide-svelte';
  import Avatar from '../Avatar.svelte';
  import type { PrivateBalance } from '$lib/types';

  // --- Props ---
  export let verusIdName: string = 'Unknown User';
  export let privateBalance: PrivateBalance = null;
  export let blockHeight: number | null = null;
  export let isTransactionPending: boolean = false;
  export let currencySymbol: string = 'VRSC'; // Dynamic currency symbol

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
  $: formattedBalance = privateBalance !== null ? `${privateBalance.toFixed(4)} ${currencySymbol}` : 'Loading...';

</script>

<div class="border-t border-dark-border-primary p-3 flex-shrink-0" style="background-color: #121214">
  <!-- Row 1: Avatar + Name + Action Buttons -->
  <div class="flex items-center justify-between mb-2">
    <!-- Left: Avatar + Name -->
    <div class="flex items-center min-w-0 flex-1">
      <Avatar userId={verusIdName} size="small" showHover={true} />
      <div class="ml-2 min-w-0 flex-1">
        <div class="font-semibold text-dark-text-primary text-sm truncate">
          {verusIdName}
        </div>
      </div>
    </div>

    <!-- Right: Action Buttons -->
    <div class="flex items-center space-x-1 ml-2">
      <button 
        on:click={handleSettings} 
        title="Settings"
        class="p-1.5 rounded-md text-white/50 hover:text-dark-text-primary focus:outline-none focus:ring-1 focus:ring-brand-green"
      >
        <Settings size={16} />
      </button>
      
      <button 
        on:click={handleLogout} 
        title="Log Out"
        class="p-1.5 rounded-md text-white/50 hover:text-dark-text-primary focus:outline-none focus:ring-1 focus:ring-brand-green"
      >
        <LogOut size={16} />
      </button>
    </div>
  </div>

  <!-- Row 2: Private Balance -->
  <div class="flex items-center justify-between text-xs mb-2">
    <span class="text-dark-text-secondary">Private Balance</span>
    <div class="flex items-center">
      <span class={`font-mono font-medium ${privateBalance !== null ? 'text-white' : 'text-dark-text-disabled'}`}>
        {formattedBalance}
      </span>
      {#if isTransactionPending}
        <div class="animate-spin text-brand-green ml-2" title="Waiting for transaction confirmation">
          <Loader size={12} />
        </div>
      {/if}
    </div>
  </div>

  <!-- Row 3: Block Height (Subtle) -->
  {#if blockHeight !== null}
    <div class="flex items-center text-xs text-white/45">
      <Layers size={10} class="mr-1" />
      <span>Block #{blockHeight}</span>
    </div>
  {/if}
</div> 