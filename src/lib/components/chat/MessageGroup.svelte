<script lang="ts">
// Component: src/lib/components/chat/MessageGroup.svelte
// Description: Message group with avatar, sender name, timestamp, and messages
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Handles grouped messages from the same sender
// - Includes avatar, sender name, and timestamp display
// - Iterates through messages in the group
// - FIXED: Added proper overflow handling to prevent horizontal scrolling

  import Avatar from '../Avatar.svelte';
  import MessageItem from './MessageItem.svelte';
  import type { MessageGroup } from '$lib/utils/messageGrouping';
  import { formatMessageTime } from '$lib/utils/messageGrouping';

  // --- Props ---
  export let group: MessageGroup;
  export let verusIdName: string;
  export let currencySymbol: string = 'VRSC';
</script>

<div class="message-group mb-6">
  <div class="flex">
    <!-- Avatar (shown only for first message in group) -->
    <div class="flex-shrink-0 mr-3">
      <Avatar 
        userId={group.sender === 'self' ? verusIdName : group.sender} 
        size="medium" 
        showHover={false} 
      />
    </div>

    <!-- Messages Content -->
    <div class="flex-grow min-w-0 overflow-hidden">
      <!-- Group Header (sender name and timestamp) -->
      <div class="flex items-baseline mb-1">
        <span class="font-semibold text-dark-text-primary text-sm mr-2 cursor-default select-none">
          {group.sender === 'self' ? 'You' : group.sender}
        </span>
        <span class="text-xs text-white/45 select-none cursor-default">
          {formatMessageTime(group.timestamp)}
        </span>
      </div>

      <!-- Messages in group -->
      {#each group.messages as message (message.id)}
        <MessageItem {message} {currencySymbol} />
      {/each}
    </div>
  </div>
</div> 