<!-- 
  Component: src/lib/components/Modal.svelte
  Description: Reusable modal component with backdrop, transitions, and accessibility features
  Provides consistent modal behavior across the application with customizable content via slots
  Changes:
  - Created as a reusable modal wrapper component
  - Handles backdrop clicks, escape key, and focus management
  - Supports customizable sizing and styling via props
  - Includes proper ARIA attributes and keyboard accessibility
-->
<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  // Props
  export let show: boolean = false;
  export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let closeOnBackdrop: boolean = true;
  export let closeOnEscape: boolean = true;
  export let preventBodyScroll: boolean = true;

  // Event dispatcher
  const dispatch = createEventDispatcher<{ close: void }>();

  // Modal element reference
  let modalElement: HTMLElement;
  let modalHeaderId = `modal-header-${Math.random().toString(36).substring(2)}`;

  // Size classes mapping
  const sizeClasses = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl'
  };

  function handleClose() {
    dispatch('close');
  }

  function handleBackdropClick(event: MouseEvent) {
    if (closeOnBackdrop && event.target === event.currentTarget) {
      handleClose();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (closeOnEscape && event.key === 'Escape') {
      handleClose();
    }
  }

  function handleBackdropKeydown(event: KeyboardEvent) {
    if (closeOnBackdrop && (event.key === 'Enter' || event.key === ' ')) {
      event.preventDefault();
      handleClose();
    }
  }

  // Body scroll prevention
  onMount(() => {
    if (preventBodyScroll && show) {
      document.body.style.overflow = 'hidden';
    }
  });

  onDestroy(() => {
    if (preventBodyScroll) {
      document.body.style.overflow = '';
    }
  });

  // Update body scroll when show prop changes
  $: if (preventBodyScroll) {
    if (show) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
  }

  // Add window keydown listener
  $: if (show) {
    window.addEventListener('keydown', handleKeydown);
  } else {
    window.removeEventListener('keydown', handleKeydown);
  }
</script>

{#if show}
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-60 backdrop-blur-sm p-4"
    transition:fade={{ duration: 150 }}
    on:click={handleBackdropClick}
    on:keydown={handleBackdropKeydown}
    role="button"
    tabindex="0"
    aria-label="Close modal"
  >
    <!-- Modal Content -->
    <div 
      bind:this={modalElement}
      class="bg-dark-bg-secondary rounded-lg shadow-xl w-full {sizeClasses[size]} transform transition-all duration-300 border border-dark-border-primary overflow-hidden"
      transition:scale={{ duration: 200, easing: quintOut }}
      on:click|stopPropagation
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby={modalHeaderId}
    >
      <!-- Header Slot -->
      {#if $$slots.header}
        <div class="flex items-center p-3 border-b border-dark-border-primary bg-dark-bg-primary">
          <slot name="header" {modalHeaderId} {handleClose} />
        </div>
      {/if}

      <!-- Main Content Slot -->
      <slot {handleClose} />

      <!-- Footer Slot -->
      {#if $$slots.footer}
        <div class="px-4 py-3 bg-dark-bg-primary border-t border-dark-border-primary">
          <slot name="footer" {handleClose} />
        </div>
      {/if}
    </div>
  </div>
{/if} 