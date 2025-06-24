<script lang="ts">
// Component: src/lib/components/chat/ConversationView.svelte
// Description: Main conversation view orchestrator using extracted components
// Changes:
// - MAJOR REFACTOR: Extracted large component into focused smaller components
// - Now uses: ChatHeader, MessageList, MessageGroup, MessageItem, GiftDisplay, DateSeparator, EmptyState
// - Simplified to focus on coordination: message grouping, scroll management, event handling
// - Removed component-specific UI logic (moved to respective components)
// - Maintained all existing functionality through component composition
// - Improved maintainability and testability through separation of concerns

  import { createEventDispatcher, tick } from 'svelte';
  import MessageInput from './MessageInput.svelte';
  import ChatHeader from './ChatHeader.svelte';
  import MessageList from './MessageList.svelte';
  import EmptyState from './EmptyState.svelte';
  import type { ChatMessage, PrivateBalance, UtxoInfo } from '$lib/types';
  import { groupMessages } from '$lib/utils/messageGrouping';

  // --- Props ---
  export let contactName: string | null = null;
  export let messages: ChatMessage[] = [];
  export let privateBalance: PrivateBalance = null;
  export let isTransactionPending: boolean = false;
  export let verusIdName: string; // Current user's VerusID name for dynamic message limit calculation
  export let currencySymbol: string = 'VRSC'; // Dynamic currency symbol
  export let utxoInfo: UtxoInfo | null = null; // NEW: UTXO information for Fast Messages

  // --- State ---
  let chatContainer: HTMLElement | undefined;

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
</script>

<div class="flex flex-col h-full bg-dark-bg-primary">
  {#if contactName}
    <!-- Chat Header -->
    <ChatHeader {contactName} {verusIdName} />

    <!-- Message List (Scrollable) -->
    <MessageList 
      {messageGroups} 
      {verusIdName} 
      {currencySymbol}
      bind:scrollContainer={chatContainer}
    />

    <!-- Message Input Area -->
    <div class="flex-shrink-0 bg-dark-bg-secondary">
      <MessageInput 
        {privateBalance}  
        {isTransactionPending}
        {verusIdName}
        {currencySymbol}
        {utxoInfo}
        on:sendMessage={handleSendMessage} 
      />
    </div>

  {:else}
    <!-- Empty State (No Conversation Selected) -->
    <EmptyState type="no-conversation" />
  {/if}
</div> 