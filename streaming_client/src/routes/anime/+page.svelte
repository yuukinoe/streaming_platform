<script lang="ts">
	// @ts-ignore
	import { Tabs, TabItem, Button, Checkbox } from 'flowbite-svelte';
	import { SearchOutline, FilterOutline, PlusOutline, CheckOutline } from 'flowbite-svelte-icons';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import type { TagOrStudio } from '$lib/data/client/types.ts';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { getSeasonDates } from '$lib/utils.ts';
	import { page } from '$app/state';
	import SettingsIconMobile from '$lib/data/client/components/icons/SettingsIconMobile.svelte';
	import NotFound from '$lib/data/client/components/404.svelte';
	import ServerError from '$lib/data/client/components/500.svelte';
	import type { PageProps } from '../$types.js';

	const { data }: PageProps = $props();

	let isLoading = $state(true);
	let is404 = $state(false);
	let is500 = $state(false);

	let showFilters = $state(false);

	let tagsState = $state([]) as TagOrStudio[];
	let animeTypeState = $state([]) as string[];

	// Types
	type episodeCountType = {
		label: string;
		min: number;
		max: number;
	};

	type statusType = {
		label: string;
	};

	// Filters Selected
	let search = $state('');
	let selectedTagsState = $state([]) as TagOrStudio[];
	let selectedAnimeTypesState = $state([]) as string[];
	let selectedEpisodeCountState = $state([]) as episodeCountType[];
	let selectedStatusState = $state([]) as statusType[];
	let filteredAnime = $state(data.anime);
	let episodeCount = $state([
		{
			label: 'Krótkie',
			min: 1,
			max: 6
		},
		{
			label: 'Sezonowe',
			min: 12,
			max: 26
		},
		{
			label: 'Średnie',
			min: 27,
			max: 100
		}
	]) as episodeCountType[];

	let statusOptions = $state([
		{
			label: 'Zakończone'
		},
		{
			label: 'Sezonowe (obecnie wychodzące)'
		},
		{
			label: 'W trakcie (wychodzące poza sezonem)'
		}
	]) as statusType[];
	const [seasonStart, seasonEnd] = getSeasonDates();

	function filterAnime() {
		const lowerSearch = search.toLowerCase();
		console.log(data.filteredAnime);
		filteredAnime = data.anime
			.filter((anime) => {
				const title = anime.title?.toLowerCase() || '';
				const altTitle = anime.alternative_title?.toLowerCase() || '';

				const matchesSearch = !search || title.includes(lowerSearch) || altTitle.includes(lowerSearch);

				const matchesTags = selectedTagsState.length === 0 || anime.tags?.some((tag) => selectedTagsState.some((selectedTag) => selectedTag.id.id.String === tag.id.id.String));

				const matchesAnimeType = selectedAnimeTypesState.length === 0 || selectedAnimeTypesState.includes(anime.anime_type);

				const matchesEpisodeCount = selectedEpisodeCountState.length === 0 || selectedEpisodeCountState.some((count) => anime.episodes >= count.min && anime.episodes <= count.max);

				const matchesStatus =
					selectedStatusState.length === 0 ||
					selectedStatusState.some((status) => {
						if (status.label === 'Zakończone') {
							return anime.status === 'Finished Airing';
						}
						if (status.label === 'Sezonowe (obecnie wychodzące)') {
							if (!anime.aired) return false;
							let airedDate: Date | null = null;
							if (typeof anime.aired === 'string') {
								airedDate = new Date(anime.aired);
							}
							if (!airedDate || isNaN(airedDate.getTime())) return false;
							return airedDate >= seasonStart && airedDate <= seasonEnd;
						}
						if (status.label === 'W trakcie (wychodzące poza sezonem)') {
							return anime.in_progress === true;
						}
						return false;
					});

				return matchesSearch && matchesTags && matchesAnimeType && matchesEpisodeCount && matchesStatus;
			})
			.sort((a, b) => {
				const titleA = a.title?.toLowerCase() || '';
				const titleB = b.title?.toLowerCase() || '';
				const altA = a.alternative_title?.toLowerCase() || '';
				const altB = b.alternative_title?.toLowerCase() || '';

				const aStarts = titleA.startsWith(lowerSearch) || altA.startsWith(lowerSearch);
				const bStarts = titleB.startsWith(lowerSearch) || altB.startsWith(lowerSearch);

				if (aStarts && !bStarts) return -1;
				if (!aStarts && bStarts) return 1;

				return titleA.localeCompare(titleB);
			});
		console.log(filteredAnime.length);
	}

	onMount(async () => {
		try {
			const tags = data.anime.flatMap((anime) => anime.tags || []);
			tagsState = tags.filter((tag, idx, arr) => idx === arr.findIndex((t) => t && tag && (t.id === tag.id || t.name === tag.name)));

			const animeTypes = data.anime.map((anime) => anime.anime_type).filter(Boolean);
			animeTypeState = animeTypes.filter((type, idx, arr) => arr.indexOf(type) === idx);

			if (page.url.hash) {
				const tag = tagsState.find((tag) => tag.slug === page.url.hash.slice(1));
				if (tag) {
					selectedTagsState = [tag];
					filterAnime();
				}
			}

			isLoading = false;
		} catch (error) {
			console.error(error);

			if ((error instanceof Error && error.message.includes('500')) || (error instanceof Error && error.message.includes('Internal Server Error')) || (error instanceof Error && error.message.includes('Failed to fetch'))) {
				is500 = true;
			} else {
				is404 = true;
			}
		}
	});
