<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { initializeAuth } from '$lib/stores/auth';
	import favicon from '$lib/assets/favicon.svg';

	onMount(() => {
		initializeAuth();
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<main><slot></slot></main>

<style>
	:global(body) {
		margin: 0;
		font-family: 'Inter', sans-serif;
		background-color: #1a1e27;
		min-height: 100vh;
		color: #e2e8f0;
	}

	main {
		min-height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 1rem;
		box-sizing: border-box;
	}
</style>
