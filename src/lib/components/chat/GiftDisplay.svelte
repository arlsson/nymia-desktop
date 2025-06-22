<script lang="ts">
// Component: src/lib/components/chat/GiftDisplay.svelte
// Description: Gift display for sent and received gifts with animations
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Handles both sent and received gift display variants
// - Includes all gift-related animations and styling
// - Takes gift amount, direction, and currency symbol as props

  import { Gift } from 'lucide-svelte';

  // --- Props ---
  export let amount: number;
  export let direction: 'sent' | 'received';
  export let currencySymbol: string = 'VRSC';
</script>

<div class="gift-container mb-2">
  {#if direction === 'received'}
    <!-- Enhanced Gift Display (Received) -->
    <div class="gift-notification-container relative overflow-hidden rounded-lg p-0.5 max-w-sm">
      <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-blue-500 via-teal-400 via-yellow-400 to-orange-500 gift-pulse"></div>
      <div class="flex items-center rounded-lg bg-dark-bg-secondary p-3 relative">
        <div class="flex items-center">
          <div class="p-2 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 rounded-full mr-3 text-white gift-icon-pulse">
            <Gift size={20} />
          </div>
          <div class="flex flex-col">
            <span class="text-sm font-semibold text-dark-text-primary select-none cursor-default">
              Gift received!
            </span>
            <span class="text-lg font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 via-blue-400 to-pink-400 select-none cursor-default">
              {amount.toFixed(8)} {currencySymbol}
            </span>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- Simple Gift Display (Sent) -->
    <div class="flex items-center bg-brand-green bg-opacity-20 border border-brand-green border-opacity-30 p-2 rounded-lg max-w-sm">
      <div class="p-1.5 bg-brand-green bg-opacity-20 rounded-full mr-2 text-brand-green flex-shrink-0">
        <Gift size={16} />
      </div>
      <div>
        <span class="text-sm font-medium text-dark-text-primary select-none cursor-default">Gift sent</span>
        <span class="text-sm font-semibold text-brand-green block select-none cursor-default">
          {amount.toFixed(8)} {currencySymbol}
        </span>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Gift notification animations */
  @keyframes gift-pulse {
    0% {
      opacity: 0.9;
      transform: scale(1);
    }
    50% {
      opacity: 1;
      transform: scale(1.02);
    }
    100% {
      opacity: 0.9;
      transform: scale(1);
    }
  }
  
  @keyframes gift-icon-pulse {
    0% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(147, 51, 234, 0.4);
    }
    50% {
      transform: scale(1.1);
      box-shadow: 0 0 10px 3px rgba(147, 51, 234, 0.2);
    }
    100% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(147, 51, 234, 0.4);
    }
  }

  .gift-pulse {
    animation: gift-pulse 4s ease-in-out infinite;
    background-size: 200% 200%;
    animation: gift-pulse 4s ease-in-out infinite, gradient-shift 8s linear infinite;
  }
  
  .gift-icon-pulse {
    animation: gift-icon-pulse 3s ease-in-out infinite;
  }
  
  @keyframes gradient-shift {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
</style> 