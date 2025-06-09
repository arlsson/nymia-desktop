<script lang="ts">
// Component: src/lib/components/chat/PersistencePromptModal.svelte
// Description: Modal prompting the user to enable/disable local chat persistence.
// Changes:
// - Improved UI to match NewChatModal style.
// - Reorganized content with clearer explanation.
// - Replaced "Ephemeral" with clearer language.
// - Added keyboard handling (Escape).
// - Added proper ARIA attributes for accessibility.
// - Improved visual layout to reduce cognitive load.

    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { Database, HardDriveDownload, CheckCircle, AlertCircle, Trash, MessageSquare } from 'lucide-svelte';

    // --- Props --- 
    export let showModal: boolean = false;
    export let verusIdName: string = 'your VerusID'; // Placeholder if name not passed

    // --- Events ---
    const dispatch = createEventDispatcher<{ 
        save: void;         // User chose to save locally
        dontSave: void;     // User chose not to save
    }>();

    function handleSave() {
        dispatch('save');
        // Parent component will set showModal = false
    }

    function handleDontSave() {
        dispatch('dontSave');
        // Parent component will set showModal = false
    }

    // Close modal if clicked outside (optional but good UX)
    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            // We don't automatically close here, user must make a choice
        }
    }

    // Handle Escape key press
    function handleKeydown(event: KeyboardEvent) {
        if (showModal && event.key === 'Escape') {
            // We don't close on escape, user must make an explicit choice
        }
    }

    // Setup keyboard listeners
    onMount(() => {
        window.addEventListener('keydown', handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', handleKeydown);
    });
</script>

{#if showModal}
<div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-60 backdrop-blur-sm transition-opacity duration-200 font-sans"
    class:opacity-100={showModal}
    class:opacity-0={!showModal}
    on:click={handleBackdropClick} 
    role="dialog" 
    aria-modal="true"
    aria-labelledby="persistence-modal-title"
>
    <div class="bg-dark-bg-secondary rounded-lg shadow-xl w-full max-w-md overflow-hidden transform transition-all duration-300 border border-dark-border-primary">
        <!-- Modal Header with Icon -->
        <div class="flex items-center p-3 border-b border-dark-border-primary bg-dark-bg-primary">
            <div class="p-1.5 bg-blue-500/20 rounded-full mr-2">
                <Database size={16} class="text-blue-400" />
            </div>
            <h2 class="text-base font-medium text-dark-text-primary flex-grow" id="persistence-modal-title">
                Chat Storage Settings
            </h2>
        </div>
        
        <!-- Modal Body -->
        <div class="p-4">
            <p class="text-sm text-dark-text-primary mb-3">
                Would you like to save chat history locally for <strong class="font-medium">{verusIdName}</strong>?
            </p>
            
            <!-- Two-column comparison -->
            <div class="grid grid-cols-2 gap-3 mb-4">
                <!-- Save Option Column -->
                <div class="bg-brand-green/10 rounded-lg p-3 border border-brand-green/30">
                    <div class="flex items-center mb-2">
                        <CheckCircle size={14} class="text-green-400 mr-1.5 flex-shrink-0" />
                        <h3 class="text-sm font-semibold text-green-300">If Saved</h3>
                    </div>
                    <p class="text-xs text-green-400 mb-1">
                        Conversations remain available between sessions
                    </p>
                    <p class="text-xs text-green-400">
                        Can be deleted anytime from Settings
                    </p>
                </div>
                
                <!-- Don't Save Column -->
                <div class="bg-dark-bg-primary rounded-lg p-3 border border-dark-border-secondary">
                    <div class="flex items-center mb-2">
                        <AlertCircle size={14} class="text-dark-text-secondary mr-1.5 flex-shrink-0" />
                        <h3 class="text-sm font-semibold text-dark-text-primary">If Not Saved</h3>
                    </div>
                    <p class="text-xs text-dark-text-secondary mb-1">
                        Your sent messages are lost on logout
                    </p>
                    <p class="text-xs text-dark-text-secondary">
                        Less convenient but more private
                    </p>
                </div>
            </div>
            
            <!-- Important Notes -->
            <div class="space-y-2">
                <!-- Note about received messages -->
                <div class="flex items-start">
                    <MessageSquare size={14} class="text-blue-400 mt-0.5 mr-2 flex-shrink-0" />
                    <p class="text-xs text-dark-text-secondary">
                        <span class="font-medium">Note:</span> Received messages can always be retrieved from the blockchain.
                    </p>
                </div>
                
                <!-- Warning about unencrypted storage -->
                <div class="flex items-start">
                    <AlertCircle size={14} class="text-red-400 mt-0.5 mr-2 flex-shrink-0" />
                    <p class="text-xs text-red-400">
                        <span class="font-medium">Warning:</span> All saved chat data is stored unencrypted on this computer.
                    </p>
                </div>
            </div>
        </div>

        <!-- Modal Footer -->
        <div class="px-4 py-3 bg-dark-bg-primary border-t border-dark-border-primary flex justify-end space-x-3">
            <button 
                type="button" 
                class="py-2 px-3 border border-dark-border-secondary rounded-md shadow-sm text-xs font-medium text-dark-text-primary bg-dark-bg-tertiary hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-dark-bg-primary focus:ring-dark-border-secondary transition-colors"
                on:click={handleDontSave}
            >
                Don't Save
            </button>
            <button 
                type="button" 
                class="py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white bg-brand-green hover:bg-brand-green-hover focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-dark-bg-primary focus:ring-brand-green transition-colors flex items-center"
                on:click={handleSave}
            >
                <HardDriveDownload size={14} class="mr-1.5" />
                Save Locally
            </button>
        </div>
    </div>
</div>
{/if}

<style>
    /* Add any specific styles if needed */
</style> 