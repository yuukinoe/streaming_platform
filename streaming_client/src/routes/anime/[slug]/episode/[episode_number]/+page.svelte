<script lang="ts">
	// @ts-ignore
	import { Splide, SplideSlide } from '@splidejs/svelte-splide';
	// @ts-ignore
	import { Spinner } from 'flowbite-svelte';
	import type { PageProps } from './$types.ts';
	import type { EpisodesPublic } from '$lib/data/client/types.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import NotFound from '$lib/data/client/components/404.svelte';
	import ServerError from '$lib/data/client/components/500.svelte';
	import { getDomainName, servicesEmbedSupport } from '$lib/utils.ts';
	import { fade } from 'svelte/transition';
	import { onMount } from 'svelte';
	let { data }: PageProps = $props();
	let episodeData = $state() as EpisodesPublic;
	let episodesData = $state<EpisodesPublic[]>([]) as EpisodesPublic[];
	let isLoading = $state(true);
	let is404 = $state(false);
	let is500 = $state(false);
	let currentIframe = $state<string | null>(null);
	let iframeLoaded = $state(false);
	let splideRef: any = $state();
	let scrollY = $state(0);
	// let options = {
	// 	type: 'slide',
	// 	perPage: 3,
	// 	perMove: 1,
	// 	rewind: true,
	// 	padding: '15px',
	// 	gap: '5px',
	// 	arrows: false,
	// 	lazyLoad: 'nearby',
	// 	pagination: false,
	// 	start: Number(data.episode_number) - 1
	// };
	let options = $state({
		type: 'slide',
		perPage: 3,
		perMove: 1,
		rewind: true,
		padding: '15px',
		gap: '5px',
		arrows: false,
		lazyLoad: 'nearby',
		pagination: false,
		start: Number(data.episode_number) - 1
	});
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import type { Episodes } from '$lib/data/client/types.ts';

	beforeNavigate(() => {
		scrollY = window.scrollY;
	});

	afterNavigate(() => {
		loadEpisode(); // reload when URL changes
		window.scrollTo(0, scrollY);
	});

	function goToSecondSlide(slide: number) {
		splideRef?.splide?.go(slide); // numer slajdu zaczyna się od 0
	}

	async function loadEpisode() {
		isLoading = true;
		is404 = false;
		is500 = false;
		try {
			let episode = data.currentEpisode;

			if (episode) {
				if (data.isLoggedIn.status == 200) {
					episodeData = episode as Episodes;
				} else {
					episodeData = episode as EpisodesPublic;
				}
				// options.start = episodeData.episode - 1;

				goToSecondSlide(Number(data.episode_number) - 2);
			} else {
				is404 = true;
			}
			if (episodeData.video_players && episodeData.video_players.length > 0) {
				currentIframe = episodeData.video_players[0];
			} else {
				currentIframe = null;
			}
			let episodes = data.episodes.filter((e) => e.anime.slug === data.slug);
			if (episodes) {
				if (data.isLoggedIn.status == 200) {
					episodesData = episodes as Episodes[];
				} else {
					episodesData = episodes as EpisodesPublic[];
				}
			}
			episodesData.sort((a, b) => a.episode - b.episode);
			isLoading = false;
		} catch (error) {
			console.error('Error loading episode data:', error);
			if ((error instanceof Error && error.message.includes('500')) || (error instanceof Error && error.message.includes('Internal Server Error')) || (error instanceof Error && error.message.includes('Failed to fetch'))) {
				is500 = true;
			} else {
				is404 = true;
			}
		}
	}

	onMount(async () => {
		await loadEpisode();
	});
</script>

