<script lang="ts">
// Component: src/lib/components/Avatar.svelte
// Description: Generates colorful gradient avatars based on VerusID hash
// Features:
// - Hash-based gradient generation for consistent avatars
// - Multiple size variants (24px, 32px, 40px)
// - Hover effects with subtle zoom/glow
// - Geometric patterns and vibrant gradients

export let userId: string; // VerusID or formatted name
export let size: 'small' | 'medium' | 'large' = 'medium'; // 24px, 32px, 40px
export let showHover: boolean = true;

// Size mappings
const sizeMap = {
  small: 'w-6 h-6', // 24px
  medium: 'w-8 h-8', // 32px  
  large: 'w-10 h-10' // 40px
};

// Generate consistent hash from userId
function simpleHash(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash);
}

// Generate gradient colors based on hash
function generateGradient(userId: string): { from: string; to: string; pattern: string } {
  const hash = simpleHash(userId);
  
  // Color palette for gradients
  const colors = [
    '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FFEAA7',
    '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE', '#85C1E9',
    '#F8C471', '#82E0AA', '#F1948A', '#85C1E9', '#D7BDE2',
    '#A3E4D7', '#F9E79F', '#D5A6BD', '#AED6F1', '#A9DFBF'
  ];
  
  const fromIndex = hash % colors.length;
  const toIndex = (hash + 7) % colors.length; // Offset for variety
  
  // Pattern types
  const patterns = [
    'bg-gradient-to-br', 'bg-gradient-to-tr', 
    'bg-gradient-to-bl', 'bg-gradient-to-tl',
    'bg-gradient-to-r', 'bg-gradient-to-l'
  ];
  
  const patternIndex = Math.floor(hash / colors.length) % patterns.length;
  
  return {
    from: colors[fromIndex],
    to: colors[toIndex],
    pattern: patterns[patternIndex]
  };
}

// Get initials from userId
function getInitials(userId: string): string {
  // Remove @ symbol if present and get first 2 chars
  const cleanId = userId.replace('@', '');
  if (cleanId.length === 0) return '?';
  if (cleanId.length === 1) return cleanId.toUpperCase();
  return cleanId.substring(0, 2).toUpperCase();
}

$: gradient = generateGradient(userId);
$: initials = getInitials(userId);
$: sizeClass = sizeMap[size];
</script>

<div 
  class="avatar-container relative flex-shrink-0 {sizeClass} rounded-full overflow-hidden {showHover ? 'hover-enabled' : ''}"
  style="background: linear-gradient(135deg, {gradient.from} 0%, {gradient.to} 100%);"
  title={userId}
>
  <!-- Initials overlay -->
  <div class="absolute inset-0 flex items-center justify-center text-white font-semibold select-none">
    <span class="initials-text {size === 'small' ? 'text-xs' : size === 'medium' ? 'text-sm' : 'text-base'} drop-shadow-sm">
      {initials}
    </span>
  </div>
  
  <!-- Subtle geometric pattern overlay -->
  <div class="absolute inset-0 opacity-20">
    <svg width="100%" height="100%" viewBox="0 0 40 40">
      <pattern id="dots-{userId.replace('@', '').replace(/[^a-zA-Z0-9]/g, '')}" patternUnits="userSpaceOnUse" width="8" height="8">
        <circle cx="4" cy="4" r="1" fill="white" opacity="0.3"/>
      </pattern>
      <rect width="100%" height="100%" fill="url(#dots-{userId.replace('@', '').replace(/[^a-zA-Z0-9]/g, '')})"/>
    </svg>
  </div>
</div>

<style>
  .avatar-container {
    transition: all 0.2s ease;
  }
  
  .hover-enabled:hover {
    transform: scale(1.05);
    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1), 
                0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .initials-text {
    text-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    font-weight: 600;
  }
</style> 