<script lang="ts">
	// @ts-ignore
	import { Select, Label, Input, Toggle, Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import type { NewsCategoryPublic, NewsPublic, UserModelSuperUser } from '$lib/types.ts';
	import { Dummy } from '$lib/types.ts';
	// @ts-ignore
	import { Upload } from '@lucide/svelte';

	import { FormatButtonGroup, TextEditor, AlignmentButtonGroup, BubbleMenu, ListButtonGroup, FontButtonGroup, CharacterCount, ToolbarRowWrapper, DetailsButtonGroup, Divider, ExportButtonGroup } from '@flowbite-svelte-plugins/texteditor';
	import type { Editor } from '@tiptap/core';
	import SaveButtons from '$lib/components/SaveButtons.svelte';

	let {
		isLoggedin,
		news: initialNewsData,
		save_fn,
		newsCategories: initialNewsCategories
	} = $props<{
		isLoggedin: UserModelSuperUser;
		news: NewsPublic;
		newsCategories: NewsCategoryPublic[];
		save_fn: (formData: FormData) => Promise<any>;
	}>();

	let news = $state<NewsPublic>(initialNewsData);
	let newsCategories = $state<NewsCategoryPublic[]>(initialNewsCategories);
	let editorInstance = $state<Editor | null>(null);

	function getEditorContent() {
		news.description = editorInstance?.getHTML() ?? '';
		console.log('Editor content:', news.description);
		return news.description;
	}

	function setEditorContent(content: string) {
		editorInstance?.commands.setContent(content);
	}

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

	function deleteNews(id: string) {
		console.log('deleteNews', id);
	}
</script>

<svelte:head>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/base16/google-dark.min.css" />
</svelte:head>

<div class="w-full h-full flex flex-col gap-4">
	<div class="p-4">
		<div class="grid grid-cols-2 gap-4">
			<div>
				<Label>
					Select category
					<Select
						class="mt-2"
						items={newsCategoriesOptions}
						bind:value={news.category.id.id.String}
						clearable
						required
						onchange={(e: Event) => {
							const value = (e.target as HTMLSelectElement).value;
							const foundCategory = newsCategories.find((category) => category.id.id.String === value);
							if (foundCategory) {
								news.category = foundCategory;
							}
						}}
					/>
				</Label>
				<div>
					<Label>
						News header
						<Input bind:value={news.text_header} required />
					</Label>
					<!-- hyperlink -->
					<Label>
						Hyperlink
						<Input bind:value={news.hyperlink} required />
					</Label>
					<div class="grid grid-cols-3 justify-between">
						<!-- pinned -->
						<Label>
							Color
							<Input type="color" bind:value={news.color} required />
						</Label>
						<Label>
							Pinned
							<Toggle
								color="teal"
								bind:checked={news.pinned}
								onchange={(e: Event) => {
									news.pinned = (e.target as HTMLInputElement).checked;
								}}
							/>
						</Label>
						<!-- color -->
						<!-- website -->
						<Label>
							Website
							<Toggle
								color="teal"
								bind:checked={news.website}
								onchange={(e: Event) => {
									news.website = (e.target as HTMLInputElement).checked;
								}}
							/>
						</Label>
					</div>
				</div>
			</div>
			<div>
				<div class="grid grid-cols-2 gap-4">
					<!-- image -->
					<div class="lg:col-span-1 p-4 rounded bg-[#1e2939]">
						<h2 class="font-bold mb-2 text-white">Cover Image</h2>
						<div class="flex flex-col items-center gap-4">
							<div class="relative rounded overflow-hidden">
								<!-- svelte-ignore a11y_img_redundant_alt -->
								<img src={isImageUploaded ? imagePreview : `/${news.image}`} alt="News image" class="w-full h-full" />
							</div>
							<label for="image-upload" class="cursor-pointer inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500">
								<Upload class="w-4 h-4 mr-2" />
								Upload Image
							</label>
							<input id="image-upload" type="file" accept="image/*" class="hidden" onchange={(e) => handleImageUpload(e, 'image')} />
						</div>
					</div>
					<!-- thumbnail -->
					<div class="lg:col-span-1 p-4 rounded bg-[#1e2939]">
						<h2 class="font-bold mb-2 text-white">Thumbnail</h2>
						<div class="flex flex-col items-center gap-4">
							<div class="relative rounded overflow-hidden">
								<!-- svelte-ignore a11y_img_redundant_alt -->
								<img src={isImageUploaded ? thumbnailPreview : `/${news.thumbnail}`} alt="News thumbnail" class="w-full h-full" />
							</div>
							<label for="thumbnail-upload" class="cursor-pointer inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500">
								<Upload class="w-4 h-4 mr-2" />
								Upload Image
							</label>
							<input id="thumbnail-upload" type="file" accept="image/*" class="hidden" onchange={(e) => handleImageUpload(e, 'thumbnail')} />
						</div>
					</div>
				</div>
			</div>
		</div>
		<div class="w-full h-full">
			<h2 class="font-bold mb-2 text-white">Description</h2>
			<TextEditor bind:editor={editorInstance} content={news.description} placeholder="Content of the news here..." contentprops={{ id: 'formats-ex' }}>
				<FormatButtonGroup editor={editorInstance} />
				<BubbleMenu editor={editorInstance} showStrike={false} showHighlight={false} />
				<FontButtonGroup editor={editorInstance} />

				<ToolbarRowWrapper toolbarrawprops={{ top: false }}>
					<DetailsButtonGroup editor={editorInstance} />
					<Divider />
					<ListButtonGroup editor={editorInstance} />
					<Divider />
					<AlignmentButtonGroup editor={editorInstance} />
					<Divider />
					<ExportButtonGroup editor={editorInstance} />
				</ToolbarRowWrapper>
				{#snippet footer()}
					{#if editorInstance}
						<CharacterCount editor={editorInstance} limit={2000} />
					{/if}
				{/snippet}
			</TextEditor>

			<div class="mt-4">
				<Button color="teal" onclick={() => getEditorContent()}>Save editor content</Button>
			</div>
		</div>
	</div>
	<SaveButtons
		action_name="News"
		save_url_path="/admin/management/news/add"
		save_and_continue_url_path={`/admin/management/news/${news.slug ? news.slug : 'add'}`}
		save_and_new_url_path="/admin/management/news/add"
		slug={news.slug}
		id={news.id.id.String}
		{formData}
		data={news}
		save_function={save_fn}
		{...isLoggedin.roles?.some((role: any) => role.permissions?.news?.delete === true) || isLoggedin.is_superuser ? { delete_function: deleteNews } : {}}
		can_read_logs={isLoggedin.roles?.some((role: any) => role.permissions?.logs?.read === true) || isLoggedin.is_superuser}
	/>
</div>
