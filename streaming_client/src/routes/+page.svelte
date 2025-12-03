<script lang="ts">
	// @ts-ignore
	import { Carousel, Indicators } from 'flowbite-svelte';
	// @ts-ignore
	import { Splide, SplideSlide } from '@splidejs/svelte-splide';
	import type { Anime, CarouselItem, Episodes, EpisodesPublic, NewsPublic, SSRData } from '$lib/data/client/types.ts';
	import { onMount } from 'svelte';
	import { getSeasonDates, getSeasonName, stripHTML } from '$lib/utils.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import NotFound from '$lib/data/client/components/404.svelte';
	import ServerError from '$lib/data/client/components/500.svelte';
	import type { PageProps } from './$types.js';
	import CaretCircleRight from 'phosphor-svelte/lib/CaretCircleRight';

	const { data }: PageProps = $props();
	let isLoading = $state(true);
	let is404 = $state(false);
	let is500 = $state(false);

	// add trafic

	onMount(async () => {
		try {
			isLoading = false;
		} catch (error) {
			console.error('Error fetching data:', error);
			if ((error instanceof Error && error.message.includes('500')) || (error instanceof Error && error.message.includes('Internal Server Error')) || (error instanceof Error && error.message.includes('Failed to fetch'))) {
				is500 = true;
			} else {
				is404 = true;
			}
		}
	});

	let splideRef: any = $state();

	const options = {
		type: 'loop',
		rewind: true,
		gap: '1rem',
		perPage: 4,
		perMove: 1,
		padding: '5rem',
		pagination: false,
		arrows: false,
		breakpoints: {
			1080: {
				perPage: 3,
				pagination: false
			},
			640: {
				perPage: 2,
				pagination: false,
				padding: '2rem'
			}
		}
	};

	function goPrev() {
		splideRef.go('<');
	}

	function goNext() {
		splideRef.go('>');
	}
</script>

<!-- meta discord head  -->

