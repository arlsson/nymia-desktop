<script lang="ts">
// Component: src/lib/components/chat/GiftOverlay.svelte
// Description: Clean and organized gift amount input overlay with MAX button functionality
// Changes:
// - Redesigned for cleaner, more organized appearance with proper dividers
// - Added MAX button that selects complete balance minus transaction fee
// - Improved layout with better spacing and visual hierarchy
// - Enhanced error handling and validation display
// - Added Fast Messages impact warning when gifts may affect available UTXOs

  import { createEventDispatcher } from 'svelte';
  import { Gift, AlertTriangle, Zap } from 'lucide-svelte';
  import type { PrivateBalance, UtxoInfo } from '$lib/types';
  import Button from '../Button.svelte';

  // --- Props ---
  export let show: boolean = false;
  export let amountToSend: number | null = null;
  export let privateBalance: PrivateBalance = null;
  export let currencySymbol: string = 'VRSC';
  export let showConfirmation: boolean = false;
  export let utxoInfo: UtxoInfo | null = null; // NEW: UTXO information for Fast Messages impact

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

  // Calculate maximum sendable amount (balance - fee)
  $: maxSendableAmount = privateBalance !== null ? Math.max(0, privateBalance - TX_FEE) : 0;

  // Show impact warning when gift amount is entered
  $: showFastMessagesImpact = utxoInfo && amountToSend && amountToSend > 0;

  // Check if amount exceeds balance
  $: if (amountToSend !== null && privateBalance !== null) {
    insufficientBalanceError = amountToSend > maxSendableAmount;
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

  function handleMaxAmount() {
    if (maxSendableAmount > 0) {
      amountToSend = maxSendableAmount;
      giftInputElement?.focus();
    }
  }
</script>

{#if show}
  <div class="absolute bottom-full right-0 mb-2 bg-dark-bg-primary border border-brand-green/40 rounded-xl shadow-2xl min-w-[280px] z-20 gift-overlay-colorful overflow-hidden">
    <!-- Arrow pointing down to gift icon -->
    <div class="absolute top-full right-3 w-0 h-0 border-l-4 border-r-4 border-t-4 border-l-transparent border-r-transparent border-t-purple-400"></div>
    
    <!-- Header -->
    <div class="p-4 border-b border-dark-border-primary/50">
      <div class="flex items-center text-base font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 via-blue-400 to-pink-400">
        <Gift size={18} class="mr-2 text-purple-400" />
        Send a Gift!
      </div>
    </div>
    
    <!-- Amount Input Section -->
    <div class="p-4 space-y-3">
      <div class="space-y-2">
        <label class="block text-sm font-medium text-dark-text-primary">Amount</label>
                 <div class="relative">
           <input 
             type="number" 
             step="any" 
             min="0" 
             inputmode="decimal"
             bind:value={amountToSend} 
             placeholder="0.0000" 
             class="w-full py-3 pl-4 pr-24 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-purple-500/50 bg-dark-bg-secondary border border-dark-border-primary text-dark-text-primary placeholder-dark-text-disabled font-mono {insufficientBalanceError ? 'border-red-500/50 text-red-400' : ''}"
             disabled={showConfirmation}
             bind:this={giftInputElement}
             on:keydown={handleKeyDown}
           />
           <div class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center space-x-2">
             <span class="text-xs text-dark-text-secondary font-medium pointer-events-none">
               {currencySymbol}
             </span>
             <button
               type="button"
               on:click={handleMaxAmount}
               disabled={showConfirmation || maxSendableAmount <= 0}
               class="px-2 py-1 text-xs font-medium bg-purple-600/20 hover:bg-purple-600/30 text-purple-300 border border-purple-500/30 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
             >
               MAX
             </button>
           </div>
         </div>
      </div>

      <!-- Fast Messages Impact Warning (NEW) -->
      {#if showFastMessagesImpact}
        <div class="flex items-center space-x-2 p-2 bg-blue-900/20 border border-blue-700/30 rounded-lg">
          <Zap size={12} class="text-blue-400 flex-shrink-0"/>
          <div class="text-xs text-blue-300">
            <span class="font-medium">May affect fast messages</span> - this gift may reduce available UTXOs
          </div>
        </div>
      {/if}

      <!-- Error Message -->
      {#if insufficientBalanceError}
        <div class="flex items-start space-x-2 p-3 bg-red-900/20 border border-red-700/30 rounded-lg">
          <AlertTriangle size={14} class="text-red-400 mt-0.5 flex-shrink-0"/>
          <div class="text-xs text-red-300">
            <div class="font-medium">Insufficient balance</div>
            <div class="text-red-400 mt-1">Maximum available: {maxSendableAmount.toFixed(8)} {currencySymbol}</div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Divider -->
    <div class="h-px bg-gradient-to-r from-transparent via-dark-border-primary to-transparent"></div>

    <!-- Balance Information -->
    <div class="p-4 bg-dark-bg-secondary/30 space-y-2">
      <div class="flex justify-between items-center text-xs">
        <span class="text-dark-text-secondary">Available Balance:</span>
        <span class="font-mono font-medium {privateBalance !== null ? 'text-green-300' : 'text-dark-text-disabled'}">
          {privateBalance !== null ? `${privateBalance.toFixed(8)} ${currencySymbol}` : 'Loading...'}
        </span>
      </div>
      
      <div class="flex justify-between items-center text-xs">
        <span class="text-dark-text-secondary">Transaction Fee:</span>
        <span class="font-mono text-dark-text-secondary">
          {TX_FEE.toFixed(8)} {currencySymbol}
        </span>
      </div>
      
      <div class="h-px bg-dark-border-primary/30 my-2"></div>
      
      <div class="flex justify-between items-center text-xs">
        <span class="text-dark-text-primary font-medium">Max Sendable:</span>
        <span class="font-mono font-semibold text-purple-300">
          {maxSendableAmount.toFixed(8)} {currencySymbol}
        </span>
      </div>
    </div>

    <!-- Divider -->
    <div class="h-px bg-gradient-to-r from-transparent via-dark-border-primary to-transparent"></div>
    
    <!-- Action Buttons -->
    <div class="p-4 flex justify-end space-x-3">
      <Button 
        variant="secondary"
        on:click={handleClear}
      >
        Cancel
      </Button>
      <div class="purple-gift-button">
        <Button 
          variant="primary"
          disabled={!amountToSend || amountToSend <= 0 || insufficientBalanceError}
          on:click={handleConfirm}
        >
          Confirm Gift
        </Button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Enhanced colorful styling for gift overlay */
  .gift-overlay-colorful {
    border: 2px solid transparent;
    background: linear-gradient(135deg, #0a0a0a, #0f0f0f) padding-box,
                linear-gradient(45deg, #8b5cf6, #3b82f6, #14b8a6, #eab308, #f97316) border-box;
    box-shadow: 
      0 0 30px rgba(139, 92, 246, 0.2), 
      0 0 60px rgba(139, 92, 246, 0.1),
      0 10px 30px rgba(0, 0, 0, 0.3);
    animation: gift-overlay-glow 4s ease-in-out infinite;
  }
  
  @keyframes gift-overlay-glow {
    0%, 100% {
      box-shadow: 
        0 0 30px rgba(139, 92, 246, 0.2), 
        0 0 60px rgba(139, 92, 246, 0.1),
        0 10px 30px rgba(0, 0, 0, 0.3);
    }
    33% {
      box-shadow: 
        0 0 30px rgba(59, 130, 246, 0.2), 
        0 0 60px rgba(59, 130, 246, 0.1),
        0 10px 30px rgba(0, 0, 0, 0.3);
    }
    66% {
      box-shadow: 
        0 0 30px rgba(234, 179, 8, 0.2), 
        0 0 60px rgba(234, 179, 8, 0.1),
        0 10px 30px rgba(0, 0, 0, 0.3);
    }
  }

  /* Purple gift button styling */
  .purple-gift-button :global(.btn-primary) {
    background-color: #9333ea !important; /* bg-purple-600 */
  }
  
  .purple-gift-button :global(.btn-primary:hover) {
    background-color: #7c3aed !important; /* bg-purple-700 */
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