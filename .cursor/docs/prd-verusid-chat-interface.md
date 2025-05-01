# PRD: VerusID Chat Interface

## Overview
This document outlines the product requirements for the VerusID Chat Interface, which will replace the current LoggedInView.svelte component. The interface will provide a WhatsApp-style messaging experience for users who have completed the onboarding process and logged in with their VerusID.

## Goals
- Create an intuitive messaging interface for VerusID users
- Enable secure communication between VerusID holders
- Provide essential messaging features while maintaining a clean, user-friendly design
- Focus on frontend implementation (backend functionality to be addressed separately)

## User Experience Flow
1. User completes onboarding process through OnboardingFlow.svelte
2. Upon successful login, user is presented with the Chat Interface
3. User can view existing conversations, start new ones, and manage settings

## UI Components

### Top Bar
- Display logged-in VerusID name
- Show private balance
- Include logout button
- Provide settings button
- Include refresh button to fetch new messages

### Left Sidebar (Conversations List)
- Display list of VerusID names with which the user has ongoing conversations
- Include visual indicator for conversations with unread messages
- Provide a button at the top to initiate a new conversation
- Each conversation item should display the VerusID name 

### Right Panel (Chat View)
- Show conversation history with selected contact
- Provide message input area with send button
- Include option to attach additional funds to a message
- Display message delivery status
- Empty state when no conversation is selected

## Detailed Requirements

### General Interface
- Use light mode color scheme consistent with OnboardingFlow.svelte
- Primary brand color: #419A6A
- Maintain the same design language as the onboarding process

### Message System
- Support text-only messages (for initial implementation)
- Each message costs 0.0001 vrsctest to send
- Messages are completely private and encrypted
- Display message delivery status (sent/delivered)
- No read receipts (technically not possible)

### Funds Transfer
- Allow users to attach additional vrsctest when sending messages
- Display clear UI for specifying amount to send
- Show confirmation before sending funds

### Conversation Management
- Support one-to-one conversations only
- New conversation creation requires entering a valid VerusID
- Provide UI feedback when insufficient funds for sending a message

### Technical Considerations
- Messages are blockchain-based and may have delay (block time)
- Need to periodically check for new messages (refresh functionality)
- Balance check before allowing message sending

## Visual Mockup Description

### Overall Layout
Two-panel layout with fixed top bar:
- Top bar spans the full width (height: ~60px)
- Left sidebar (~30% of width)
- Right panel (~70% of width)

### Color Scheme
- Primary brand color: #419A6A (green)
- Background colors:
  - Main background: white/light gray (#f8f9fa)
  - Top bar: #419A6A
  - Left sidebar: white with light borders
  - Message bubbles: sender (#419A6A with opacity), receiver (light gray)

### Top Bar Details
- VerusID name displayed prominently on the left
- Private balance shown in the center
- Buttons aligned to the right (refresh, settings, logout)
- All icons/text in white or light color for contrast against #419A6A

### Left Sidebar Details
- "New Chat" button at top (#419A6A background, white text)
- Each conversation item:
  - VerusID name in bold
  - Light border separating conversations
  - Unread indicator: small dot in #419A6A color
  - Selected conversation: light background highlight

### Right Panel Details
- Empty state: centered prompt to select a conversation
- Conversation view:
  - VerusID name of contact at the top
  - Messages displayed in bubbles (sender right, receiver left)
  - Message input at bottom with send button
  - Option to attach funds (small icon or "+" button)
  - Fund attachment UI: slide-up panel with amount input

## User Feedback & Error States

### Error Handling
- Insufficient funds error: Show non-blocking notification with clear message about the 0.0001 vrsctest cost
- Failed message delivery: Visual indicator on message with retry option
- Invalid VerusID input: Immediate validation feedback in the new conversation form
- Network/connection issues: Status indicator in top bar, with reconnect option

### Loading States
- Initial conversation loading: Skeleton UI or subtle loading spinner
- Message sending: Visual indicator on message (pending state)
- Balance updating: Subtle animation when balance changes

### User Feedback
- Successful message send: Visual confirmation (checkmark or status text)
- Funds transfer confirmation: Clear success message after funds sent
- New message indicator: Visual cue in the sidebar

## Implementation Notes

### Component Structure
- ChatInterface.svelte (main container component)
- TopBar.svelte (header with user info and controls)
- ConversationsList.svelte (left sidebar component)
- ConversationView.svelte (right panel component)
- MessageInput.svelte (input area with funds attachment option)

### State Management
- Track active conversations
- Maintain message history for each conversation
- Track message delivery status
- Monitor user balance for message sending capability

## Questions and Considerations

### Open Items for Resolution
1. Specific functionality of the settings button
2. Implementation details for the refresh mechanism
3. How to validate VerusID entries when starting a new conversation
4. How to handle insufficient funds scenarios
5. Storage mechanism for conversation history
6. Exact visual design of the fund attachment interface

## Future Considerations

### Potential Features for Later Phases
1. Group conversation support
2. File/media sharing capabilities
3. Message search functionality
4. Message history loading (pagination)
5. Contact management (favorites, blocking)
6. Desktop notifications
7. More extensive profile information display
8. Enhanced funds transfer features (recurring payments, request funds)

### Performance Considerations
- Optimize for slower blockchain-based messaging
- Consider local caching strategies for message history
- Implement efficient balance checking to avoid unnecessary network calls

### Security Notes
- Ensure all message content remains encrypted end-to-end
- Consider secure storage of conversation history
- Implement secure handling of private keys for message decryption

## Design Inspirations
- WhatsApp web interface
- OnboardingFlow.svelte and CredentialsStep.svelte for UI styling consistency 