<script lang="ts">
	import type { PageProps } from './$types.js';
	import { onMount } from 'svelte';
	import type { NewsCategoryPublic, NewsPublic } from '$lib/types.ts';
	import { Dummy } from '$lib/types.ts';

	import type { Editor } from '@tiptap/core';
	import SaveAddNews from '$lib/components/SaveAddNews.svelte';
	import { patchNews } from '$lib/data/admin/patch.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let isLoading = $state(true);

	let { data }: PageProps = $props();

	let newsCategories = $state<NewsCategoryPublic[]>([]);
	let news = $state<NewsPublic>(Dummy.NewsPublic());
	function deleteNews(id: string) {
		console.log('deleteNews', id);
	}

	onMount(async () => {
		newsCategories = (data.newsCategories as NewsCategoryPublic[]) || [];
		let foundNews = data.news?.find((news: NewsPublic) => news.slug === data.slug);
		news = foundNews || Dummy.NewsPublic();
		isLoading = false;
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/base16/google-dark.min.css" />
</svelte:head>

{#if hasPermission(data.isLoggedIn.message, 'news', 'edit')}
	<div class="w-full h-full">
		{#if isLoading}
			<Loading />
		{:else}
			<SaveAddNews isLoggedin={data.isLoggedIn.message} {news} {newsCategories} save_fn={patchNews} />
		{/if}
	</div>
{:else}
	<InsufficientPermissions />
{/if}
