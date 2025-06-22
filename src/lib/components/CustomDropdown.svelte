<script lang="ts">
// Component: src/lib/components/CustomDropdown.svelte
// Description: A clean and sophisticated custom dropdown component for dark theme.
// Changes:
// - Simplified design with cleaner styling and better proportions
// - Removed excessive icons and visual elements for a more streamlined look
// - Improved scrolling behavior and z-index handling
// - Maintained sophisticated feel with subtle gradients and smooth animations

  import { createEventDispatcher, tick } from 'svelte';
  import { fly, scale } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';

  interface Option {
    id: string | number | null;
    name: string;
    enabled: boolean;
  }

  export let options: Option[] = [];
  export let selectedId: string | number | null = null;
  export let label: string = '';
  export let placeholder: string = '-- Select --';
  export let disabled: boolean = false;

  let isOpen = false;
  let selectedOptionName: string = placeholder;
  let dropdownElement: HTMLElement;

  const dispatch = createEventDispatcher<{ change: string | number | null }>();

  // Update selected name when selectedId prop changes externally
  $: {
      const foundOption = options.find(opt => opt.id === selectedId);
      selectedOptionName = foundOption ? foundOption.name : placeholder;
  }

  function toggleDropdown() {
    if (disabled) return;
    isOpen = !isOpen;
  }

  async function selectOption(option: Option) {
    if (!option.enabled || disabled) return;
    selectedId = option.id;
    selectedOptionName = option.name;
    isOpen = false;
    await tick();
    dispatch('change', selectedId);
  }

  function handleClickOutside(event: MouseEvent) {
    if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
      isOpen = false;
    }
  }
  
  function handleKeydown(event: KeyboardEvent, option: Option) {
      if (event.key === 'Enter') {
          selectOption(option);
      }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="relative w-full" bind:this={dropdownElement}>
  {#if label}
    <label class="block text-sm font-medium text-dark-text-primary mb-2">{label}</label>
  {/if}
  
  <button 
    type="button"
    class="relative w-full bg-dark-bg-primary border border-dark-border-primary hover:border-brand-green/50 focus:border-brand-green rounded-lg shadow-sm pl-3 pr-10 py-2.5 text-left focus:outline-none focus:ring-2 focus:ring-brand-green/30 disabled:bg-dark-bg-secondary disabled:opacity-50 disabled:cursor-not-allowed {isOpen ? 'border-brand-green ring-2 ring-brand-green/30' : ''}"
    on:click={toggleDropdown}
    aria-haspopup="listbox"
    aria-expanded={isOpen}
    {disabled}
  >
    <span 
      class="block truncate font-medium"
      class:text-dark-text-primary={selectedId !== null}
      class:text-dark-text-disabled={selectedId === null}
    >
      {selectedOptionName}
    </span>
    
    <span class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
      <svg 
        class="h-4 w-4 text-dark-text-secondary transition-transform duration-200 {isOpen ? 'rotate-180' : ''}"
        xmlns="http://www.w3.org/2000/svg" 
        viewBox="0 0 20 20" 
        fill="currentColor" 
        aria-hidden="true"
      >
        <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
      </svg>
    </span>
  </button>

  {#if isOpen}
    <div 
      class="absolute z-50 mt-1 w-full"
      transition:fly={{ y: -4, duration: 150, easing: cubicOut }}
    >
      <ul 
        class="bg-dark-bg-secondary border border-dark-border-primary shadow-xl max-h-60 rounded-lg py-1 text-sm overflow-auto focus:outline-none"
        tabindex="-1"
        role="listbox"
        aria-labelledby="listbox-label"
        in:scale={{ duration: 100, start: 0.98, easing: cubicOut }}
      >
        {#each options as option (option.id)}
          <li 
            class="relative cursor-pointer select-none px-3 py-2  {option.enabled
              ? 'hover:bg-dark-bg-tertiary text-dark-text-primary'
              : 'cursor-not-allowed opacity-60 text-dark-text-disabled'} {option.id === selectedId
              ? 'bg-brand-green/10 text-brand-green font-medium'
              : ''}"
            role="option"
            aria-selected={option.id === selectedId}
            on:click={() => selectOption(option)}
            on:keydown={(e) => handleKeydown(e, option)}
            tabindex="0"
          >
            <div class="flex items-center justify-between">
              <span class="block truncate">
                {option.name}
              </span>
              
              {#if !option.enabled}
                <span class="ml-2 inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-yellow-900/40 text-yellow-300">
                  Soon
                </span>
              {:else if option.id === selectedId}
                <span class="ml-2 text-brand-green">
                  <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </span>
              {/if}
            </div>
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</div>