</script>

<svelte:head>
	<title>teacup - Anime</title>
	<meta property="og:title" content="teacup - Anime" />
	<meta property="og:description" content="teacup - oglądaj i ciesz się każdą chwilą." />
	<meta property="og:url" content="https://teacup.pl" />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="teacup" />
	<meta property="og:locale" content="pl_PL" />
</svelte:head>

<div class="w-full h-full">
	{#if is500}
		<ServerError />
	{:else if is404}
		<NotFound />
	{:else if isLoading}
		<Loading />
	{:else}
		<div class="flex flex-col gap-10 [&>div]:[&>div]:mx-3 [&>div]:[&>div]:sm:mx-10 [&>div]:[&>div]:flex [&>div]:[&>div]:flex-col [&>div]:[&>div]:gap-8">
			<div class="bg-[#0E131CA6] mix-blend-plus-lighter h-15 flex items-center">
				<div>
					<div class="text-[#EEEDF0B2] font-roboto font-[400] flex gap-1">
						<div class="text-[20px]">Anime</div>
					</div>
				</div>
			</div>

			<div>
				<div>
					<div class="flex gap-2 w-full">
						<div class="relative w-full md:w-5/12">
							<div class="h-full absolute flex items-center justify-center left-4 top-[1.5px]">
								<SearchOutline color="#C4C4C4" size="lg" />
							</div>
							<input
								class="indent-9 bg-[#0E131CA6] mix-blend-plus-lighter rounded-full border-0 text-white text-2xl placeholder:text-[#C4C4C4] w-full"
								placeholder="Search"
								oninput={(e) => {
									search = (e.target as HTMLInputElement).value;
									filterAnime();
								}}
							/>
						</div>
						<Button
							id="filter_button"
							class={`flex gap-2 hover:!bg-[#1f1f1f00] !bg-[#1f1f1f00] focus-within:!ring-0 transition-all md:focus-within:!ring-0 md:focus-visible:!ring-0 ${showFilters ? 'md:!bg-[#AD142D]' : 'md:!bg-[#6c1d1d]'}  !p-0 md:!py-[10px] md:!px-[20px] md:hover:!bg-[#6c1d1db5] cursor-pointer`}
							color="red"
							pill
							onclick={() => (showFilters = !showFilters)}
						>
							<!-- <ListOutline class="md:hidden block h-[48px] w-[48px] " /> -->
							<SettingsIconMobile className="hidden md:block h-[48px] w-[48px]" />
							<FilterOutline class="hidden md:block" size="lg" />
							<span class="text-xl hidden md:block"> Filtruj </span>
						</Button>
					</div>

					{#if showFilters}
						<div id="anime_filters" class="bg-[#0E131CA6] mix-blend-plus-lighter hidden md:flex flex-col gap-5 p-8 text-white rounded-xl text-xl font-roboto [&>div]:gap-2" transition:slide>
							<div class="flex">
								<div id="filter_label" class="min-w-[150px] max-w-[150px] w-full">GATUNEK:</div>
								<div class="flex gap-3 flex-wrap">
									{#each tagsState as tag (tag.id.id.String)}
										<div class="flex items-center gap-0 [&>div]:[&>label]:[&>input]:-translate-y-1 [&>div]:[&>label]:!h-[16px]">
											<Checkbox
												value={tag.id.id.String}
												id={tag.id.id.String}
												class="text-[#ad142d] focus:ring-[#ad142d]"
												checked={selectedTagsState.some((t) => t.id.id.String === tag.id.id.String)}
												onclick={() => {
													if (selectedTagsState.some((t) => t.id.id.String === tag.id.id.String)) {
														selectedTagsState = selectedTagsState.filter((t) => t.id.id.String !== tag.id.id.String);
													} else {
														selectedTagsState = [...selectedTagsState, tag];
													}
													filterAnime();
												}}
											/>
											<span class="text-[#EEEDF0CC] font-roboto">{tag.name}</span>
										</div>
									{/each}
								</div>
							</div>
							<div class="h-[1px] w-full bg-[#3c3c3d]"></div>
							<div class="flex">
								<div id="filter_label" class="min-w-[150px] max-w-[150px] w-full">TYP SERII:</div>
								<div class="flex gap-3 flex-wrap">
									{#each animeTypeState as series_type (series_type)}
										<div class="flex items-center gap-0 [&>div]:[&>label]:[&>input]:-translate-y-1 [&>div]:[&>label]:!h-[16px]">
											<Checkbox
												value={series_type}
												id={series_type}
												class="text-[#ad142d] focus:ring-[#ad142d]"
												checked={selectedAnimeTypesState.includes(series_type)}
												onclick={() => {
													if (selectedAnimeTypesState.includes(series_type)) {
														selectedAnimeTypesState = selectedAnimeTypesState.filter((t) => t !== series_type);
													} else {
														selectedAnimeTypesState = [...selectedAnimeTypesState, series_type];
													}
													filterAnime();
												}}
											/>
											<span class="text-[#EEEDF0CC] font-roboto">{series_type}</span>
										</div>
									{/each}
								</div>
							</div>

							<div class="h-[1px] w-full bg-[#3c3c3d]"></div>
							<div class="flex">
								<div id="filter_label" class="min-w-[150px] max-w-[150px] w-full">ILOŚĆ ODCINKÓW:</div>
								<div class="flex gap-3 flex-wrap">
									{#each episodeCount as episodeAmount (episodeAmount.label)}
										<div class="flex items-center gap-0 [&>div]:[&>label]:[&>input]:-translate-y-1 [&>div]:[&>label]:!h-[16px]">
											<Checkbox
												value={episodeAmount.label}
												id={episodeAmount.label}
												class="text-[#ad142d] focus:ring-[#ad142d]"
												checked={selectedEpisodeCountState.some((t) => t.label === episodeAmount.label)}
												onclick={() => {
													if (selectedEpisodeCountState.some((t) => t.label === episodeAmount.label)) {
														selectedEpisodeCountState = selectedEpisodeCountState.filter((t) => t.label !== episodeAmount.label);
													} else {
														selectedEpisodeCountState = [...selectedEpisodeCountState, episodeAmount];
													}
													filterAnime();
												}}
											/>
											<span class="text-[#EEEDF0CC] font-roboto">{episodeAmount.label} ({episodeAmount.min}-{episodeAmount.max}odc.)</span>
										</div>
									{/each}
								</div>
							</div>
							<div class="h-[1px] w-full bg-[#3c3c3d]"></div>
							<div class="flex">
								<div id="filter_label" class="min-w-[150px] max-w-[150px] w-full">STATUS:</div>
								<div class="flex gap-3 flex-wrap">
									{#each statusOptions as status (status.label)}
										<div class="flex items-center gap-0 [&>div]:[&>label]:[&>input]:-translate-y-1 [&>div]:[&>label]:!h-[16px]">
											<Checkbox
												value={status.label}
												id={status.label}
												class="text-[#ad142d] focus:ring-[#ad142d]"
												checked={selectedStatusState.some((s) => s.label === status.label)}
												onclick={() => {
													if (selectedStatusState.some((s) => s.label === status.label)) {
														selectedStatusState = selectedStatusState.filter((s) => s.label !== status.label);
													} else {
														selectedStatusState = [...selectedStatusState, status];
													}
													filterAnime();
												}}
											/>
											<span class="text-[#EEEDF0CC] font-roboto">{status.label}</span>
										</div>
									{/each}
								</div>
							</div>
						</div>
						<div class="[&>div:first-of-type]:!bg-[#2a2a2a] [&>div]:!bg-[#1a1a1a] [&>div]:rounded-none [&>div]:mt-0 [&>div:first-of-type]:h-[2px] block md:hidden" transition:slide>
							<Tabs tabStyle="underline">
								<TabItem open title="GATUNEK" class="w-full [&>button]:text-[13px] [&>button]:p-1 [&>button]:pb-3 [&>button]:w-full">
									<div class="w-full">
										<div class="flex flex-wrap gap-3">
											{#each tagsState as tag (tag.id.id.String)}
												<button
													class={`relative flex items-center gap-3 justify-center text-white rounded-full px-5 py-1 text-[12px] ${selectedTagsState.some((t) => t.id.id.String === tag.id.id.String) ? 'bg-[#6a1b1b]' : 'bg-[#2b2b2b]'}`}
													onclick={() => {
														if (selectedTagsState.some((t) => t.id.id.String === tag.id.id.String)) {
															selectedTagsState = selectedTagsState.filter((t) => t.id.id.String !== tag.id.id.String);
														} else {
															selectedTagsState = [...selectedTagsState, tag];
														}
														filterAnime();
													}}
												>
													<span>
														{tag.name}
													</span>
													<span>
														{#if selectedTagsState.includes(tag)}
															<CheckOutline size="sm" />
														{:else}
															<PlusOutline size="sm" />
														{/if}
													</span>
												</button>
											{/each}
										</div>
									</div>
								</TabItem>
								<TabItem title="TYP SERII" class="w-full text-nowrap [&>button]:text-[13px] [&>button]:p-1 [&>button]:pb-3 [&>button]:w-full">
									<div class="w-full">
										<div class="flex flex-wrap gap-3">
											{#each animeTypeState as series_type (series_type)}
												<button
													class={`relative flex items-center gap-3 justify-center text-white rounded-full px-5 py-1 text-[12px] ${selectedAnimeTypesState.includes(series_type) ? 'bg-[#6a1b1b]' : 'bg-[#2b2b2b]'}`}
													onclick={() => {
														if (selectedAnimeTypesState.includes(series_type)) {
															selectedAnimeTypesState = selectedAnimeTypesState.filter((t) => t !== series_type);
														} else {
															selectedAnimeTypesState = [...selectedAnimeTypesState, series_type];
														}
														filterAnime();
													}}
												>
													<span>{series_type}</span>
													<span>
														{#if selectedAnimeTypesState.includes(series_type)}
															<CheckOutline size="sm" />
														{:else}
															<PlusOutline size="sm" />
														{/if}
													</span>
												</button>
											{/each}
										</div>
									</div>
								</TabItem>
								<TabItem title="ILOŚĆ ODCINKÓW" class="w-full text-nowrap [&>button]:text-[13px] [&>button]:p-1 [&>button]:pb-3 [&>button]:w-full">
									<div class="w-full">
										<div class="flex flex-wrap gap-3">
											{#each episodeCount as episodeAmount (episodeAmount.label)}
												<button
													class={`relative flex items-center gap-3 justify-center text-white rounded-full px-5 py-1 text-[12px] ${selectedEpisodeCountState.some((t) => t.label === episodeAmount.label) ? 'bg-[#6a1b1b]' : 'bg-[#2b2b2b]'}`}
													onclick={() => {
														if (selectedEpisodeCountState.some((t) => t.label === episodeAmount.label)) {
															selectedEpisodeCountState = selectedEpisodeCountState.filter((t) => t.label !== episodeAmount.label);
														} else {
															selectedEpisodeCountState = [...selectedEpisodeCountState, episodeAmount];
														}
														filterAnime();
													}}
												>
													<span>{episodeAmount.label} ({episodeAmount.min}-{episodeAmount.max}odc.)</span>
													<span>
														{#if selectedEpisodeCountState.includes(episodeAmount)}
															<CheckOutline size="sm" />
														{:else}
															<PlusOutline size="sm" />
														{/if}
													</span>
												</button>
											{/each}
										</div>
									</div>
								</TabItem>
								<TabItem title="STATUS" class="w-full [&>button]:text-[13px] [&>button]:p-1 [&>button]:pb-3 [&>button]:w-full">
									<div class="w-full">
										<div class="flex flex-wrap gap-3">
											{#each statusOptions as status (status.label)}
												<button
													class={`relative flex items-center gap-3 justify-center text-white rounded-full px-5 py-1 text-[12px] ${selectedStatusState.some((s) => s.label === status.label) ? 'bg-[#6a1b1b]' : 'bg-[#2b2b2b]'}`}
													onclick={() => {
														if (selectedStatusState.some((s) => s.label === status.label)) {
															selectedStatusState = selectedStatusState.filter((s) => s.label !== status.label);
														} else {
															selectedStatusState = [...selectedStatusState, status];
														}
														filterAnime();
													}}
												>
													<span>{status.label}</span>
													<span>
														{#if selectedStatusState.includes(status)}
															<CheckOutline size="sm" />
														{:else}
															<PlusOutline size="sm" />
														{/if}
													</span>
												</button>
											{/each}
										</div>
									</div>
								</TabItem>
							</Tabs>
						</div>
					{/if}

					<div class="flex flex-col gap-5">
						<div class="flex gap-3 items-center text-white font-roboto">
							<div class="border-l-6 border-l-red-700 h-10"></div>
							<span class="text-4xl">SERIE</span>
						</div>
						<div class={`grid ${filteredAnime.length !== 0 ? `grid-cols-2 md:grid-cols-3 lg:grid-cols-4` : `grid-cols-1`} gap-5`}>
							{#if filteredAnime.length === 0}
								<div class="text-[#bdbdbd] font-roboto text-center text-2xl">Brak wyników</div>
							{/if}
							{#each filteredAnime as anime (anime.id.id.String)}
								<a class="bg-[#0E131CA6] mix-blend-plus-lighter p-2 sm:p-5 rounded-lg sm:rounded-2xl flex flex-col gap-3 max-h-[660px] hover:bg-[#0e1119] hover:scale-[101%] transition-all" href={`/anime/${anime.slug}`}>
									<img src={`/${anime.image}`} alt={anime.title} class="w-full max-h-[560px] h-full rounded-lg sm:rounded-2xl shadow-mainpage-anime-carousel" />
									<div class="w-full">
										<span class="text-white font-roboto text-sm lg:text-md xl:text-3xl line-clamp-1 text-center">
											{anime.title}
										</span>
									</div>
								</a>
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
