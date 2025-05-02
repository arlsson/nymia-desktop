<script lang="ts">
// Component: src/lib/components/chat/NewChatModal.svelte
// Description: Modal popup for starting a new chat conversation.
// Handles VerusID input, eligibility checks, history import option, and initiation.
// Changes:
// - Refined UI with transitions
// - Matched button styling with OnboardingFlow.svelte
// - Used consistent green color scheme from OnboardingFlow
// - Made UI more compact
// - Added Escape key listener for modal dismissal (Accessibility)
// - Added ARIA roles and attributes to modal container (Accessibility)
// - Added Enter key listener to input field to trigger Find User/Start Chat.

	import { createEventDispatcher, onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { X, Loader2, Search, Send, CheckCircle, AlertTriangle, Info, UserPlus } from 'lucide-svelte';
	import type { FormattedIdentity, ChatMessage } from '$lib/types';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	// --- Props --- 
	export let showModal = false; // Controlled by parent
	export let loggedInUserPrivateAddress: string | null = null; // Needed for history check
	export let existingConversationIds: string[] = []; // Pass list of current convo IDs

	// --- State --- 
	let targetVerusId = '';
	let isLoading = false;
	let statusMessage = '';
	let isError = false;
	let isSuccess = false; // Indicates eligibility check passed
	let eligibleIdentity: FormattedIdentity | null = null;
	let foundHistory: ChatMessage[] | null = null;
	let importHistory = true; // Default to checked as per PRD
    let checkAttempted = false; // Track if eligibility check was run
    let modalElement: HTMLElement; // Reference to modal for focus management (optional improvement)
    let modalHeaderId = `modal-header-${Math.random().toString(36).substring(2)}`; // Unique ID for aria-labelledby

	// --- Events --- 
	const dispatch = createEventDispatcher<{ 
		close: void;
		'start-chat': { identity: FormattedIdentity; history?: ChatMessage[] }; 
	}>();

    // --- Lifecycle & Reactivity --- 
    // Reset state when modal is opened/closed
    $: if (showModal) {
        resetState();
    } else {
        // Delay reset slightly on close to allow fade-out
        setTimeout(resetState, 300);
    }

    // Handle Escape key press
    function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeModal();
		}
	}

    onMount(() => {
		window.addEventListener('keydown', handleKeydown);
        // Optional: Focus management when modal opens
        // if (modalElement) modalElement.focus();
	});

	onDestroy(() => {
		window.removeEventListener('keydown', handleKeydown);
	});

    // Check for existing conversation reactively
    $: if (targetVerusId && existingConversationIds.includes(targetVerusId.trim().toLowerCase())) {
        if (!isLoading) { // Avoid overwriting loading/error states
            statusMessage = 'You already have a conversation with this user.';
            isError = true;
            isSuccess = false;
            checkAttempted = false; // Reset check flag
        }
    } else if (isError && statusMessage === 'You already have a conversation with this user.') {
        // Clear the specific error if ID changes and no longer matches existing
        clearStatus();
    }

    // Enable/disable Find button
    $: isFindDisabled = isLoading || targetVerusId.trim().length < 3 || !targetVerusId.includes('@') || (isError && statusMessage === 'You already have a conversation with this user.');
    // Enable/disable Start Chat button
    $: isStartDisabled = isLoading || !isSuccess;

	// --- Functions --- 
	function closeModal() {
		dispatch('close');
	}

    function resetState() {
        targetVerusId = '';
        isLoading = false;
        statusMessage = '';
        isError = false;
        isSuccess = false;
        eligibleIdentity = null;
        foundHistory = null;
        importHistory = true;
        checkAttempted = false;
    }

    function clearStatus() {
        statusMessage = '';
        isError = false;
    }

	async function handleFindUser() {
		clearStatus(); 
        checkAttempted = true;
        isLoading = true;
        isSuccess = false;
        eligibleIdentity = null;
        foundHistory = null;
		const targetId = targetVerusId.trim().toLowerCase();

		try {
            // 1. Check Eligibility
			statusMessage = 'Checking eligibility...';
			console.log(`Calling check_identity_eligibility for: ${targetId}`);
            eligibleIdentity = await invoke<FormattedIdentity>('check_identity_eligibility', { targetIdentityName: targetId });
            console.log('Eligibility check successful:', eligibleIdentity);
            
            // Ensure we have the logged-in user's private address for history check
            if (!loggedInUserPrivateAddress) {
                throw new Error("Logged-in user's private address is missing.");
            }

            // 2. Check History (only if eligible)
            statusMessage = 'User found. Checking history...';
            console.log(`Calling get_chat_history from: ${targetId} for owner: ${loggedInUserPrivateAddress}`);
            foundHistory = await invoke<ChatMessage[]>('get_chat_history', { 
                targetIdentityName: targetId, 
                ownPrivateAddress: loggedInUserPrivateAddress 
            });
            console.log('History check successful, found:', foundHistory.length);

            // 3. Update State on Success
            statusMessage = 'User ready to chat.';
            isSuccess = true;
		} catch (error: any) {
			console.error('Error finding user or history:', error);
            isError = true;
            isSuccess = false;
            eligibleIdentity = null;
            foundHistory = null;

            // Handle specific backend errors based on PRD
            if (error?.RpcSpecific) {
                const rpcError = error.RpcSpecific;
                if (rpcError === 'NotFoundOrIneligible' || rpcError.NotFoundOrIneligible) {
                    statusMessage = 'User not found or cannot receive private messages.';
                } else if (rpcError === 'InvalidFormat' || rpcError.InvalidFormat) {
                    statusMessage = 'Invalid VerusID format (e.g., user@).';
                } else if (rpcError.Rpc) { 
                    statusMessage = `Connection error: ${rpcError.Rpc.message}`;
                } else {
                     statusMessage = `Error: ${JSON.stringify(rpcError)}`; // Fallback for other RPC errors
                }
            } else if (typeof error === 'string' && error.includes("private address is missing")) {
                statusMessage = 'Internal error: Cannot check history.'; // User shouldn't see this normally
            } else {
                statusMessage = 'An unexpected error occurred while checking the user.';
            }
		} finally {
			isLoading = false;
		}
	}

	function handleStartChat() {
		if (!eligibleIdentity) return;
		console.log('Dispatching start-chat event');
		dispatch('start-chat', { 
			identity: eligibleIdentity,
			history: (foundHistory && importHistory) ? foundHistory : undefined 
		});
		closeModal();
	}

    function handleInputKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault(); // Prevent default form submission behavior
            if (isSuccess && !isStartDisabled) {
                handleStartChat();
            } else if (!isSuccess && !isFindDisabled) {
                handleFindUser();
            }
        }
    }

