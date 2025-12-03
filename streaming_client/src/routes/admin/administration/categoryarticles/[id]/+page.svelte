<script lang="ts">
	//@ts-ignore
	import { Input, Label, Helper, Toggle } from 'flowbite-svelte';
	import { Dummy, type NewsCategoryStaff } from '$lib/types.ts';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { patchNewsCategory } from '$lib/data/admin/patch.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let categoryData = $state() as NewsCategoryStaff;
	import { onMount } from 'svelte';
	import Loading from '$lib/data/client/components/Loading.svelte';
	let isLoading = $state(true);
	let { data } = $props();

	onMount(() => {
		if (data && Array.isArray(data.newsCategories)) {
			// Cast newsCategories to NewsCategoryStaff[] for correct typing
			const staffCategories = data.newsCategories as NewsCategoryStaff[];
			const found = staffCategories.find((category) => category?.id?.id?.String === data.id);
			if (found) {
				categoryData = found;
			}
		}
		isLoading = false;
	});

	let formData = new FormData();
</script>

{#if hasPermission(data.isLoggedIn.message, 'news_categories', 'edit')}
	{#if isLoading}
		<div class="w-full h-full flex justify-center items-center">
			<Loading />
		</div>
	{:else}
		<div class="w-full h-full">
			<h1 class="text-white">the ui for this is ugly af</h1>
			<div class="flex">
				<div class="w-full">
					<div class="w-full">
						<Label for="name" class="mb-2">Name</Label>
						<div class="flex">
							<Input type="text" id="name" onchange={(e: Event) => (categoryData.name = (e.target as HTMLInputElement).value)} placeholder="Name of the category" required />
							<div class="">
								<Label for="visible" class="mb-2">Visible</Label>
								<Toggle color="teal" checked={categoryData ? categoryData.visible : false} onchange={(e: Event) => (categoryData.visible = (e.target as HTMLInputElement).checked)} />
							</div>
						</div>
					</div>

					<div class="w-full">
						<Label for="discord_webhook" class="mb-2">Discord webhook</Label>
						<div class="flex">
							<Input type="text" id="discord_webhook" onchange={(e: Event) => (categoryData.discord_webhook = (e.target as HTMLInputElement).value)} placeholder="Discord webhook" required />
							<div class="">
								<Label for="allowed_everyone" class="text-nowrap">Allowed everyone</Label>
								<Toggle color="teal" checked={categoryData ? categoryData.allowed_everyone : false} onchange={(e: Event) => (categoryData.allowed_everyone = (e.target as HTMLInputElement).checked)} />
							</div>
						</div>
					</div>
				</div>
			</div>
			<SaveButtons
				action_name="Category"
				save_url_path={`/admin/administration/categoryarticles`}
				save_and_continue_url_path={`/admin/administration/categoryarticles/${categoryData ? categoryData.slug : ''}`}
				save_and_new_url_path="/admin/administration/categoryarticles/add"
				slug={categoryData ? categoryData.slug : ''}
				id={categoryData ? categoryData.id.id.String : ''}
				{formData}
				data={categoryData}
				save_function={patchNewsCategory}
				delete_function={() => {}}
			/>
		</div>
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
