<script lang="ts">
// Component: src/lib/components/ChatInterface.svelte
// Description: Main container for the VerusID chat interface.
// Manages the overall layout including TopBar, ConversationsList, and ConversationView.
// Changes:
// - Changed mockMessages from const to let to allow modification.
// - Fixed comment syntax for privateBalance prop.
// - Updated layout dimensions for a more compact design
// - Adjusted left sidebar to be narrower
// - Added more gray tones to overall interface

  import { createEventDispatcher } from 'svelte';
  import TopBar from './chat/TopBar.svelte';
  import ConversationsList from './chat/ConversationsList.svelte';
  import ConversationView from './chat/ConversationView.svelte';
  import type { FormattedIdentity, PrivateBalance } from '$lib/types';

  // --- Props ---
  export let loggedInIdentity: FormattedIdentity | null = null; // Received from parent (+page.svelte)
  export let blockHeight: number | null = null; // Received from parent (+page.svelte)
  export let privateBalance: PrivateBalance = null; // Use the new type alias

  // --- Event dispatcher ---
  const dispatch = createEventDispatcher<{
    logout: void;
  }>();

  // --- State ---
  let selectedConversationId: string | null = null; // Example state to track active chat

  // Mock data for conversations (replace with real data later)
  const mockConversations = [
    { id: 'verususer1@', name: 'Verus User 1', unread: true }, // Added unread flag example
    { id: 'mike@', name: 'Mike', unread: false },
    { id: 'alice.vrsc@', name: 'Alice', unread: false },
  ];

  // Mock data for messages (replace with real data later)
  // Changed to let to allow updates
  let mockMessages: { [key: string]: any[] } = {
    'verususer1@': [
      { id: 1, sender: 'verususer1@', text: 'Hello there!', timestamp: Date.now() - 10000, status: 'delivered' },
      { id: 2, sender: 'self', text: 'Hi! How are you?', timestamp: Date.now() - 5000, status: 'delivered' },
    ],
    'mike@': [
      { id: 3, sender: 'mike@', text: 'Meeting at 3 PM?', timestamp: Date.now() - 20000, status: 'delivered' },
    ],
     'alice.vrsc@': [], // Empty conversation
  };

  // --- Event Handlers ---
  function handleSelectConversation(event: CustomEvent<{ conversationId: string }>) {
    selectedConversationId = event.detail.conversationId;
    console.log("ChatInterface: Selected conversation", selectedConversationId);
    // Placeholder: Mark conversation as read
    const convoIndex = mockConversations.findIndex(c => c.id === selectedConversationId);
    if (convoIndex !== -1) {
        mockConversations[convoIndex].unread = false;
        // Force reactivity for the array
        // mockConversations = [...mockConversations]; // This can be expensive, consider more targeted updates
    }
  }

  function handleSendMessage(event: CustomEvent<{ message: string; amount?: number }>) {
      console.log("ChatInterface: Send message request", event.detail);
      // Placeholder: Add message to mock data and update UI
      if (selectedConversationId) {
        mockMessages[selectedConversationId] = [
            ...(mockMessages[selectedConversationId] || []),
            { id: Date.now(), sender: 'self', text: event.detail.message, timestamp: Date.now(), status: 'sent' }
        ];
        // Trigger reactivity (Svelte might need help seeing deep changes)
        mockMessages = { ...mockMessages }; 
      }
  }

  function handleLogout() {
      console.log("ChatInterface: Logout requested");
      dispatch('logout'); // Dispatch event to parent (+page.svelte)
  }
    
  function handleRefresh() {
      console.log("ChatInterface: Refresh requested");
      // Placeholder: Implement logic to fetch new messages/data
  }
    
  function handleSettings() {
      console.log("ChatInterface: Settings requested");
      // Placeholder: Implement logic to open settings
  }
    
  function handleNewChat() {
      console.log("ChatInterface: New chat requested");
      // Placeholder: Implement logic to start a new chat (e.g., show a modal)
  }


  // Reactive computation for selected conversation's messages
  $: selectedMessages = selectedConversationId ? mockMessages[selectedConversationId] : [];
  $: selectedContactName = selectedConversationId ? mockConversations.find(c => c.id === selectedConversationId)?.name : null;

</script>

<div class="flex flex-col h-screen bg-gray-50 font-sans text-sm">
  <!-- Top Bar -->
  <TopBar 
    verusIdName={loggedInIdentity?.formatted_name || 'Unknown User'} 
    privateBalance={privateBalance}
    blockHeight={blockHeight}
    on:logout={handleLogout}
    on:refresh={handleRefresh}
    on:settings={handleSettings}
  />

  <!-- Main Content Area (Sidebar + Chat View) -->
  <div class="flex flex-grow overflow-hidden border-t border-gray-200"> 
    
    <!-- Left Sidebar -->
    <div class="w-[25%] max-w-[300px] min-w-[200px] flex-shrink-0 bg-white border-r border-gray-200 flex flex-col shadow-sm">
      <ConversationsList 
        conversations={mockConversations}
        selectedConversationId={selectedConversationId}
        on:selectConversation={handleSelectConversation}
        on:newChat={handleNewChat}
      />
    </div>

    <!-- Right Panel (Chat View) -->
    <div class="flex-grow flex flex-col bg-gray-50">
      <ConversationView 
        contactName={selectedContactName}
        messages={selectedMessages}
        on:sendMessage={handleSendMessage}
      />
    </div>
  </div>
</div> 