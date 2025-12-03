<script lang="ts">
	// @ts-ignore
	import { Tabs, TabItem, Range, Button } from 'flowbite-svelte';

	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import type { Anime, Episodes } from '$lib/data/client/types.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import { getSeasonFromDate } from '$lib/utils.ts';
	import { fade, slide } from 'svelte/transition';
	import type { EpisodesPublic } from '$lib/data/client/types.ts';
	import NotFound from '$lib/data/client/components/404.svelte';
	import ServerError from '$lib/data/client/components/500.svelte';
	import { updateAnimeBackgroundPosition } from '$lib/data/client/post.ts';
	import CaretCircleRight from 'phosphor-svelte/lib/CaretCircleRight';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
	let isLoading = $state(true);

	let showFullDescription = $state(false);
	let descriptionLimit = 100;

	let is404 = $state(false);
	let is500 = $state(false);
	let backgroundPosition = $state(data.currentAnime?.background_position || 7);
	const statusTranslations: Record<string, string> = {
		'Finished Airing': 'ZAKOŃCZONE',
		'Currently Airing': 'W TRAKCIE',
		'Not yet aired': 'NIE WYEMITOWANO'
	};

	onMount(async () => {
		try {
			if (!data || !data.slug) {
				is404 = true;
			}

			if (!data.currentAnime) {
				is404 = true;
				throw new Error('Anime not found');
			}

			isLoading = false;
		} catch (error) {
			console.error('Error loading data:', error);
			if ((error instanceof Error && error.message.includes('500')) || (error instanceof Error && error.message.includes('Internal Server Error')) || (error instanceof Error && error.message.includes('Failed to fetch'))) {
				is500 = true;
			} else {
				is404 = true;
			}
		}
	});
</script>

