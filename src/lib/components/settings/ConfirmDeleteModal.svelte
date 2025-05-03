<svelte:window on:keydown={handleKeydown}/>

<script lang="ts">
// Component: src/lib/components/settings/ConfirmDeleteModal.svelte
// Description: Modal prompting the user to confirm deletion of local chat history.
// Changes:
// - Created component.
// - Moved svelte:window to top level.
// - Improved UI to match PersistencePromptModal style.
// - Enhanced visual organization and messaging.
// - Added better iconography and consistent styling.

    import { createEventDispatcher } from 'svelte';
    import { Trash2, AlertTriangle, MessageSquare, CheckCircle } from 'lucide-svelte';

    // --- Props --- 
    export let showModal: boolean = false;

    // --- Events ---
    const dispatch = createEventDispatcher<{ 
        confirm: void; // User confirmed deletion
        cancel: void;  // User cancelled
    }>();

    function handleConfirm() {
        dispatch('confirm');
        // Parent will close modal
    }

    function handleCancel() {
        dispatch('cancel');
         // Parent will close modal
    }

    // Close modal if clicked outside
    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            handleCancel();
        }
    }

    // Handle backdrop keydown for accessibility
    function handleBackdropKeydown(event: KeyboardEvent) {
        if (event.target === event.currentTarget && (event.key === 'Enter' || event.key === ' ')) {
            event.preventDefault();
            handleCancel();
        }
    }

    // Handle Escape key press
    function handleKeydown(event: KeyboardEvent) {
        // Only act if the modal is actually shown
		if (showModal && event.key === 'Escape') {
			handleCancel();
		}
	}

</script>

{#if showModal}
<div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40 backdrop-blur-sm transition-opacity duration-200 font-sans"
    class:opacity-100={showModal}
    class:opacity-0={!showModal}
    on:click={handleBackdropClick}
    on:keydown={handleBackdropKeydown}
    role="dialog" 
    tabindex="-1"
    aria-modal="true"
    aria-labelledby="confirm-delete-modal-title"
>
    <div class="bg-white rounded-lg shadow-xl w-full max-w-md overflow-hidden transform transition-all duration-300 border border-gray-100">
        <!-- Modal Header with Icon -->
        <div class="flex items-center p-3 border-b border-gray-200 bg-gray-50">
            <div class="p-1.5 bg-red-50 rounded-full mr-2">
                <Trash2 size={16} class="text-red-600" />
            </div>
            <h2 class="text-base font-medium text-gray-800 flex-grow" id="confirm-delete-modal-title">
                Delete Chat History
            </h2>
        </div>
        
        <!-- Modal Body -->
        <div class="p-4">
            <p class="text-sm text-gray-700 mb-3">
                Are you sure you want to delete all locally saved chat data for this VerusID?
            </p>
            
            <!-- Warning Box -->
            <div class="bg-red-50 rounded-lg p-3 border border-red-100 mb-4">
                <div class="flex items-center mb-2">
                    <AlertTriangle size={14} class="text-red-600 mr-1.5 flex-shrink-0" />
                    <h3 class="text-sm font-semibold text-red-800">Warning</h3>
                </div>
                <p class="text-xs text-red-700 mb-1">
                    This action cannot be undone. All locally stored conversations and message history will be permanently deleted.
                </p>
            </div>
            
            <!-- What's Affected Section -->
            <div class="space-y-2 mb-3">
                <div class="flex items-start">
                    <CheckCircle size={14} class="text-gray-500 mt-0.5 mr-2 flex-shrink-0" />
                    <p class="text-xs text-gray-600">
                        <span class="font-medium">Your sent messages</span> will be permanently lost from this device.
                    </p>
                </div>
                <div class="flex items-start">
                    <MessageSquare size={14} class="text-blue-500 mt-0.5 mr-2 flex-shrink-0" />
                    <p class="text-xs text-gray-600">
                        <span class="font-medium">Received messages</span> can still be retrieved from the blockchain.
                    </p>
                </div>
            </div>
        </div>

        <!-- Modal Footer -->
        <div class="px-4 py-3 bg-gray-50 border-t border-gray-200 flex justify-end space-x-3">
            <button 
                type="button" 
                class="py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-200 transition-colors"
                on:click={handleCancel}
            >
                Cancel
            </button>
            <button 
                type="button" 
                class="py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-colors flex items-center"
                on:click={handleConfirm}
            >
                <Trash2 size={14} class="mr-1.5" />
                Yes, Delete All
            </button>
        </div>
    </div>
</div>
{/if}

<style>
    /* Add any specific styles if needed */
</style> 