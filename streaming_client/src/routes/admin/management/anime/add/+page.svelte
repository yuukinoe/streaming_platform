<script lang="ts">
	// @ts-ignore
	import { Label, Input, Button, Toast } from 'flowbite-svelte';
	import { CloseCircleSolid, FireOutline } from 'flowbite-svelte-icons';
	import { enhance } from '$app/forms';
	import { addAnime } from '$lib/data/admin/post.ts';
	import { redirect } from '@sveltejs/kit';
	import type { Anime } from '$lib/types.js';
	import { goto } from '$app/navigation';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	import type { PageProps } from './$types.js';
	let { data }: PageProps = $props();
	let success = $state(false);
	let failed = $state(true);
	let message_error = $state('');
	const handleSubmit = async () => {
		let malInput = document.getElementById('mal-input') as HTMLInputElement;
		try {
			let anime_data_response = await addAnime({ message: malInput.value });
			malInput.value = '';
			let anime_data: Anime = anime_data_response.message as Anime;
			success = true;
			await goto(`/admin/management/anime/${anime_data.slug}`);
		} catch (err) {
			// alert('Error: ' + anime_data_response.message + err);
			failed = true;
			message_error = err instanceof Error ? err.message : 'Unknown error';

			// console.log(message_error);
		}
	};
</script>

{#if hasPermission(data.isLoggedIn.message, 'anime', 'add')}
	<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> Add anime </span>
			</div>
			<div>
				<span class="text-2xl"> Add anime to the database </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> Just put here link from myanimelist and then u can edit it. </span>
			</div>
		</div>
		<form>
			<div class="mb-6 flex flex-col">
				<Label for="mal-input" class="mb-2 block">MyAnimeList anime url</Label>
				<div class="flex">
					<Input
						onkeydown={(e: KeyboardEvent) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								handleSubmit();
							}
						}}
						autocomplete="off"
						id="mal-input"
						placeholder="https://myanimelist.net/anime/..."
					/>
					<Button onclick={handleSubmit} type="submit" class="ml-2">Add</Button>
				</div>
			</div>
		</form>
		<div class="flex flex-col fixed right-5 bottom-5 gap-3">
			{#if success}
				<Toast>
					{#snippet icon()}
						<FireOutline class="text-primary-500 bg-primary-100 dark:bg-primary-800 dark:text-primary-200 h-6 w-6" />
					{/snippet}
					Anime added successfully.
				</Toast>
			{/if}
			{#if failed}
				<Toast
					onclose={() => {
						failed = false;
					}}
					color="red"
				>
					{#snippet icon()}
						<CloseCircleSolid class="text-red-500 bg-red-100 dark:bg-red-800 dark:text-red-200 h-6 w-6" />
					{/snippet}
					{message_error}
				</Toast>
			{/if}
		</div>
	</div>
{:else}
	<InsufficientPermissions />
{/if}
