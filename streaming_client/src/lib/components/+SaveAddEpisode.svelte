<script lang="ts">
	// @ts-ignore
	import { FloatingLabelInput, Label, Helper, Button, ButtonGroup, Input, MultiSelect, Badge, Select, Textarea, Toggle, Datepicker, Alert, Fileupload, InputAddon } from 'flowbite-svelte';
	// @ts-ignore
	import { CalendarIcon, Upload, X, Plus } from '@lucide/svelte';
	import { PlusOutline, MinusOutline, VideoCameraOutline, CirclePlusOutline, ClockOutline } from 'flowbite-svelte-icons';
	import { type AnimeList, type ComboBoxType, Dummy, type Episodes, type MultiSelectType, type UserPublic } from '$lib/types.ts';
	import { onMount } from 'svelte';
	import SaveButtons from './SaveButtons.svelte';
	import { getDomainName } from '$lib/utils.ts';
	import { episodeWebhook } from '$lib/data/admin/post.ts';
	import { deleteEpisode } from '$lib/data/admin/delete.ts';
	import { Combobox } from 'bits-ui';
	import CaretUpDown from 'phosphor-svelte/lib/CaretUpDown';
	import Check from 'phosphor-svelte/lib/Check';
	import CaretDoubleUp from 'phosphor-svelte/lib/CaretDoubleUp';
	import CaretDoubleDown from 'phosphor-svelte/lib/CaretDoubleDown';
	import CirclesFour from 'phosphor-svelte/lib/CirclesFour';
	import CirclesThree from 'phosphor-svelte/lib/CirclesThree';
	import Loading from '$lib/data/client/components/Loading.svelte';

	let isLoading = $state(true);
	let colors = ['indigo', 'blue', 'green', 'yellow', 'red', 'purple', 'pink'];
	let {
		episodeData: initialEpisodeData,
		save_fn,
		users,
		anime
	} = $props<{
		episodeData?: Episodes;
		save_fn: (formData: FormData) => Promise<any>;
		users: UserPublic[];
		anime: AnimeList;
	}>();
	let episodeData = $state<Episodes>(Dummy.Episodes());

	$effect(() => {
		if (initialEpisodeData) {
			episodeData = initialEpisodeData;
			if (isLoading) {
				isLoading = false;
			}
		}
	});
	let uploadedImageFile: File | null = null;
	let formData = new FormData();
	let isImageUploaded = $state(false);
	let isThereImage = $state(false);
	let imagePreview = $state<string | null>(null);

	$effect(() => {
		isThereImage = !!episodeData?.image;
	});
	let animeList = $state<AnimeList>([]);
	// let animeListMultiSelect = $state<MultiSelectType[]>([]);
	let animeListComboBox = $state<ComboBoxType[]>([]);
	let subbersListMultiSelect = $state<MultiSelectType[]>([]);
	let subbersListCombobox = $state<ComboBoxType[]>([]);
	let subbersList = $state<UserPublic[]>([]);

	function handleImageUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			uploadedImageFile = file;

			if (formData.has('image')) {
				formData.delete('image');
			}
			formData.append('image', file);

			const reader = new FileReader();
			reader.onload = (e) => {
				if (episodeData !== undefined) {
					imagePreview = e.target?.result as string;
					// episodeData.image = imagePreview;
				}

				isImageUploaded = true;
			};
			reader.readAsDataURL(file);
		}
	}
	function handleTorrentUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			if (formData.has('torrent')) {
				formData.delete('torrent');
			}
			formData.append('torrent', file);
		}
	}
	function handleSubtitlesUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			if (formData.has('subtitles')) {
				formData.delete('subtitles');
			}
			formData.append('subtitles', file);
		}
	}

	function handleVideoPlayerAdd() {
		let player = document.getElementById('video_players_input') as HTMLInputElement;
		if (player && player.value.trim() !== '') {
			if (!episodeData.video_players) {
				episodeData.video_players = [];
			}
			const value = player.value.trim();
			let finalUrl = value;

			// Extract URL from iframe if it's an iframe code
			let iframeMatch = value.match(/<iframe[^>]*src=["']([^"']+)["'][^>]*>/i);
			if (iframeMatch && iframeMatch[1]) {
				finalUrl = iframeMatch[1];
			}

			// Check for duplicates
			const isDuplicate = episodeData.video_players.some((existingUrl) => existingUrl.trim() === finalUrl);

			if (!isDuplicate) {
				episodeData.video_players.push(finalUrl);
				player.value = '';
			} else {
				console.warn('Video player URL already exists');
			}
		}
	}

	let searchAnimeValue = $state('');
	let searchTranslatorsValue = $state('');
	let searchProofreadersValue = $state('');
	let searchUploadersValue = $state('');
	let searchTypesettersValue = $state('');

	const filteredAnime = $derived(searchAnimeValue === '' ? animeListComboBox : animeListComboBox.filter((ani) => ani.label.toLowerCase().includes(searchAnimeValue.toLowerCase())));
	const filteredTranslators = $derived(searchTranslatorsValue === '' ? subbersListCombobox : subbersListCombobox.filter((sb) => sb.label.toLowerCase().includes(searchTranslatorsValue.toLowerCase())));
	const filteredProofreaders = $derived(searchProofreadersValue === '' ? subbersListCombobox : subbersListCombobox.filter((sb) => sb.label.toLowerCase().includes(searchProofreadersValue.toLowerCase())));
	const filteredUploaders = $derived(searchUploadersValue === '' ? subbersListCombobox : subbersListCombobox.filter((sb) => sb.label.toLowerCase().includes(searchUploadersValue.toLowerCase())));
	const filteredTypesetters = $derived(searchTypesettersValue === '' ? subbersListCombobox : subbersListCombobox.filter((sb) => sb.label.toLowerCase().includes(searchTypesettersValue.toLowerCase())));

	onMount(async () => {
		animeList = anime;
		for (const anime of animeList) {
			// animeListMultiSelect.push({
			// 	name: anime.title,
			// 	value: anime.id.id.String,
			// 	color: colors[Math.floor(Math.random() * colors.length)]
			// });
			animeListComboBox.push({
				label: anime.title,
				value: anime.id.id.String
			});
		}

		subbersList = users;
		for (const subber of subbersList) {
			subbersListMultiSelect.push({
				name: subber.name,
				value: subber.id.id.String,
				color: colors[Math.floor(Math.random() * colors.length)]
			});
			subbersListCombobox.push({
				label: subber.name,
				value: subber.id.id.String
			});
		}
		// console.log(episodeData);
		// Only set loading to false if we don't have initial episode data
		// If we have initial episode data, the effect will handle setting isLoading to false
		if (!initialEpisodeData) {
			isLoading = false;
		}
	});
