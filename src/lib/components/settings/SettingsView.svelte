<script lang="ts">
// Component: src/lib/components/settings/SettingsView.svelte
// Description: Displays application settings, replacing the ConversationView.
// Changes:
// - Created component with Persistence Toggle and Delete History button.
// - Improved visual design with more compact UI.
// - Enhanced iconography and layout.
// - Made text consistent with PersistencePromptModal.
// - Added better visual hierarchy.

    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { Trash2, Database, RotateCcw, ArrowLeft, HardDriveDownload, AlertCircle, MessageSquare, Save } from 'lucide-svelte';
    import ConfirmDeleteModal from './ConfirmDeleteModal.svelte';

    // --- Props ---
    export let currentPersistenceSetting: boolean | null = null; // The current value from parent
    export let loggedInUserIAddress: string | null = null; // Needed for delete command

    // --- State ---
    let showConfirmDelete = false;
    let internalPersistenceEnabled: boolean = false;

    // --- Events ---
    const dispatch = createEventDispatcher<{ 
        togglePersistence: { enabled: boolean }; // Inform parent of toggle change
        deleteHistory: void; // Request parent to delete history
        closeSettings: void; // Request parent to close the settings view
    }>();

    // --- Lifecycle & Reactivity ---
    // Sync internal state when prop changes
    $: if (currentPersistenceSetting !== null) {
        internalPersistenceEnabled = currentPersistenceSetting;
    }

    // Handle Escape key press to close settings
    function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
            // Close settings view if no modal is open
            if (!showConfirmDelete) {
			    dispatch('closeSettings');
            }
		}
	}

    onMount(() => {
		window.addEventListener('keydown', handleKeydown);
	});

	onDestroy(() => {
		window.removeEventListener('keydown', handleKeydown);
	});

    // --- Functions ---
    function handleToggleChange() {
        // Update internal state immediately for visual feedback
        internalPersistenceEnabled = !internalPersistenceEnabled;
        // Dispatch the new value to the parent to handle saving
        dispatch('togglePersistence', { enabled: internalPersistenceEnabled });
    }

    function openConfirmDeleteModal() {
        showConfirmDelete = true;
    }

    function handleConfirmDelete() {
        showConfirmDelete = false;
        dispatch('deleteHistory');
    }

    function handleCancelDelete() {
        showConfirmDelete = false;
    }

    function handleGoBack() {
        dispatch('closeSettings');
    }

</script>

<div class="flex flex-col h-full bg-gray-50">
    <!-- Settings Header -->
    <div class="flex items-center h-[50px] px-3 bg-white border-b border-gray-200 flex-shrink-0 shadow-sm">
        <button 
            on:click={handleGoBack}
            class="p-1.5 rounded text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-200 transition-colors duration-150 mr-2"
            title="Back to Chat"
        >
            <ArrowLeft size={16} />
        </button>
        <span class="font-medium text-base text-gray-800">Settings</span>
    </div>

    <!-- Settings Content -->
    <div class="flex-grow p-3 space-y-4 overflow-y-auto">

        <!-- Persistence Setting Card -->
        <div class="bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
            <div class="border-b border-gray-100 bg-gray-50 px-3 py-2 flex items-center">
                <Database size={14} class="text-blue-600 mr-2" />
                <h3 class="text-sm font-medium text-gray-800">Chat History Storage</h3>
            </div>
            
            <div class="p-3">
                <div class="flex items-center justify-between mb-3">
                    <div class="space-y-1 mr-4">
                        <p class="text-sm font-medium text-gray-700">
                            Save chat history locally
                        </p>
                        <p class="text-xs text-gray-500">
                            {internalPersistenceEnabled 
                                ? 'Your conversations will remain available between sessions' 
                                : 'Your sent messages will be lost when you log out'}
                        </p>
                    </div>
                    
                    {#if currentPersistenceSetting !== null}
                        <!-- Toggle Switch -->
                        <button 
                            role="switch"
                            aria-checked={internalPersistenceEnabled}
                            on:click={handleToggleChange}
                            class={`relative inline-flex items-center h-6 rounded-full w-11 transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 ${internalPersistenceEnabled ? 'bg-[#419A6A]' : 'bg-gray-300'}`}
                        >
                            <span class="sr-only">Enable chat history persistence</span>
                            <span 
                                class={`inline-block w-4 h-4 transform bg-white rounded-full transition-transform duration-200 ease-in-out ${internalPersistenceEnabled ? 'translate-x-6' : 'translate-x-1'}`}
                            />
                        </button>
                    {:else}
                        <span class="text-xs text-gray-500 italic">Loading...</span>
                    {/if}
                </div>
                
                <!-- Important Notes -->
                <div class="space-y-1.5">
                    <!-- Note about received messages -->
                    <div class="flex items-start">
                        <MessageSquare size={12} class="text-blue-500 mt-0.5 mr-1.5 flex-shrink-0" />
                        <p class="text-xs text-gray-600">
                            Received messages can always be retrieved from the blockchain.
                        </p>
                    </div>
                    
                    <!-- Warning about unencrypted storage -->
                    <div class="flex items-start">
                        <AlertCircle size={12} class="text-red-500 mt-0.5 mr-1.5 flex-shrink-0" />
                        <p class="text-xs text-red-600">
                            All saved chat data is stored unencrypted on this computer.
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Delete History Card -->
        <div class="bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
            <div class="border-b border-gray-100 bg-gray-50 px-3 py-2 flex items-center">
                <Trash2 size={14} class="text-red-500 mr-2" />
                <h3 class="text-sm font-medium text-gray-800">Data Management</h3>
            </div>
            
            <div class="p-3">
                <div class="flex items-start justify-between">
                    <div class="space-y-1 mr-4">
                        <p class="text-sm font-medium text-gray-700">
                            Delete chat history
                        </p>
                        <p class="text-xs text-gray-500">
                            Remove all locally stored conversations for this VerusID
                        </p>
                        <p class="text-xs text-red-500 font-medium mt-1">
                            This action cannot be undone
                        </p>
                    </div>
                    
                    <button 
                        type="button"
                        on:click={openConfirmDeleteModal}
                        disabled={currentPersistenceSetting === null}  
                        class="py-1.5 px-3 border border-red-300 rounded shadow-sm text-xs font-medium text-red-700 bg-white hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-red-400 transition-colors ease-in-out disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
                    >
                        <Trash2 size={14} class="mr-1.5"/>
                        Delete All
                    </button>
                </div>
            </div>
        </div>
        
        <!-- Could add more settings cards here in the future -->
        
    </div>
</div>

<!-- Confirmation Modal -->
<ConfirmDeleteModal 
    bind:showModal={showConfirmDelete}
    on:confirm={handleConfirmDelete}
    on:cancel={handleCancelDelete}
/>

<style>
/* Styles for the toggle switch can be added or adjusted here if needed */
</style> 