<svelte:head>
	<title>teacup - {data.currentEpisode?.anime?.title || 'Anime'} - Odcinek {data.currentEpisode?.episode || ''}</title>
	<meta property="og:title" content="teacup - {data.currentEpisode?.anime?.title || 'Anime'} - Odcinek {data.currentEpisode?.episode || ''}" />
	<meta property="og:description" content={data.currentEpisode?.description || 'Zgaduje, że to naprawde fajne anime.'} />
	<meta property="og:url" content="https://teacup.pl/anime/{data.currentEpisode?.anime?.slug || data?.slug || ''}/episode/{data.episode_number}" />
	<meta property="og:image" content={`https://teacup.pl/${data.currentEpisode?.image || ''}`} />
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
		<div class="flex flex-col gap-10 [&>div]:[&>div]:mx-3 [&>div]:[&>div]:sm:mx-10 [&>div]:[&>div]:flex [&>div]:[&>div]:flex-col [&>div]:[&>div]:gap-5 [&>div]:[&>div]:sm:gap-15 mb-20">
			<div class="bg-[#2c374b5e] mix-blend-plus-lighter h-15 sm:flex items-center hidden">
				<div>
					<div class="text-[#EEEDF0B2] font-roboto font-[400] flex gap-1">
						<div class="text-[20px] text-[#828282]">Anime</div>
						<div class="text-[20px] text-[#828282]">/</div>
						<div class="text-[20px] text-[#828282]">
							<a href={`/anime/${episodeData.anime.slug}`}>
								{episodeData.anime.title}
							</a>
						</div>
						<div class="text-[20px] text-[#828282]">/</div>
						<div class="text-[20px]">Odcinek {episodeData.episode}</div>
					</div>
				</div>
			</div>
			<div>
				<div>
					<div class="player flex justify-center flex-col items-center w-full gap-5">
						<div class="iframe-container rounded-3xl aspect-video w-full relative">
							<div in:fade out:fade class="w-full h-full relative overflow-hidden [&>*:nth-child(2)]:transition-all hover:[&>*:nth-child(2)]:blur-none [&>*:nth-child(2)]:blur-sm rounded-3xl">
								{#if !iframeLoaded}
									<div class={`absolute w-full h-full z-999  ${iframeLoaded ? 'hidden' : 'block'}`}>
										<div class="flex justify-center items-center w-full h-full">
											<div class="flex flex-col justify-center items-center gap-3">
												<div class="relative text-center">
													<Spinner class="w-[10vw]" size="xl" color="red" />
												</div>
											</div>
										</div>
									</div>

									<img class={`w-full h-full rounded-3xl opacity-50 absolute ${iframeLoaded ? 'hidden' : 'block'}`} src={`/${episodeData.image}`} alt={episodeData.title} />
								{/if}
								<iframe allowfullscreen in:fade out:fade class={`w-full h-full rounded-3xl ${iframeLoaded ? 'block' : 'hidden'}`} onload={() => (iframeLoaded = true)} title={episodeData.title} src={currentIframe ? servicesEmbedSupport(currentIframe) : ''}> </iframe>
							</div>
						</div>
						<div class="available-players w-full flex items-center justify-center text-[#EEEDF0B2] gap-2 sm:gap-5 font-roboto font-[400] relative h-[calc(3.5vw+1vw)] lg:h-[36px]">
							{#each episodeData.video_players ?? [] as player, index (index)}
								<button
									class={`player-button select-none transition-all ${currentIframe === player ? 'text-[calc(3.5vw+1vw)] lg:text-4xl text-[#EEEDF0]' : 'text-[3.5vw] lg:text-2xl'}`}
									onclick={() => {
										currentIframe = player;
										iframeLoaded = false;
									}}
									class:active={currentIframe === player}
								>
									{getDomainName(player)}
								</button>
							{/each}
						</div>
					</div>
					<div class="flex justify-between gap-10">
						<div class="bg-[#2c374b5e] mix-blend-plus-lighter p-2 sm:p-5 rounded-lg sm:rounded-2xl hidden md:flex flex-col gap-5 w-3/10 max-w-[368px]">
							<a href={`/anime/${episodeData.anime.slug}`}>
								<img src={`/${episodeData.anime.image}`} alt={episodeData.anime.title} class="w-full rounded-lg sm:rounded-2xl shadow-mainpage-anime-carousel aspect-[13/19]" />
							</a>
							<div class="w-full">
								<span class="text-[#EEEDF0] font-roboto text-lg lg:text-xl xl:text-2xl text-left font-semibold">
									{episodeData.anime.title}
									<br />
									<span class="text-[#EEEDF0CC] font-[400]">
										{episodeData.anime.alternative_title}
									</span>
								</span>
							</div>
						</div>
						<div class="flex flex-col md:w-7/10 gap-5">
							<div class="bg-[#2c374b5e] mix-blend-plus-lighter w-full p-8 rounded-2xl font-roboto font-[400] text-[#EEEDF0] md:text-xl lg:text-2xl xl:text-3xl">
								<div class={`flex flex-col ${episodeData.uploaders && episodeData.typesetters ? '' : 'gap-2 md:gap-10'} ${!episodeData.uploaders && episodeData.typesetters ? '' : 'gap-2 md:gap-5'} ${episodeData.uploaders && !episodeData.typesetters ? '' : 'gap-2 md:gap-5'}`}>
									<div class="flex justify-between">
										<div class="flex flex-col gap-5">
											<span>Numer odcinka: {episodeData.episode}</span>
											<div class="flex flex-col">
												<span>Tytuł odcinka: </span>
												<span class="font-[300] text-sm md:text-base lg:text-lg font-roboto text-[#EEEDF0CC]">{episodeData.title ? episodeData.title : 'Brak danych'}</span>
											</div>
										</div>
										<div class="flex flex-col md:mr-10">
											<span>Autorzy odcinka:</span>
											<div class="flex flex-col text-sm md:text-base lg:text-lg">
												<span class="font-roboto font-[400] text-[#EEEDF0CC]"
													>Tłumacz: <span class="font-[300]">
														{#if episodeData.translators && episodeData.translators.length > 0}
															{#each episodeData.translators as translator, index}
																<a href="/about/user/{translator.slug}" class="hover:text-white transition-colors">{translator.name}</a>{index < episodeData.translators.length - 1 ? ', ' : ''}
															{/each}
														{:else}
															Brak danych
														{/if}
													</span>
												</span>
												<span class="font-roboto font-[400] text-[#EEEDF0CC]"
													>Korektor: <span class="font-[300]">
														{#if episodeData.proofreaders && episodeData.proofreaders.length > 0}
															{#each episodeData.proofreaders as proofreader, index}
																<a href="/about/user/{proofreader.slug}" class="hover:text-white transition-colors">{proofreader.name}</a>{index < episodeData.proofreaders.length - 1 ? ', ' : ''}
															{/each}
														{:else}
															Brak danych
														{/if}
													</span>
												</span>
												{#if episodeData.uploaders}
													<span class="font-roboto font-[400] text-[#EEEDF0CC]"
														>Uploader: <span class="font-[300]">
															{#if episodeData.uploaders && episodeData.uploaders.length > 0}
																{#each episodeData.uploaders as uploader, index}
																	<a href="/about/user/{uploader.slug}" class="hover:text-white transition-colors">{uploader.name}</a>{index < episodeData.uploaders.length - 1 ? ', ' : ''}
																{/each}
															{:else}
																Brak danych
															{/if}
														</span>
													</span>
												{/if}
												{#if episodeData.typesetters}
													<span class="font-roboto font-[400] text-[#EEEDF0CC]"
														>Typesetter: <span class="font-[300]">
															{#if episodeData.typesetters && episodeData.typesetters.length > 0}
																{#each episodeData.typesetters as typesetter, index}
																	<a href="/about/user/{typesetter.slug}" class="hover:text-white transition-colors">{typesetter.name}</a>{index < episodeData.typesetters.length - 1 ? ', ' : ''}
																{/each}
															{:else}
																Brak danych
															{/if}
														</span>
													</span>
												{/if}
											</div>
										</div>
									</div>

									<div class="flex flex-col">
										<span>Opis odcinka:</span>

										<span class="font-[300] text-sm md:text-base lg:text-lg font-roboto text-[#EEEDF0CC]">
											{episodeData.description ? episodeData.description : 'Brak opisu.'}
										</span>
									</div>
								</div>
							</div>
							<div class="anime-mobile-card-episode md:hidden">
								<div class="flex justify-center items-center gap-3 px-5">
									<div class="aspect-[2/3]">
										<a href={`/anime/${episodeData.anime.slug}`}>
											<img src={`/${episodeData.anime.image}`} alt={episodeData.anime.title} class="max-w-[100px] max-h-[150px] rounded-lg sm:rounded-2xl aspect-[13/19]" />
										</a>
									</div>
									<div class="flex flex-col overflow-hidden text-ellipsis">
										<span class="text-[#EEEDF0] font-roboto text-lg lg:text-xl xl:text-2xl">
											{episodeData.anime.title}
										</span>
										<span class="text-[#EEEDF0CC] font-roboto text-sm md:text-base lg:text-lg">
											{episodeData.anime.alternative_title}
										</span>
									</div>
								</div>
							</div>
							<div class="w-full">
								<div class="flex flex-col w-full gap-1 md:gap-3">
									<div>
										<span class="text-[#EEEDF0] font-roboto font-bold md:text-xl lg:text-2xl xl:text-3xl"> Następne odcinki: </span>
									</div>
									<div class="relative md:block hidden">
										<!-- 355x243 -->
										<Splide bind:this={splideRef} {options}>
											{#each episodesData as episode (episode.id.id.String)}
												<SplideSlide>
													<a class="max-w-[355px]" href={`/anime/${episode.anime.slug}/episode/${episode.episode}`}>
														<div class="flex flex-col">
															<img src={episode?.image ? `/${episode.image}` : ''} alt="prop" class="rounded-t-md aspect-video" />
															<div class="bg-[#2c374b5e] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
																<div class="flex flex-col">
																	<div>
																		<span class="text-white font-roboto text-sm xl:text-base line-clamp-1 font-semibold max-w-[315px] text-ellipsis">
																			{episode.title ? episode.title : episode.anime.title}
																		</span>
																	</div>
																	<span class="text-white font-roboto text-sm xl:text-base line-clamp-1 font-light">
																		Odcinek {episode.episode}
																	</span>
																</div>
															</div>
														</div>
													</a>
												</SplideSlide>
											{/each}
										</Splide>
									</div>
									<div class="relative block md:hidden">
										<div class="flex flex-col gap-3 mt-2 text-[#EEEDF0]">
											{#each episodesData as episode (episode.id.id.String)}
												<a class="bg-[#2c374b5e] mix-blend-plus-lighter p-2 w-full rounded-lg" href={`/anime/${episode.anime.slug}/episode/${episode.episode}`}>
													<div class="flex gap-3">
														<div>
															<img src={`/${episode.image}`} alt="Miniaturka" class="aspect-video h-[85px] w-[140px] min-h-[85px] min-w-[140px] sm:h-30 sm:w-[213px] sm:min-h-30 sm:min-w-[213px] rounded-lg shadow-md" />
														</div>
														<div class="flex flex-col gap-1 md:gap-3 justify-between">
															<div>
																<span class="text-[13px] sm:text-lg font-roboto overflow-hidden text-ellipsis line-clamp-1"
																	><span class="font-semibold">
																		Odc.{episode.episode}
																	</span>
																	- {episode.title ? episode.title : episode.anime.title}</span
																>
															</div>
															<div class="text-[12px] sm:text-base text-ellipsis">
																<div class="line-clamp-1">
																	<span> Tłumaczenie: </span>
																	<span class="text-[#EEEDF0CC]">{episode.translators ? episode.translators.map((translator) => translator.name).join(', ') : 'Brak danych'}</span>
																	<span> | </span>
																	<span> Korekta: </span>
																	<span class="text-[#EEEDF0CC]">{episode.proofreaders ? episode.proofreaders.map((proofreader) => proofreader.name).join(', ') : 'Brak danych'}</span>
																</div>
																{#if episode.uploaders || episode.typesetters}
																	<div class="line-clamp-1">
																		{#if episode.uploaders}
																			<span> Upload: </span>
																			<span class="text-[#EEEDF0CC]">{episode.uploaders.map((uploader) => uploader.name).join(', ')}</span>
																		{/if}
																		{#if episode.uploaders && episode.typesetters}
																			<span> | </span>
																		{/if}
																		{#if episode.typesetters}
																			<span> Typesetting: </span>
																			<span class="text-[#EEEDF0CC]">{episode.typesetters.map((typesetter) => typesetter.name).join(', ')}</span>
																		{/if}
																	</div>
																{/if}
															</div>
															<div>
																<div class="font-roboto text-[#EEEDF0CC] sm:text-base text-[12px] line-clamp-1">
																	<span class="font-roboto text-[#EEEDF0CC] line-clamp-1">
																		{#if episode.description.length > 0}
																			<span class="text-white"> Opis:</span> {episode.description}
																		{:else}
																			Brak opisu
																		{/if}
																	</span>
																</div>
															</div>
														</div>
													</div>
												</a>
											{/each}
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
