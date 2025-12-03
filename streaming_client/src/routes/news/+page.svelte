<script lang="ts">
	// flowbite import avatar
	// @ts-ignore
	import { Avatar } from 'flowbite-svelte';
	import { getTimeAgo, stripHTMLWithoutDomParser } from '$lib/utils.ts';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import { page } from '$app/state';
	import Loading from '$lib/data/client/components/Loading.svelte';

	let isLoading = $state(true);
	let { data }: PageProps = $props();
	let currentNews = $state('');
	onMount(async () => {
		if (page.url.hash) {
			currentNews = page.url.hash.replace('#', '');
		} else {
			if (data.news && data.news.length > 0) {
				currentNews = data.news[0].slug;
			}
		}
		isLoading = false;
	});
</script>

<svelte:head>
	<title>teacup - {data.news && data.news.length > 0 ? data.news[0].text_header : 'News'}</title>
	<meta property="og:title" content="teacup - {data.news && data.news.length > 0 ? data.news[0].text_header : 'News'}" />
	<meta property="og:description" content={stripHTMLWithoutDomParser(data.news && data.news.length > 0 ? data.news[0].description.slice(0, 150) + '...' : 'Ile można się dowiedzieć?')} />
	<meta property="og:image" content={`https://teacup.pl/${data.news && data.news.length > 0 ? data.news[0].image : ''}`} />
	<meta property="og:url" content="https://teacup.pl/news" />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="teacup" />
	<meta property="og:locale" content="pl_PL" />
</svelte:head>

{#if isLoading}
	<Loading />
{:else}
	<div class="w-full h-full mt-20 mb-20">
		<div class="sm:mx-10 mx-3 flex flex-col gap-4">
			{#if data.news && data.news.length > 0}
				{#each data.news as newsItem}
				{#if newsItem.website}
					<div id={newsItem.slug} class="w-full flex justify-center items-center">
						{#if currentNews === newsItem.slug}
							<div class="w-full bg-[#0E131CA6] mix-blend-plus-lighter grid grid-cols-1 gap-4 p-5 md:p-13 rounded-2xl">
								<div class="w-full h-full flex justify-center items-center">
									<img src={newsItem.image} alt={newsItem.text_header} class=" object-cover" />
								</div>
								<div class="w-full h-full text-xl min-[450px]:text-3xl sm:text-4xl md:text-5xl items-center flex text-white font-roboto font-bold text-center justify-center">
									{#if newsItem.hyperlink || newsItem.hyperlink.length > 0}
										<a href={newsItem.hyperlink} target="_blank" class="cursor-pointer underline">
											<span>{newsItem.text_header}</span>
										</a>
									{:else}
										<span>{newsItem.text_header}</span>
									{/if}
								</div>
								<div class="text-white min-[450px]:text-lg sm:text-xl md:text-3xl">{@html newsItem.description}</div>

								<div class="w-full h-full grid grid-cols-2">
									<div class="">
										<div class="border-t-white border-t-1 mb-3 w-3/4"></div>
										<div class="flex items-center gap-2">
											<Avatar src={newsItem.author?.avatar} class=" w-12 h-12" />

											<div class="flex flex-col font-roboto">
												<span class="text-white text-xl">{newsItem.author?.name}</span>
												<span class="text-white text-[15px] font-light">{newsItem.date ? getTimeAgo(newsItem.date) : ''}</span>
											</div>
										</div>
									</div>
								</div>
							</div>
						{:else}
							<button class="w-full bg-[#0E131CA6] mix-blend-plus-lighter lg:h-[250px] grid lg:grid-cols-2 gap-4 p-4 rounded-2xl cursor-pointer" onclick={() => (currentNews = newsItem.slug)}>
								<div class="w-full h-full">
									<!-- <img src={newsItem.image} alt={newsItem.text_header} class="w-full h-full object-cover" /> -->
									<div style={`background-image: url(${newsItem.image}); background-size: cover; background-position: center;`} class="lg:block hidden w-full h-full rounded-2xl"></div>
									<img src={newsItem.image} alt={newsItem.text_header} class="lg:hidden block w-full h-full rounded-2xl object-cover" />
								</div>
								<div class="w-full h-full text-xl min-[450px]:text-3xl sm:text-4xl md:text-5xl items-center flex justify-center lg:justify-start text-white font-roboto font-bold">
									<span>{newsItem.text_header}</span>
								</div>
							</button>
						{/if}
					</div>
				{/if}
			{/each}
			{:else}
				<div class="w-full flex justify-center items-center">
					<span class="text-white text-xl">Brak wiadomości</span>
				</div>
			{/if}
		</div>
	</div>
{/if}
