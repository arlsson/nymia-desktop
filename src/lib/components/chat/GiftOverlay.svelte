<script lang="ts">
// Component: src/lib/components/chat/GiftOverlay.svelte
// Description: Gift amount input overlay with colorful animated border and validation
// Extracted from MessageInput.svelte to improve code organization and maintainability

  import { createEventDispatcher } from 'svelte';
  import { Gift, AlertTriangle } from 'lucide-svelte';
  import type { PrivateBalance } from '$lib/types';

  // --- Props ---
  export let show: boolean = false;
  export let amountToSend: number | null = null;
  export let privateBalance: PrivateBalance = null;
  export let currencySymbol: string = 'VRSC';
  export let showConfirmation: boolean = false;

  // --- Constants ---
  const TX_FEE = 0.0001;

  // --- State ---
  let giftInputElement: HTMLInputElement;
  let insufficientBalanceError: boolean = false;

  // --- Events ---
  const dispatch = createEventDispatcher<{
    confirm: { amount: number };
    cancel: void;
    clear: void;
  }>();

  // Check if amount exceeds balance
  $: if (amountToSend !== null && privateBalance !== null) {
    insufficientBalanceError = amountToSend > (privateBalance - TX_FEE);
  } else {
    insufficientBalanceError = false;
  }

  // Focus input when overlay shows
  $: if (show && giftInputElement) {
    setTimeout(() => {
      giftInputElement?.focus();
    }, 50);
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Confirm on Enter
    if (event.key === 'Enter' && !insufficientBalanceError) {
      event.preventDefault();
      handleConfirm();
    }
    
    // Cancel on Escape
    if (event.key === 'Escape') {
      event.preventDefault();
      handleCancel();
    }
    
    // Stop event propagation
    event.stopPropagation();
  }

  function handleConfirm() {
    if (amountToSend && amountToSend > 0 && !insufficientBalanceError) {
      dispatch('confirm', { amount: amountToSend });
    }
  }

  function handleCancel() {
    dispatch('cancel');
  }

  function handleClear() {
    dispatch('clear');
  }
</script>

{#if show}
  <div class="absolute bottom-full right-0 mb-2 bg-dark-bg-primary border border-brand-green rounded-lg shadow-lg p-8 min-w-[240px] z-20 gift-overlay-colorful">
    <!-- Arrow pointing down to gift icon -->
    <div class="absolute top-full right-3 w-0 h-0 border-l-4 border-r-4 border-t-4 border-l-transparent border-r-transparent border-t-purple-400"></div>
    
    <div class="text-sm font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 via-blue-400 to-pink-400 mb-3 flex items-center">
      <Gift size={16} class="mr-2 text-purple-400" />
      Send a Gift!
    </div>
    
    <div class="relative mb-3">
      <input 
        type="number" 
        step="any" 
        min="0" 
        inputmode="decimal"
        bind:value={amountToSend} 
        placeholder="0.0000" 
        class={`w-full py-2 px-3 rounded text-sm focus:outline-none pr-12 bg-white/5 text-dark-text-primary placeholder-dark-text-disabled ${insufficientBalanceError ? 'text-red-400' : ''}`}
        disabled={showConfirmation}
        bind:this={giftInputElement}
        on:keydown={handleKeyDown}
      />
      <div class="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-dark-text-secondary font-medium">
        {currencySymbol}
      </div>
    </div>
    
    <!-- Error message inside overlay -->
    {#if insufficientBalanceError}
      <div class="mb-3 text-xs text-red-400 flex items-start">
        <AlertTriangle size={12} class="mr-1 mt-0.5 shrink-0"/>
        <span>Insufficient balance. Max: {(privateBalance !== null ? (privateBalance - TX_FEE) : 0).toFixed(4)} {currencySymbol}</span>
      </div>
    {/if}
    
    <div class="mb-6 text-xs text-dark-text-secondary">
      <span class="text-white/60 mr-1">Available:</span>
      <span class={`font-medium font-mono ${privateBalance !== null ? 'text-white' : 'text-dark-text-disabled'}`}>
        {privateBalance !== null ? `${privateBalance.toFixed(4)} ${currencySymbol}` : 'Loading...'}
      </span>
    </div>
    
    <!-- Confirm/Cancel buttons -->
    <div class="flex justify-end space-x-2">
      <button 
        on:click={handleClear}
        class="px-2 py-1 text-dark-text-secondary text-xs hover:text-dark-text-primary transition-colors"
      >
        Cancel
      </button>
      <button 
        on:click={handleConfirm}
        disabled={!amountToSend || amountToSend <= 0 || insufficientBalanceError}
        class="px-3 py-1 bg-purple-500 hover:bg-purple-600 disabled:bg-gray-600 disabled:opacity-50 text-white text-xs rounded transition-colors font-medium"
      >
        Confirm Gift
      </button>
    </div>
  </div>
{/if}

<style>
  /* Colorful styling for gift overlay - matching Gift received colors exactly */
  .gift-overlay-colorful {
    border: 2px solid transparent;
    background: linear-gradient(135deg, #0a0a0a, #0a0a0a) padding-box,
                linear-gradient(to right, #8b5cf6, #3b82f6, #14b8a6, #eab308, #f97316) border-box;
    box-shadow: 0 0 30px rgba(139, 92, 246, 0.3), 0 0 60px rgba(139, 92, 246, 0.1);
    animation: gift-overlay-glow 3s ease-in-out infinite;
  }
  
  @keyframes gift-overlay-glow {
    0%, 100% {
      box-shadow: 0 0 30px rgba(139, 92, 246, 0.3), 0 0 60px rgba(139, 92, 246, 0.1);
    }
    33% {
      box-shadow: 0 0 30px rgba(59, 130, 246, 0.3), 0 0 60px rgba(59, 130, 246, 0.1);
    }
    66% {
      box-shadow: 0 0 30px rgba(234, 179, 8, 0.3), 0 0 60px rgba(234, 179, 8, 0.1);
    }
  }

  /* Remove number input spinners */
  input[type="number"]::-webkit-outer-spin-button,
  input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  
  /* Firefox */
  input[type="number"] {
    -moz-appearance: textfield;
  }
</style> 