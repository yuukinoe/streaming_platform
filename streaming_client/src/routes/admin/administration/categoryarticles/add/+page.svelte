<script lang="ts">
	//@ts-ignore
	import { Input, Label, Helper, Toggle } from 'flowbite-svelte';
	import { Dummy, type NewsCategoryStaff } from '$lib/types.ts';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { addCategory } from '$lib/data/admin/post.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	import type { PageProps } from './$types.js';
	let dummy = Dummy.NewsCategoryStaff();
	let categoryData = $state(dummy) as NewsCategoryStaff;
	let { data }: PageProps = $props();
	let formData = new FormData();
</script>

{#if hasPermission(data.isLoggedIn.message, 'news_categories', 'add')}
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
			save_function={addCategory}
			delete_function={() => {}}
		/>
	</div>
{:else}
	<InsufficientPermissions />
{/if}
