<script lang="ts">
// Component: src/lib/components/chat/ConversationView.svelte
// Description: Displays the chat messages for the selected conversation and the message input area.
// Changes:
// - More subtle, modern design with cleaner appearance
// - Reduced padding and more compact layout
// - Minimal usage of brand color
// - More contemporary chat bubble styling
// - Fixed duplicate class:text-gray-800 attribute
// - Added more gray tones to backgrounds
// - Added display for received amount on historical messages.
// - Added relative time display using confirmations for historical messages.
// - Imported and used formatRelativeTimeFromConfirmations utility.
// - Updated message type definition to match ChatMessage from types.ts
// - Added privateBalance prop to pass down to MessageInput.
// - Added debug log to reactive block checking messages prop.
// - Added simpler gift display for sent messages.

  import { createEventDispatcher, tick } from 'svelte';
  import MessageInput from './MessageInput.svelte';
  import type { ChatMessage, PrivateBalance } from '$lib/types'; // Import ChatMessage & PrivateBalance types
  import { formatRelativeTimeFromConfirmations, formatRelativeTimeFromTimestamp } from '$lib/utils/timeFormatter'; // Import utilities
  // Update imports to remove ArrowDown
  import { Gift } from 'lucide-svelte';

  // Use ChatMessage type directly
  type Message = ChatMessage;

  // --- Props ---
  export let contactName: string | null = null;
  export let messages: Message[] = [];
  export let privateBalance: PrivateBalance = null; // Add privateBalance prop
  export let isTransactionPending: boolean = false; // Add pending state prop

  // --- State ---
  let chatContainer: HTMLElement;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      sendMessage: { message: string; amount?: number }; 
  }>();

  function handleSendMessage(event: CustomEvent<{ message: string; amount?: number }>) {
    dispatch('sendMessage', event.detail);
  }

  // Scroll to bottom when messages change or contact changes
  $: if (messages || contactName) {
      // DEBUG LOG:
      console.log(`ConversationView: messages prop updated or contact changed. Message count: ${messages?.length ?? 0}. Scrolling to bottom.`);
      scrollToBottom();
  }

  async function scrollToBottom() {
    // Wait for the DOM to update
    await tick(); 
    if (chatContainer) {
      chatContainer.scrollTop = chatContainer.scrollHeight;
    }
  }

</script>

