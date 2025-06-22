<script lang="ts">
// Component: src/lib/components/chat/ChatHeader.svelte
// Description: Chat header with avatar, contact name, and privacy link
// Changes:
// - Extracted from ConversationView.svelte to create focused, reusable component
// - Manages privacy modal state internally
// - Receives contactName and verusIdName as props
// - Clean separation of header-specific concerns

  import Avatar from '../Avatar.svelte';
  import PrivacyInfoModal from '../onboarding/PrivacyInfoModal.svelte';
  import { Lock } from 'lucide-svelte';

  // --- Props ---
  export let contactName: string;
  export let verusIdName: string;

  // --- State ---
  let showPrivacyModal = false;

  function handleShowPrivacyInfo() {
    showPrivacyModal = true;
  }

  function handleClosePrivacyModal() {
    showPrivacyModal = false;
  }
</script>

<div class="flex items-center justify-between h-12 px-4 bg-dark-bg-primary border-b border-dark-border-primary flex-shrink-0 shadow-sm">
  <!-- Left: Avatar + Contact Name -->
  <div class="flex items-center min-w-0 flex-1">
    <Avatar userId={contactName} size="small" showHover={false} />
    <span class="font-semibold text-dark-text-primary ml-3 truncate select-none cursor-default">{contactName}</span>
  </div>
  
  <!-- Right: Privacy Link -->
  <button 
    class="flex items-center text-xs text-white/45 hover:text-white/75 cursor-pointer select-none"
    on:click={handleShowPrivacyInfo}
  >
    <Lock size={15} class="mr-1" />
    100% Private
  </button>
</div>

<!-- Privacy Info Modal -->
{#if showPrivacyModal}
  <PrivacyInfoModal on:close={handleClosePrivacyModal} />
{/if} 