<svelte:head>
	<title>teacup - {data.currentAnime?.title || 'Anime'}</title>
	<meta property="og:title" content="teacup - {data.currentAnime?.title || 'Anime'}" />
	<meta property="og:description" content={data.currentAnime?.description || 'Zgaduje, że to naprawde fajne anime.'} />
	<meta property="og:url" content="https://teacup.pl/anime/{data.currentAnime?.slug || data?.slug || ''}" />
	<meta property="og:image" content={`https://teacup.pl/${data.currentAnime?.image || ''}`} />
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
	{:else if data.currentAnime}
		<div class="flex flex-col gap-10 [&>div]:[&>div]:mx-3 [&>div]:[&>div]:sm:mx-10 [&>div]:[&>div]:flex [&>div]:[&>div]:flex-col [&>div]:[&>div]:gap-8">
			<div class="bg-[#2c374b5e] mix-blend-plus-lighter h-15 sm:flex items-center hidden w-full">
				<div class="w-full">
					<div class="flex justify-between w-full">
						<div class="text-[#EEEDF0B2] font-roboto font-[400] flex gap-1 line-clamp-1">
							<div class="text-[20px]">Anime</div>
							<div class="text-[20px]">/</div>
							<div class="text-[20px] line-clamp-1">{data.currentAnime?.title}</div>
						</div>
						<div>
							{#if hasPermission(data.isLoggedIn.message, 'anime', 'edit')}
								<Button class="justify-self-end w-10 h-6 rounded-none " onclick={() => updateAnimeBackgroundPosition(data.currentAnime?.id.id.String || '', backgroundPosition)}>Set</Button>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="w-full" transition:fade>
			<div class="relative h-75 md:h-150 [&>div]:h-75 [&>div]:md:h-150 bg-cover [&>input]:z-999 [&>input]:relative [&>button]:z-999 [&>button]:relative [&>input]:input-vertical [&>input]:w-full" style={`background-image: url('/${data.currentAnime?.image.replace('\\', '/')}'); background-position-y: ${backgroundPosition}%;`}>
				{#if hasPermission(data.isLoggedIn.message, 'anime', 'edit')}
					<div class="flex gap-2 relative justify-end [&>input]:input-vertical">
						<input type="range" class="stylized-input rotate-180 relative z-999" bind:value={backgroundPosition} />
					</div>
				{/if}
				<div class="absolute inset-0 bg-gradient-to-b from-black/60 to-transparent shadow-anime-bg"></div>
			</div>

			<div class="">
				<div class="relative text-white translate-y-[-120px] p-4 pt-20 rounded-b-xl mx-3 sm:mx-10 flex flex-col gap-8">
					<div class="relative left-4 w-full">
						<div class="flex flex-row-reverse md:flex-row gap-3 sm:gap-10 w-full">
							<img src={`/${data.currentAnime?.image}`} alt="Miniaturka" class="w-full h-full max-w-30 max-h-50 min-[405px]:max-w-35 min-[405px]:max-h-55 min-[475px]:max-w-40 min-[475px]:max-h-60 sm:max-w-60 sm:max-h-80 md:max-w-80 md:max-h-120 rounded-lg shadow-md z-20" />
							<div class="flex flex-col mt-7 md:mt-20 md:my-20 justify-start text-right md:text-left w-full">
								<span class="text-xl sm:text-2xl md:text-2xl lg:text-4xl font-roboto mb-2 break-words">{data.currentAnime?.title}</span>
								<div class="flex gap-2 text-[12px] min-[475px]:text-sm sm:text-lg lg:text-2xl text-nowrap flex-row-reverse md:flex-row">
									<span class=" text-[#EEEDF0B2]">{getSeasonFromDate(data.currentAnime?.aired)}</span>
									<span class=" text-[#fff] translate-y-[-1px]">|</span>
									{#if data.currentAnime?.tags && data.currentAnime?.tags.length}
										{#each data.currentAnime?.tags.slice(0, 3) as tag, index}
											<a href={`/anime#${tag.slug}`} class="text-[#EEEDF0B2]">{tag.name}{index < 2 ? ',' : ''}</a>
										{/each}
									{/if}
								</div>
								<div class="bg-[#2c374b5e] mix-blend-plus-lighter px-7 py-5 rounded-lg mt-4 w-full text-md text-lg sm:block hidden" transition:fade>
									<div class="flex flex-col gap-3">
										<div class="flex flex-col gap-1">
											<!-- <span class="font-roboto">TYTUŁY:</span> -->
											<span class="font-roboto font-bold">Tytuły:</span>
											<span class="font-roboto text-[#EEEDF0CC]">
												{data.currentAnime?.title}
												<br />
												{data.currentAnime?.alternative_title}
											</span>
										</div>
										<div class="flex flex-col gap-1">
											<span class="font-roboto font-bold">Opis serii:</span>
											<span class="font-roboto text-[#EEEDF0CC]">
												{#if data.currentAnime?.description && data.currentAnime?.description.length > descriptionLimit}
													{#if !showFullDescription}
														{data.currentAnime?.description.slice(0, descriptionLimit)}...
														<button class="text-gray-600 underline" onclick={() => (showFullDescription = !showFullDescription)}>{showFullDescription ? '(Pokaż mniej)' : '(Pokaż więcej)'}</button>
													{:else}
														{data.currentAnime?.description}
														<button class="text-gray-600 underline text-right" onclick={() => (showFullDescription = !showFullDescription)}>{showFullDescription ? '(Pokaż mniej)' : '(Pokaż więcej)'}</button>
													{/if}
												{:else}
													{data.currentAnime?.description ? data.currentAnime?.description : 'Brak opisu dla tej serii.'}
												{/if}
											</span>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
					<div class="bg-[#2c374b5e] mix-blend-plus-lighter px-7 py-5 rounded-lg mt-4 w-full text-md text-lg sm:hidden block" transition:fade>
						<div class="flex flex-col gap-3">
							<div class="flex flex-col gap-1">
								<span class="font-roboto font-bold">Tytuły:</span>
								<span class="font-roboto text-[#EEEDF0CC]">
									{data.currentAnime?.title}
									<br />
									{data.currentAnime?.alternative_title}
								</span>
							</div>
							<div class="flex flex-col gap-1">
								<span class="font-roboto font-bold">Opis serii:</span>
								<span class="font-roboto text-[#EEEDF0CC]">
									{#if data.currentAnime?.description && data.currentAnime?.description.length > descriptionLimit}
										{#if !showFullDescription}
											{data.currentAnime?.description.slice(0, descriptionLimit)}...
											<button class="text-gray-600 underline" onclick={() => (showFullDescription = !showFullDescription)}>{showFullDescription ? '(Pokaż mniej)' : '(Pokaż więcej)'}</button>
										{:else}
											{data.currentAnime?.description}
											<button class="text-gray-600 underline text-right" onclick={() => (showFullDescription = !showFullDescription)}>{showFullDescription ? '(Pokaż mniej)' : '(Pokaż więcej)'}</button>
										{/if}
									{:else}
										{data.currentAnime?.description ? data.currentAnime?.description : 'Brak opisu dla tej serii.'}
									{/if}
								</span>
							</div>
						</div>
					</div>
					<div class="[&>div:first-of-type]:!bg-[#2a2a2a] [&>div]:!bg-[#1f1f1f00] [&>div]:p-0 [&>div]:rounded-none [&>div]:mt-0 [&>div:first-of-type]:h-[2px]" transition:slide>
						<Tabs tabStyle="underline">
							<TabItem open title="Odcinki" class="w-full text-nowrap [&>button]:w-full [&>button]:p-1 [&>button]:pb-3 sm:[&>button]:p-4 [&>button]:bg-[#2c374b5e] hover:[&>button]:bg-[#2c374b5e] [&>button]:mix-blend-plus-lighter">
								<!-- <div class="flex flex-col gap-3 mt-2"> -->
								<div class="sm:grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 sm:gap-5 flex flex-col gap-3 mt-2">
									{#if data.episodesForAnime.length === 0}
										<div class="col-span-5 text-center text-gray-500">Brak odcinków dla tej serii.</div>
									{/if}
									{#each data.episodesForAnime as episode (episode.id.id.String)}
										<a class="bg-[#2c374b5e] mix-blend-plus-lighter p-2 w-full rounded-lg cursor-pointer sm:hidden block" href={`/anime/${episode.anime.slug}/episode/${episode.episode}`}>
											<div class="flex gap-3">
												<div>
													<img src={`/${episode.image}`} alt="Miniaturka" class="aspect-video h-[85px] w-[140px] min-h-[85px] min-w-[140px] sm:h-30 sm:w-[213px] sm:min-h-30 sm:min-w-[213px] rounded-lg shadow-md" />
												</div>
												<div class="flex flex-col gap-1 md:gap-3 justify-center align-center font-roboto">
													<!-- <div></div> -->
													<span class="text-white font-bold line-clamp-2">{episode.title ? episode.title : episode.anime.title}</span>
													<span class="text-[#BDBDBD] text-xs">Odcinek {episode.episode}</span>
													<!-- <div>
														<span class="text-[13px] sm:text-lg font-roboto overflow-hidden text-ellipsis line-clamp-1"
															><span class="font-semibold">
																Odc.{episode.episode}
															</span>
															- {episode.title}</span
														>
													</div>
													<div class="text-[12px] sm:text-base text-ellipsis line-clamp-1">
														<div class="line-clamp-1 gap-1">
															<span> Tłumaczenie: </span>
															<span class="text-[#EEEDF0CC] "> {episode.translators ? episode.translators.map((t) => t.name).join(', ') : 'Brak danych'} </span>
															<span> | </span>
															<span> Korekta: </span>
															<span class="text-[#EEEDF0CC]">{episode.proofreaders ? episode.proofreaders.map((p) => p.name).join(', ') : 'Brak danych'}</span>
														</div>
														{#if episode.uploaders || episode.typesetters}
															<div class="line-clamp-1 gap-1 ">
																{#if episode.uploaders}
																	<span> Upload: </span>
																	<span class="text-[#EEEDF0CC]">{episode.uploaders.map((u) => u.name).join(', ')}</span>
																{/if}
																{#if episode.uploaders && episode.typesetters}
																	<span> | </span>
																{/if}
																{#if episode.typesetters}
																	<span> Typesetting: </span>
																	<span class="text-[#EEEDF0CC]">{episode.typesetters.map((t) => t.name).join(', ')}</span>
																{/if}
															</div>
														{/if}
													</div>
													<div class="line-clamp-1">
														<div class="font-roboto text-[#EEEDF0CC] sm:text-base text-[12px] line-clamp-1">
															<span class="font-roboto text-[#EEEDF0CC] line-clamp-1">
																{#if episode.description.length > 0}
																	<span class="text-white">
																		Opis:</span> {episode.description}
																{:else}
																	Brak opisu
																{/if}
															</span>
														</div>
													</div> -->
												</div>
											</div>
										</a>

										<a class="hidden sm:flex flex-col hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${episode.anime.slug}/episode/${episode.episode}`}>
											<div class="relative">
												<img src={episode?.image ? `/${episode.image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-xs lg:text-base line-clamp-1">
															{episode.episode}/{episode.anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#2c374b5e] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
												<div class="flex flex-col gap-1">
													<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
														{episode.title ? episode.title : episode.anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
														{episode.anime.title}
													</span>
												</div>
											</div>
										</a>
									{/each}
								</div>
							</TabItem>
							<!-- <TabItem title="POWIĄZANE SERIE" class="w-full text-nowrap [&>button]:w-full [&>button]:p-1 [&>button]:pb-3 sm:[&>button]:p-4"
							></TabItem> -->
							<TabItem title="Szczegóły" class="w-full text-nowrap [&>button]:w-full [&>button]:p-1 [&>button]:pb-3 sm:[&>button]:p-4 [&>button]:bg-[#2c374b5e] hover:[&>button]:bg-[#2c374b5e] [&>button]:mix-blend-plus-lighter">
								<div class="grid grid-cols-2 md:grid-cols-3 mt-5 gap-5 text-nowrap">
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">TYP</span>
										<span class="text-[#EEEDF0CC]">{data.currentAnime?.anime_type}</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">LICZBA ODCINKÓW</span>
										<span class="text-[#EEEDF0CC]">{data.currentAnime?.episodes}</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">DŁUGOŚĆ ODCINKA</span>
										<span class="text-[#EEEDF0CC]">{data.episodesForAnime[0]?.length || 'Brak informacji'}</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">STATUS</span>
										<span class="text-[#EEEDF0CC]">
											{statusTranslations[data.currentAnime?.status] || data.currentAnime?.status.toLocaleUpperCase()}
										</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">SEZON</span>
										<span class="text-[#EEEDF0CC]">{getSeasonFromDate(data.currentAnime?.aired).toLocaleUpperCase()}</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">DATA EMISJI</span>
										<span class="text-[#EEEDF0CC]">{new Date(data.currentAnime?.aired).getDate()}.{new Date(data.currentAnime?.aired).getMonth()}.{new Date(data.currentAnime?.aired).getFullYear()}</span>
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">GATUNEK</span>
										{#if data.currentAnime?.tags && data.currentAnime?.tags.length}
											<span class="text-[#EEEDF0CC]">
												{data.currentAnime?.tags.map((tag) => tag.name).join(', ')}
											</span>
										{/if}
									</div>
									<div class="flex flex-col gap-1 font-roboto justify-center text-center items-center">
										<span class="font-bold">STUDIO</span>
										{#if data.currentAnime?.studio && data.currentAnime?.studio.length}
											<span class="text-[#EEEDF0CC]">
												{data.currentAnime?.studio.map((studio) => studio.name).join(', ')}
											</span>
										{/if}
									</div>
								</div>
							</TabItem>
						</Tabs>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<NotFound />
	{/if}
</div>