<div class="flex flex-col h-full bg-gray-50">
  {#if contactName}
    <!-- Chat Header with improved name display -->
    <div class="flex items-center h-[50px] px-3 bg-white border-b border-gray-200 flex-shrink-0 shadow-sm">
      <span class="font-medium text-base text-gray-800 truncate max-w-[80%]">{contactName}</span>
      <!-- Add other header elements like contact info button later -->
    </div>

    <!-- Message List (Scrollable & Reversed) -->
    <div 
        bind:this={chatContainer} 
        class="flex-grow p-3 overflow-y-auto bg-gray-100 flex flex-col-reverse"
    >
      {#if messages.length > 0}
          {#each messages as message (message.id)}
            <div class="flex mb-3" class:justify-end={message.sender === 'self'}>
              <div 
                class="py-2 px-3 rounded-lg max-w-[75%] shadow-sm text-gray-800 flex flex-col"
                class:bg-white={message.sender !== 'self'}
                class:bg-[#F0F7F4]={message.sender === 'self'} 
                class:border-[#E0F0E9]={message.sender === 'self'}
                class:border={message.sender === 'self'}
              >
                <!-- Enhanced Amount Display with Colorful Gradient & Animation (if received and > 0) -->
                {#if message.direction === 'received' && message.amount > 0}
                    <div class="gift-notification-container relative mb-2.5 overflow-hidden rounded-md p-0.5">
                        <!-- Rainbow gradient background -->
                        <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-blue-500 via-teal-400 via-yellow-400 to-orange-500 gift-pulse"></div>
                        <div class="flex items-center rounded-md bg-white bg-opacity-85 backdrop-blur-sm p-2.5 relative">
                           <div class="flex items-center">
                               <div class="p-1.5 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 rounded-full mr-2.5 text-white gift-icon-pulse">
                                   <Gift size={18} />
                               </div>
                               <div class="flex flex-col">
                                   <span class="text-xs font-semibold text-gray-700">
                                       <!-- Updated Text -->
                                       {message.sender} sent you a gift!
                                   </span>
                                   <span class="text-sm font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-600 via-blue-600 to-pink-600">
                                       {message.amount.toFixed(8)} VRSC
                                   </span>
                               </div>
                           </div>
                        </div>
                     </div>
                <!-- Simple Gift Display for Sent Messages (no animation) -->
                {:else if message.direction === 'sent' && message.amount > 0}
                    <div class="flex items-center mb-2.5 bg-[#E0F0E9] p-2 rounded-md">
                        <div class="p-1 bg-[#419A6A] rounded-full mr-2 text-white flex-shrink-0">
                            <Gift size={14} />
                        </div>
                        <div>
                            <span class="text-xs font-medium text-gray-700">You sent a gift</span>
                            <span class="text-sm font-semibold text-[#419A6A] block">
                                {message.amount.toFixed(8)} VRSC
                            </span>
                        </div>
                    </div>
                {/if}
                
                <!-- Message Text -->
                <p class="text-sm" style="font-family: 'IBM Plex Mono', monospace;">{message.text}</p>
                
                <!-- Timestamp / Status Row -->
                <div class="text-xs mt-1.5 text-gray-500 text-right flex justify-end items-center self-end">
                  <!-- Relative time formatting -->
                  <span>
                      {#if message.direction === 'received'}
                          {formatRelativeTimeFromConfirmations(message.confirmations)}
                      {:else} 
                          {formatRelativeTimeFromTimestamp(message.timestamp)}
                      {/if}
                  </span>
                  
                  <!-- Status indicator removed -->
                  <!-- {#if message.sender === 'self' && message.status} ... {/if} -->
                </div>
              </div>
            </div>
          {/each}
      {:else}
          <p class="text-center text-gray-400 text-xs py-4">Start the conversation!</p>
      {/if}
    </div>

    <!-- Message Input Area -->
    <div class="flex-shrink-0 bg-white border-t border-gray-200">
      <MessageInput 
        {privateBalance}  
        {isTransactionPending}
        on:sendMessage={handleSendMessage} 
      />
    </div>

  {:else}
    <!-- Empty State (No Conversation Selected) -->
    <div class="flex-grow flex items-center justify-center bg-gray-100">
      <div class="text-center text-gray-400 p-4">
         <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto h-10 w-10 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
        <p class="mt-2 text-sm font-medium">Select a conversation</p>
        <p class="text-xs">Or start a new chat</p>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Subtle scrollbar styling */
  ::-webkit-scrollbar {
    width: 4px;
  }
  ::-webkit-scrollbar-track {
    background: transparent; 
  }
  ::-webkit-scrollbar-thumb {
    background: #ddd;
    border-radius: 2px;
  }
  ::-webkit-scrollbar-thumb:hover {
    background: #ccc;
  }

  /* Gift notification animation */
  @keyframes gift-pulse {
    0% {
      opacity: 0.85;
      transform: scale(1);
    }
    50% {
      opacity: 0.95;
      transform: scale(1.03);
    }
    100% {
      opacity: 0.85;
      transform: scale(1);
    }
  }
  
  @keyframes gift-icon-pulse {
    0% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(147, 51, 234, 0.5);
    }
    50% {
      transform: scale(1.1);
      box-shadow: 0 0 10px 3px rgba(147, 51, 234, 0.3);
    }
    100% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(147, 51, 234, 0.5);
    }
  }

  .gift-pulse {
    animation: gift-pulse 3s ease-in-out infinite;
    background-size: 200% 200%;
    animation: gift-pulse 3s ease-in-out infinite, gradient-shift 6s linear infinite;
  }
  
  .gift-icon-pulse {
    animation: gift-icon-pulse 2s ease-in-out infinite;
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