<script lang="ts">
// File: src/routes/+page.svelte
// Description: Main application page. Initially displays the authentication form.
// Changes: Removed test classes, using standard styling.
	import AuthForm from '$lib/components/AuthForm.svelte';
	// TODO: Add state management to show AuthForm conditionally
	// based on whether the user is authenticated.
</script>

<svelte:head>
	<title>Chat-dApp - Connect</title>
</svelte:head>

<div class="app-container">
	<!-- For now, always show the AuthForm -->
	<!-- Later, this will conditionally render based on auth status -->
	<AuthForm />
</div>
