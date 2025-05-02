<script lang="ts">
// Component: src/lib/components/ChatInterface.svelte
// Description: Main container for the VerusID chat interface.
// Manages the overall layout and integrates the NewChatModal.
// Changes:
// - Removed mock data initialization for conversations and messages.
// - Initialized conversations and messages state as empty.
// - Added periodic polling for new messages using setInterval.
// - Implemented message merging logic, preventing duplicates.
// - Added unread indicator logic for conversations.
// - Refined sorting logic to prioritize confirmations for received messages.

  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import TopBar from './chat/TopBar.svelte';
  import ConversationsList from './chat/ConversationsList.svelte';
  import ConversationView from './chat/ConversationView.svelte';
  import NewChatModal from './chat/NewChatModal.svelte'; // Import the modal
  import type { FormattedIdentity, PrivateBalance, ChatMessage } from '$lib/types';

  // --- Props ---
  export let loggedInIdentity: FormattedIdentity | null = null; // Received from parent (+page.svelte)
  export let blockHeight: number | null = null; // Received from parent (+page.svelte)
  export let privateBalance: PrivateBalance = null; // Use the new type alias

  // --- Constants ---
  const POLLING_INTERVAL_MS = 30000; // 30 seconds

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
  let pollingIntervalId: ReturnType<typeof setInterval> | null = null;
  let isPolling = false; // Prevent overlapping polls

  // --- Computed State ---
  $: existingConversationIds = conversations.map(c => c.id.toLowerCase());
  $: loggedInUserPrivateAddress = loggedInIdentity?.private_address;

  // --- Lifecycle ---
  onMount(() => {
      startPolling();
      // TODO: Fetch initial conversations and messages here
  });

  onDestroy(() => {
      stopPolling();
  });

  // --- Polling Logic ---
  function startPolling() {
      stopPolling(); // Ensure no duplicate intervals
      if (loggedInUserPrivateAddress) {
          console.log(`ChatInterface: Starting message polling every ${POLLING_INTERVAL_MS / 1000}s`);
          fetchNewMessages(); // Initial fetch
          pollingIntervalId = setInterval(fetchNewMessages, POLLING_INTERVAL_MS);
      } else {
          console.warn("ChatInterface: Cannot start polling, logged in user private address not available.");
      }
  }

  function stopPolling() {
      if (pollingIntervalId) {
          console.log("ChatInterface: Stopping message polling.");
          clearInterval(pollingIntervalId);
          pollingIntervalId = null;
      }
      isPolling = false;
  }

  async function fetchNewMessages() {
      if (!loggedInUserPrivateAddress || isPolling) {
          if (isPolling) console.log("ChatInterface: Skipping poll, already fetching.");
          return; 
      }
      
      isPolling = true;
      console.log("ChatInterface: Polling for new messages...");
      
      try {
          const newMessages = await invoke<ChatMessage[]>('get_new_received_messages', { 
              ownPrivateAddress: loggedInUserPrivateAddress 
          });
          
          console.log(`ChatInterface: Received ${newMessages.length} potential new messages from poll.`);
          if (newMessages.length > 0) {
              processNewMessages(newMessages);
          }

      } catch (error) {
          console.error("ChatInterface: Error polling for new messages:", error);
          // Consider stopping polling on certain errors? Or just log?
      } finally {
          isPolling = false;
      }
  }

  function processNewMessages(newMessages: ChatMessage[]) {
      let messagesUpdated = false;
      let conversationsUpdated = false;
      const currentMessages = { ...messages }; // Clone to modify

      for (const newMessage of newMessages) {
          const senderId = newMessage.sender; // Sender ID from memo
          
          // Check if we have an existing conversation with this sender
          const convoIndex = conversations.findIndex(c => c.id === senderId);
          if (convoIndex !== -1) {
              // Conversation exists, check if message is new
              const currentList = currentMessages[senderId] || [];
              const existingMsgIndex = currentList.findIndex(m => m.id === newMessage.id);
              
              if (existingMsgIndex === -1) {
                  // New message for this known conversation
                  console.log(`ChatInterface: Adding new message ${newMessage.id} from ${senderId}`);
                  currentList.push(newMessage);
                  currentMessages[senderId] = currentList; // Update the list in our temporary object
                  messagesUpdated = true;

                  // Mark conversation as unread if it's not the currently selected one
                  if (senderId !== selectedConversationId) {
                      if (!conversations[convoIndex].unread) {
                           console.log(`ChatInterface: Marking conversation ${senderId} as unread.`);
                           conversations[convoIndex].unread = true;
                           conversationsUpdated = true;
                      }
                  }
              } else {
                 // Message already exists, potentially update confirmations if needed (optional)
                 if (currentList[existingMsgIndex].confirmations !== newMessage.confirmations) {
                     console.log(`ChatInterface: Updating confirmations for message ${newMessage.id} from ${senderId}`);
                     currentList[existingMsgIndex].confirmations = newMessage.confirmations;
                     messagesUpdated = true; // Mark as updated if confirmations changed
                 }
              }
          } else {
              // Conversation does not exist, ignore message as per requirements
              // console.log(`ChatInterface: Ignoring message ${newMessage.id} from unknown sender ${senderId}`);
          }
      }

      if (messagesUpdated) {
          // Sort affected conversations after adding/updating messages
          Object.keys(currentMessages).forEach(convoId => {
              currentMessages[convoId].sort(compareMessages);
          });
          messages = currentMessages; // Assign the updated object back to trigger reactivity
          console.log("ChatInterface: Messages state updated.");
      }
      if (conversationsUpdated) {
          conversations = [...conversations]; // Trigger reactivity for conversations list
          console.log("ChatInterface: Conversations state updated (unread).", conversations);
      }
  }

  // Comparison function for sorting messages
  function compareMessages(a: ChatMessage, b: ChatMessage): number {
     // Prioritize confirmations for received messages if timestamps are placeholder (0)
     if (a.direction === 'received' && b.direction === 'received' && a.timestamp === 0 && b.timestamp === 0) {
         return a.confirmations - b.confirmations; // Sort by confirmations ascending
     }
     // Fallback to timestamp sorting
     return a.timestamp - b.timestamp;
  }

  // --- Event Handlers ---
  function handleSelectConversation(event: CustomEvent<{ conversationId: string }>) {
    selectedConversationId = event.detail.conversationId;
    console.log("ChatInterface: Selected conversation", selectedConversationId);
    const convoIndex = conversations.findIndex(c => c.id === selectedConversationId);
    if (convoIndex !== -1) {
      // Mark as read when selected
      if (conversations[convoIndex].unread) {
          conversations[convoIndex].unread = false;
          conversations = [...conversations]; // Trigger reactivity
      }
    }
    // TODO: Fetch initial messages if not already loaded
    // if (!messages[selectedConversationId]) { fetchAndSetMessages(selectedConversationId); }
  }

  function handleSendMessage(event: CustomEvent<{ message: string; amount?: number }>) {
    console.log("ChatInterface: Send message request", event.detail);
    if (selectedConversationId && loggedInIdentity) {
      const newMessage: ChatMessage = {
          id: `msg-${Date.now()}`,
          sender: 'self',
          text: event.detail.message,
          timestamp: Date.now(),
          status: 'sent',
          amount: event.detail.amount || 0,
          confirmations: 0,
          direction: 'sent'
      };
       // Optimistic update
      const currentList = messages[selectedConversationId] || [];
      currentList.push(newMessage);
      currentList.sort(compareMessages); // Sort after adding
      messages[selectedConversationId] = currentList;
      messages = { ...messages };
       // TODO: Send message to backend
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
      const newConversationId = identity.formatted_name; 
      console.log(`ChatInterface: Start chat event received for ${newConversationId}`, history);

      // 1. Add conversation
      if (!conversations.some(c => c.id === newConversationId)) {
          conversations = [
              ...conversations,
              { id: newConversationId, name: identity.formatted_name, unread: false }
          ];
      }

      // 2. Add/Merge history messages
      const existingMessages = messages[newConversationId] || [];
      let mergedMessages = existingMessages;
      if (history && history.length > 0) {
          const existingIds = new Set(existingMessages.map(m => m.id));
          const newHistoryMessages = history.filter(hm => !existingIds.has(hm.id));
          mergedMessages = [...newHistoryMessages, ...existingMessages];
      }
      // Sort after merging
      messages[newConversationId] = mergedMessages.sort(compareMessages);
      messages = { ...messages };

      // 3. Select the new conversation
      selectedConversationId = newConversationId;

      // 4. Close modal
      showNewChatModal = false;
  }

  function handleLogout() {
      console.log("ChatInterface: Logout requested");
      stopPolling(); // Stop polling on logout
      dispatch('logout'); 
  }
    
  function handleRefresh() {
      console.log("ChatInterface: Refresh requested");
      fetchNewMessages(); // Trigger manual refresh
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
        privateBalance={privateBalance}
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