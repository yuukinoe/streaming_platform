<script lang="ts">
	import { patchAnimeData } from '$lib/data/admin/patch.ts';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import type { Anime, TagOrStudio, MultiSelectType } from '$lib/types.js';
	// @ts-ignore
	import { FloatingLabelInput, Label, Helper, Button, ButtonGroup, Input, MultiSelect, Badge, Select, Textarea, Toggle, Datepicker } from 'flowbite-svelte';
	import { PlusOutline, MinusOutline, VideoCameraOutline } from 'flowbite-svelte-icons';
	// import { PageLoad } from '$routes/admin/management/anime/%5Bslug%5D/%2Bpage.js';
	// @ts-ignore
	import { CalendarIcon, Upload, X, Plus } from '@lucide/svelte';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { deleteAnime } from '$lib/data/admin/delete.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import { getAnimeBySlug } from '$lib/data/admin/get.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';

	const formData = new FormData();

	let { data }: PageProps = $props();
	let animeData = $state({}) as Anime;
	let tagsData: TagOrStudio[] = $state([]);
	let studiosData: TagOrStudio[] = $state([]);
	let tagsDataMultiSelect: MultiSelectType[] = $state([]);
	let studiosDataMultiSelect: MultiSelectType[] = $state([]);
	let imagePreview = 'not_found.png';
	let uploadedImageFile: File | null = null;
	let isImageUploaded = $state(false);
	let selectedTags: string[] = $state([]);
	let selectedStudios: string[] = $state([]);
	let colors = ['indigo', 'blue', 'green', 'yellow', 'red', 'purple', 'pink'];
	let date = $state(new Date());
	let anime_types = [
		{ value: 'TV', name: 'TV' },
		{ value: 'Movie', name: 'Movie' },
		{ value: 'OVA', name: 'OVA' },
		{ value: 'ONA', name: 'ONA' },
		{ value: 'Special', name: 'Special' },
		{ value: 'Music', name: 'Music' }
	];

	let anime_status = [
		{ value: 'Not yet aired', name: 'Not yet aired' },
		{ value: 'Currently Airing', name: 'Currently Airing' },
		{ value: 'Finished Airing', name: 'Finished Airing' }
	];

	let anime_sources = [
		{ value: 'Unknown', name: 'Unknown' },
		{ value: 'Anime', name: 'Anime' },
		{ value: 'Original', name: 'Original' },
		{ value: 'Manga', name: 'Manga' },
		{ value: 'Web manga', name: 'Web manga' },
		{ value: 'Webtoon', name: 'Webtoon' },
		{ value: 'Web novel', name: 'Web novel' },
		{ value: 'Light novel', name: 'Light Novel' },
		{ value: 'Visual novel', name: 'Visual Novel' },
		{ value: 'Game', name: 'Game' },
		// 4-koma manga
		{ value: '4-koma manga', name: '4-koma manga' },
		{ value: 'Novel', name: 'Novel' },
		{ value: 'Picture book', name: 'Picture book' },
		// Manhwa
		{ value: 'Manhwa', name: 'Manhwa' },
		{ value: 'Manhua', name: 'Manhua' },
		{ value: 'Other', name: 'Other' }
	];
	function handleImageUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			uploadedImageFile = file;

			if (formData.has('image')) {
				formData.delete('image');
			}
			formData.append('image', file);

			// console.log('Image uploaded:', {
			// 	name: file.name,
			// 	size: file.size,
			// 	type: file.type
			// });

			const reader = new FileReader();
			reader.onload = (e) => {
				imagePreview = e.target?.result as string;
				animeData.image = imagePreview;
				isImageUploaded = true;
			};
			reader.readAsDataURL(file);
		}
	}
	// import { }
	// Anime Type

	function filterSelectedItems(items: string[], type: 'tags' | 'studio') {
		if (type === 'studio') {
			return studiosData.filter((studio) => items.includes(studio.id.id.String));
		} else if (type === 'tags') {
			return tagsData.filter((tag) => items.includes(tag.id.id.String));
		}
		return [];
	}
	let loading = $state(true);
	onMount(async () => {
		animeData = data.anime.find((anime) => anime.slug === data.slug) as Anime;

		if (!animeData) {
			animeData = (await getAnimeBySlug(data.slug)) as Anime;
		}

		tagsData = data.tags as TagOrStudio[];
		studiosData = data.studios as TagOrStudio[];
		for (const tag of tagsData as TagOrStudio[]) {
			tagsDataMultiSelect.push({
				value: tag.id.id.String,
				name: tag.name,
				color: colors[Math.floor(Math.random() * colors.length)]
			});
		}
		for (const studio of studiosData as TagOrStudio[]) {
			studiosDataMultiSelect.push({
				value: studio.id.id.String,
				name: studio.name,
				color: colors[Math.floor(Math.random() * colors.length)]
			});
		}
		for (const selected_tag of animeData.tags) {
			selectedTags.push(selected_tag.id.id.String);
		}
		for (const selected_studio of animeData.studio) {
			selectedStudios.push(selected_studio.id.id.String);
		}

		date = new Date(animeData.aired);
		loading = false;
		// console.log('Selected Tags:', selectedTags);
		// console.log(animeData);
		// console.log(tagsData);
		// console.log(studiosData);
	});

	// function debugFormData() {
	// 	console.log('=== FormData Debug (from main page) ===');
	// 	if (formData) {
	// 		console.log('FormData exists');
	// 		for (let [key, value] of formData.entries()) {
	// 			if (value instanceof File) {
	// 				console.log(`${key}:`, {
	// 					name: value.name,
	// 					size: value.size,
	// 					type: value.type,
	// 					lastModified: value.lastModified
	// 				});
	// 			} else {
	// 				console.log(`${key}:`, value);
	// 			}
	// 		}
	// 	} else {
	// 		console.log('FormData is null/undefined');
	// 	}
	// 	console.log('animeData:', animeData);
	// 	console.log('uploadedImageFile:', uploadedImageFile);
	// 	console.log('=== End FormData Debug ===');
	// }
