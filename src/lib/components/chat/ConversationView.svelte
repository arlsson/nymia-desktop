<script lang="ts">
// Component: src/lib/components/chat/ConversationView.svelte
// Description: Discord-style message display with grouping, avatars, and date separators
// Changes:
// - BREAKING: Complete redesign to Discord-style flat layout
// - Removed speech bubbles in favor of left-aligned flat messages
// - Implemented 30-minute message grouping logic
// - Added Avatar integration for all senders
// - Added date separators between different days
// - Updated to black/dark gray color scheme with minimal brand green
// - Enhanced time display: shows MM/DD/YYYY, HH:MM for older messages, time only for today/yesterday
// - Improved gift display integration
// - Simplified message ordering to chronological (oldest first) for cleaner data flow
// - Added copy message functionality with hover copy icon and visual feedback
// - Improved text contrast throughout the interface
// - Fixed UX: Removed cursor-pointer from message containers to prevent false clickable affordance
// - Added verusIdName prop to pass current user's identity name to MessageInput for dynamic character limits
// - Added dynamic currency symbol support based on selected blockchain

  import { createEventDispatcher, tick } from 'svelte';
  import MessageInput from './MessageInput.svelte';
  import Avatar from '../Avatar.svelte';
  import type { ChatMessage, PrivateBalance } from '$lib/types';
  import { groupMessages, formatMessageTime } from '$lib/utils/messageGrouping';
  import type { MessageGroup } from '$lib/utils/messageGrouping';
  import { Gift, Copy } from 'lucide-svelte';

  // --- Props ---
  export let contactName: string | null = null;
  export let messages: ChatMessage[] = [];
  export let privateBalance: PrivateBalance = null;
  export let isTransactionPending: boolean = false;
  export let verusIdName: string; // Current user's VerusID name for dynamic message limit calculation
  export let currencySymbol: string = 'VRSC'; // Dynamic currency symbol

  // --- State ---
  let chatContainer: HTMLElement;
  let hoveredMessageId: string | null = null;
  let copiedMessageId: string | null = null;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
      sendMessage: { message: string; amount?: number }; 
  }>();

  function handleSendMessage(event: CustomEvent<{ message: string; amount?: number }>) {
    dispatch('sendMessage', event.detail);
  }

  // Scroll to bottom when messages change or contact changes
  $: if (messages || contactName) {
      console.log(`ConversationView: messages prop updated or contact changed. Message count: ${messages?.length ?? 0}. Scrolling to bottom.`);
      scrollToBottom();
  }

  // Group messages for display
  $: messageGroups = groupMessages(messages);

  async function scrollToBottom() {
    await tick(); 
    if (chatContainer) {
      chatContainer.scrollTop = chatContainer.scrollHeight;
    }
  }

  function handleMessageHover(messageId: string | null) {
    hoveredMessageId = messageId;
  }

  // Copy message text to clipboard
  async function copyMessage(messageText: string, messageId: string) {
    try {
      await navigator.clipboard.writeText(messageText);
      console.log('Message copied to clipboard');
      
      // Show feedback
      copiedMessageId = messageId;
      setTimeout(() => {
        copiedMessageId = null;
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
      copiedMessageId = messageId;
      setTimeout(() => {
        copiedMessageId = null;
      }, 1500);
    }
  }
</script>

<div class="flex flex-col h-full bg-dark-bg-primary">
  {#if contactName}
    <!-- Chat Header -->
    <div class="flex items-center h-12 px-4 bg-dark-bg-secondary border-b border-dark-border-primary flex-shrink-0 shadow-sm">
      <Avatar userId={contactName} size="small" showHover={false} />
      <span class="font-semibold text-dark-text-primary ml-3 truncate">{contactName}</span>
    </div>

    <!-- Message List (Scrollable) -->
    <div 
        bind:this={chatContainer} 
        class="flex-grow overflow-y-auto bg-dark-bg-primary flex flex-col px-4 py-2"
    >
      {#if messageGroups.length > 0}
          {#each messageGroups as group (group.id)}
            <div class="message-group mb-6">
              
              <!-- Date Separator -->
              {#if group.showDateSeparator && group.dateLabel}
                <div class="flex items-center my-6">
                  <div class="flex-grow h-px bg-dark-border-primary"></div>
                  <div class="px-3 text-xs font-medium text-white/60 bg-dark-bg-primary">
                    {group.dateLabel}
                  </div>
                  <div class="flex-grow h-px bg-dark-border-primary"></div>
                </div>
              {/if}

              <!-- Message Group -->
              <div class="flex">
                <!-- Avatar (shown only for first message in group) -->
                <div class="flex-shrink-0 mr-3">
                  <Avatar 
                    userId={group.sender === 'self' ? verusIdName : group.sender} 
                    size="medium" 
                    showHover={true} 
                  />
                </div>

                <!-- Messages Content -->
                <div class="flex-grow min-w-0">
                  <!-- Group Header (sender name and timestamp) -->
                  <div class="flex items-baseline mb-1">
                    <span class="font-semibold text-dark-text-primary text-sm mr-2">
                      {group.sender === 'self' ? 'You' : group.sender}
                    </span>
                    <span class="text-xs text-white/45">
                      {formatMessageTime(group.timestamp)}
                    </span>
                  </div>

                  <!-- Messages in group -->
                  {#each group.messages as message (message.id)}
                    <div 
                      class="message-item mb-1 px-2 py-0.5 -mx-2 rounded transition-colors relative"
                      on:mouseenter={() => handleMessageHover(message.id)}
                      on:mouseleave={() => handleMessageHover(null)}
                    >
                      
                      <!-- Gift Display (if message has amount) -->
                      {#if message.amount > 0}
                        <div class="gift-container mb-2">
                          {#if message.direction === 'received'}
                            <!-- Enhanced Gift Display (Received) -->
                            <div class="gift-notification-container relative overflow-hidden rounded-lg p-0.5 max-w-sm">
                              <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-blue-500 via-teal-400 via-yellow-400 to-orange-500 gift-pulse"></div>
                              <div class="flex items-center rounded-lg bg-dark-bg-tertiary bg-opacity-95 backdrop-blur-sm p-3 relative">
                                <div class="flex items-center">
                                  <div class="p-2 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 rounded-full mr-3 text-white gift-icon-pulse">
                                    <Gift size={20} />
                                  </div>
                                  <div class="flex flex-col">
                                    <span class="text-sm font-semibold text-dark-text-primary">
                                      Gift received!
                                    </span>
                                    <span class="text-lg font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 via-blue-400 to-pink-400">
                                      {message.amount.toFixed(8)} {currencySymbol}
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
                                <span class="text-sm font-medium text-dark-text-primary">Gift sent</span>
                                <span class="text-sm font-semibold text-brand-green block">
                                  {message.amount.toFixed(8)} {currencySymbol}
                                </span>
                              </div>
                            </div>
                          {/if}
                        </div>
                      {/if}
                      
                      <!-- Message Text -->
                      {#if message.text.trim()}
                        <div class="message-text text-dark-text-primary text-sm leading-relaxed font-mono">
                          {message.text}
                        </div>
                      {/if}

                      <!-- Copy Icon (appears on hover) or Copied feedback -->
                      {#if message.text.trim()}
                        {#if copiedMessageId === message.id}
                          <div class="absolute right-2 top-1 px-2 py-1 text-xs text-green-400 bg-dark-bg-tertiary rounded">
                            Copied!
                          </div>
                        {:else if hoveredMessageId === message.id}
                          <button
                            class="absolute right-2 top-1 p-1 text-dark-text-secondary hover:text-dark-text-primary bg-dark-bg-secondary hover:bg-dark-bg-tertiary rounded transition-colors"
                            on:click|stopPropagation={() => copyMessage(message.text, message.id)}
                            title="Copy message"
                          >
                            <Copy size={14} />
                          </button>
                        {/if}
                      {/if}
                    </div>
                  {/each}
                </div>
              </div>
            </div>
          {/each}
      {:else}
          <div class="flex-grow flex items-center justify-center">
            <div class="text-center text-dark-text-secondary p-8">
              <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto h-12 w-12 text-dark-text-secondary mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              <p class="text-sm font-medium text-dark-text-primary mb-1">Start the conversation!</p>
              <p class="text-xs text-dark-text-secondary">Send a message or gift to get started.</p>
            </div>
          </div>
      {/if}
    </div>

    <!-- Message Input Area -->
    <div class="flex-shrink-0 bg-dark-bg-secondary border-t border-dark-border-primary">
      <MessageInput 
        {privateBalance}  
        {isTransactionPending}
        {verusIdName}
        {currencySymbol}
        on:sendMessage={handleSendMessage} 
      />
    </div>

  {:else}
    <!-- Empty State (No Conversation Selected) -->
    <div class="flex-grow flex items-center justify-center bg-dark-bg-primary">
      <div class="text-center text-dark-text-secondary p-8">
         <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto h-16 w-16 text-dark-text-secondary mb-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
        <p class="text-lg font-semibold text-dark-text-primary mb-2">Select a conversation</p>
        <p class="text-sm text-dark-text-secondary">Choose a chat from the sidebar or start a new conversation</p>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Custom scrollbar for dark theme */
  ::-webkit-scrollbar {
    width: 6px;
  }
  ::-webkit-scrollbar-track {
    background: transparent; 
  }
  ::-webkit-scrollbar-thumb {
    background: #404040; 
    border-radius: 3px;
  }
  ::-webkit-scrollbar-thumb:hover {
    background: #525252; 
  }

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

  .message-item {
    position: relative;
  }

  .message-text {
    font-family: 'IBM Plex Mono', monospace;
    word-wrap: break-word;
    line-height: 1.5;
  }
</style> 