<script lang="ts">
// Component: src/lib/components/onboarding/BlockchainDetectionStep.svelte
// Description: Automatic blockchain detection and selection with list-based UI.
// Replaces the old BlockchainStep + CredentialsStep manual workflow with automatic detection.
// Features:
// - Parallel detection of all supported blockchains on component mount
// - Skeleton loading states during detection
// - Clean list layout with Lucide icons for status indicators
// - Manual folder selection for custom config locations
// - Passes credentials via props (saved later during login, not here)

    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { slide, fade } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';
    
    // Import Lucide Icons
    import { 
        CheckCircle, 
        Loader, 
        XCircle, 
        Clock, 
        AlertTriangle, 
        Search,
        ChevronRight,
        FolderOpen
    } from 'lucide-svelte';
    
    // Import Types
    import type { 
        ParallelDetectionResult, 
        BlockchainDetectionResult, 
        BlockchainStatus,
        Credentials 
    } from '$lib/types';

    // --- Props ---
    export let selectedBlockchainId: string | null = null;

    // --- State ---
    let detectionState: 'loading' | 'completed' | 'error' = 'loading';
    let detectionResults: BlockchainDetectionResult[] = [];
    let detectionError: string | null = null;
    let detectionDuration: number = 0;
    let showManualSelection = false;
    let isManualDetecting = false;
    let refreshInterval: number | null = null;

    // --- Event Dispatcher ---
    const dispatch = createEventDispatcher<{
        blockchainSelected: { 
            blockchainId: string; 
            credentials: Credentials; 
            blockHeight: number;
        };
        detectionCompleted: { availableCount: number };
    }>();

    // --- Lifecycle ---
    onMount(() => {
        startDetection();
        // Set up auto-refresh every minute
        refreshInterval = setInterval(() => {
            if (detectionState === 'completed') {
                console.log('BlockchainDetectionStep: Auto-refreshing detection...');
                startDetection();
            }
        }, 60000); // 60 seconds
    });

    onDestroy(() => {
        if (refreshInterval) {
            clearInterval(refreshInterval);
        }
    });

    // --- Reactive Statements ---
    // Filter chains to only show Available, Loading, and some Error types
    $: filteredResults = detectionResults.filter(chain => {
        // Always show Available and Loading chains
        if (chain.status === 'Available' || chain.status === 'Loading') {
            return true;
        }
        
        // Hide NotFound chains
        if (chain.status === 'NotFound') {
            return false;
        }
        
        // Hide HTTP request failed errors
        if (chain.status === 'Error' && chain.error_message?.includes('HTTP request failed')) {
            return false;
        }
        
        // Show other error types
        return true;
    });

    // Show manual selection only if detection is complete but no visible results
    $: if (detectionState === 'completed') {
        showManualSelection = filteredResults.length === 0 && detectionResults.length > 0;
    }

    // --- Detection Logic ---
    async function startDetection() {
        console.log('BlockchainDetectionStep: Starting automatic detection...');
        detectionState = 'loading';
        detectionError = null;
        showManualSelection = false;

        try {
            const result: ParallelDetectionResult = await invoke('detect_all_blockchains');
            
            detectionResults = result.blockchains;
            detectionDuration = result.detection_duration_ms;
            detectionState = 'completed';
            
            console.log(`BlockchainDetectionStep: Detection completed. ${result.total_detected} available blockchains found in ${result.detection_duration_ms}ms`);
            
            // Show manual selection option if no blockchains are available/visible
            // Note: filteredResults is computed after this, so check again in reactive statement
            if (result.total_detected === 0) {
                showManualSelection = true;
            }

            // Dispatch completion event
            dispatch('detectionCompleted', { availableCount: result.total_detected });

        } catch (error) {
            console.error('BlockchainDetectionStep: Detection failed:', error);
            detectionState = 'error';
            detectionError = String(error);
            showManualSelection = true; // Allow manual selection as fallback
        }
    }

    async function selectManualFolder() {
        console.log('BlockchainDetectionStep: Opening manual folder selection...');
        isManualDetecting = true;

        try {
            const selectedPath: string | null = await invoke('select_folder_dialog');
            
            if (selectedPath) {
                console.log('BlockchainDetectionStep: User selected folder:', selectedPath);
                
                // Detect blockchains from the custom path
                const result: ParallelDetectionResult = await invoke('detect_blockchain_from_path', {
                    path: selectedPath
                });

                // Add the results to our existing detection results or replace them
                const newAvailableChains = result.blockchains.filter(b => b.status === 'Available');
                
                if (newAvailableChains.length > 0) {
                    console.log(`BlockchainDetectionStep: Found ${newAvailableChains.length} blockchain(s) in custom folder`);
                    
                    // Merge with existing results, replacing any with same ID
                    for (const newChain of newAvailableChains) {
                        const existingIndex = detectionResults.findIndex(r => r.blockchain_id === newChain.blockchain_id);
                        if (existingIndex >= 0) {
                            detectionResults[existingIndex] = newChain;
                        } else {
                            detectionResults.push(newChain);
                        }
                    }
                    
                    // Sort results by the intended order
                    const order = ["verus", "chips", "vdex", "varrr", "verus-testnet"];
                    detectionResults.sort((a, b) => {
                        const aIndex = order.indexOf(a.blockchain_id);
                        const bIndex = order.indexOf(b.blockchain_id);
                        return (aIndex === -1 ? 999 : aIndex) - (bIndex === -1 ? 999 : bIndex);
                    });
                    
                    detectionState = 'completed';
                    showManualSelection = false;
                    
                    const totalAvailable = detectionResults.filter(r => r.status === 'Available').length;
                    dispatch('detectionCompleted', { availableCount: totalAvailable });
                } else {
                    console.log('BlockchainDetectionStep: No valid blockchains found in selected folder');
                    detectionError = 'No valid blockchain configuration files found in the selected folder.';
                }
            } else {
                console.log('BlockchainDetectionStep: User cancelled folder selection');
            }
        } catch (error) {
            console.error('BlockchainDetectionStep: Manual folder selection failed:', error);
            detectionError = `Failed to read configurations from selected folder: ${String(error)}`;
        } finally {
            isManualDetecting = false;
        }
    }

    async function selectBlockchain(blockchain: BlockchainDetectionResult) {
        // Only allow selection of Available blockchains with complete data
        if (blockchain.status !== 'Available') {
            console.error(`BlockchainDetectionStep: Cannot select blockchain - status is ${blockchain.status}, not Available`);
            return;
        }
        
        if (!blockchain.credentials || blockchain.block_height === null) {
            console.error('BlockchainDetectionStep: Cannot select blockchain - missing credentials or block height');
            return;
        }

        console.log(`BlockchainDetectionStep: Blockchain selected: ${blockchain.blockchain_name}`);
        selectedBlockchainId = blockchain.blockchain_id;

        // Dispatch selection event with credentials (will be saved after successful login)
        dispatch('blockchainSelected', {
            blockchainId: blockchain.blockchain_id,
            credentials: blockchain.credentials,
            blockHeight: blockchain.block_height
        });
    }

    function getStatusColor(status: BlockchainStatus): string {
        switch (status) {
            case 'Available': return 'border-brand-green bg-brand-green/10 hover:bg-brand-green/20';
            case 'Loading': return 'border-blue-500/40 bg-blue-800/20';
            case 'Error': return 'border-red-600/40 bg-red-800/20';
            case 'Timeout': return 'border-yellow-600/40 bg-yellow-800/20';
            case 'ParseError': return 'border-orange-600/40 bg-orange-800/20';
            case 'NotFound': return 'border-dark-border-secondary bg-dark-bg-tertiary/50';
            default: return 'border-dark-border-secondary bg-dark-bg-tertiary';
        }
    }

    function getStatusText(status: BlockchainStatus): string {
        switch (status) {
            case 'Available': return 'Ready to connect';
            case 'Loading': return 'Starting up - please wait...';
            case 'Error': return 'Connection failed';
            case 'Timeout': return 'Daemon not responding';
            case 'ParseError': return 'Configuration error';
            case 'NotFound': return 'Not found';
            default: return 'Unknown status';
        }
    }
