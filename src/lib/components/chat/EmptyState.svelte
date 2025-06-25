<script lang="ts">
// Component: src/lib/components/chat/EmptyState.svelte
// Description: Empty state displays for chat interface
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Handles both "no conversation" and "no messages" empty states
// - Clean, consistent empty state design

  // --- Props ---
  export let type: 'no-conversation' | 'no-messages' = 'no-conversation';
</script>

<div class="flex-grow flex items-center justify-center">
  <div class="text-center text-dark-text-secondary p-8">
    {#if type === 'no-conversation'}
      <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto h-16 w-16 text-dark-text-secondary mb-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
        <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      <p class="text-lg font-semibold text-dark-text-primary mb-2 cursor-default select-none">Select a conversation</p>
      <p class="text-sm text-dark-text-secondary cursor-default select-none">Choose a chat from the sidebar or start a new conversation</p>
    {:else}
      <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto h-12 w-12 text-dark-text-secondary mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
        <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      <p class="text-sm font-medium text-dark-text-primary mb-1 cursor-default select-none">Start the conversation!</p>
      <p class="text-xs text-dark-text-secondary cursor-default select-none">Send a message or gift to get started.</p>
    {/if}
  </div>
</div> 