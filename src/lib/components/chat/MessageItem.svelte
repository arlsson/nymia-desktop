<script lang="ts">
// Component: src/lib/components/chat/MessageItem.svelte
// Description: Individual message item with hover states and copy functionality
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Manages hover and copy states internally
// - Handles both text messages and gift display
// - Clean separation of individual message concerns
// - FIXED: Added proper text overflow handling to prevent horizontal scrolling
// - FIXED: Added right padding to prevent copy button overlap with text

  import GiftDisplay from './GiftDisplay.svelte';
  import type { ChatMessage } from '$lib/types';
  import { Copy } from 'lucide-svelte';

  // --- Props ---
  export let message: ChatMessage;
  export let currencySymbol: string = 'VRSC';

  // --- State ---
  let isHovered = false;
  let isCopied = false;

  function handleMouseEnter() {
    isHovered = true;
  }

  function handleMouseLeave() {
    isHovered = false;
  }

  // Copy message text to clipboard
  async function copyMessage(messageText: string) {
    try {
      await navigator.clipboard.writeText(messageText);
      console.log('Message copied to clipboard');
      
      // Show feedback
      isCopied = true;
      setTimeout(() => {
        isCopied = false;
      }, 1500);
      
    } catch (err) {
      console.error('Failed to copy message:', err);
      // Fallback for older browsers
      const textArea = document.createElement('textarea');
      textArea.value = messageText;
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      
      // Show feedback even for fallback
      isCopied = true;
      setTimeout(() => {
        isCopied = false;
      }, 1500);
    }
  }
</script>

<div 
  class="message-item mb-1 px-2 py-0.5 -mx-2 rounded transition-colors relative"
  on:mouseenter={handleMouseEnter}
  on:mouseleave={handleMouseLeave}
>
  
  <!-- Gift Display (if message has amount) -->
  {#if message.amount > 0}
    <GiftDisplay 
      amount={message.amount} 
      direction={message.direction} 
      {currencySymbol} 
    />
  {/if}
  
  <!-- Message Text -->
  {#if message.text.trim()}
    <div class="message-text text-dark-text-primary text-sm leading-relaxed font-mono relative pr-8">
      {message.text}
      
      <!-- Copy Icon (appears on hover) or Copied feedback -->
      {#if isCopied}
        <div class="absolute right-0 top-0 px-2 py-1 text-xs text-green-400 bg-dark-bg-tertiary rounded">
          Copied!
        </div>
      {:else if isHovered}
        <button
          class="absolute right-0 top-0 p-1 text-dark-text-secondary hover:text-dark-text-primary bg-dark-bg-secondary hover:bg-dark-bg-tertiary rounded transition-colors"
          on:click|stopPropagation={() => copyMessage(message.text)}
          title="Copy message"
        >
          <Copy size={14} />
        </button>
      {/if}
    </div>
  {/if}
</div>

<style>
  .message-item {
    position: relative;
    /* Ensure the item itself doesn't overflow horizontally */
    max-width: 100%;
    overflow: hidden;
  }

  .message-text {
    font-family: 'IBM Plex Mono', monospace;
    line-height: 1.5;
    /* Enhanced text wrapping to prevent horizontal overflow */
    word-wrap: break-word;
    word-break: break-word;
    overflow-wrap: break-word;
    hyphens: auto;
    /* Ensure text doesn't overflow horizontally */
    max-width: 100%;
    overflow: hidden;
    /* Add white-space handling for better text flow */
    white-space: pre-wrap;
  }
</style> 