</script>

<!-- Blockchain Detection Step Content -->
<div class="step-content-area">
    <h1 class="text-2xl font-semibold text-dark-text-primary mb-2">Connect to Blockchain</h1>
    <p class="text-dark-text-secondary text-normal mb-6">
        {#if detectionState === 'loading'}
            Detecting available blockchain daemons...
        {:else if detectionState === 'completed'}
            {#if filteredResults.length > 0}
                Select a blockchain to connect to:
            {:else}
                No running blockchain daemons detected. You can browse for a custom configuration folder below.
            {/if}
        {:else}
            Detection failed. You can try browsing for a configuration folder manually.
        {/if}
    </p>
    
    <!-- Detection Results / Loading States -->
    <div class="mb-6">
        {#if detectionState === 'loading'}
            <!-- Skeleton Loading States -->
            <div class="space-y-3">
                {#each Array(4) as _, i}
                    <div class="blockchain-item skeleton" transition:fade={{ delay: i * 100, duration: 200 }}>
                        <div class="flex items-center space-x-4">
                            <div class="w-8 h-8 bg-dark-bg-tertiary rounded-full animate-pulse flex-shrink-0"></div>
                            <div class="flex-1 space-y-2">
                                <div class="w-32 h-4 bg-dark-bg-tertiary rounded animate-pulse"></div>
                                <div class="w-24 h-3 bg-dark-bg-tertiary rounded animate-pulse"></div>
                            </div>
                            <div class="w-6 h-6 bg-dark-bg-tertiary rounded animate-pulse flex-shrink-0"></div>
                        </div>
                    </div>
                {/each}
            </div>
        {:else}
            <!-- Detection Results -->
            <div class="space-y-2">
                {#each filteredResults as blockchain, i (blockchain.blockchain_id)}
                    <button
                        type="button"
                        on:click={() => selectBlockchain(blockchain)}
                        disabled={blockchain.status !== 'Available'}
                        class="blockchain-item {getStatusColor(blockchain.status)} {blockchain.blockchain_id === selectedBlockchainId ? 'ring-2 ring-brand-green' : ''} {blockchain.status === 'Available' ? 'cursor-pointer hover:scale-[1.01] transition-all duration-150' : 'cursor-not-allowed opacity-75'}"
                        transition:slide={{ delay: i * 50, duration: 250, easing: quintOut }}
                    >
                        <div class="flex items-center space-x-4">
                            <!-- Status Icon -->
                            <div class="flex-shrink-0 w-8 h-8 flex items-center justify-center">
                                {#if blockchain.status === 'Available'}
                                    <CheckCircle class="w-6 h-6 text-brand-green" />
                                {:else if blockchain.status === 'Loading'}
                                    <Loader class="w-6 h-6 text-blue-400 animate-spin" />
                                {:else if blockchain.status === 'Error'}
                                    <XCircle class="w-6 h-6 text-red-400" />
                                {:else if blockchain.status === 'Timeout'}
                                    <Clock class="w-6 h-6 text-yellow-400" />
                                {:else if blockchain.status === 'ParseError'}
                                    <AlertTriangle class="w-6 h-6 text-orange-400" />
                                {:else if blockchain.status === 'NotFound'}
                                    <Search class="w-6 h-6 text-dark-text-tertiary" />
                                {:else}
                                    <AlertTriangle class="w-6 h-6 text-dark-text-tertiary" />
                                {/if}
                            </div>
                            
                            <!-- Blockchain Info -->
                            <div class="flex-1 text-left min-w-0">
                                <div class="font-semibold text-dark-text-primary text-base truncate">
                                    {blockchain.blockchain_name}
                                </div>
                                <div class="text-sm text-dark-text-secondary flex items-center space-x-2">
                                    <span>
                                        {#if blockchain.status === 'Loading' && blockchain.error_message}
                                            {blockchain.error_message}
                                        {:else}
                                            {getStatusText(blockchain.status)}
                                        {/if}
                                    </span>
                                    {#if blockchain.block_height !== null}
                                        <span class="text-dark-text-tertiary">•</span>
                                        <span class="font-mono text-xs">
                                            Block {blockchain.block_height.toLocaleString()}
                                        </span>
                                    {/if}
                                </div>
                                {#if blockchain.error_message && blockchain.status !== 'Loading' && blockchain.status !== 'NotFound'}
                                    <div class="text-xs text-red-300 mt-1 truncate">
                                        {blockchain.error_message}
                                    </div>
                                {/if}
                            </div>
                            
                            <!-- Selection Indicator -->
                            <div class="flex-shrink-0">
                                {#if blockchain.status === 'Available'}
                                    {#if blockchain.blockchain_id === selectedBlockchainId}
                                        <CheckCircle class="w-5 h-5 text-brand-green" />
                                    {:else}
                                        <ChevronRight class="w-5 h-5 text-dark-text-tertiary" />
                                    {/if}
                                {/if}
                            </div>
                        </div>
                    </button>
                {/each}
            </div>
        {/if}
    </div>

    <!-- Manual Folder Selection -->
    {#if showManualSelection}
        <div class="border border-dark-border-secondary rounded-lg p-6 bg-dark-bg-secondary" transition:slide={{ duration: 300 }}>
            <div class="text-center">
                <div class="flex justify-center mb-4">
                    <FolderOpen class="w-12 h-12 text-dark-text-secondary" />
                </div>
                <h3 class="text-lg font-medium text-dark-text-primary mb-2">Custom Configuration Folder</h3>
                <p class="text-sm text-dark-text-secondary mb-6 max-w-md mx-auto">
                    If your blockchain configuration files are in a custom location, you can browse for the folder containing your .conf files.
                </p>
                <button
                    type="button"
                    on:click={selectManualFolder}
                    disabled={isManualDetecting}
                    class="inline-flex items-center px-6 py-3 border border-dark-border-primary rounded-lg shadow-sm text-sm font-medium text-dark-text-primary bg-dark-bg-tertiary hover:bg-dark-bg-primary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand-green disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-150"
                >
                    {#if isManualDetecting}
                        <Loader class="w-4 h-4 mr-2 animate-spin" />
                        Checking folder...
                    {:else}
                        <FolderOpen class="w-4 h-4 mr-2" />
                        Browse for Folder
                    {/if}
                </button>
            </div>
        </div>
    {/if}

    <!-- Error Display -->
    {#if detectionError}
        <div class="mt-4 bg-red-800/30 border border-red-600/40 rounded-lg p-4" transition:slide={{ duration: 200 }}>
            <div class="flex items-start space-x-3">
                <XCircle class="w-5 h-5 text-red-300 flex-shrink-0 mt-0.5" />
                <p class="text-sm text-red-300 flex-1">{detectionError}</p>
            </div>
        </div>
    {/if}

    <!-- Detection Info -->
    {#if detectionState === 'completed' && detectionDuration > 0}
        <div class="mt-6 text-center">
            <p class="text-xs text-dark-text-tertiary">
                Detection completed in {detectionDuration}ms
                {#if detectionResults.filter(r => r.status === 'Available').length > 0}
                    • {detectionResults.filter(r => r.status === 'Available').length} available
                {/if}
                {#if detectionResults.filter(r => r.status === 'Loading').length > 0}
                    • {detectionResults.filter(r => r.status === 'Loading').length} starting
                {/if}
            </p>
        </div>
    {/if}
</div>

<style>
    .step-content-area {
        width: 100%;
    }

    .blockchain-item {
        @apply w-full p-4 border-2 rounded-lg transition-all duration-200;
        min-height: 72px;
    }

    .blockchain-item.skeleton {
        @apply border-dark-border-secondary bg-dark-bg-tertiary/30;
    }

    .blockchain-item:not(.skeleton):not(:disabled):hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .blockchain-item:not(.skeleton):focus {
        outline: none;
        ring-width: 2px;
        ring-color: rgb(34 197 94 / 0.5);
    }
</style> 