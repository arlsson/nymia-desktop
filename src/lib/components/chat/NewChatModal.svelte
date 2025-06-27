<script lang="ts">
// Component: src/lib/components/chat/NewChatModal.svelte
// Description: Modal popup for starting a new chat conversation (Dark Theme).
// Handles VerusID input, eligibility checks, history import option, and initiation.
// Changes:
// - Refactored to use the reusable Modal component
// - Moved content into appropriate slots (header, body, footer)
// - Removed duplicate modal functionality now handled by Modal component
// - Simplified event handling and state management

	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { X, Loader2, Search, Send, CheckCircle, AlertTriangle, Info, UserPlus } from 'lucide-svelte';
	import type { FormattedIdentity, ChatMessage } from '$lib/types';
	import { fly } from 'svelte/transition';
	import Modal from '../Modal.svelte';
	import Button from '../Button.svelte';

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
    let inputElement: HTMLInputElement; // Reference to input element

	// --- Events --- 
	const dispatch = createEventDispatcher<{ 
		close: void;
		'start-chat': { identity: FormattedIdentity; history?: ChatMessage[] }; 
	}>();

    // --- Reactivity --- 
    // Reset state when modal is opened/closed
    $: if (showModal) {
        resetState();
        // Auto-focus input when modal opens
        setTimeout(() => {
            if (inputElement) {
                inputElement.focus();
            }
        }, 100);
    } else {
        // Delay reset slightly on close to allow fade-out
        setTimeout(resetState, 300);
    }

    // Re-focus input after state changes (like after finding a user)
    $: if (isSuccess && inputElement) {
        setTimeout(() => {
            inputElement.focus();
        }, 50);
    }

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

    // Reactive class string for status message box
    $: statusMessageClasses = `p-4 rounded-lg border backdrop-blur-sm
        ${isError ? 'bg-red-900/30 border-red-700/40 text-red-200' : 
          !isError && isLoading ? 'bg-blue-900/30 border-blue-700/40 text-blue-200' : 
          isSuccess ? 'bg-brand-green/15 border-brand-green/25 text-green-200' : 
          !isError && !isLoading && !isSuccess && checkAttempted ? 'bg-yellow-900/30 border-yellow-700/40 text-yellow-200' : 
          !isError && !isLoading && !isSuccess && !checkAttempted && statusMessage ? 'bg-dark-bg-secondary/50 border-dark-border-primary text-dark-text-secondary' : ''}`;

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
        console.log('Input keydown:', event.key, { isSuccess, isStartDisabled, isFindDisabled });
        if (event.key === 'Enter') {
            event.preventDefault(); // Always prevent default
            event.stopPropagation(); // Prevent event bubbling
            
            if (isSuccess && !isStartDisabled) {
                console.log('Starting chat via Enter');
                handleStartChat();
            } else if (!isSuccess && !isFindDisabled) {
                console.log('Finding user via Enter');
                handleFindUser();
            } else {
                console.log('Enter key ignored - conditions not met');
            }
            // If both conditions fail, we simply do nothing but still prevent default behavior
        }
    }

    // Global keydown handler for the modal
    function handleModalKeydown(event: CustomEvent<KeyboardEvent>) {
        const keyboardEvent = event.detail;
        if (keyboardEvent.key === 'Enter' && !isLoading) {
            console.log('Modal keydown Enter:', { isSuccess, isStartDisabled, isFindDisabled });
            if (isSuccess && !isStartDisabled) {
                keyboardEvent.preventDefault();
                keyboardEvent.stopPropagation();
                handleStartChat();
            } else if (!isSuccess && !isFindDisabled) {
                keyboardEvent.preventDefault();
                keyboardEvent.stopPropagation();
                handleFindUser();
            }
        }
    }
</script>

<Modal 
	show={showModal} 
	size="sm" 
	on:close={closeModal}
	on:keydown={handleModalKeydown}
