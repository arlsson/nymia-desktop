<script lang="ts">
// Component: src/lib/components/chat/ConversationsList.svelte
// Description: Discord-style conversation list with avatars and enhanced design
// Changes:
// - Added Avatar integration for each conversation
// - Updated color scheme to black/dark gray with minimal brand green
// - Enhanced visual design with better spacing and hover states
// - Improved new chat button styling
// - Added conversation preview functionality foundation
// - STYLING UPDATE: Implemented Discord-style selection with rounded backgrounds
// - Removed green left border indicator for selected conversations
// - Updated to darker background throughout (bg-dark-bg-primary)
// - Removed border divider between New Chat button and conversation list
// - Fixed horizontal scrolling issue by removing horizontal margins

  import { createEventDispatcher } from 'svelte';
  import { Plus } from 'lucide-svelte';
  import Avatar from '../Avatar.svelte';

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

<div class="flex flex-col h-full bg-dark-bg-primary">
  <!-- Header/New Chat Button -->
  <div class="p-3 bg-dark-bg-primary">
    <button 
      on:click={handleNewChat}
      class="w-full flex items-center justify-center py-2 px-3 bg-dark-bg-primary hover:bg-dark-bg-tertiary/80 border border-dark-border-secondary text-dark-text-primary rounded-md focus:outline-none focus:ring-2 focus:ring-brand-green focus:ring-offset-2 focus:ring-offset-dark-bg-secondary transition-all duration-150 text-sm font-medium"
    >
      <Plus size={16} class="mr-2 text-brand-green" />
      New Chat
    </button>
  </div>

  <!-- Conversation List (Scrollable) -->
  <div class="flex-grow overflow-y-auto bg-dark-bg-primary px-1.5">
    {#if conversations.length === 0}
        <div class="flex items-center justify-center h-full p-6">
            <div class="text-center">
              <div class="w-12 h-12 mx-auto mb-3 rounded-full bg-dark-bg-tertiary flex items-center justify-center">
                <Plus size={20} class="text-dark-text-disabled" />
              </div>
              <p class="text-sm text-dark-text-secondary font-medium mb-1">No conversations yet</p>
              <p class="text-xs text-dark-text-disabled">Start a new chat to get started</p>
            </div>
        </div>
    {:else}
        {#each conversations as conversation (conversation.id)}
          <button 
            on:click={() => handleSelect(conversation.id)}
            class={`w-full text-left px-3 py-3 flex items-center transition-all duration-150 group my-1 rounded-md
            ${selectedConversationId === conversation.id ? 
              'bg-white/10' : 
              'hover:bg-white/5'}`}
          >
            <!-- Avatar -->
            <div class="flex-shrink-0 mr-3">
              <Avatar 
                userId={conversation.name} 
                size="small" 
                showHover={false}
              />
            </div>
            
            <!-- Conversation Info -->
            <div class="flex-grow min-w-0 flex items-center justify-between">
              <div class="flex-grow min-w-0">
                <div class="flex items-center">
                  <span class={`text-sm font-medium truncate ${
                    selectedConversationId === conversation.id ? 
                    'text-dark-text-primary' : 
                    'text-dark-text-secondary group-hover:text-dark-text-primary'
                  }`}>
                    {conversation.name}
                  </span>
                </div>
                
                <!-- Message Preview Placeholder (for future enhancement) -->
                <!-- <p class="text-xs text-dark-text-disabled truncate mt-0.5">
                  Last message preview...
                </p> -->
              </div>
              
              <!-- Unread Indicator -->
              {#if conversation.unread}
                <div class="flex-shrink-0 ml-2">
                  <div class="w-2 h-2 bg-brand-green rounded-full" title="Unread messages"></div>
                </div>
              {/if}
            </div>
          </button>
        {/each}
    {/if}
  </div>
</div>

<style>
  /* Custom scrollbar for conversation list */
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
</style> 