</script>

{#if hasPermission(data.isLoggedIn.message, 'anime', 'edit')}
	{#if loading}
		<Loading />
	{:else}
		<div>
			<div class="flex gap-10 p-4">
				<div class="lg:col-span-1 p-4 rounded bg-[#1e2939] max-w-[220px] max-h-[390px]">
					<h2 class="font-bold mb-2 text-white">Cover Image</h2>
					<div class="flex flex-col items-center gap-4">
						<div class="relative w-48 h-64 rounded overflow-hidden">
							<img src={isImageUploaded ? animeData.image : `/${animeData.image}`} alt="Anime cover" class="w-full h-full object-cover" />
						</div>
						<label for="image-upload" class="cursor-pointer inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500">
							<Upload class="w-4 h-4 mr-2" />
							Upload Image
						</label>
						<input id="image-upload" type="file" accept="image/*" class="hidden" onchange={handleImageUpload} />
					</div>
				</div>
				<div class="flex flex-col gap-3 [&>div]:[&>input]:max-h-[44px] w-full">
					<FloatingLabelInput value={animeData.mal} clearable variant="filled" id="mal" name="mal" onchange={(e: Event) => (animeData.mal = (e.target as HTMLInputElement).value)} type="text">MyAnimeList</FloatingLabelInput>
					<FloatingLabelInput value={animeData.title} clearable variant="filled" id="title" name="title" type="text" onchange={(e: Event) => (animeData.title = (e.target as HTMLInputElement).value)} class="bg-[#101828] rounded-lg">Title</FloatingLabelInput>
					<FloatingLabelInput value={animeData.alternative_title} clearable variant="filled" id="alternative_title" name="alternative_title" onchange={(e: Event) => (animeData.alternative_title = (e.target as HTMLInputElement).value)} type="text">Alternative title</FloatingLabelInput>
					<!-- <Label for="textarea-id" class="mb-2">Your message</Label> -->
					<Textarea clearable class="description resize-none" placeholder="Description" rows={3} name="message" id="textarea-id" onchange={(e: Event) => (animeData.description = (e.target as HTMLTextAreaElement).value)} value={animeData.description} />
					<MultiSelect
						items={tagsDataMultiSelect}
						value={selectedTags}
						size="lg"
						placeholder="Tags"
						class="bg-[#364153]"
						onchange={(e: CustomEvent<string[]>) => {
							if (e.currentTarget) {
								const value = (e.currentTarget as HTMLInputElement).value;
								const selected = Array.isArray(value)
									? value
									: value
											.split(',')
											.map((v) => v.trim())
											.filter(Boolean);
								animeData.tags = filterSelectedItems(selected, 'tags');
							}
						}}
					>
						{#snippet children({ item, clear }: { item: MultiSelectType; clear: () => void })}
							<Badge color={item.color} dismissable params={{ duration: 100 }} onclose={clear}>
								{item.name}
							</Badge>
						{/snippet}
					</MultiSelect>
					<MultiSelect
						items={studiosDataMultiSelect}
						value={selectedStudios}
						size="lg"
						placeholder="Studio"
						class="bg-[#364153]"
						onchange={(e: CustomEvent<string[]>) => {
							if (e.currentTarget) {
								const value = (e.currentTarget as HTMLInputElement).value;
								const selected = Array.isArray(value)
									? value
									: value
											.split(',')
											.map((v) => v.trim())
											.filter(Boolean);
								animeData.studio = filterSelectedItems(selected, 'studio');
							}
						}}
					>
						{#snippet children({ item, clear }: { item: MultiSelectType; clear: () => void })}
							<Badge color={item.color} dismissable params={{ duration: 100 }} onclose={clear}>
								{item.name}
							</Badge>
						{/snippet}
					</MultiSelect>
				</div>
				<div class="flex flex-col gap-3 [&>div]:[&>input]:max-h-[44px]">
					<div>
						<Select items={anime_types} bind:value={animeData.anime_type} onchange={(e: Event) => (animeData.anime_type = (e.target as HTMLSelectElement).value)} />
					</div>
					<div>
						<Select items={anime_status} bind:value={animeData.status} onchange={(e: Event) => (animeData.status = (e.target as HTMLSelectElement).value)} />
					</div>
					<div>
						<Select items={anime_sources} bind:value={animeData.source} onchange={(e: Event) => (animeData.source = (e.target as HTMLSelectElement).value)} />
					</div>
					<div>
						<Datepicker id="aired" bind:value={date} placeholder="Aired" aria-label="Aired" aria-describedby="helper-text-explanation" class="bg-[#364153]" disabled />
					</div>
					<div class="relative flex items-center w-full">
						<ButtonGroup>
							<Button
								type="button"
								id="decrement-button"
								onclick={() => {
									if (animeData.episodes > 0) animeData.episodes -= 1;
								}}
								class="h-11 p-3"
							>
								<MinusOutline />
							</Button>
							<Input
								min="0"
								bind:value={animeData.episodes}
								type="number"
								id="quantity-input"
								aria-describedby="helper-text-explanation"
								placeholder=" "
								required
								class="h-11 w-28 pb-6 text-center"
								onchange={(e: Event) => {
									const value = parseInt((e.target as HTMLInputElement).value, 10);
									if (!isNaN(value)) {
										animeData.episodes = value;
									}
								}}
							/>
							<div class="absolute start-1/2 bottom-0 flex -translate-x-1/2 items-center space-x-1 text-xs text-gray-400 rtl:translate-x-1/2 rtl:space-x-reverse">
								<VideoCameraOutline class="h-4 w-4" />
								<span>Episodes</span>
							</div>
							<Button type="button" id="increment-button" onclick={() => (animeData.episodes += 1)} class="h-11 p-3">
								<PlusOutline />
							</Button>
						</ButtonGroup>
					</div>
					<FloatingLabelInput value={animeData.discord_role_id} clearable variant="filled" id="discord_role_id" name="discord_role_id" type="text" onchange={(e: Event) => (animeData.discord_role_id = (e.target as HTMLInputElement).value)} class="bg-[#101828] rounded-lg">Role ID</FloatingLabelInput>
					<div class="flex items-center justify-between">
						<span class="text-white">In progress</span>
						<Toggle color="teal" checked={animeData.in_progress} onchange={(e: Event) => (animeData.in_progress = (e.target as HTMLInputElement).checked)}></Toggle>
					</div>
				</div>
			</div>
			<!-- Debug button -->
			<!-- <div class="p-4">
		<Button color="yellow" onclick={debugFormData}>Debug FormData</Button>
	</div> -->
			<SaveButtons
				action_name="Anime"
				save_url_path={`/admin/management/anime`}
				save_and_continue_url_path={`/admin/management/anime/${data.slug}`}
				save_and_new_url_path="/admin/management/anime/add"
				slug={data.slug}
				id={animeData.id.id.String}
				{formData}
				data={animeData}
				save_function={patchAnimeData}
				{...data.isLoggedIn.message.roles?.some((role: any) => role.permissions?.anime?.delete === true) || data.isLoggedIn.message.is_superuser ? { delete_function: deleteAnime } : {}}
				can_read_logs={data.isLoggedIn.message.roles?.some((role: any) => role.permissions?.logs?.read === true) || data.isLoggedIn.message.is_superuser}
			/>
		</div>
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
