<script lang="ts">
// Component: src/lib/components/chat/MessageInput.svelte
// Description: Input area for composing and sending chat messages.
// Changes:
// - Completely redesigned with larger text area (2 lines default)
// - Improved funds attachment UI with clear labeling
// - Removed spinners from amount input
// - Revised informational text about message privacy

  import { createEventDispatcher } from 'svelte';
  import { Send, DollarSign } from 'lucide-svelte';

  // --- Constants ---
  const MAX_MESSAGE_LENGTH = 412; // Maximum message length in bytes/characters

  // --- State ---
  let messageText: string = '';
  let showFundsInput: boolean = false; // State to control fund input visibility
  let amountToSend: number | null = null;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      sendMessage: { message: string; amount?: number }; 
  }>();

  function handleSubmit() {
    const trimmedMessage = messageText.trim();
    if (!trimmedMessage && (!amountToSend || amountToSend <= 0)) return; // Don't send empty messages without funds
    if (trimmedMessage.length > MAX_MESSAGE_LENGTH) return; // Don't send if exceeds limit

    console.log(`MessageInput: Sending message: "${trimmedMessage}", Amount: ${amountToSend}`);
    
    dispatch('sendMessage', {
      message: trimmedMessage, // Allow empty message if sending funds
      // Only include amount if it's a positive number
      amount: amountToSend && amountToSend > 0 ? amountToSend : undefined 
    });

    // Reset input fields
    messageText = '';
    amountToSend = null;
    showFundsInput = false; 
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Send message on Enter key press (unless Shift is held for newline)
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault(); // Prevent default newline behavior
      handleSubmit();
    }
  }

  function toggleFundsInput() {
      showFundsInput = !showFundsInput;
      if (!showFundsInput) {
          amountToSend = null; // Clear amount if hiding the input
      }
  }

  // Calculate message length and status
  $: messageLength = messageText.length;
  $: isOverLimit = messageLength > MAX_MESSAGE_LENGTH;
  $: remainingChars = MAX_MESSAGE_LENGTH - messageLength;
  $: charCountClass = isOverLimit 
      ? 'text-red-500 font-medium' 
      : remainingChars < 20 
        ? 'text-orange-500' 
        : 'text-gray-400';

</script>

<div class="p-3 pt-2 bg-white">
    <!-- Main message input area - now taller -->
    <div class="relative mb-1.5">
        <textarea
            bind:value={messageText}
            on:keydown={handleKeyDown}
            rows="2"
            placeholder="Type your message... (412 characters max)"
            class={`w-full py-2 px-3 border rounded resize-none focus:outline-none focus:ring-1 text-sm h-20 ${isOverLimit ? 'border-red-300 focus:ring-red-500 focus:border-red-300' : 'border-gray-200 focus:ring-[#419A6A] focus:border-[#419A6A]'}`}
        ></textarea>
        
        <!-- Character Counter -->
        <div class={`absolute bottom-2 right-3 text-xs ${charCountClass}`}>
            {messageLength}/{MAX_MESSAGE_LENGTH}
        </div>
    </div>
    
    <!-- Bottom row with send button and attach funds button -->
    <div class="flex items-center justify-between">
        <!-- Left side: Info text and Send Funds option -->
        <div class="flex items-center">
            <!-- Add Funds Button -->
            <button 
                on:click={toggleFundsInput}
                class={`flex items-center rounded-md py-1 px-2 mr-2 text-xs font-medium transition-colors duration-150 ${showFundsInput ? 'bg-[#419A6A] text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
            >
                <DollarSign size={14} class="mr-1" />
                Send VRSC
            </button>
            
            <!-- Information Text -->
            <span class="text-xs text-gray-500 hidden sm:inline">
                Messages cost 0.0001 VRSC and are end-to-end encrypted.
            </span>
        </div>
        
        <!-- Right Side: Send Button -->
        <button
            on:click={handleSubmit}
            disabled={isOverLimit || (!messageText.trim() && (!amountToSend || amountToSend <= 0))}
            class="p-1.5 rounded text-white hover:bg-[#378A5A] focus:outline-none focus:ring-1 focus:ring-offset-1 focus:ring-[#419A6A] disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150"
            style="background-color: {(isOverLimit || (!messageText.trim() && (!amountToSend || amountToSend <= 0))) ? '#9fcfb8' : '#419A6A'};"
        >
            <Send size={16} />
        </button>
    </div>
    
    <!-- Optional Funds Input Area - completely redesigned -->
    {#if showFundsInput}
        <div class="mt-2 flex items-center space-x-3 p-3 border border-gray-200 rounded bg-gray-50">
            <div class="font-medium text-sm text-gray-600">Amount:</div>
            <div class="relative flex-grow max-w-[180px]">
                <input 
                    type="text" 
                    inputmode="decimal"
                    bind:value={amountToSend} 
                    placeholder="0.0000" 
                    class="w-full py-1.5 px-2 border border-gray-200 rounded text-sm focus:outline-none focus:ring-1 focus:ring-[#419A6A] focus:border-[#419A6A] pr-12"
                />
                <div class="absolute right-2 top-1/2 -translate-y-1/2 text-sm text-gray-600 font-medium">
                    VRSC
                </div>
            </div>
            
            <div class="text-xs py-1 px-2 bg-gray-100 rounded text-gray-600">
                + 0.0001 fee
            </div>
        </div>
    {/if}
</div> 