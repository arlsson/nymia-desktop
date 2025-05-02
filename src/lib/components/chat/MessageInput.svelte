<script lang="ts">
// Component: src/lib/components/chat/MessageInput.svelte
// Description: Input area for composing and sending chat messages.
// Changes:
// - Renamed "Send VRSC" to "Gift VRSC" with rainbow gradient style
// - Added confirmation dialog when sending messages/gifts
// - Moved informational text to the bottom of the component
// - Better integrated message and gift functionality
// - Added animation effects to gift button and input
// - Ensured confirmation dialog appears on send click.
// - FIX: Corrected logic to show confirmation when only gift amount is entered.
// - STYLE: Matched confirmation dialog buttons to NewChatModal styles.
// - FEAT: Added privateBalance prop and validation to prevent sending more than available.

  import { createEventDispatcher } from 'svelte';
  import { Send, DollarSign, Gift, X, Check, AlertTriangle } from 'lucide-svelte';
  import type { PrivateBalance } from '$lib/types'; // Import PrivateBalance

  // --- Constants ---
  const MAX_MESSAGE_LENGTH = 412; // Maximum message length in bytes/characters
  const TX_FEE = 0.0001;

  // --- Props ---
  export let privateBalance: PrivateBalance = null; // Add privateBalance prop

  // --- State ---
  let messageText: string = '';
  let showFundsInput: boolean = false; // State to control fund input visibility
  let amountToSend: number | null = null;
  let showConfirmation: boolean = false;
  let insufficientBalanceError: boolean = false;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      sendMessage: { message: string; amount?: number }; 
  }>();

  function handleSubmit() {
    const trimmedMessage = messageText.trim();
    
    // FIX: Corrected condition to trigger confirmation if message OR gift amount is valid
    const isMessageValid = trimmedMessage.length > 0 && trimmedMessage.length <= MAX_MESSAGE_LENGTH;
    const isAmountValid = amountToSend !== null && amountToSend > 0;

    if (!isMessageValid && !isAmountValid) return; // Neither message nor valid gift amount
    if (trimmedMessage.length > MAX_MESSAGE_LENGTH) return; // Message too long

    console.log("handleSubmit called, setting showConfirmation to true"); // Debug log
    // Show confirmation dialog instead of immediately sending
    showConfirmation = true;
  }

  function confirmSend() {
    const trimmedMessage = messageText.trim();
    
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
    showConfirmation = false;
  }

  function cancelSend() {
    showConfirmation = false;
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Send message on Enter key press (unless Shift is held for newline)
    if (event.key === 'Enter' && !event.shiftKey && !showConfirmation) { // Add !showConfirmation here
      event.preventDefault(); // Prevent default newline behavior
      handleSubmit();
    }
    
    // Cancel confirmation on Escape key
    if (event.key === 'Escape' && showConfirmation) {
      cancelSend();
    }
  }

  function toggleFundsInput() {
      showFundsInput = !showFundsInput;
      if (!showFundsInput) {
          amountToSend = null; // Clear amount if hiding the input
      }
  }

  // Check if amount exceeds balance
  $: if (amountToSend !== null && privateBalance !== null) {
    insufficientBalanceError = amountToSend > (privateBalance - TX_FEE);
  } else {
    insufficientBalanceError = false;
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
  $: summaryText = amountToSend && amountToSend > 0 
      ? `Send ${messageText.trim() ? 'a message with ' : ''}${amountToSend} VRSC gift` 
      : 'Send message';
      
  // FIX: Corrected disabled logic: button enabled if (message is valid OR amount is valid) AND confirmation is not shown AND balance is sufficient
  $: sendButtonDisabled = 
    (messageText.trim().length > MAX_MESSAGE_LENGTH) || // Message too long
    (messageText.trim().length === 0 && (amountToSend === null || amountToSend <= 0)) || // No message and no gift
    (amountToSend !== null && privateBalance !== null && amountToSend > (privateBalance - TX_FEE)) || // Insufficient balance
    showConfirmation; // Confirmation dialog is open

</script>

<div class="p-3 pt-2 bg-white">
    <!-- Main message input area -->
    <div class="relative mb-1.5">
        <textarea
            bind:value={messageText}
            on:keydown={handleKeyDown}
            rows="2"
            placeholder="Type your message... (412 characters max)"
            class={`w-full py-2 px-3 border rounded resize-none focus:outline-none focus:ring-1 text-sm h-20 ${isOverLimit ? 'border-red-300 focus:ring-red-500 focus:border-red-300' : 'border-gray-200 focus:ring-[#419A6A] focus:border-[#419A6A]'}`}
            disabled={showConfirmation}
        ></textarea>
        
        <!-- Character Counter -->
        <div class={`absolute bottom-2 right-3 text-xs ${charCountClass}`}>
            {messageLength}/{MAX_MESSAGE_LENGTH}
        </div>
    </div>
    
    <!-- Bottom row with send button and gift button -->
    <div class="flex items-center justify-between">
        <!-- Left side: Gift VRSC Button -->
        <div class="flex items-center">
            <!-- Gift Button with Rainbow Styling -->
            <button 
                on:click={toggleFundsInput}
                disabled={showConfirmation}
                class="gift-button relative overflow-hidden flex items-center rounded-md py-1.5 px-3 text-xs font-medium transition-all duration-150 hover:shadow-md border border-transparent"
                class:gift-button-active={showFundsInput}
            >
                <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-blue-500 via-teal-400 via-yellow-400 to-orange-500 gift-gradient-shift opacity-90"></div>
                <div class="relative z-10 flex items-center text-white">
                    <Gift size={14} class="mr-1.5" />
                    Gift VRSC
                </div>
            </button>
        </div>
        
        <!-- Right Side: Send Button -->
        <button
            type="button" 
            on:click={handleSubmit}
            disabled={sendButtonDisabled}
            class="p-1.5 rounded text-white hover:bg-[#378A5A] focus:outline-none focus:ring-1 focus:ring-offset-1 focus:ring-[#419A6A] disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150"
            style="background-color: {sendButtonDisabled ? '#9fcfb8' : '#419A6A'};"
        >
            <Send size={16} />
        </button>
    </div>
    
    <!-- Optional Funds Input Area - Redesigned -->
    {#if showFundsInput}
        <div class="mt-2 p-3 border border-gray-200 rounded bg-gray-50 shadow-sm">
            <div class="flex items-center space-x-3">
                <div class="font-medium text-sm text-gray-600">Gift amount:</div>
                <div class="relative flex-grow max-w-[180px]">
                    <input 
                        type="number" 
                        step="any" 
                        min="0" 
                        inputmode="decimal"
                        bind:value={amountToSend} 
                        placeholder="0.0000" 
                        class={`w-full py-1.5 px-2 border rounded text-sm focus:outline-none focus:ring-1 pr-12 ${insufficientBalanceError ? 'border-red-400 focus:ring-red-500 focus:border-red-500' : 'border-gray-200 focus:ring-[#419A6A] focus:border-[#419A6A]'}`}
                        disabled={showConfirmation}
                    />
                    <div class="absolute right-2 top-1/2 -translate-y-1/2 text-sm text-gray-600 font-medium">
                        VRSC
                    </div>
                </div>
            </div>
            
            <!-- Only show error message when balance is insufficient -->
            {#if insufficientBalanceError}
            <div class="text-xs mt-2">
                <span class="text-red-600 flex items-center">
                    <AlertTriangle size=14 class="mr-1 shrink-0"/> Insufficient balance (max: {(privateBalance !== null ? (privateBalance - TX_FEE) : 0).toFixed(4)} VRSC)
                </span>
            </div>
            {/if}

            <div class="text-xs mt-2 text-gray-500">
                Adding a gift to your message makes it special and supports the receiver.
            </div>
        </div>
    {/if}
    
    <!-- Information text (moved to the bottom) -->
    <div class="mt-2 text-xs text-center text-gray-500">
        Messages cost 0.0001 VRSC and are end-to-end encrypted.
    </div>
    
    <!-- Confirmation Dialog -->
    {#if showConfirmation}
        <div class="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50" on:keydown={handleKeyDown} role="dialog" aria-modal="true" aria-labelledby="confirmation-title">
            <!-- STYLE: Adjusted padding and added border like NewChatModal -->
            <div class="bg-white rounded-lg shadow-xl w-full max-w-sm overflow-hidden transform transition-all duration-300 border border-gray-100">
                <!-- STYLE: Matched header padding and background -->
                <div class="flex justify-between items-center p-3 border-b border-gray-200 bg-gray-50">
                    <h3 id="confirmation-title" class="font-semibold text-gray-800 text-base">Confirm Send</h3>
                    <button on:click={cancelSend} class="text-gray-400 hover:text-gray-600 p-1 rounded-full hover:bg-gray-100 transition-colors focus:outline-none focus:ring-1 focus:ring-gray-200" aria-label="Close confirmation">
                        <X size={16} strokeWidth={2.5}/>
                    </button>
                </div>
                
                <!-- STYLE: Adjusted body padding -->
                <div class="p-4">
                    <p class="text-gray-600 mb-3 text-sm">Are you sure you want to send this?</p>
                    
                    <!-- Message Preview -->
                    {#if messageText.trim()}
                        <div class="bg-gray-50 p-2 rounded text-sm mb-2 border border-gray-200">
                            {messageText}
                        </div>
                    {/if}
                    
                    <!-- Gift Preview -->
                    {#if amountToSend && amountToSend > 0}
                        <div class="flex items-center p-2 mb-2 gift-notification-container relative overflow-hidden rounded">
                            <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-blue-500 via-teal-400 via-yellow-400 to-orange-500 gift-gradient-shift opacity-80"></div>
                            <div class="relative z-10 flex items-center text-white font-medium">
                                <Gift size={14} class="mr-1.5" />
                                <span>Gift: {amountToSend} VRSC</span>
                            </div>
                        </div>
                    {/if}
                    
                    <!-- Fee Details -->
                    <div class="mt-3 mb-4 text-xs text-gray-500 flex justify-between items-center">
                        <span>Transaction fee: 0.0001 VRSC</span>
                        <span class="font-medium">
                            Total: {((amountToSend || 0) + 0.0001).toFixed(8)} VRSC
                        </span>
                    </div>
                </div>
                
                <!-- STYLE: Matched footer padding, background, and button styles -->
                <div class="px-4 py-3 bg-gray-50 border-t border-gray-200 flex justify-end space-x-3">
                    <button 
                        type="button"
                        on:click={cancelSend}
                        class="py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-200 transition-colors"
                    >
                        Cancel
                    </button>
                    <button 
                        type="button"
                        on:click={confirmSend}
                        class="py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-60 disabled:cursor-not-allowed transition-colors hover:bg-green-700 flex items-center"
                        style={`background-color: #419A6A;`}
                    >
                        <Check size={14} class="mr-1.5" />
                        Confirm
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    /* Gift button animation */
    .gift-button {
        transition: all 0.2s ease;
    }
    
    .gift-button-active {
        transform: scale(1.05);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    }
    
    /* Gradient animation */
    .gift-gradient-shift {
        background-size: 200% 200%;
        animation: gradient-shift 6s linear infinite;
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