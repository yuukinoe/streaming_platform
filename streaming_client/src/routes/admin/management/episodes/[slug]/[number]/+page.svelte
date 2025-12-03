<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import { type Episodes } from '$lib/types.ts';
	import SaveAddEpisode from '$lib/components/+SaveAddEpisode.svelte';
	import { patchEpisodeData } from '$lib/data/admin/patch.ts';
	import { getEpisodeBySlugAndNumber } from '$lib/data/admin/get.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
	let episodeData = $state<Episodes | undefined>(undefined);
	let isLoading = $state(true);

	onMount(async () => {
		try {
			let episode_number = Number(data.number);
			let episode_slug = data.slug;

			// First try to find in existing data (if it exists)
			if (data.episodes && Array.isArray(data.episodes)) {
				episodeData = data.episodes.find((episode) => episode.anime.slug === episode_slug && episode.episode === episode_number) as Episodes;
			}

			// If not found, fetch from API
			if (!episodeData) {
				const fetchedEpisode = await getEpisodeBySlugAndNumber(episode_slug, episode_number.toString());
				episodeData = fetchedEpisode as Episodes;
			}
		} catch (error) {
			console.error('Error loading episode data:', error);
		} finally {
			isLoading = false;
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'episodes', 'edit')}
	{#if isLoading}
		<div class="w-full h-full flex items-center justify-center">
			<div>Loading episode data...</div>
		</div>
	{:else if episodeData}
		<SaveAddEpisode {episodeData} save_fn={patchEpisodeData} users={data.users} anime={data.anime} />
	{:else}
		<div class="w-full h-full flex items-center justify-center">
			<div class="text-red-500">Failed to load episode data</div>
		</div>
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
