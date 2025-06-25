<script lang="ts">
// Component: src/lib/components/chat/MessageInput.svelte
// Description: Compact Discord-like input area for composing and sending chat messages (Dark Theme).
// Changes:
// - REDESIGN: Complete layout overhaul to Discord-like compact design
// - Single line input that expands vertically when needed
// - Unified border containing message text, character counter, gift icon, and send button
// - Gift amount input appears as popup overlay above gift icon
// - Subtle character counter styling inside input
// - Gift icon with pulse/glow and color shift animations
// - Error messages shown inside gift overlay
// - Maintained all existing functionality with new compact layout
// - DYNAMIC LIMITS: Implemented dynamic character limits based on current user's VerusID name length
// - Replaced hardcoded 412 character limit with calculation considering memo format overhead
// - Added verusIdName prop and integrated with messageLimit utility function
// - BUG FIX: Fixed gift overlay cancel button to properly clear amount instead of just hiding overlay
// - Added dynamic currency symbol support based on selected blockchain
// - REFACTOR: Extracted gift overlay into separate GiftOverlay component for better code organization
// - UI IMPROVEMENT: Redesigned gift amount badge with better contrast, standard styling, and improved readability
// - FAST MESSAGES: Changed send button logic to use UTXO availability instead of pending transaction state
//   Allows rapid message sending when multiple UTXOs are available

  import { createEventDispatcher } from 'svelte';
  import { Send, Gift, X, Check } from 'lucide-svelte';
  import type { PrivateBalance, UtxoInfo } from '$lib/types'; // Import PrivateBalance and UtxoInfo
  import { calculateMaxMessageLength } from '$lib/utils/messageLimit';
  import GiftOverlay from './GiftOverlay.svelte';
  import Modal from '../Modal.svelte';
  import Button from '../Button.svelte';

  // --- Constants ---
  const TX_FEE = 0.0001;

  // --- Props ---
  export let privateBalance: PrivateBalance = null; // Add privateBalance prop
  export let verusIdName: string; // Current user's VerusID name for dynamic message limit calculation
  export let currencySymbol: string = 'VRSC'; // Dynamic currency symbol
  export let utxoInfo: UtxoInfo | null = null; // NEW: UTXO information for Fast Messages impact

  // --- State ---
  let messageText: string = '';
  let showFundsInput: boolean = false; // State to control fund input visibility
  let amountToSend: number | null = null;
  let showConfirmation: boolean = false;
  let textareaElement: HTMLTextAreaElement;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      sendMessage: { message: string; amount?: number }; 
  }>();

  function handleSubmit() {
    const trimmedMessage = messageText.trim();
    
    // FIX: Corrected condition to trigger confirmation if message OR gift amount is valid
    const isMessageValid = trimmedMessage.length > 0 && trimmedMessage.length <= maxMessageLength;
    const isAmountValid = amountToSend !== null && amountToSend > 0;

    if (!isMessageValid && !isAmountValid) return; // Neither message nor valid gift amount
    if (trimmedMessage.length > maxMessageLength) return; // Message too long

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
    resetTextareaHeight();
  }

  function cancelSend() {
    showConfirmation = false;
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Send message on Enter key press (unless Shift is held for newline)
    if (event.key === 'Enter' && !event.shiftKey && !showConfirmation && !showFundsInput) { // Don't send when gift overlay is open
      event.preventDefault(); // Prevent default newline behavior
      handleSubmit();
    }

    // Close gift input on Escape (confirmation dialog escape is now handled by Modal component)
    if (event.key === 'Escape' && showFundsInput) {
      toggleFundsInput();
    }
  }

  function toggleFundsInput() {
      showFundsInput = !showFundsInput;
      if (!showFundsInput) {
          // Don't clear amount when hiding - keep it for display
      }
  }

  function confirmGiftAmount(event: CustomEvent<{ amount: number }>) {
    const { amount } = event.detail;
    amountToSend = amount;
    showFundsInput = false;
  }

  function clearGiftAmount() {
    amountToSend = null;
    showFundsInput = false;
  }

  function resetTextareaHeight() {
    if (textareaElement) {
      textareaElement.style.height = 'auto';
    }
  }

  function adjustTextareaHeight() {
    if (textareaElement) {
      textareaElement.style.height = 'auto';
      textareaElement.style.height = Math.min(textareaElement.scrollHeight, 120) + 'px';
    }
  }

  // Calculate dynamic message limit and status
  $: maxMessageLength = calculateMaxMessageLength(verusIdName);
  $: messageLength = messageText.length;
  $: isOverLimit = messageLength > maxMessageLength;
  $: remainingChars = maxMessageLength - messageLength;
  $: charCountClass = isOverLimit 
      ? 'text-red-400 font-medium' 
      : remainingChars < 20 
        ? 'text-orange-400' 
        : 'text-white/45';
  $: summaryText = amountToSend && amountToSend > 0 
      ? `Send ${messageText.trim() ? 'a message with ' : ''}${amountToSend} ${currencySymbol} gift` 
      : 'Send message';
      
  // Send button disabled logic: use Fast Messages availability instead of pending transaction state
  $: sendButtonDisabled = 
    (messageText.trim().length > maxMessageLength) || // Message too long
    (messageText.trim().length === 0 && (amountToSend === null || amountToSend <= 0)) || // No message and no gift
    (amountToSend !== null && privateBalance !== null && amountToSend > (privateBalance - TX_FEE)) || // Insufficient balance
    (utxoInfo !== null && utxoInfo.usable_utxos === 0) || // No fast messages available
    showConfirmation; // Confirmation dialog is open

  // Tooltip text for the send button
  $: sendButtonTitle = utxoInfo !== null && utxoInfo.usable_utxos === 0 
    ? "No fast messages available - need UTXOs to send" 
    : utxoInfo === null 
      ? "Loading UTXO information..." 
      : "Send message/gift";

  // Function to handle input and enforce ASCII
  function handleInput(event: Event) {
      const target = event.target as HTMLTextAreaElement;
      const originalValue = target.value;
      const filteredValue = originalValue.replace(/[^\x00-\x7F]/g, '');
      
      if (originalValue !== filteredValue) {
          // Preserve cursor position after filtering
          const start = target.selectionStart;
          const end = target.selectionEnd;
          messageText = filteredValue; // Update the bound variable
          // Use tick to wait for Svelte to update the DOM, then restore cursor
          import('svelte').then(({ tick }) => {
              tick().then(() => {
                  // Adjust cursor position based on removed characters before the cursor
                  let charsRemovedBeforeStart = 0;
                  for (let i = 0; i < Math.min(start, originalValue.length); i++) {
                      if (originalValue.charCodeAt(i) > 127) {
                          charsRemovedBeforeStart++;
                      }
                  }
                  const newStart = start - charsRemovedBeforeStart;
                  target.setSelectionRange(newStart, newStart);
              });
          });
      } else {
          messageText = originalValue; // No change, just update bound variable normally
      }
      
      // Adjust textarea height after input
      adjustTextareaHeight();
  }

