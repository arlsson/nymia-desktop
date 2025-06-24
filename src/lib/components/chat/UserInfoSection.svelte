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
// - Added comma formatting for block height numbers and removed # symbol
// - Added Fast Messages row with UTXO count display and clickable tooltip
// - Added UtxoInfo props and loading states for Fast Messages feature

  import { createEventDispatcher } from 'svelte';
  import { LogOut, Settings, Layers, Loader, FastForward, HelpCircle } from 'lucide-svelte';
  import Avatar from '../Avatar.svelte';
  import type { PrivateBalance, UtxoInfo } from '$lib/types';

  // --- Props ---
  export let verusIdName: string = 'Unknown User';
  export let privateBalance: PrivateBalance = null;
  export let blockHeight: number | null = null;
  export let isTransactionPending: boolean = false;
  export let currencySymbol: string = 'VRSC'; // Dynamic currency symbol
  export let utxoInfo: UtxoInfo | null = null; // NEW: UTXO information for Fast Messages
  export let isUtxoLoading: boolean = false; // NEW: Loading state for UTXO data

  // --- Events ---
  const dispatch = createEventDispatcher<{
    logout: void;
    settings: void;
  }>();

  // --- State ---
  let showTooltip = false;

  function handleLogout() {
    dispatch('logout');
  }
  
  function handleSettings() {
    dispatch('settings');
  }

  function showTooltipOnHover() {
    showTooltip = true;
  }

  function hideTooltipOnLeave() {
    showTooltip = false;
  }

  // Format balance for display
  $: formattedBalance = privateBalance !== null ? `${privateBalance.toFixed(4)} ${currencySymbol}` : 'Loading...';

  // Format block height with commas
  function formatBlockHeight(height: number): string {
    return height.toLocaleString();
  }

</script>

<!-- Tooltip overlay -->
{#if showTooltip}
  <div class="absolute bottom-16 left-4 bg-black border border-dark-border-primary rounded-lg p-3 shadow-lg max-w-xs z-50 cursor-default select-none">
    <div class="text-sm text-dark-text-primary mb-2 font-semibold">Fast Messages</div>
    <div class="text-xs text-dark-text-secondary leading-relaxed">
      Fast Messages are UTXOs (≥0.0001) that can send immediate transactions without waiting for confirmations.
      {#if utxoInfo}
        <div class="mt-2 pt-2 border-t border-dark-border-primary">
          <div class="text-dark-text-primary">Current breakdown:</div>
          <div class="mt-1">
            • {utxoInfo.usable_utxos} usable UTXOs<br>
            {#if utxoInfo.usable_utxos > 1}
              • Largest: {utxoInfo.largest_utxo.toFixed(4)} {currencySymbol}<br>
              • Smallest: {utxoInfo.smallest_utxo.toFixed(4)} {currencySymbol}
            {:else if utxoInfo.usable_utxos === 1}
              • Largest: {utxoInfo.largest_utxo.toFixed(4)} {currencySymbol}
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<div class="border-t border-dark-border-primary p-3 flex-shrink-0" style="background-color: #121214">
  <!-- Row 1: Avatar + Name + Action Buttons -->
  <div class="flex items-center justify-between mb-4">
    <!-- Left: Avatar + Name -->
    <div class="flex items-center min-w-0 flex-1">
      <Avatar userId={verusIdName} size="small" showHover={false} />
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

  <!-- Subtle separator -->
  <div class="h-px bg-white/10 mb-3"></div>

  <!-- Row 2: Private Balance -->
  <div class="flex items-center justify-between text-xs mb-3">
    <span class="text-white/45 cursor-default select-none">Private Balance</span>
    <div class="flex items-center">
      <span class={`font-mono font-bold cursor-default select-none ${privateBalance !== null ? 'text-green-300' : 'text-dark-text-disabled'}`}>
        {formattedBalance}
      </span>
      {#if isTransactionPending}
        <div class="animate-spin text-brand-green ml-2" title="Waiting for transaction confirmation">
          <Loader size={12} />
        </div>
      {/if}
    </div>
  </div>

  <!-- Row 3: Fast Messages (NEW) -->
  {#if utxoInfo !== null}
    <div class="flex items-center justify-between text-xs mb-3 relative">
      <div class="flex items-center">
        <span class="text-white/45 cursor-default select-none">Fast Messages</span>
        <div class="ml-1"
             on:mouseenter={showTooltipOnHover}
             on:mouseleave={hideTooltipOnLeave}>
          <HelpCircle size={14} class="text-white/50 hover:text-white/80" />
        </div>
      </div>
      <div class="flex items-center">
        <span class={`font-mono font-bold cursor-default select-none ${utxoInfo.usable_utxos > 0 ? 'text-dark-text-primary' : 'text-white/30'}`}>
          {utxoInfo.usable_utxos}
        </span>
        <FastForward size={14} class="ml-1 text-white/40" />
      </div>
    </div>
  {/if}

  <!-- Separator before block height -->
  <div class="h-px bg-white/10 my-3"></div>

  <!-- Row 4: Block Height (Subtle) -->
  {#if blockHeight !== null}
    <div class="flex items-center text-xs text-white/45 cursor-default select-none">
      <Layers size={10} class="mr-1" />
      <span>Block {formatBlockHeight(blockHeight)}</span>
    </div>
  {/if}
</div> 