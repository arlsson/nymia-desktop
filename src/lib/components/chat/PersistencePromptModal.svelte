<script lang="ts">
// Component: src/lib/components/chat/PersistencePromptModal.svelte
// Description: Modal prompting the user to enable/disable local chat persistence.
// Changes:
// - Refactored to use Modal.svelte component for consistency
// - Redesigned with compact, minimalist layout inspired by ResponsibilityStep.svelte
// - Reduced text sizes and spacing for cleaner, less overwhelming design
// - Simplified comparison cards with concise messaging
// - Streamlined information sections with better visual hierarchy

    import { createEventDispatcher } from 'svelte';
    import { Download, Clock, MessageSquare, AlertTriangle } from 'lucide-svelte';
    import Modal from '../Modal.svelte';
    import Button from '../Button.svelte';

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
    }

    function handleDontSave() {
        dispatch('dontSave');
    }

    function handleClose() {
        // Don't allow closing without making a choice - user must decide
    }
</script>

<Modal 
    bind:show={showModal} 
    size="md"
    closeOnBackdrop={false}
    closeOnEscape={false}
>
    <!-- Header -->
    <div slot="header" let:modalHeaderId>
        <div class="flex items-center">
            <h2 class="text-base font-semibold text-dark-text-primary cursor-default select-none" id={modalHeaderId}>
                Chat Storage Settings
            </h2>
        </div>
    </div>

    <!-- Body -->
    <div class="p-4">
        <!-- Question -->
        <div class="mb-4">
            <p class="text-sm text-dark-text-primary cursor-default select-none">
                Would you like to save chat history locally for <span class="font-medium text-white">{verusIdName}</span>?
            </p>
        </div>

        <!-- Options Comparison -->
        <div class="grid grid-cols-2 gap-3 mb-4">
            <!-- Save Option -->
            <div class=" rounded-lg p-3 ">
                <div class="flex items-center mb-2">
                    <div class="flex items-center justify-center w-6 h-6 bg-green-500/20 rounded-lg mr-2">
                        <Download size={12} class="text-green-400" />
                    </div>
                    <h3 class="text-xs font-semibold text-green-300 cursor-default select-none">If Saved</h3>
                </div>
                
                <div class="space-y-1">
                    <p class="text-xs text-green-200/80 cursor-default select-none">
                        Conversations remain between sessions.
                    </p>
                    <p class="text-xs text-green-200/80 cursor-default select-none">
                        Can be deleted from Settings.
                    </p>
                </div>
            </div>

            <!-- Don't Save Option -->
            <div class="rounded-lg p-3">
                <div class="flex items-center mb-2">
                    <div class="flex items-center justify-center w-6 h-6 bg-gray-500/20 rounded-lg mr-2">
                        <Clock size={12} class="text-gray-400" />
                    </div>
                    <h3 class="text-xs font-semibold text-gray-300 cursor-default select-none">If Not Saved</h3>
                </div>
                
                <div class="space-y-1">
                    <p class="text-xs text-gray-300/80 cursor-default select-none">
                        Sent messages lost on logout.
                    </p>
                    <p class="text-xs text-gray-300/80 cursor-default select-none">
                        More private, less convenient.
                    </p>
                </div>
            </div>
        </div>

        <!-- Important Information -->
        <div class="space-y-2">
            <!-- Note about received messages -->
            <div class="flex items-start p-2 bg-blue-500/5 border border-blue-500/20 rounded">
                <MessageSquare size={12} class="text-blue-400 mt-1 mr-2 flex-shrink-0" />
                <p class="text-xs text-blue-200/90 cursor-default select-none">
                    <span class="font-medium">Note:</span> Received messages can always be retrieved from the blockchain.
                </p>
            </div>

            <!-- Warning about encryption -->
            <div class="flex items-start p-2 bg-orange-500/5 border border-orange-500/20 rounded">
                <AlertTriangle size={12} class="text-orange-400 mt-1 mr-2 flex-shrink-0" />
                <p class="text-xs text-orange-200/90 cursor-default select-none">
                    <span class="font-medium">Warning:</span> All saved data is stored unencrypted on this computer.
                </p>
            </div>
        </div>
    </div>

    <!-- Footer -->
    <div slot="footer">
        <div class="flex justify-end space-x-3">
            <Button 
                variant="secondary"
                on:click={handleDontSave}
            >
                Don't Save
            </Button>
            <Button 
                variant="primary"
                iconComponent={Download}
                on:click={handleSave}
            >
                Save Locally
            </Button>
        </div>
    </div>
</Modal>

<style>
    /* Add any specific styles if needed */
</style> 