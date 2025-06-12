// File: src/lib/utils/messageGrouping.ts
// Description: Utility functions for message grouping and date formatting in Discord-style chat
// Features:
// - 30-minute message grouping logic (expects messages in chronological order)
// - Date separator detection
// - Enhanced time formatting: shows date + time for older messages, time only for today/yesterday
// Note: Messages should be sorted chronologically (oldest first) before calling groupMessages()

import type { ChatMessage } from '$lib/types';

export interface MessageGroup {
  id: string;
  sender: string | 'self';
  timestamp: number;
  messages: ChatMessage[];
  showDateSeparator: boolean;
  dateLabel?: string;
}

// Group messages by sender within 30-minute windows
export function groupMessages(messages: ChatMessage[]): MessageGroup[] {
  if (messages.length === 0) return [];

  const groups: MessageGroup[] = [];
  const GROUPING_THRESHOLD = 30 * 60; // 30 minutes in seconds

  for (let i = 0; i < messages.length; i++) {
    const currentMessage = messages[i];
    const previousMessage = i > 0 ? messages[i - 1] : null;
    
    // Check if we should start a new group
    const shouldStartNewGroup = !previousMessage || 
      previousMessage.sender !== currentMessage.sender ||
      (currentMessage.timestamp - previousMessage.timestamp) > GROUPING_THRESHOLD ||
      !isSameDay(previousMessage.timestamp, currentMessage.timestamp);

    if (shouldStartNewGroup) {
      // Check if we need a date separator
      const showDateSeparator = !previousMessage || 
        !isSameDay(previousMessage.timestamp, currentMessage.timestamp);
      
      const dateLabel = showDateSeparator ? formatDateSeparator(currentMessage.timestamp) : undefined;

      groups.push({
        id: `group-${currentMessage.timestamp}-${currentMessage.sender}`,
        sender: currentMessage.sender,
        timestamp: currentMessage.timestamp,
        messages: [currentMessage],
        showDateSeparator,
        dateLabel
      });
    } else {
      // Add to existing group
      const lastGroup = groups[groups.length - 1];
      lastGroup.messages.push(currentMessage);
    }
  }

  return groups;
}

// Check if two timestamps are on the same day
export function isSameDay(timestamp1: number, timestamp2: number): boolean {
  const date1 = new Date(timestamp1 * 1000);
  const date2 = new Date(timestamp2 * 1000);
  
  return date1.getFullYear() === date2.getFullYear() &&
         date1.getMonth() === date2.getMonth() &&
         date1.getDate() === date2.getDate();
}

// Format date separator labels
export function formatDateSeparator(timestamp: number): string {
  const messageDate = new Date(timestamp * 1000);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (isSameDay(timestamp, Math.floor(today.getTime() / 1000))) {
    return 'Today';
  } else if (isSameDay(timestamp, Math.floor(yesterday.getTime() / 1000))) {
    return 'Yesterday';
  } else {
    // Format as "January 15, 2024"
    return messageDate.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  }
}

// Format time for message display (24h format) - includes date for older messages
export function formatMessageTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  // Check if it's today or yesterday
  if (isSameDay(timestamp, Math.floor(today.getTime() / 1000))) {
    // Today - show only time
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit'
    });
  } else if (isSameDay(timestamp, Math.floor(yesterday.getTime() / 1000))) {
    // Yesterday - show only time  
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit'
    });
  } else {
    // Older - show date and time in MM/DD/YYYY, HH:MM format
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    const year = date.getFullYear();
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    
    return `${month}/${day}/${year}, ${hours}:${minutes}`;
  }
}

// Format detailed timestamp for hover
export function formatDetailedTimestamp(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('en-US', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  });
} 