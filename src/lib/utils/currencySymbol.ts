// File: src/lib/utils/currencySymbol.ts
// Description: Utility function to map blockchain IDs to their corresponding currency symbols
// Changes:
// - Created centralized currency symbol mapping for dynamic blockchain support

/**
 * Maps blockchain IDs to their corresponding currency symbols
 * @param blockchainId - The blockchain identifier from BlockchainDetectionStep
 * @returns The currency symbol to display in the UI
 */
export function getCurrencySymbol(blockchainId: string | null): string {
    if (!blockchainId) {
        return 'VRSC'; // Default fallback
    }

    switch (blockchainId.toLowerCase()) {
        case 'varrr':
            return 'VARRR';
        case 'chips':
            return 'CHIPS';
        case 'vdex':
            return 'VDEX';
        case 'verus':
        case 'verus-testnet':
            return 'VRSC';
        default:
            return 'VRSC'; // Default fallback for unknown blockchains
    }
} 