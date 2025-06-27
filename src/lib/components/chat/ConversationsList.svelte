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
// - Updated background color to specific hex color #121214
// - Removed empty state text (handled in ConversationView)
// - Added animated highlight to New Chat button when no conversations exist
// - Enhanced unread indicator with MessageSquareLock icon and glow effect

  import { createEventDispatcher } from 'svelte';
  import { Plus, MessageSquareDiff } from 'lucide-svelte';
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

<div class="flex flex-col h-full" style="background-color: #121214">
  <!-- Header/New Chat Button -->
  <div class="p-3" style="background-color: #121214">
    <button 
      on:click={handleNewChat}
      class={`w-full flex items-center justify-center py-2 px-3 border text-dark-text-primary rounded-md focus:outline-none focus:ring-2 focus:ring-brand-green focus:ring-offset-2 focus:ring-offset-dark-bg-secondary text-sm font-medium ${
        conversations.length === 0 
          ? 'border-brand-green bg-brand-green/10 animate-pulse-glow shadow-lg shadow-brand-green/20' 
          : 'border-dark-border-secondary hover:border-brand-green'
      }`}
      style="background-color: {conversations.length === 0 ? '' : '#121214'}"
    >
      <Plus size={16} class="mr-2 text-brand-green" />
      New Chat
    </button>
  </div>

  <!-- Conversation List (Scrollable) -->
  <div class="flex-grow overflow-y-auto px-1.5" style="background-color: #121214">
    {#each conversations as conversation (conversation.id)}
      <button 
        on:click={() => handleSelect(conversation.id)}
        class={`w-full text-left px-3 pl-2 py-2 flex items-center group my-1 rounded-md
        ${selectedConversationId === conversation.id ? 
          'bg-white/5' : 
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
                'text-white/50 group-hover:text-dark-text-primary'
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
              <div class="unread-icon-glow" title="Unread messages">
                <MessageSquareDiff 
                  size={22} 
                  class="text-brand-green"
                />
              </div>
            </div>
          {/if}
        </div>
      </button>
    {/each}
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

  /* Custom pulse glow animation for new chat button */
  @keyframes pulse-glow {
    0%, 100% {
      box-shadow: 0 0 20px rgba(34, 197, 94, 0.2);
      border-color: rgba(34, 197, 94, 0.8);
    }
    50% {
      box-shadow: 0 0 30px rgba(34, 197, 94, 0.4);
      border-color: rgba(34, 197, 94, 1);
    }
  }

  .animate-pulse-glow {
    animation: pulse-glow 2s ease-in-out infinite;
  }

  /* Unread icon glow effect */
  .unread-icon-glow {
    filter: drop-shadow(0 0 4px rgba(34, 197, 94, 0.6));
    animation: icon-glow 2s ease-in-out infinite;
  }

  @keyframes icon-glow {
    0%, 100% {
      filter: drop-shadow(0 0 4px rgba(34, 197, 94, 0.6));
    }
    50% {
      filter: drop-shadow(0 0 8px rgba(34, 197, 94, 0.8));
    }
  }
</style> 