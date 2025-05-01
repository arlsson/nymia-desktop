<script lang="ts">
// Component: src/lib/components/ChatInterface.svelte
// Description: Main container for the VerusID chat interface.
// Manages the overall layout and integrates the NewChatModal.
// Changes:
// - Removed mock data initialization for conversations and messages.
// - Initialized conversations and messages state as empty.

  import { createEventDispatcher } from 'svelte';
  import TopBar from './chat/TopBar.svelte';
  import ConversationsList from './chat/ConversationsList.svelte';
  import ConversationView from './chat/ConversationView.svelte';
  import NewChatModal from './chat/NewChatModal.svelte'; // Import the modal
  import type { FormattedIdentity, PrivateBalance, ChatMessage } from '$lib/types';

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
  let showNewChatModal = false; // State to control modal visibility

  // Define Conversation type locally for clarity (could move to types.ts)
  type Conversation = {
    id: string;         // Unique ID, likely the VerusID i-address or formatted name
    name: string;       // Display name (VerusID name)
    unread?: boolean;   // Optional flag for unread messages
  };

  // --- Data State (Initialized Empty) ---
  let conversations: Conversation[] = [];
  let messages: { [conversationId: string]: ChatMessage[] } = {};

  // --- Computed State ---
  $: existingConversationIds = conversations.map(c => c.id.toLowerCase());
  $: loggedInUserPrivateAddress = loggedInIdentity?.private_address;

  // --- Event Handlers ---
  function handleSelectConversation(event: CustomEvent<{ conversationId: string }>) {
    selectedConversationId = event.detail.conversationId;
    console.log("ChatInterface: Selected conversation", selectedConversationId);
    const convoIndex = conversations.findIndex(c => c.id === selectedConversationId);
    if (convoIndex !== -1) {
      conversations[convoIndex].unread = false;
      conversations = [...conversations]; // Trigger reactivity
    }
    // TODO: Fetch messages for the selected conversation
    // messages[selectedConversationId] = await fetchMessages(selectedConversationId);
  }

  function handleSendMessage(event: CustomEvent<{ message: string; amount?: number }>) {
    console.log("ChatInterface: Send message request", event.detail);
    if (selectedConversationId && loggedInIdentity) {
      const newMessage: ChatMessage = {
          id: `msg-${Date.now()}`,
          sender: 'self',
          text: event.detail.message,
          timestamp: Date.now(),
          status: 'sent', // This will eventually be updated based on backend response
          amount: event.detail.amount || 0,
          confirmations: 0,
          direction: 'sent'
      };
       // Optimistic update (add immediately to UI)
      messages[selectedConversationId] = [
          ...(messages[selectedConversationId] || []),
          newMessage
      ];
      messages = { ...messages }; // Trigger reactivity
       // TODO: Send message to backend and update status on confirmation/error
       // sendMessageToBackend(selectedConversationId, newMessage);
    }
  }

  function handleOpenNewChatModal() {
      console.log("ChatInterface: Open new chat modal requested");
      showNewChatModal = true;
  }

  function handleCloseNewChatModal() {
      showNewChatModal = false;
  }

  function handleStartChat(event: CustomEvent<{ identity: FormattedIdentity; history?: ChatMessage[] }>) {
      const { identity, history } = event.detail;
      const newConversationId = identity.formatted_name; // Assuming formatted_name is the desired display ID
      console.log(`ChatInterface: Start chat event received for ${newConversationId}`, history);

      // 1. Add conversation if it doesn't exist
      if (!conversations.some(c => c.id === newConversationId)) {
          conversations = [
              ...conversations,
              { id: newConversationId, name: identity.formatted_name, unread: false }
          ];
      }

      // 2. Add history messages if provided
      const existingMessages = messages[newConversationId] || [];
      let mergedMessages = existingMessages;
      if (history && history.length > 0) {
          // More robust merge: Add history only if message ID doesn't exist
          const existingIds = new Set(existingMessages.map(m => m.id));
          const newHistoryMessages = history.filter(hm => !existingIds.has(hm.id));
          mergedMessages = [...newHistoryMessages, ...existingMessages];
      }
      // Sort all messages after merging
      messages[newConversationId] = mergedMessages.sort((a, b) => a.timestamp - b.timestamp);
      messages = { ...messages }; // Trigger reactivity

      // 3. Select the new conversation
      selectedConversationId = newConversationId;

      // 4. Close modal
      showNewChatModal = false;
  }

  function handleLogout() {
      console.log("ChatInterface: Logout requested");
      dispatch('logout'); // Dispatch event to parent (+page.svelte)
  }
    
  function handleRefresh() {
      console.log("ChatInterface: Refresh requested");
      // TODO: Implement logic to fetch new conversations/messages
  }
    
  function handleSettings() {
      console.log("ChatInterface: Settings requested");
      // Placeholder: Implement logic to open settings
  }

  // Reactive computation for selected conversation's messages
  $: selectedMessages = selectedConversationId ? (messages[selectedConversationId] || []) : []; // Use reactive messages
  $: selectedContactName = selectedConversationId ? conversations.find(c => c.id === selectedConversationId)?.name : null; // Use reactive conversations

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
        conversations={conversations}
        selectedConversationId={selectedConversationId}
        on:selectConversation={handleSelectConversation}
        on:openNewChatModal={handleOpenNewChatModal}  
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

  <!-- New Chat Modal -->
  <NewChatModal 
    bind:showModal={showNewChatModal}
    loggedInUserPrivateAddress={loggedInUserPrivateAddress}
    existingConversationIds={existingConversationIds}
    on:close={handleCloseNewChatModal}
    on:start-chat={handleStartChat}
  />

</div> 