<svelte:head>
	<title>teacup</title>
	<meta property="og:title" content="teacup" />
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
		<div class="flex flex-col items-center gap-18 w-full [&>div]:w-full pb-20">
			<div>
				<Carousel class="!max-h-[200px] sm:!max-h-[300px] md:!max-h-[400px] lg:!max-h-[500px] xl:!max-h-[550px] !h-screen rounded-none relative" images={data.mainPage.newsImages}>
					{#snippet slide({ index, Slide }: { index: number; Slide: typeof Carousel.Slide })}
						<div class="transition-all">
							<Slide image={data.mainPage.newsImages[index]} />
						</div>
						<a
							class="absolute bottom-0 left-0 right-0 p-2 text-white text-sm w-full h-full hover:[&>div]:flex [&>div]:hidden [&>div]:transition-all hover:bg-[#000000bc] transition-all"
							href={`${data.mainPage.newsImages[index].item_type == 'episode' ? `/anime/${data.mainPage.newsImages[index].anime_slug}/episode/${data.mainPage.newsImages[index].episode_number}` : `/news#${data.mainPage.newsImages[index].anime_slug}`}`}
						>
							<div class="flex justify-center items-center h-full w-full transition-all">
								<div class="mx-[10%] w-full">
									<div></div>
									<div class="flex flex-col font-roboto gap-2 text-[#eeedf0c5]">
										<span class="sm:text-md md:text-xl lg:text-3xl xl:text-[50px] font-roboto font-bold">
											{data.mainPage.newsImages[index].anime_title}
										</span>
										<span class="line-clamp-2 sm:text-md md:text-lg lg:text-xl xl:text-[30px] font-light w-[60%]">
											{data.mainPage.newsImages[index].episode_description}
										</span>
									</div>
									<div></div>
								</div>
							</div>
						</a>
					{/snippet}
					<Indicators class="gap-5 sm:[&>button]:[&>div]:w-5 sm:[&>button]:[&>div]:h-5 sm:mb-5" />
					<!-- <Controls /> -->
				</Carousel>
			</div>
			<div class="flex flex-col">
				<div class="mx-3 sm:mx-10 flex flex-col gap-18">
					<div class="flex flex-col gap-8">
						<div class="flex gap-3 items-center text-white font-roboto">
							<div class="border-l-6 border-l-red-700 h-10"></div>
							<span class="text-[30px]">Najnowsze odcinki</span>
						</div>
						<div class="flex flex-col gap-2 sm:gap-3 md:gap-5 lg:gap-8">
							<div class="grid sm:grid-cols-2 gap-2 sm:gap-3 md:gap-5 lg:gap-8">
								<div class="flex flex-col gap-2 sm:gap-3 md:gap-5 lg:gap-8">
									<div class="h-full">
										<a class="flex flex-col h-full sm:h-full hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${data.mainPage.latestEpisodes[0].anime.slug}/episode/${data.mainPage.latestEpisodes[0].episode}`}>
											<div class="relative">
												<img src={data.mainPage.latestEpisodes[0]?.image ? `/${data.mainPage.latestEpisodes[0].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-4 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-base sm:text-lg md:text-xl line-clamp-1">
															{data.mainPage.latestEpisodes[0].episode}/{data.mainPage.latestEpisodes[0].anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#171f2d81] mix-blend-plus-lighter p-2 px-5 pb-1 rounded-b-2xl h-full flex">
												<div class="flex flex-col sm:mt-3 gap-1">
													<span class="text-white font-bold font-roboto text-2xl md:text-[28px] lg:text-[32px] line-clamp-1">
														{data.mainPage.latestEpisodes[0].title ? data.mainPage.latestEpisodes[0].title : data.mainPage.latestEpisodes[0].anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-sm lg:text-[15px] line-clamp-1 font-light">
														{data.mainPage.latestEpisodes[0].anime.title}
													</span>
												</div>
											</div>
										</a>
									</div>
								</div>
								<div class="grid grid-cols-2 gap-2 sm:gap-3 md:gap-5 lg:gap-8">
									<div class="flex flex-col gap-[6.5px] sm:gap-[9.5px] md:gap-[9px] lg:gap-[14px]">
										<a class="flex flex-col hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${data.mainPage.latestEpisodes[1].anime.slug}/episode/${data.mainPage.latestEpisodes[1].episode}`}>
											<div class="relative">
												<img src={data.mainPage.latestEpisodes[1]?.image ? `/${data.mainPage.latestEpisodes[1].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-xs lg:text-base line-clamp-1">
															{data.mainPage.latestEpisodes[1].episode}/{data.mainPage.latestEpisodes[1].anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#171f2d81] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
												<div class="flex flex-col gap-1">
													<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
														{data.mainPage.latestEpisodes[1].title ? data.mainPage.latestEpisodes[1].title : data.mainPage.latestEpisodes[1].anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
														{data.mainPage.latestEpisodes[1].anime.title}
													</span>
												</div>
											</div>
										</a>

										<a class="flex flex-col hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${data.mainPage.latestEpisodes[2].anime.slug}/episode/${data.mainPage.latestEpisodes[2].episode}`}>
											<div class="relative">
												<img src={data.mainPage.latestEpisodes[2]?.image ? `/${data.mainPage.latestEpisodes[2].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-xs lg:text-base line-clamp-1">
															{data.mainPage.latestEpisodes[2].episode}/{data.mainPage.latestEpisodes[2].anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#171f2d81] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
												<div class="flex flex-col gap-1">
													<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
														{data.mainPage.latestEpisodes[2].title ? data.mainPage.latestEpisodes[2].title : data.mainPage.latestEpisodes[2].anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
														{data.mainPage.latestEpisodes[2].anime.title}
													</span>
												</div>
											</div>
										</a>
									</div>
									<div class="flex flex-col gap-[6.5px] sm:gap-[9.5px] md:gap-[9px] lg:gap-[14px]">
										<a class="flex flex-col hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${data.mainPage.latestEpisodes[3].anime.slug}/episode/${data.mainPage.latestEpisodes[3].episode}`}>
											<div class="relative">
												<img src={data.mainPage.latestEpisodes[3]?.image ? `/${data.mainPage.latestEpisodes[3].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-xs lg:text-base line-clamp-1">
															{data.mainPage.latestEpisodes[3].episode}/{data.mainPage.latestEpisodes[3].anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#171f2d81] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
												<div class="flex flex-col gap-1">
													<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
														{data.mainPage.latestEpisodes[3].title ? data.mainPage.latestEpisodes[3].title : data.mainPage.latestEpisodes[3].anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
														{data.mainPage.latestEpisodes[3].anime.title}
													</span>
												</div>
											</div>
										</a>
										<a class="flex flex-col hover:[&>div]:[&>div]:transition-all hover:[&>div]:[&>div]:flex hover:[&>div]:[&>img]:opacity-50 [&>div]:[&>img]:transition-all hover:scale-101 transition-all" href={`/anime/${data.mainPage.latestEpisodes[4].anime.slug}/episode/${data.mainPage.latestEpisodes[4].episode}`}>
											<div class="relative">
												<img src={data.mainPage.latestEpisodes[4]?.image ? `/${data.mainPage.latestEpisodes[4].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
												<div class="absolute bottom-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mb-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-xs lg:text-base line-clamp-1">
															{data.mainPage.latestEpisodes[4].episode}/{data.mainPage.latestEpisodes[4].anime.episodes}
														</span>
													</div>
												</div>
												<div class="absolute top-0 left-0 h-full w-full justify-center items-center hidden">
													<CaretCircleRight color="white" size="70%" mirrored={false} />
												</div>
											</div>
											<div class="bg-[#171f2d81] mix-blend-plus-lighter p-2 px-5 pb-3 rounded-b-2xl">
												<div class="flex flex-col gap-1">
													<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
														{data.mainPage.latestEpisodes[4].title ? data.mainPage.latestEpisodes[4].title : data.mainPage.latestEpisodes[4].anime.title}
													</span>
													<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
														{data.mainPage.latestEpisodes[4].anime.title}
													</span>
												</div>
											</div>
										</a>
									</div>
								</div>
							</div>
							<!-- <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 sm:gap-3 md:gap-5 lg:gap-8">
								<a class="flex flex-col hover:scale-101 transition-all" href={`/anime/${latestEpisodes[5].anime.slug}/episode/${latestEpisodes[5].episode}`}>
									<img src={latestEpisodes[5]?.image ? `/${latestEpisodes[5].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
									<div class="bg-[#191e27] p-2 px-5 pb-1 rounde3-b-2xl">
										<div class="flex flex-col gap-1">
											<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
												{latestEpisodes[5].title ? latestEpisodes[5].title : latestEpisodes[5].anime.title}
											</span>
											<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
												Odcinek {latestEpisodes[5].episode}
											</span>
										</div>
									</div>
								</a>
								<a class="flex flex-col hover:scale-101 transition-all" href={`/anime/${latestEpisodes[6].anime.slug}/episode/${latestEpisodes[6].episode}`}>
									<img src={latestEpisodes[6]?.image ? `/${latestEpisodes[6].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
									<div class="bg-[#191e27] p-2 px-5 pb-1 rounde3-b-2xl">
										<div class="flex flex-col gap-1">
											<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
												{latestEpisodes[6].title ? latestEpisodes[6].title : latestEpisodes[6].anime.title}
											</span>
											<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
												Odcinek {latestEpisodes[6].episode}
											</span>
										</div>
									</div>
								</a>
								<a class="flex flex-col hover:scale-101 transition-all" href={`/anime/${latestEpisodes[7].anime.slug}/episode/${latestEpisodes[7].episode}`}>
									<img src={latestEpisodes[7]?.image ? `/${latestEpisodes[7].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
									<div class="bg-[#191e27] p-2 px-5 pb-1 rounde3-b-2xl">
										<div class="flex flex-col gap-1">
											<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
												{latestEpisodes[7].title ? latestEpisodes[7].title : latestEpisodes[7].anime.title}
											</span>
											<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
												Odcinek {latestEpisodes[7].episode}
											</span>
										</div>
									</div>
								</a>
								<a class="flex flex-col hover:scale-101 transition-all" href={`/anime/${latestEpisodes[8].anime.slug}/episode/${latestEpisodes[8].episode}`}>
									<img src={latestEpisodes[8]?.image ? `/${latestEpisodes[8].image}` : ''} alt="prop" class="rounded-t-md aspect-video h-full w-full" />
									<div class="bg-[#191e27] p-2 px-5 pb-1 rounde3-b-2xl">
										<div class="flex flex-col gap-1">
											<span class="text-white font-bold font-roboto text-sm xl:text-[16px] line-clamp-1">
												{latestEpisodes[8].title ? latestEpisodes[8].title : latestEpisodes[8].anime.title}
											</span>
											<span class="text-[#9f9f9f] font-roboto text-[10px] line-clamp-1 font-light">
												Odcinek {latestEpisodes[8].episode}
											</span>
										</div>
									</div>
								</a>
							</div> -->
						</div>
					</div>
					<div class="flex flex-col gap-8">
						<div class="flex gap-3 items-center text-white font-roboto">
							<div class="border-l-6 border-l-red-700 h-10"></div>
							<span class="text-[30px]">{getSeasonName()}</span>
						</div>
						<div class="relative">
							<Splide bind:this={splideRef} {options}>
								{#each data.mainPage.seasonAnimeList as anime (anime.id)}
									<SplideSlide>
										<a class="bg-[#171f2d81] mix-blend-plus-lighter h-full p-2 sm:p-5 rounded-lg sm:rounded-2xl flex flex-col gap-3 max-h-[660px] hover:bg-[#222b3981] transition-all" href={`/anime/${anime.slug}`}>
											<div class="relative">
												<div class="absolute top-0 right-0">
													<div class="bg-[#ad142d] p-1 px-3 rounded-2xl mt-2 mr-2">
														<span class="text-white font-roboto text-xs sm:text-sm md:text-base line-clamp-1">
															{anime.anime_type}
														</span>
													</div>
												</div>
												<img src={`/${anime.image}`} alt={anime.title} class="w-full max-h-[560px] h-full rounded-lg sm:rounded-2xl shadow-mainpage-anime-carousel" />
											</div>
											<div class="w-full flex flex-col gap-2">
												<span class="text-white font-roboto text-base sm:text-lg md:text-xl lg:text-2xl xl:text-3xl line-clamp-2">
													{anime.title}
												</span>
												<span class="text-[#BDBDBD] font-roboto font-light text-[10px] sm:text-xs xl:text-sm line-clamp-1">
													{anime.tags
														.slice(0, 3)
														.map((tag) => tag.name)
														.join(' / ')}
												</span>
											</div>
										</a>
									</SplideSlide>
								{/each}
							</Splide>
							<!-- <button onclick={goPrev} class="absolute left-0 top-1/2 -translate-y-1/2 active:!bg-[#742424] hover:bg-[#d9d9d94b] transition-all cursor-pointer text-white py-1 rounded-l-2xl z-10 h-full w-[10%]">
								<div class="flex justify-center items-center h-full w-full">
									<img src="/arrow.svg" alt="Previous" class="" />
								</div>
							</button>

							<button onclick={goNext} class="absolute right-0 top-1/2 -translate-y-1/2 active:!bg-[#742424] hover:bg-[#d9d9d94b] transition-all cursor-pointer text-white py-1 rounded-r-2xl z-10 h-full w-[10%]">
								<div class="flex justify-center items-center h-full w-full">
									<img src="/arrow.svg" alt="Next" class="rotate-180" />
								</div>
							</button> -->
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
