<script lang="ts">
// Component: src/lib/components/LoggedInView.svelte
// Description: Placeholder view shown after successful login.
// Displays selected VerusID details and provides a logout option.

  import { createEventDispatcher } from 'svelte';

  export let identityName: string | null = null;
  export let identityIAddress: string | null = null;
  export let identityPrivateAddress: string | null = null;
  export let blockHeight: number | null = null;

  const dispatch = createEventDispatcher<{ logout: void }>();

  function handleLogout() {
    dispatch('logout');
  }
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-gray-50 p-6">
    <div class="w-full max-w-lg p-8 space-y-6 bg-white rounded-lg shadow-md text-center">
        <h1 class="text-3xl font-bold text-green-700 mb-4">Successfully Logged In!</h1>

        {#if identityName}
            <div class="mb-3">
                <p class="text-sm text-gray-500">Logged in as:</p>
                <p class="text-lg font-semibold text-gray-800">{identityName}</p>
            </div>
        {/if}

        {#if identityIAddress}
            <div class="mb-3">
                <p class="text-sm text-gray-500">VerusID i-address:</p>
                <p class="text-md font-mono text-gray-700 break-all">{identityIAddress}</p>
            </div>
        {/if}

        {#if identityPrivateAddress}
            <div class="mb-3">
                <p class="text-sm text-gray-500">Private Address:</p>
                <p class="text-md font-mono text-gray-700 break-all">{identityPrivateAddress}</p>
            </div>
        {:else}
         <div class="mb-3">
                <p class="text-sm text-orange-600">(Private address not available)</p>
            </div>
        {/if}

        {#if blockHeight !== null}
            <div class="mb-6">
                <p class="text-sm text-gray-500">Current Block Height:</p>
                <p class="text-lg font-semibold text-teal-600">{blockHeight}</p>
            </div>
        {/if}

        <button
            on:click={handleLogout}
            class="w-full px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition duration-150 ease-in-out"
        >
            Log Out
        </button>
    </div>
</div> 