// File: src/lib/utils/messageLimit.ts
// Description: Utility function to calculate dynamic message length limits based on memo field constraints.
// Changes:
// - Created calculateMaxMessageLength function that accounts for memo format overhead
// - Uses 512 byte memo limit minus fixed separators, timestamp, signature, safety margin, and dynamic sender name length

/**
 * Calculate the maximum message length based on memo field constraints and sender identity name.
 * 
 * Memo format: {message_text}//f//{sender_identity}//t//{unix_timestamp}//{signature}
 * Total memo limit: 512 bytes
 * 
 * @param verusIdName - The sender's VerusID name (e.g., "username@" or "user.parent@")
 * @returns Maximum allowed message length in characters
 */
export function calculateMaxMessageLength(verusIdName: string): number {
  const MEMO_LIMIT = 512;
  const SEPARATORS = 12; // "//f//" (5) + "//t//" (5) + "//" (2)
  const TIMESTAMP = 10; // Unix timestamp length
  const SIGNATURE = 100; // Signature length (with safety buffer)
  const SAFETY_MARGIN = 5; // Additional safety buffer
  const FALLBACK_LIMIT = 350; // Fallback if verusIdName is invalid
  
  // Validate input
  if (!verusIdName || verusIdName.length === 0) {
    console.warn('calculateMaxMessageLength: Invalid verusIdName, using fallback limit');
    return FALLBACK_LIMIT;
  }
  
  const calculatedLimit = MEMO_LIMIT - SEPARATORS - TIMESTAMP - SIGNATURE - SAFETY_MARGIN - verusIdName.length;
  
  // Log the calculation for debugging
  console.debug(`Message limit calculation: ${MEMO_LIMIT} - ${SEPARATORS} - ${TIMESTAMP} - ${SIGNATURE} - ${SAFETY_MARGIN} - ${verusIdName.length} = ${calculatedLimit}`);
  
  return calculatedLimit;
} 