
<script lang="ts">
// Component: src/lib/components/AuthForm.svelte
// Description: Handles user input for Verus daemon RPC credentials and initiates the connection process.
// Changes:
// - Calls the actual `connect_verus_daemon` Tauri command.
// - Handles errors returned from the backend command.
// - Changed import from '@tauri-apps/api/tauri' to '@tauri-apps/api/core'.
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let rpcUser = '';
  let rpcPassword = '';
  let connectionStatus: 'idle' | 'connecting' | 'connected' | 'error' = 'idle';
  let blockHeight: number | null = null;
  let errorMessage: string | null = null;

  async function connect() {
    connectionStatus = 'connecting';
    errorMessage = null;
    blockHeight = null;

    try {
      console.log(`Attempting connect with user: ${rpcUser}`); // Basic frontend log
      const result = await invoke<number>('connect_verus_daemon', {
        rpcUser: rpcUser, // Pass camelCase keys matching Rust command arguments
        rpcPass: rpcPassword,
      });

      blockHeight = result;
      connectionStatus = 'connected';
      console.log('Connection successful, block height:', blockHeight);
      // TODO: Store credentials securely on successful connection (Requirement 2.3)
    } catch (err: any) {
      connectionStatus = 'error';
      // Error from Rust backend is likely a string within the error object
      // Try to extract a more specific message if available
      let backendErrorMessage: string | null = null;
      if (typeof err === 'string') {
        backendErrorMessage = err;
      } else if (typeof err === 'object' && err !== null && err.message) {
        backendErrorMessage = err.message; // Might contain the CommandError string
      }

      errorMessage = backendErrorMessage || 'Failed to connect. Check logs or daemon status.';
      console.error('Connection failed:', err);
    }
  }

  // TODO: Implement returning user flow (Requirement: Returning User)
  // TODO: Implement periodic connection check (Requirement 2.4)
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
  <div class="w-full max-w-md p-8 space-y-6 bg-white rounded-lg shadow-md">
    <h2 class="text-2xl font-semibold text-center text-gray-700">Connect to Verus Daemon</h2>

    <form on:submit|preventDefault={connect} class="space-y-4">
      <div>
        <label for="rpcUser" class="block text-sm font-medium text-gray-600">RPC Username</label>
        <input
          type="text"
          id="rpcUser"
          bind:value={rpcUser}
          required
          class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-teal-500 focus:border-teal-500 sm:text-sm"
          placeholder="Your RPC username"
        />
      </div>
      <div>
        <label for="rpcPassword" class="block text-sm font-medium text-gray-600">RPC Password</label>
        <input
          type="password"
          id="rpcPassword"
          bind:value={rpcPassword}
          required
          class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-teal-500 focus:border-teal-500 sm:text-sm"
          placeholder="Your RPC password"
        />
      </div>

      <button
        type="submit"
        disabled={connectionStatus === 'connecting'}
        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-teal-600 hover:bg-teal-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if connectionStatus === 'connecting'}
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Connecting...
        {:else}
          Connect
        {/if}
      </button>
    </form>

    {#if connectionStatus === 'connected' && blockHeight !== null}
      <div class="mt-4 p-4 bg-green-100 border border-green-300 rounded-md text-center">
        <p class="text-sm font-medium text-green-800">
          Successfully connected!
        </p>
        <p class="text-lg font-semibold text-green-900">
          Current Block Height: {blockHeight}
        </p>
      </div>
    {/if}

    {#if connectionStatus === 'error' && errorMessage}
      <div class="mt-4 p-4 bg-red-100 border border-red-300 rounded-md text-center">
        <p class="text-sm font-medium text-red-800">Connection Failed</p>
        <p class="text-xs text-red-700">{errorMessage}</p>
      </div>
    {/if}
  </div>
</div> 