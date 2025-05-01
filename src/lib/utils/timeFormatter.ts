// File: src/lib/utils/timeFormatter.ts
// Description: Utility function to format relative time based on block confirmations.

/**
 * Formats a relative time string based on the number of confirmations.
 * Assumes 1 confirmation ≈ 1 minute.
 * 
 * @param confirmations - The number of confirmations for the transaction.
 * @returns A formatted relative time string (e.g., "Just now", "5m ago", "2h ago", "1d ago").
 */
export function formatRelativeTimeFromConfirmations(confirmations: number | null | undefined): string {
    if (confirmations === null || typeof confirmations === 'undefined' || confirmations < 0) {
        return ''; // Return empty or a placeholder if confirmations are invalid
    }

    const minutesAgo = confirmations; // Approximately 1 confirmation per minute

    if (minutesAgo <= 1) {
        return "Just now";
    }
    if (minutesAgo < 60) {
        return `${minutesAgo}m ago`;
    }
    
    const hoursAgo = Math.floor(minutesAgo / 60);
    if (hoursAgo < 24) {
        return `${hoursAgo}h ago`;
    }

    const daysAgo = Math.floor(hoursAgo / 24);
    // As per requirement, switch to date after 2 days.
    // PROBLEM: We don't have the absolute date from confirmations alone.
    // For now, we will continue showing days ago until absolute timestamps are available.
    // if (daysAgo <= 2) {
    //     return `${daysAgo}d ago`;
    // }
    // // TODO: Switch to absolute date formatting when available
    return `${daysAgo}d ago`;
} 