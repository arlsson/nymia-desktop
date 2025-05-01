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

  import { createEventDispatcher, tick } from 'svelte';
  import MessageInput from './MessageInput.svelte';

  // --- Type (could be moved to $lib/types) ---
  type Message = {
    id: number | string; // Unique message ID
    sender: 'self' | string; // 'self' or sender's VerusID
    text: string;
    timestamp: number; // Unix timestamp
    status?: 'sent' | 'delivered' | 'failed'; // Delivery status
    // Add other fields like amountSent later
  };

  // --- Props ---
  export let contactName: string | null = null;
  export let messages: Message[] = [];

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
    <!-- Chat Header -->
    <div class="flex items-center h-[50px] px-3 bg-white border-b border-gray-200 flex-shrink-0 shadow-sm">
      <span class="font-medium text-base text-gray-800">{contactName}</span>
      <!-- Add other header elements like contact info button later -->
    </div>

    <!-- Message List (Scrollable) -->
    <div bind:this={chatContainer} class="flex-grow p-3 overflow-y-auto space-y-3 bg-gray-100">
      {#if messages.length > 0}
          {#each messages as message (message.id)}
            <div class="flex" class:justify-end={message.sender === 'self'}>
              <div 
                class="py-2 px-3 rounded-lg max-w-[75%] shadow-sm text-gray-800"
                class:bg-white={message.sender !== 'self'}
                class:bg-[#F0F7F4]={message.sender === 'self'} 
                class:border-[#E0F0E9]={message.sender === 'self'}
                class:border={message.sender === 'self'}
              >
                <p class="text-sm">{message.text}</p>
                <div class="text-xs mt-1 text-gray-500 text-right flex justify-end items-center">
                  <!-- Basic timestamp -->
                  <span>{new Date(message.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                  {#if message.sender === 'self' && message.status}
                     <!-- Status indicator -->
                     <span class="ml-1 text-xs">
                        {#if message.status === 'sent'}
                          <span class="text-gray-400">✓</span>
                        {:else if message.status === 'delivered'}
                          <span class="text-[#419A6A]">✓✓</span>
                        {:else if message.status === 'failed'}
                          <span class="text-red-500">!</span>
                        {/if}
                     </span>
                  {/if}
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
      <MessageInput on:sendMessage={handleSendMessage} />
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
</style> 