</script>

{#if isLoading}
	<div class="w-full h-full">
		<Loading />
	</div>
{:else}
	<div>
		<div class="flex gap-10 p-4">
			<div class="flex flex-col gap-5">
				<div class="lg:col-span-1 p-4 rounded bg-[#1e2939] max-w-[5000px] max-h-[312px]">
					<h2 class="font-bold mb-2 text-white">Cover Image</h2>
					<div class="flex flex-col items-center gap-4">
						<div class="relative w-84 h-48 rounded overflow-hidden">
							{#if isImageUploaded && imagePreview}
								<img src={imagePreview} alt="Episode cover" class="w-full h-full object-cover" />
							{:else if isThereImage && episodeData.image}
								<img src={`/${episodeData.image}`} alt="Episode cover" class="w-full h-full object-cover" />
							{:else}
								<button class="flex items-center justify-center w-full h-full rounded shadow-around-inset cursor-pointer hover:bg-[#253246] transition-colors" onclick={() => document.getElementById('image-upload')?.click()}>
									<CirclePlusOutline size="xl" color="white" />
								</button>
							{/if}
						</div>
						<label for="image-upload" class="cursor-pointer inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500">
							<Upload class="w-4 h-4 mr-2" />
							Upload Image
						</label>
						<input id="image-upload" type="file" accept="image/*" class="hidden" onchange={handleImageUpload} />
					</div>
				</div>
				<div class="flex justify-between">
					<div>
						<Label for="discord_ping" class="pb-2">Discord ping</Label>
						<Toggle
							id="discord_ping"
							label="Discord ping"
							bind:checked={episodeData.discord_ping}
							onchange={(e: Event) => {
								episodeData.discord_ping = (e.target as HTMLInputElement).checked;
							}}
						/>
					</div>
					<div>
						<Label for="webhook" class="pb-2">Webhook</Label>
						<Toggle
							id="webhook"
							label="Webhook"
							bind:checked={episodeData.webhook}
							onchange={(e: Event) => {
								episodeData.webhook = (e.target as HTMLInputElement).checked;
							}}
						/>
					</div>
					<div class="flex flex-col">
						<Label for="color-picker" class="pb-2">Webhook color</Label>
						<Input
							type="color"
							id="color-picker"
							class="h-12 w-15"
							value={episodeData.color || '#ffffff'}
							onchange={(e: Event) => {
								episodeData.color = (e.target as HTMLInputElement).value;
							}}
						/>
					</div>
				</div>
			</div>
			<div class="flex flex-col gap-5 [&>div]:[&>input]:max-h-[44px] w-full">
				<div>
					<!-- <Select
						items={animeListMultiSelect}
						value={episodeData.anime.id.id.String}
						placeholder="Select anime"
						allowDeselect={true}
						required
						onchange={(e: Event) => {
							const value = (e.target as HTMLSelectElement).value;
							const foundAnime = animeList.find((anime) => anime.id.id.String === value);
							if (foundAnime) {
								episodeData.anime = foundAnime;
							}
						}}
					/> -->
					<Combobox.Root
						type="single"
						name="animeSearch"
						value={episodeData.anime.id.id.String}
						onValueChange={(value) => {
							const foundAnime = animeList.find((anime) => anime.id.id.String === value);
							if (foundAnime) {
								episodeData.anime = foundAnime;
							}
						}}
						onOpenChange={(o) => {
							if (!o) searchAnimeValue = '';
						}}
					>
						<div class="relative w-full">
							<CirclesThree color="#67707f" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
							<Combobox.Input
								defaultValue={episodeData.anime.title}
								oninput={(e) => (searchAnimeValue = e.currentTarget.value)}
								class="inline-flex h-input w-full text-white bg-[#364153] rounded-md truncate rounded-9px border border-border-input bg-background px-11 text-base transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background sm:text-sm active:border-white"
								placeholder="Search anime"
								aria-label="Search anime"
							/>
							<Combobox.Trigger class="absolute end-3 top-1/2 size-6 -translate-y-1/2">
								<CaretUpDown color="#67707f" class="size-6 text-muted-foreground" />
							</Combobox.Trigger>
						</div>
						<Combobox.Portal>
							<Combobox.Content class="max-h-96 w-[var(--bits-combobox-anchor-width)] min-w-[var(--bits-combobox-anchor-width)] rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-none bg-[#364153]" sideOffset={10}>
								<Combobox.ScrollUpButton class="flex w-full items-center justify-center">
									<CaretDoubleUp class="size-3" color="#67707f" />
								</Combobox.ScrollUpButton>
								<Combobox.Viewport class="p-1 bg-[#364153]">
									{#each filteredAnime as ani, i (i + ani.value)}
										<Combobox.Item class="flex h-10 w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none  data-[highlighted]:bg-muted bg-[#364153] text-white" value={ani.value} label={ani.label}>
											{#snippet children({ selected })}
												{ani.label}
												{#if selected}
													<div class="ml-auto">
														<Check />
													</div>
												{/if}
											{/snippet}
										</Combobox.Item>
									{:else}
										<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found, try again. </span>
									{/each}
								</Combobox.Viewport>
								<Combobox.ScrollDownButton class="flex w-full items-center justify-center">
									<CaretDoubleDown class="size-3" color="#67707f" />
								</Combobox.ScrollDownButton>
							</Combobox.Content>
						</Combobox.Portal>
					</Combobox.Root>
				</div>
				<div class="grid grid-cols-2 gap-5 [&>div]:flex [&>div]:flex-col [&>div]:gap-5 [&>div]:[&>div]:[&>input]:max-h-[44px]">
					<div>
						<input value={episodeData.title} placeholder={'Title (*this will be autopopulated (probably).)'} id="title" name="title" type="text" onchange={(e: Event) => (episodeData.title = (e.target as HTMLInputElement).value)} class="bg-[#364153] text-white rounded-lg pb-[10px]" />
						<div>
							<Label for="with_subtitles" class="pb-2">Subtitles</Label>
							<Fileupload accept=".ass, .ssa, .srt" onchange={handleSubtitlesUpload} id="with_subtitles" class="mb-2" />
							{#if episodeData.subtitles}
								<Helper>File: <a class="underline" href={`/${episodeData.subtitles}`}>{episodeData.subtitles}</a></Helper>
							{/if}
						</div>
						<!-- <div>
							<Select
								items={subbersListMultiSelect}
								value={episodeData.translators?.id?.id?.String ?? ''}
								placeholder="Select translator"
								required
								onchange={(e: Event) => {
									episodeData.translators = subbersList.find((subber: UserPublic) => subber.id.id.String === (e.target as HTMLSelectElement).value) ?? null;
								}}
							/>
						</div>
						<div>
							<Select
								items={subbersListMultiSelect}
								value={episodeData.uploaders?.id?.id?.String ?? ''}
								placeholder="Select uploader"
								onchange={(e: Event) => {
									episodeData.uploaders = subbersList.find((subber: UserPublic) => subber.id.id.String === (e.target as HTMLSelectElement).value) ?? null;
								}}
							/>
						</div> -->
					</div>
					<div>
						<div class="grid grid-cols-2 gap-5">
							<div>
								<ButtonGroup class="w-full">
									<Button
										type="button"
										id="decrement-button"
										onclick={() => {
											if (episodeData.episode > 0) episodeData.episode -= 1;
										}}
										class="h-11 p-3"
									>
										<MinusOutline />
									</Button>
									<Input
										min="0"
										bind:value={episodeData.episode}
										type="number"
										id="quantity-input"
										aria-describedby="helper-text-explanation"
										placeholder=" "
										required
										class="h-11 w-full pb-6 text-center appearance-none"
										onchange={(e: Event) => {
											const value = parseInt((e.target as HTMLInputElement).value, 10);
											if (!isNaN(value)) {
												episodeData.episode = value;
											}
										}}
									/>
									<div class="absolute start-1/2 bottom-0 flex -translate-x-1/2 items-center space-x-1 text-xs text-gray-400 rtl:translate-x-1/2 rtl:space-x-reverse">
										<VideoCameraOutline class="h-4 w-4" />
										<span>Episode</span>
									</div>
									<Button type="button" id="increment-button" onclick={() => (episodeData.episode += 1)} class="h-11 p-3">
										<PlusOutline />
									</Button>
								</ButtonGroup>
							</div>
							<div>
								<ButtonGroup class="w-full">
									<Button
										type="button"
										id="decrement-button"
										onclick={() => {
											if (episodeData.length > 0) episodeData.length -= 1;
										}}
										class="h-11 p-3"
									>
										<MinusOutline />
									</Button>
									<Input
										min="0"
										bind:value={episodeData.length}
										type="number"
										id="quantity-input"
										aria-describedby="helper-text-explanation"
										placeholder=" "
										required
										class="h-11 w-full pb-6 text-center appearance-none"
										onchange={(e: Event) => {
											const value = parseInt((e.target as HTMLInputElement).value, 10);
											if (!isNaN(value)) {
												episodeData.length = value;
											}
										}}
									/>
									<div class="absolute start-1/2 bottom-0 flex -translate-x-1/2 items-center space-x-1 text-xs text-gray-400 rtl:translate-x-1/2 rtl:space-x-reverse">
										<ClockOutline class="h-4 w-4" />
										<span>Length</span>
									</div>
									<Button type="button" id="increment-button" onclick={() => (episodeData.length += 1)} class="h-11 p-3">
										<PlusOutline />
									</Button>
								</ButtonGroup>
							</div>
						</div>
						<div>
							<Label for="with_torrent" class="pb-2">Torrent</Label>
							<Fileupload accept=".torrent" onchange={handleTorrentUpload} id="with_torrent" class="mb-2" />
							{#if episodeData.torrent}
								<Helper>File: <a class="underline" href={`/${episodeData.torrent}`}>{episodeData.torrent}</a></Helper>
							{/if}
						</div>
						<!-- <div>
							<Select
								items={subbersListMultiSelect}
								value={episodeData.proofreaders?.id?.id?.String ?? ''}
								placeholder="Select proofreader"
								required
								onchange={(e: Event) => {
									episodeData.proofreaders = subbersList.find((subber: UserPublic) => subber.id.id.String === (e.target as HTMLSelectElement).value) ?? null;
								}}
							/>
						</div>
						<div>
							<Select
								items={subbersListMultiSelect}
								value={episodeData.typesetters?.id?.id?.String ?? ''}
								placeholder="Select typesetter"
								onchange={(e: Event) => {
									episodeData.typesetters = subbersList.find((subber: UserPublic) => subber.id.id.String === (e.target as HTMLSelectElement).value) ?? null;
								}}
							/>
						</div> -->
					</div>
				</div>

				<div class="grid grid-cols-2 gap-5 [&>div]:flex [&>div]:flex-col [&>div]:gap-5 [&>div]:[&>div]:[&>input]:max-h-[44px]">
					<Combobox.Root
						type="multiple"
						name="translatorsSearch"
						value={episodeData.translators?.map((translator) => translator.id.id.String)}
						allowDeselect={true}
						onValueChange={(value) => {
							episodeData.translators = subbersList.filter((subber: UserPublic) => value.includes(subber.id.id.String)) ?? null;
						}}
						onOpenChange={(o) => {
							if (!o) searchTranslatorsValue = '';
						}}
					>
						<div class="relative w-full">
							<CirclesThree color="#67707f" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
							<Combobox.Input
								defaultValue={episodeData.translators?.map((translator) => translator.name).join(', ')}
								clearOnDeselect={true}
								oninput={(e) => (searchTranslatorsValue = e.currentTarget.value)}
								class="inline-flex h-input w-full text-white bg-[#364153] rounded-md truncate rounded-9px border border-border-input bg-background px-11 text-base transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background sm:text-sm active:border-white"
								placeholder="Search translators"
								aria-label="Search translators"
							/>
							<Combobox.Trigger class="absolute end-3 top-1/2 size-6 -translate-y-1/2">
								<CaretUpDown color="#67707f" class="size-6 text-muted-foreground" />
							</Combobox.Trigger>
						</div>
						<Combobox.Portal>
							<Combobox.Content class="max-h-96 w-[var(--bits-combobox-anchor-width)] min-w-[var(--bits-combobox-anchor-width)] rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-none bg-[#364153]" sideOffset={10}>
								<Combobox.ScrollUpButton class="flex w-full items-center justify-center">
									<CaretDoubleUp class="size-3" color="#67707f" />
								</Combobox.ScrollUpButton>
								<Combobox.Viewport class="p-1 bg-[#364153]">
									{#each filteredTranslators as sb, i (i + sb.value)}
										<Combobox.Item class="flex h-10 w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none  data-[highlighted]:bg-muted bg-[#364153] text-white" value={sb.value} label={sb.label}>
											{#snippet children({ selected })}
												{sb.label}
												{#if selected}
													<div class="ml-auto">
														<Check />
													</div>
												{/if}
											{/snippet}
										</Combobox.Item>
									{:else}
										<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found, try again. </span>
									{/each}
								</Combobox.Viewport>
								<Combobox.ScrollDownButton class="flex w-full items-center justify-center">
									<CaretDoubleDown class="size-3" color="#67707f" />
								</Combobox.ScrollDownButton>
							</Combobox.Content>
						</Combobox.Portal>
					</Combobox.Root>
					<Combobox.Root
						type="multiple"
						name="proofreadersSearch"
						value={episodeData.proofreaders?.map((proofreader) => proofreader.id.id.String)}
						allowDeselect={true}
						onValueChange={(value) => {
							episodeData.proofreaders = subbersList.filter((subber: UserPublic) => value.includes(subber.id.id.String)) ?? null;
						}}
						onOpenChange={(o) => {
							if (!o) searchProofreadersValue = '';
						}}
					>
						<div class="relative w-full">
							<CirclesThree color="#67707f" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
							<Combobox.Input
								defaultValue={episodeData.proofreaders?.map((proofreader) => proofreader.name).join(', ')}
								clearOnDeselect={true}
								oninput={(e) => (searchProofreadersValue = e.currentTarget.value)}
								class="inline-flex h-input w-full text-white bg-[#364153] rounded-md truncate rounded-9px border border-border-input bg-background px-11 text-base transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background sm:text-sm active:border-white"
								placeholder="Search proofreaders"
								aria-label="Search proofreaders"
							/>
							<Combobox.Trigger class="absolute end-3 top-1/2 size-6 -translate-y-1/2">
								<CaretUpDown color="#67707f" class="size-6 text-muted-foreground" />
							</Combobox.Trigger>
						</div>
						<Combobox.Portal>
							<Combobox.Content class="max-h-96 w-[var(--bits-combobox-anchor-width)] min-w-[var(--bits-combobox-anchor-width)] rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-none bg-[#364153]" sideOffset={10}>
								<Combobox.ScrollUpButton class="flex w-full items-center justify-center">
									<CaretDoubleUp class="size-3" color="#67707f" />
								</Combobox.ScrollUpButton>
								<Combobox.Viewport class="p-1 bg-[#364153]">
									{#each filteredProofreaders as sb, i (i + sb.value)}
										<Combobox.Item class="flex h-10 w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none  data-[highlighted]:bg-muted bg-[#364153] text-white" value={sb.value} label={sb.label}>
											{#snippet children({ selected })}
												{sb.label}
												{#if selected}
													<div class="ml-auto">
														<Check />
													</div>
												{/if}
											{/snippet}
										</Combobox.Item>
									{:else}
										<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found, try again. </span>
									{/each}
								</Combobox.Viewport>
								<Combobox.ScrollDownButton class="flex w-full items-center justify-center">
									<CaretDoubleDown class="size-3" color="#67707f" />
								</Combobox.ScrollDownButton>
							</Combobox.Content>
						</Combobox.Portal>
					</Combobox.Root>

					<Combobox.Root
						type="multiple"
						name="uploadersSearch"
						value={episodeData.uploaders?.map((uploader) => uploader.id.id.String)}
						allowDeselect={true}
						onValueChange={(value) => {
							episodeData.uploaders = subbersList.filter((subber: UserPublic) => value.includes(subber.id.id.String)) ?? null;
						}}
						onOpenChange={(o) => {
							if (!o) searchUploadersValue = '';
						}}
					>
						<div class="relative w-full">
							<CirclesThree color="#67707f" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
							<Combobox.Input
								defaultValue={episodeData.uploaders?.map((uploader) => uploader.name).join(', ')}
								clearOnDeselect={true}
								oninput={(e) => (searchUploadersValue = e.currentTarget.value)}
								class="inline-flex h-input w-full text-white bg-[#364153] rounded-md truncate rounded-9px border border-border-input bg-background px-11 text-base transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background sm:text-sm active:border-white"
								placeholder="Search uploaders"
								aria-label="Search uploaders"
							/>
							<Combobox.Trigger class="absolute end-3 top-1/2 size-6 -translate-y-1/2">
								<CaretUpDown color="#67707f" class="size-6 text-muted-foreground" />
							</Combobox.Trigger>
						</div>
						<Combobox.Portal>
							<Combobox.Content class="max-h-96 w-[var(--bits-combobox-anchor-width)] min-w-[var(--bits-combobox-anchor-width)] rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-none bg-[#364153]" sideOffset={10}>
								<Combobox.ScrollUpButton class="flex w-full items-center justify-center">
									<CaretDoubleUp class="size-3" color="#67707f" />
								</Combobox.ScrollUpButton>
								<Combobox.Viewport class="p-1 bg-[#364153]">
									{#each filteredUploaders as sb, i (i + sb.value)}
										<Combobox.Item class="flex h-10 w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none  data-[highlighted]:bg-muted bg-[#364153] text-white" value={sb.value} label={sb.label}>
											{#snippet children({ selected })}
												{sb.label}
												{#if selected}
													<div class="ml-auto">
														<Check />
													</div>
												{/if}
											{/snippet}
										</Combobox.Item>
									{:else}
										<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found, try again. </span>
									{/each}
								</Combobox.Viewport>
								<Combobox.ScrollDownButton class="flex w-full items-center justify-center">
									<CaretDoubleDown class="size-3" color="#67707f" />
								</Combobox.ScrollDownButton>
							</Combobox.Content>
						</Combobox.Portal>
					</Combobox.Root>

					<Combobox.Root
						type="multiple"
						name="typesettersSearch"
						value={episodeData.typesetters?.map((typesetter) => typesetter.id.id.String)}
						allowDeselect={true}
						onValueChange={(value) => {
							episodeData.typesetters = subbersList.filter((subber: UserPublic) => value.includes(subber.id.id.String)) ?? null;
						}}
						onOpenChange={(o) => {
							if (!o) searchTypesettersValue = '';
						}}
					>
						<div class="relative w-full">
							<CirclesThree color="#67707f" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
							<Combobox.Input
								defaultValue={episodeData.typesetters?.map((typesetter) => typesetter.name).join(', ')}
								clearOnDeselect={true}
								oninput={(e) => (searchTypesettersValue = e.currentTarget.value)}
								class="inline-flex h-input w-full text-white bg-[#364153] rounded-md truncate rounded-9px border border-border-input bg-background px-11 text-base transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background sm:text-sm active:border-white"
								placeholder="Search typesetters"
								aria-label="Search typesetters"
							/>
							<Combobox.Trigger class="absolute end-3 top-1/2 size-6 -translate-y-1/2">
								<CaretUpDown color="#67707f" class="size-6 text-muted-foreground" />
							</Combobox.Trigger>
						</div>
						<Combobox.Portal>
							<Combobox.Content class="max-h-96 w-[var(--bits-combobox-anchor-width)] min-w-[var(--bits-combobox-anchor-width)] rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-none bg-[#364153]" sideOffset={10}>
								<Combobox.ScrollUpButton class="flex w-full items-center justify-center">
									<CaretDoubleUp class="size-3" color="#67707f" />
								</Combobox.ScrollUpButton>
								<Combobox.Viewport class="p-1 bg-[#364153]">
									{#each filteredTypesetters as sb, i (i + sb.value)}
										<Combobox.Item class="flex h-10 w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none  data-[highlighted]:bg-muted bg-[#364153] text-white" value={sb.value} label={sb.label}>
											{#snippet children({ selected })}
												{sb.label}
												{#if selected}
													<div class="ml-auto">
														<Check />
													</div>
												{/if}
											{/snippet}
										</Combobox.Item>
									{:else}
										<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found, try again. </span>
									{/each}
								</Combobox.Viewport>
								<Combobox.ScrollDownButton class="flex w-full items-center justify-center">
									<CaretDoubleDown class="size-3" color="#67707f" />
								</Combobox.ScrollDownButton>
							</Combobox.Content>
						</Combobox.Portal>
					</Combobox.Root>
				</div>

				<div class="flex flex-col gap-5 [&>div]:[&>input]:max-h-[44px]">
					<div>
						<Textarea clearable class="description resize-none" placeholder="Description" rows={3} name="message" id="textarea-id" onchange={(e: Event) => (episodeData.description = (e.target as HTMLTextAreaElement).value)} value={episodeData.description} />
					</div>
					<div class="flex flex-col">
						<div class="flex">
							<Input
								id="video_players_input"
								class="rounded-none"
								autocomplete="off"
								clearable
								placeholder="Video player"
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter') {
										handleVideoPlayerAdd();
									}
								}}
							/>
							<Button type="button" class="rounded-none bg-blue-600 text-white hover:bg-blue-500 cursor-pointer" onclick={handleVideoPlayerAdd}>
								<Plus class="h-5 w-5 text-white" />
							</Button>
						</div>
						{#each episodeData.video_players ?? [] as player, index (index)}
							<div class="flex">
								<ButtonGroup class="w-full flex">
									<InputAddon class="!rounded-none">{getDomainName(player)}</InputAddon>
									<Input class="!rounded-none" bind:value={episodeData.video_players![index]} disabled placeholder="Video player" />
								</ButtonGroup>
								<Button
									type="button"
									class="rounded-none w-[60px] bg-red-600 text-white hover:bg-red-500 cursor-pointer"
									onclick={() => {
										if (episodeData.video_players) {
											episodeData.video_players.splice(index, 1);
										}
									}}
								>
									<X class="h-5 w-5 text-white" />
								</Button>
							</div>
						{/each}
					</div>
				</div>
			</div>
		</div>
		<SaveButtons
			action_name="Episodes"
			save_url_path={`/admin/management/episodes`}
			save_and_continue_url_path={`/admin/management/episodes/${episodeData.anime.slug}/${episodeData.episode}`}
			save_and_new_url_path="/admin/management/episodes/add"
			slug={episodeData.slug}
			id={episodeData.id.id.String}
			webhook_func={episodeWebhook}
			{formData}
			data={episodeData}
			save_function={save_fn}
			delete_function={deleteEpisode}
		/>
	</div>
{/if}