</script>

<div class="p-4 bg-[#070A07]">
    <!-- Compact unified input container -->
    <div class="relative mb-2">
        <!-- Main input container with unified border -->
        <div class={`relative flex items-end border rounded-md bg-[#121214] ${isOverLimit ? 'border-red-600' : 'border-white/5'} transition-colors`}>
            <!-- Message textarea -->
            <textarea
                bind:this={textareaElement}
                bind:value={messageText}
                on:input={handleInput}
                on:keydown={handleKeyDown}
                rows="1"
                placeholder="Type your message..."
                class="flex-1 py-[8px] px-4 bg-transparent text-dark-text-primary placeholder:text-white/45 resize-none focus:outline-none text-sm min-h-[44px] max-h-[120px] leading-snug"
                style="font-family: 'IBM Plex Mono', monospace;"
                disabled={showConfirmation}
            ></textarea>
            
            <!-- Right side controls container -->
            <div class="flex items-center space-x-2 px-3 py-2">
                <!-- Character counter -->
                <div class={`text-xs ${charCountClass} select-none cursor-default`}>
                    {messageLength}/{maxMessageLength}
                </div>
                
                <!-- Gift button with animation -->
                <div class="relative">
                    <button 
                        on:click={toggleFundsInput}
                        disabled={showConfirmation}
                        class="gift-button p-1.5 rounded-md transition-all duration-200 hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
                        class:gift-button-active={showFundsInput}
                        title={`Add a gift of ${currencySymbol} to your message`}
                    >
                        <Gift size={26} class="gift-icon" />
                    </button>
                    
                    <!-- Gift amount overlay -->
                    <GiftOverlay
                        show={showFundsInput}
                        bind:amountToSend
                        {privateBalance}
                        {currencySymbol}
                        {showConfirmation}
                        {utxoInfo}
                        on:confirm={confirmGiftAmount}
                        on:cancel={toggleFundsInput}
                        on:clear={clearGiftAmount}
                    />
                </div>
                
                <!-- Show confirmed gift amount badge in normal flow -->
                {#if amountToSend && amountToSend > 0 && !showFundsInput}
                    <div class="bg-purple-600 border border-purple-500 text-white text-xs px-2.5 py-1.5 rounded-md font-medium flex items-center shadow-sm whitespace-nowrap gift-amount-badge">
                        
                        <span class="font-mono tracking-wide">{amountToSend} {currencySymbol}</span>
                        <button 
                            on:click={clearGiftAmount}
                            class="ml-2 hover:bg-purple-700 rounded-sm p-0.5"
                            title="Remove gift"
                        >
                            <X size={12} stroke-width="2.5" class="text-purple-200 hover:text-white" />
                        </button>
                    </div>
                {/if}
                
                <!-- Send button -->
                <button
                    type="button" 
                    on:click={handleSubmit}
                    disabled={sendButtonDisabled}
                    title={sendButtonTitle}
                    class="p-1.5 rounded text-white hover:bg-brand-green-hover focus:outline-none focus:ring-1 focus:ring-offset-1 focus:ring-offset-dark-bg-secondary focus:ring-brand-green disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150 shrink-0"
                    style="background-color: {sendButtonDisabled ? '#9fcfb8' : '#419A6A'};"
                >
                    <Send size={16} />
                </button>
            </div>
        </div>
    </div>
    
    <!-- Confirmation Dialog -->
    <Modal 
        show={showConfirmation} 
        size="sm" 
        on:close={cancelSend}
    >
        <!-- Header -->
        <svelte:fragment slot="header" let:modalHeaderId let:handleClose>
            <h3 class="font-semibold text-dark-text-primary text-base flex-grow cursor-default select-none" id={modalHeaderId}>Confirm Send</h3>
            <button 
                on:click={handleClose} 
                class="text-dark-text-secondary hover:text-dark-text-primary p-1 rounded-full hover:bg-dark-bg-tertiary transition-colors focus:outline-none focus:ring-1 focus:ring-dark-border-secondary" 
                aria-label="Close confirmation"
            >
                <X size={16} strokeWidth={2.5}/>
            </button>
        </svelte:fragment>

        <!-- Body -->
        <div class="p-4">
            <p class="text-dark-text-secondary mb-3 text-sm cursor-default select-none">Are you sure you want to send this?</p>
            
            <!-- Message Preview -->
            {#if messageText.trim()}
                <div class="bg-dark-bg-tertiary p-2 rounded text-sm mb-2 border border-dark-border-secondary text-dark-text-primary" style="font-family: 'IBM Plex Mono', monospace; word-wrap: break-word;">
                    {messageText}
                </div>
            {/if}
            
            <!-- Gift Preview with matching gradient -->
            {#if amountToSend && amountToSend > 0}
                <div class="flex items-center p-2 mb-2 gift-notification-container relative overflow-hidden rounded">
                    <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 gift-gradient-shift opacity-90"></div>
                    <div class="relative z-10 flex items-center text-white font-medium">
                        <Gift size={14} class="mr-1.5" />
                        <span>Gift: {amountToSend} {currencySymbol}</span>
                    </div>
                </div>
            {/if}
            
            <!-- Fee Details -->
            <div class="mt-3 mb-4 text-xs text-white/45 flex justify-between items-center cursor-default select-none">
                <span>Transaction fee: 0.0001 {currencySymbol}</span>
                <span class="font-medium text-white cursor-default select-none">
                    Total: {((amountToSend || 0) + 0.0001).toFixed(8)} {currencySymbol}
                </span>
            </div>
        </div>

        <!-- Footer -->
        <svelte:fragment slot="footer">
            <div class="flex justify-end space-x-3">
                <Button 
                    variant="secondary"
                    on:click={cancelSend}
                >
                    Cancel
                </Button>
                <Button 
                    variant="primary"
                    iconComponent={Check}
                    on:click={confirmSend}
                >
                    Confirm
                </Button>
            </div>
        </svelte:fragment>
    </Modal>
</div>

<style>
    /* Simple gift button styling that actually works */
    .gift-button {
        transition: all 0.3s ease;
        position: relative;
    }
    
    /* Target Lucide SVG with correct stroke properties */
    :global(.gift-button .gift-icon) {
        color: #8b5cf6;
        stroke: currentColor;
        fill: none;
        animation: gift-pulse 2s ease-in-out infinite;
        filter: drop-shadow(0 0 4px rgba(139, 92, 246, 0.6));
        transform-origin: center;
        transition: none;
    }
    
    /* Hover state for button */
    .gift-button:hover {
        transform: scale(1.1);
    }
    
    /* Hover animation for icon - lighter purple and enhanced glow */
    :global(.gift-button:hover .gift-icon) {
        color: #c4b5fd;
        filter: drop-shadow(0 0 8px rgba(139, 92, 246, 1));
    }
    
    /* Active state when overlay is open */
    .gift-button-active {
        background-color: rgba(139, 92, 246, 0.2);
        transform: scale(1.05);
    }
    
    :global(.gift-button-active .gift-icon) {
        animation: gift-bounce 1s ease-in-out infinite;
        color: #7c3aed;
        filter: drop-shadow(0 0 12px rgba(139, 92, 246, 1));
    }
    
    /* Simple, effective animations */
    @keyframes gift-pulse {
        0%, 100% {
            transform: scale(1);
            filter: drop-shadow(0 0 4px rgba(139, 92, 246, 0.6));
        }
        50% {
            transform: scale(1.15);
            filter: drop-shadow(0 0 12px rgba(139, 92, 246, 1));
        }
    }
    
    @keyframes gift-bounce {
        0%, 100% {
            transform: scale(1);
        }
        25% {
            transform: scale(1.2) rotate(5deg);
        }
        50% {
            transform: scale(1.3);
        }
        75% {
            transform: scale(1.2) rotate(-5deg);
        }
    }
    
    /* Enhanced gradient animation for confirmation */
    .gift-gradient-shift {
        background-size: 200% 200%;
        animation: gradient-shift 3s linear infinite;
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
    
    /* Gift amount badge styling */
    .gift-amount-badge {
        backdrop-filter: blur(8px);
        transition: all 0.2s ease;
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
    
    /* Textarea scrollbar styling */
    textarea::-webkit-scrollbar {
        width: 4px;
    }
    
    textarea::-webkit-scrollbar-track {
        background: transparent;
    }
    
    textarea::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.2);
        border-radius: 2px;
    }
    
    textarea::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.3);
    }
</style> 