>
	<!-- Header -->
	<svelte:fragment slot="header" let:modalHeaderId let:handleClose>
		<h2 class="text-base font-medium text-dark-text-primary flex-grow cursor-default select-none" id={modalHeaderId}>Start New Chat</h2>
		<button 
			on:click={handleClose}
			class="text-dark-text-secondary hover:text-dark-text-primary p-1 rounded-full hover:bg-dark-bg-tertiary transition-colors focus:outline-none focus:ring-1 focus:ring-dark-border-secondary"
			aria-label="Close modal"
		>
			<X size={16} strokeWidth={2.5} />
		</button>
	</svelte:fragment>

	<!-- Body -->
	<div class="p-6 space-y-4">
		<div class="space-y-2">
			<label for="verusid-input" class="block text-sm font-semibold text-dark-text-primary cursor-default select-none">Enter VerusID</label>
			<div class="relative">
				<input 
					type="text" 
					id="verusid-input"
					bind:value={targetVerusId}
					placeholder="friend@ or friend.parent@"
					disabled={isLoading}
					class="w-full px-4 py-3 border rounded-lg shadow-sm text-sm focus:ring-2 transition-all duration-200 font-medium {isSuccess 
						? 'border-brand-green bg-brand-green/5 text-brand-green focus:ring-brand-green/30 focus:border-brand-green/60' 
						: 'border-dark-border-primary bg-dark-bg-primary text-dark-text-primary focus:ring-brand-green/30 focus:border-brand-green'} disabled:bg-dark-bg-primary disabled:cursor-not-allowed placeholder-dark-text-disabled"
					on:keydown={handleInputKeydown}
					bind:this={inputElement}
				/>
				<div class="absolute inset-0 rounded-lg bg-gradient-to-r from-transparent via-brand-green/5 to-transparent opacity-0 focus-within:opacity-100 transition-opacity duration-200 pointer-events-none {isSuccess ? 'opacity-30' : ''}"></div>
			</div>
			<p class="text-xs text-dark-text-secondary cursor-default select-none">
				{#if isSuccess}
					Press <kbd class="px-1 py-0.5 bg-dark-bg-tertiary border border-dark-border-secondary rounded text-xs font-mono cursor-default select-none">Enter</kbd> to start chatting
				{:else}
					Enter a VerusID to start a private conversation
				{/if}
			</p>
		</div>

		<!-- Status/Error Area -->
		{#if statusMessage}
			<div 
				class="relative overflow-hidden {statusMessageClasses}"
				transition:fly={{ y: 5, duration: 200 }}
			>
				<div class="flex items-start space-x-3">
					<div class="mt-0.5 flex-shrink-0">
						{#if isLoading}
							<Loader2 size={16} class="animate-spin text-blue-400" />
						{:else if isError}
							<AlertTriangle size={16} class="text-red-400" />
						{:else if isSuccess}
							<CheckCircle size={16} class="text-green-400" />
						{:else}
							<Info size={16} class="text-dark-text-secondary" />
						{/if}
					</div>
					<span class="text-sm font-medium leading-relaxed">{statusMessage}</span>
				</div>
				<!-- Subtle background gradient for depth -->
				<div class="absolute inset-0 opacity-10 bg-gradient-to-r from-transparent via-current to-transparent"></div>
			</div>
		{/if}

		<!-- History Import Option -->
		{#if isSuccess && foundHistory && foundHistory.length > 0}
			<div 
				class="relative p-4 bg-gradient-to-r from-brand-green/5 to-emerald-500/5 border border-brand-green/20 rounded-lg transition-all duration-300"
				transition:fly={{ y: 10, duration: 300 }}
			>
				<div class="flex items-start space-x-3">
					<input 
						type="checkbox" 
						id="import-history-checkbox"
						bind:checked={importHistory}
						class="mt-0.5 h-4 w-4 text-brand-green border-dark-border-secondary rounded focus:ring-brand-green focus:ring-2 bg-dark-bg-tertiary"
					/>
					<div class="flex-1">
						<label for="import-history-checkbox" class="block text-sm font-semibold text-dark-text-primary cursor-pointer">
							Import existing chat history
						</label>
						<p class="text-xs text-dark-text-secondary mt-1">
							{foundHistory.length} {foundHistory.length === 1 ? 'message' : 'messages'} found from previous conversations
						</p>
					</div>
				</div>
				<!-- Subtle shine effect -->
				<div class="absolute inset-0 rounded-lg bg-gradient-to-r from-transparent via-brand-green/10 to-transparent opacity-0 hover:opacity-100 transition-opacity duration-200 pointer-events-none"></div>
			</div>
		{/if}
	</div>

	<!-- Footer -->
	<svelte:fragment slot="footer">
		<div class="flex justify-end space-x-3">
		<Button 
			variant="secondary"
			on:click={closeModal}
		>
			Cancel
		</Button>
		
		{#if !isSuccess}
			<Button 
				variant="primary"
				disabled={isFindDisabled}
				loading={isLoading}
				loadingText="Checking..."
				iconComponent={Search}
				on:click={handleFindUser}
			>
				Find User
			</Button>
		{:else}
			<Button 
				variant="primary"
				disabled={isStartDisabled}
				iconComponent={Send}
				on:click={handleStartChat}
			>
				Start Chat
			</Button>
		{/if}
		</div>
	</svelte:fragment>
</Modal>

<style>
    /* Add any component-specific styles here if needed */
</style> 