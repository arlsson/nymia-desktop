// File: src/lib/utils/timeFormatter.ts
// Description: Utility functions to format relative time for chat messages.
// Changes:
// - Updated formatRelativeTimeFromTimestamp to work with Unix timestamps in seconds (UTC).
// - Deprecated formatRelativeTimeFromConfirmations (legacy function for old messages).
// - Timestamps are stored in UTC but displayed in user's local timezone automatically.

/**
 * Formats a relative time string based on the number of confirmations.
 * Assumes 1 confirmation ≈ 1 minute.
 * 
 * @deprecated This function is used for legacy messages without timestamps.
 * New messages use timestamp-based ordering via formatRelativeTimeFromTimestamp().
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

/**
 * Formats a relative time string based on a Unix timestamp in seconds.
 * The Date constructor automatically converts to user's local timezone for display.
 * 
 * @param timestamp - The Unix timestamp in seconds (UTC) since epoch.
 * @returns A formatted relative time string (e.g., "Just now", "5m ago", "2h ago", "Yesterday", "MM/DD/YYYY").
 */
export function formatRelativeTimeFromTimestamp(timestamp: number | null | undefined): string {
    if (!timestamp) {
        return ''; // Return empty if timestamp is invalid
    }

    const now = Math.floor(Date.now() / 1000); // Convert current time to seconds
    const secondsAgo = now - timestamp;

    if (secondsAgo < 60) {
        return "Just now";
    }
    
    const minutesAgo = Math.round(secondsAgo / 60);
    if (minutesAgo < 60) {
        return `${minutesAgo}m ago`;
    }
    
    const hoursAgo = Math.round(minutesAgo / 60);
    if (hoursAgo < 24) {
        return `${hoursAgo}h ago`;
    }

    const daysAgo = Math.round(hoursAgo / 24);
    if (daysAgo === 1) {
        return "Yesterday";
    }
    
    // After Yesterday, show the date
    const date = new Date(timestamp * 1000); // Convert Unix seconds to milliseconds for Date constructor
    const year = date.getFullYear();
    const currentYear = new Date().getFullYear();
    
    // Simple date format MM/DD or MM/DD/YYYY if not current year
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');

    if (year === currentYear) {
        return `${month}/${day}`;
    } else {
        return `${month}/${day}/${year}`;
    }
} 