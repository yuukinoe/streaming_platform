<script lang="ts">
	import type { PageProps } from './$types.js';
	import { onMount } from 'svelte';
	import type { NewsCategoryPublic, NewsPublic } from '$lib/types.ts';
	import { Dummy } from '$lib/types.ts';

	import type { Editor } from '@tiptap/core';
	import SaveAddNews from '$lib/components/SaveAddNews.svelte';
	import { addNews } from '$lib/data/admin/post.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let isLoading = $state(true);
	let editorInstance = $state<Editor | null>(null);

	function getEditorContent() {
		news.description = editorInstance?.getHTML() ?? '';
		return news.description;
	}

	function setEditorContent(content: string) {
		editorInstance?.commands.setContent(content);
	}

	let newsCategories = $state<NewsCategoryPublic[]>([]);
	let newsCategoriesOptions = $derived.by(() => {
		return newsCategories.map((category) => ({
			value: category.id.id.String,
			name: category.name
		}));
	});
	let formData = new FormData();
	let isImageUploaded = $state(false);
	let uploadedImageFile = $state<File | null>(null);
	let imagePreview = $state<string | null>(null);
	let thumbnailPreview = $state<string | null>(null);
	function handleImageUpload(event: Event, type: string) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			uploadedImageFile = file;

			if (formData.has(type)) {
				formData.delete(type);
			}
			formData.append(type, file);

			const reader = new FileReader();
			reader.onload = (e) => {
				if (type === 'image') {
					imagePreview = e.target?.result as string;
					news.image = imagePreview;
				} else {
					thumbnailPreview = e.target?.result as string;
					news.thumbnail = thumbnailPreview;
				}
				isImageUploaded = true;
			};
			reader.readAsDataURL(file);
		}
	}

	let news = $state<NewsPublic>({
		id: Dummy.IDWrapper(),
		text_header: '',
		description: '',
		category: Dummy.NewsCategoryPublic(),
		date: null,
		thumbnail: '',
		image: '',
		hyperlink: '',
		pinned: false,
		color: '',
		website: false,
		slug: '',
		discord_id: null,
		author: null
	});
	let { data }: PageProps = $props();

	function deleteNews(id: string) {
		console.log('deleteNews', id);
	}

	let clearableSelected = $state('');
	onMount(async () => {
		newsCategories = (data.newsCategories as NewsCategoryPublic[]) || [];
		isLoading = false;
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/base16/google-dark.min.css" />
</svelte:head>

{#if hasPermission(data.isLoggedIn.message, 'news', 'add')}
	<div class="w-full h-full">
		{#if isLoading}
			<Loading />
		{:else}
			<SaveAddNews isLoggedin={data.isLoggedIn.message} {news} {newsCategories} save_fn={addNews} />
		{/if}
	</div>
{:else}
	<InsufficientPermissions />
{/if}
