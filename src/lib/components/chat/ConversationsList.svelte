<script lang="ts">
// Component: src/lib/components/chat/ConversationsList.svelte
// Description: Displays the list of conversations in the left sidebar.
// Changes:
// - More compact and modern design with subtle colors
// - Smaller UI elements and reduced padding
// - Cleaner visual hierarchy
// - Enhanced background with more gray tones

  import { createEventDispatcher } from 'svelte';
  import { Plus } from 'lucide-svelte';

  // --- Type (could be moved to $lib/types) ---
  type Conversation = {
    id: string;         // Unique ID, likely the VerusID i-address
    name: string;       // Display name (VerusID name)
    unread?: boolean;   // Optional flag for unread messages
    // Add other potential fields like lastMessageTimestamp, lastMessagePreview etc. later
  };

  // --- Props ---
  export let conversations: Conversation[] = [];
  export let selectedConversationId: string | null = null;

  // --- Events ---
  const dispatch = createEventDispatcher<{ 
    selectConversation: { conversationId: string };
    openNewChatModal: void;
  }>();

  function handleSelect(id: string) {
    dispatch('selectConversation', { conversationId: id });
  }
  
  function handleNewChat() {
      dispatch('openNewChatModal');
  }

</script>

<div class="flex flex-col h-full">
  <!-- Header/New Chat Button -->
  <div class="p-2 border-b border-dark-border-primary bg-dark-bg-primary">
    <button 
      on:click={handleNewChat}
      class="w-full flex items-center justify-center py-1.5 px-3 bg-dark-bg-tertiary border border-dark-border-secondary text-dark-text-primary rounded hover:bg-gray-700 focus:outline-none focus:ring-1 focus:ring-brand-green transition duration-150 ease-in-out text-sm shadow-sm"
    >
      <Plus size={14} class="mr-1.5 text-brand-green" />
      New Chat
    </button>
  </div>

  <!-- Conversation List (Scrollable) -->
  <div class="flex-grow overflow-y-auto bg-dark-bg-secondary">
    {#if conversations.length === 0}
        <div class="flex items-center justify-center h-full">
            <p class="text-center text-dark-text-secondary p-3 text-xs">No conversations yet.</p>
        </div>
    {:else}
        {#each conversations as conversation (conversation.id)}
          <button 
            on:click={() => handleSelect(conversation.id)}
            class={`w-full text-left px-3 py-2 flex items-center justify-between hover:bg-dark-bg-tertiary transition-colors duration-100 
            ${selectedConversationId === conversation.id ? 'bg-dark-bg-tertiary border-l-2 border-l-brand-green' : 'border-b border-dark-border-primary'}`}
          >
            <span class="text-sm text-dark-text-primary truncate max-w-[85%]">
              {conversation.name}
            </span>
            {#if conversation.unread}
              <span class="w-1.5 h-1.5 bg-brand-green rounded-full flex-shrink-0 ml-1.5" title="Unread messages"></span>
            {/if}
          </button>
        {/each}
    {/if}
  </div>
</div> 