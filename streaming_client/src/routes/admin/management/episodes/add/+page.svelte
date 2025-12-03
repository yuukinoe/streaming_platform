<script lang="ts">
	import SaveAddEpisode from '$lib/components/+SaveAddEpisode.svelte';
	import { addEpisode } from '$lib/data/admin/post.ts';
	// @ts-ignore
	import { Alert } from 'flowbite-svelte';
	import type { PageProps } from './$types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
</script>

{#if hasPermission(data.isLoggedIn.message, 'episodes', 'add')}
	<div>
		<Alert color="yellow" class="m-4">
			<span class="font-medium">Warning!</span>
			Some of the fields might change after you save due to autopopulating.
		</Alert>
		<SaveAddEpisode save_fn={addEpisode} users={data.users} anime={data.anime} />
	</div>
{:else}
	<InsufficientPermissions />
{/if}
