// Updated dark theme colors to be much darker and more neutral (less blue)
// Changed from Tailwind gray palette to custom very dark grays and blacks
// Added standardized accent colors and design tokens for consistency
import defaultTheme from 'tailwindcss/defaultTheme';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'brand-green': {
          DEFAULT: '#419A6A', // Primary green
          hover: '#378A5A',   // Hover for primary green
          disabled: '#9fcfb8', // Disabled primary green
          lightest: '#F0F7F4',        // Will need to re-evaluate for dark theme
          'light-border': '#E0F0E9', // Will need to re-evaluate for dark theme
          'gradient-start': '#5ab88a' // Green for radial gradient
        },
        'dark': {
          'bg-primary': '#0a0a0a',      // Very dark gray, almost black
          'bg-secondary': '#171717',    // Dark gray
          'bg-tertiary': '#262626',     // Medium dark gray
          'text-primary': '#f5f5f5',    // Light gray, neutral
          'text-secondary': '#d4d4d4',  // Medium light gray, neutral
          'text-disabled': '#525252',   // Medium gray, neutral
          'border-primary': '#262626',  // Medium dark gray
          'border-secondary': '#404040' // Lighter dark gray
        },
        // Standardized accent colors for consistency across the app
        'accent-red': {
          DEFAULT: '#ef4444',    // Standard red for errors
          light: '#fca5a5',      // Light red for backgrounds (red-300 equivalent)
          lighter: '#fecaca',    // Lighter red for subtle backgrounds (red-200 equivalent)
          dark: '#dc2626',       // Dark red for borders/emphasis (red-600 equivalent)
          darker: '#b91c1c'      // Darker red for strong emphasis (red-700 equivalent)
        },
        'accent-blue': {
          DEFAULT: '#3b82f6',    // Standard blue for info
          light: '#93c5fd',      // Light blue for backgrounds (blue-300 equivalent)
          lighter: '#dbeafe',    // Lighter blue for subtle backgrounds (blue-100 equivalent)
          dark: '#2563eb',       // Dark blue for borders/emphasis (blue-600 equivalent)
          darker: '#1d4ed8'      // Darker blue for strong emphasis (blue-700 equivalent)
        },
        'accent-orange': {
          DEFAULT: '#f97316',    // Standard orange for warnings
          light: '#fdba74',      // Light orange for backgrounds (orange-300 equivalent)
          lighter: '#fed7aa',    // Lighter orange for subtle backgrounds (orange-200 equivalent)
          dark: '#ea580c',       // Dark orange for borders/emphasis (orange-600 equivalent)
          darker: '#c2410c'      // Darker orange for strong emphasis (orange-700 equivalent)
        },
        'accent-yellow': {
          DEFAULT: '#eab308',    // Standard yellow for warnings
          light: '#fde047',      // Light yellow for backgrounds (yellow-300 equivalent)
          lighter: '#fef3c7',    // Lighter yellow for subtle backgrounds (yellow-100 equivalent)
          dark: '#ca8a04',       // Dark yellow for borders/emphasis (yellow-600 equivalent)
          darker: '#a16207'      // Darker yellow for strong emphasis (yellow-700 equivalent)
        }
      },
      fontFamily: {
        sans: ['Inter', ...defaultTheme.fontFamily.sans],
        mono: ['IBM Plex Mono', ...defaultTheme.fontFamily.mono],
      },
      // Standardized typography scale
      fontSize: {
        'caption': ['0.75rem', { lineHeight: '1rem', fontWeight: '400' }],      // 12px - small helper text
        'caption-medium': ['0.75rem', { lineHeight: '1rem', fontWeight: '500' }], // 12px - labels, emphasized captions
        'body-sm': ['0.875rem', { lineHeight: '1.25rem', fontWeight: '400' }],    // 14px - small body text
        'body-sm-medium': ['0.875rem', { lineHeight: '1.25rem', fontWeight: '500' }], // 14px - emphasized small text
        'body': ['1rem', { lineHeight: '1.5rem', fontWeight: '400' }],             // 16px - default body text
        'body-medium': ['1rem', { lineHeight: '1.5rem', fontWeight: '500' }],      // 16px - emphasized body text
        'heading-sm': ['1.125rem', { lineHeight: '1.75rem', fontWeight: '600' }], // 18px - small headings
        'heading': ['1.25rem', { lineHeight: '1.75rem', fontWeight: '600' }],     // 20px - default headings
        'heading-lg': ['1.5rem', { lineHeight: '2rem', fontWeight: '600' }],      // 24px - large headings
      },
      // Standardized spacing for component consistency
      spacing: {
        'input-y': '0.5rem',   // 8px - standard input vertical padding
        'input-x': '0.75rem',  // 12px - standard input horizontal padding
        'btn-y': '0.5rem',     // 8px - standard button vertical padding
        'btn-x': '0.75rem',    // 12px - standard button horizontal padding
        'card': '1rem',        // 16px - standard card padding
        'modal': '1rem',       // 16px - standard modal body padding
      },
      // Standardized border radius
      borderRadius: {
        'input': '0.375rem',   // 6px - standard input border radius
        'btn': '0.375rem',     // 6px - standard button border radius
        'card': '0.5rem',      // 8px - standard card border radius
        'modal': '0.5rem',     // 8px - standard modal border radius
      },
      // Standardized shadows
      boxShadow: {
        'input': '0 1px 2px 0 rgb(0 0 0 / 0.05)',           // Standard input shadow
        'btn': '0 1px 2px 0 rgb(0 0 0 / 0.05)',             // Standard button shadow
        'card': '0 1px 3px 0 rgb(0 0 0 / 0.1)',             // Standard card shadow
        'modal': '0 25px 50px -12px rgb(0 0 0 / 0.25)',      // Standard modal shadow
        'dropdown': '0 10px 15px -3px rgb(0 0 0 / 0.1)',     // Standard dropdown shadow
      }
    },
  },
  plugins: [],
} 