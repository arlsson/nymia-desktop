<script lang="ts">
// Component: src/lib/components/chat/DateSeparator.svelte
// Description: Date separator line for chat messages
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Simple component for displaying date dividers between message groups
// - Clean, minimal design with centered date label

  // --- Props ---
  export let dateLabel: string;
</script>

<div class="flex items-center my-6">
  <div class="flex-grow h-px bg-dark-border-primary"></div>
  <div class="px-3 text-xs font-medium text-white/60 bg-dark-bg-primary select-none cursor-default">
    {dateLabel}
  </div>
  <div class="flex-grow h-px bg-dark-border-primary"></div>
</div> 