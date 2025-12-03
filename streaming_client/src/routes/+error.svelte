<script>
	export let error;
	export let status;
	import { page } from '$app/state';
	let _ = page.error;
	let __ = error;
	import Error404 from '$lib/data/client/components/404.svelte';
	import Error500 from '$lib/data/client/components/500.svelte';
</script>

{#if status === 404}
	<Error404 />
{:else if status === 500}
	<Error500 />
{:else}
	<Error500 />
{/if}
