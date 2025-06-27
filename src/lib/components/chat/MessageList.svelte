<script lang="ts">
// Component: src/lib/components/chat/MessageList.svelte
// Description: Scrollable message list container with message groups and empty states
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Handles message group iteration and rendering
// - Manages empty state display
// - Provides scroll container reference to parent
// - FIXED: Added proper overflow handling to prevent horizontal scrolling

  import MessageGroup from './MessageGroup.svelte';
  import DateSeparator from './DateSeparator.svelte';
  import EmptyState from './EmptyState.svelte';
  import type { MessageGroup as MessageGroupType } from '$lib/utils/messageGrouping';

  // --- Props ---
  export let messageGroups: MessageGroupType[];
  export let verusIdName: string;
  export let currencySymbol: string = 'VRSC';

  // --- Expose scroll container for parent component ---
  export let scrollContainer: HTMLElement | undefined = undefined;
</script>

<div 
  bind:this={scrollContainer}
  class="flex-grow overflow-y-auto overflow-x-hidden bg-dark-bg-primary flex flex-col px-4 py-2"
>
  {#if messageGroups.length > 0}
    {#each messageGroups as group (group.id)}
      <!-- Date Separator -->
      {#if group.showDateSeparator && group.dateLabel}
        <DateSeparator dateLabel={group.dateLabel} />
      {/if}

      <!-- Message Group -->
      <MessageGroup {group} {verusIdName} {currencySymbol} />
    {/each}
  {:else}
    <EmptyState type="no-messages" />
  {/if}
</div>

<style>
  /* Custom scrollbar for dark theme */
  :global(::-webkit-scrollbar) {
    width: 6px;
  }
  :global(::-webkit-scrollbar-track) {
    background: transparent; 
  }
  :global(::-webkit-scrollbar-thumb) {
    background: #404040; 
    border-radius: 3px;
  }
  :global(::-webkit-scrollbar-thumb:hover) {
    background: #525252; 
  }
</style> 