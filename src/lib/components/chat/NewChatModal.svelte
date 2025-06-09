<script lang="ts">
// Component: src/lib/components/chat/NewChatModal.svelte
// Description: Modal popup for starting a new chat conversation (Dark Theme).
// Handles VerusID input, eligibility checks, history import option, and initiation.
// Changes:
// - Refined UI with transitions
// - Matched button styling with OnboardingFlow.svelte
// - Used consistent green color scheme from OnboardingFlow
// - Made UI more compact
// - Added Escape key listener for modal dismissal (Accessibility)
// - Added ARIA roles and attributes to modal container, including tabindex="-1" on dialog role (Accessibility)
// - Added Enter key listener to input field to trigger Find User/Start Chat.
// - Added keyboard accessibility (Enter/Space) and tabindex to modal backdrop with button role.


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

    // Reactive class strings for buttons (Dark Theme)
    $: findUserButtonClasses = `py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-dark-bg-primary focus:ring-brand-green disabled:opacity-60 disabled:cursor-not-allowed transition-colors flex items-center hover:bg-brand-green-hover ${isFindDisabled ? 'bg-brand-green-disabled' : 'bg-brand-green'}`;
    $: startChatButtonClasses = `py-2 px-3 border border-transparent rounded-md shadow-sm text-xs font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-dark-bg-primary focus:ring-brand-green disabled:opacity-60 disabled:cursor-not-allowed transition-colors flex items-center hover:bg-brand-green-hover ${isStartDisabled ? 'bg-brand-green-disabled' : 'bg-brand-green'}`;

    // Reactive class string for status message box
    $: statusMessageClasses = `p-2 text-xs rounded-md border flex items-start space-x-2 
        ${isError ? 'bg-red-900/40 border-red-700/50 text-red-300' : 
          !isError && isLoading ? 'bg-blue-900/40 border-blue-700/50 text-blue-300' : 
          isSuccess ? 'bg-brand-green/20 border-brand-green/30 text-green-300' : 
          !isError && !isLoading && !isSuccess && checkAttempted ? 'bg-yellow-900/40 border-yellow-700/50 text-yellow-300' : 
          !isError && !isLoading && !isSuccess && !checkAttempted && statusMessage ? 'bg-dark-bg-primary border-dark-border-secondary text-dark-text-secondary' : ''}`;

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

    function handleBackdropKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            closeModal();
        }
    }

</script>

{#if showModal}
<div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-60 backdrop-blur-sm transition-opacity duration-200" 
    class:opacity-100={showModal}
    class:opacity-0={!showModal}
    transition:fade={{ duration: 150 }}
    on:click={closeModal}
    on:keydown={handleBackdropKeydown}
    role="button"
    tabindex="0"
>
    <!-- Modal Content -->
    <div 
        bind:this={modalElement}
        class="bg-dark-bg-secondary rounded-lg shadow-xl w-full max-w-sm overflow-hidden transform transition-all duration-300 border border-dark-border-primary"
        class:scale-100={showModal}
        class:scale-95={!showModal}
        transition:scale={{ duration: 200, easing: quintOut }}
        on:click|stopPropagation
        role="dialog"
        tabindex="-1"
        aria-modal="true"
        aria-labelledby={modalHeaderId}
    >
        <!-- Modal Header with Icon -->
        <div class="flex items-center p-3 border-b border-dark-border-primary bg-dark-bg-primary">
            <div class="p-1.5 bg-brand-green/20 rounded-full mr-2">
                <UserPlus size={16} class="text-brand-green" />
            </div>
            <h2 class="text-base font-medium text-dark-text-primary flex-grow" id={modalHeaderId}>Start New Chat</h2>
            <button 
                on:click={closeModal}
                class="text-dark-text-secondary hover:text-dark-text-primary p-1 rounded-full hover:bg-dark-bg-tertiary transition-colors focus:outline-none focus:ring-1 focus:ring-dark-border-secondary"
                aria-label="Close modal"
            >
                <X size={16} strokeWidth={2.5} />
            </button>
        </div>

        <!-- Modal Body -->
        <div class="p-4 space-y-3">
            <div class="space-y-1.5">
                <label for="verusid-input" class="block text-sm font-medium text-dark-text-secondary">Enter VerusID</label>
                <div class="relative">
                    <input 
                        type="text" 
                        id="verusid-input"
                        bind:value={targetVerusId}
                        placeholder="friend@ or friend.parent@"
                        disabled={isLoading || isSuccess}
                        class="w-full p-2 border border-dark-border-secondary rounded-md shadow-sm text-sm focus:ring-1 focus:ring-brand-green focus:border-brand-green disabled:bg-dark-bg-primary disabled:cursor-not-allowed transition-colors duration-200 bg-dark-bg-tertiary text-dark-text-primary placeholder-dark-text-disabled"
                        on:keydown={handleInputKeydown}
                    />
                </div>
            </div>

            <!-- Status/Error Area -->
            {#if statusMessage}
                <div 
                    class={statusMessageClasses}
                    transition:fly={{ y: 5, duration: 200 }}
                >
                    <div class="mt-0.5">
                        {#if isLoading}
                            <Loader2 size={14} class="animate-spin text-blue-400" />
                        {:else if isError}
                            <AlertTriangle size={14} class="text-red-400" />
                        {:else if isSuccess}
                            <CheckCircle size={14} class="text-green-400" />
                        {:else}
                            <Info size={14} class="text-dark-text-secondary" />
                        {/if}
                    </div>
                    <span>{statusMessage}</span>
                </div>
            {/if}

            <!-- History Import Option -->
            {#if isSuccess && foundHistory && foundHistory.length > 0}
                <div 
                    class="flex items-center p-2 bg-brand-green/10 border border-brand-green/20 rounded-md mt-2 transition-all"
                    transition:fly={{ y: 10, duration: 300 }}
                >
                    <input 
                        type="checkbox" 
                        id="import-history-checkbox"
                        bind:checked={importHistory}
                        class="h-4 w-4 text-brand-green border-dark-border-secondary rounded focus:ring-brand-green bg-dark-bg-tertiary"
                    />
                    <label for="import-history-checkbox" class="ml-2 block text-xs text-dark-text-primary font-medium">
                        Import existing chat history 
                        <span class="font-normal text-dark-text-secondary">({foundHistory.length} {foundHistory.length === 1 ? 'message' : 'messages'})</span>
                    </label>
                </div>
            {/if}
        </div>

        <!-- Modal Footer -->
        <div class="px-4 py-3 bg-dark-bg-primary border-t border-dark-border-primary flex justify-end space-x-3">
            <button 
                type="button"
                on:click={closeModal}
                class="py-2 px-3 border border-dark-border-secondary rounded-md shadow-sm text-xs font-medium text-dark-text-primary bg-dark-bg-tertiary hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-dark-bg-primary focus:ring-dark-border-secondary transition-colors"
            >
                Cancel
            </button>
            
            {#if !isSuccess}
                <button 
                    type="button"
                    on:click={handleFindUser} 
                    disabled={isFindDisabled} 
                    class={findUserButtonClasses}
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
                    class={startChatButtonClasses}
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