</script>

{#if showModal}
<div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40 backdrop-blur-sm transition-opacity duration-200" 
    class:opacity-100={showModal}
    class:opacity-0={!showModal}
    transition:fade={{ duration: 150 }}
    on:click={closeModal} 
    role="presentation"
>
    <!-- Modal Content -->
    <div 
        bind:this={modalElement}
        class="bg-white rounded-lg shadow-xl w-full max-w-sm overflow-hidden transform transition-all duration-300 border border-gray-100"
        class:scale-100={showModal}
        class:scale-95={!showModal}
        transition:scale={{ duration: 200, easing: quintOut }}
        on:click|stopPropagation
        role="dialog" 
        aria-modal="true"
        aria-labelledby={modalHeaderId}
    >
        <!-- Modal Header with Icon -->
        <div class="flex items-center p-3 border-b border-gray-200 bg-gray-50">
            <div class="p-1.5 bg-green-50 rounded-full mr-2">
                <UserPlus size={16} class="text-green-600" />
            </div>
            <h2 class="text-base font-medium text-gray-800 flex-grow" id={modalHeaderId}>Start New Chat</h2>
            <button 
                on:click={closeModal}
                class="text-gray-400 hover:text-gray-600 p-1 rounded-full hover:bg-gray-100 transition-colors focus:outline-none focus:ring-1 focus:ring-gray-200"
                aria-label="Close modal"
            >
                <X size={16} strokeWidth={2.5} />
            </button>
        </div>

        <!-- Modal Body -->
        <div class="p-4 space-y-3">
            <div class="space-y-1.5">
                <label for="verusid-input" class="block text-sm font-medium text-gray-700">Enter VerusID</label>
                <div class="relative">
                    <input 
                        type="text" 
                        id="verusid-input"
                        bind:value={targetVerusId}
                        placeholder="friend@ or friend.parent@"
                        disabled={isLoading || isSuccess}
                        class="w-full p-2 border border-gray-300 rounded-md shadow-sm text-sm focus:ring-1 focus:ring-green-500 focus:border-green-500 disabled:bg-gray-50 disabled:cursor-not-allowed transition-colors duration-200"
                        on:keydown={handleInputKeydown}
                    />
                </div>
            </div>

            <!-- Status/Error Area -->
            {#if statusMessage}
                <div 
                    class="p-2 text-xs rounded-md border flex items-start space-x-2" 
                    class:bg-red-50={isError} class:border-red-200={isError} class:text-red-700={isError}
                    class:bg-blue-50={!isError && isLoading} class:border-blue-200={!isError && isLoading} class:text-blue-700={!isError && isLoading}
                    class:bg-green-50={isSuccess} class:border-green-200={isSuccess} class:text-green-700={isSuccess}
                    class:bg-yellow-50={!isError && !isLoading && !isSuccess && checkAttempted} class:border-yellow-200={!isError && !isLoading && !isSuccess && checkAttempted} class:text-yellow-700={!isError && !isLoading && !isSuccess && checkAttempted}
                    class:bg-gray-50={!isError && !isLoading && !isSuccess && !checkAttempted && statusMessage} class:border-gray-200={!isError && !isLoading && !isSuccess && !checkAttempted && statusMessage} class:text-gray-600={!isError && !isLoading && !isSuccess && !checkAttempted && statusMessage}
                    transition:fly={{ y: 5, duration: 200 }}
                >
                    <div class="mt-0.5">
                        {#if isLoading}
                            <Loader2 size={14} class="animate-spin text-blue-500" />
                        {:else if isError}
                            <AlertTriangle size={14} class="text-red-500" />
                        {:else if isSuccess}
                            <CheckCircle size={14} class="text-green-600" />
                        {:else}
                            <Info size={14} class="text-gray-500" />
                        {/if}
                    </div>
                    <span>{statusMessage}</span>
                </div>
            {/if}

            <!-- History Import Option -->
            {#if isSuccess && foundHistory && foundHistory.length > 0}
                <div 
                    class="flex items-center p-2 bg-green-50 border border-green-100 rounded-md mt-2 transition-all"
                    transition:fly={{ y: 10, duration: 300 }}
                >
                    <input 
                        type="checkbox" 
                        id="import-history-checkbox"
                        bind:checked={importHistory}
                        class="h-4 w-4 text-green-600 border-gray-300 rounded focus:ring-green-500"
                    />
                    <label for="import-history-checkbox" class="ml-2 block text-xs text-gray-700 font-medium">
                        Import existing chat history 
                        <span class="font-normal text-gray-500">({foundHistory.length} {foundHistory.length === 1 ? 'message' : 'messages'})</span>
                    </label>
                </div>
            {/if}
        </div>

        <!-- Modal Footer -->
        <div class="px-4 py-3 bg-gray-50 border-t border-gray-200 flex justify-end space-x-3">
            <button 
                type="button"
                on:click={closeModal}
                class="py-2 px-3 border border-gray-300 rounded-md shadow-sm text-xs font-medium text-gray-700 bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-200 transition-colors"
            >
                Cancel
            </button>
            
            {#if !isSuccess}
                <button 
                    type="button"
                    on:click={handleFindUser} 
                    disabled={isFindDisabled} 
                    class="py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-60 disabled:cursor-not-allowed transition-colors flex items-center hover:bg-green-700"
                    style={`background-color: ${isFindDisabled ? '#9fcfb8' : '#419A6A'};`}
                >
                    {#if isLoading}
                        <Loader2 size={14} class="animate-spin mr-1.5" />
                        Checking...
                    {:else}
                        <Search size={14} class="mr-1.5" />
                        Find User
                    {/if}
                </button>
            {:else}
                <button 
                    type="button"
                    on:click={handleStartChat}
                    disabled={isStartDisabled}
                    class="py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-60 disabled:cursor-not-allowed transition-colors hover:bg-green-700 flex items-center"
                    style={`background-color: ${isStartDisabled ? '#9fcfb8' : '#419A6A'};`}
                >
                    <Send size={14} class="mr-1.5" />
                    Start Chat
                </button>
            {/if}
        </div>
    </div>
</div>
{/if}

<style>
    /* Add any component-specific styles here if needed */
</style> 