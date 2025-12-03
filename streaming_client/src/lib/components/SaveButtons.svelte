<script lang="ts">
	// @ts-ignore
	import { Button, Footer, FooterLinkGroup, Toast, Modal } from 'flowbite-svelte';
	import { CloseCircleSolid, FireOutline } from 'flowbite-svelte-icons';
	import { goto, invalidateAll } from '$app/navigation';

	const { action_name, id, webhook_func, save_url_path, save_and_continue_url_path, save_and_new_url_path, save_function, delete_function, can_read_logs, formData, slug, data } = $props<{
		action_name: string;
		id?: string;
		webhook_func?: (id: string) => Promise<any>;
		save_url_path: string;
		save_and_continue_url_path: string;
		save_and_new_url_path: string;
		save_function: (formData: FormData) => Promise<any>;
		delete_function?: (id: string) => void;
		can_read_logs?: boolean;
		formData: FormData;
		slug: string;
		data: any;
	}>();
	let message_error = $state('');
	let success = $state(false);
	let failed = $state(false);

	let defaultModal = $state(false);
	function handleAppendingFormData() {
		// Clear any existing 'data' field to avoid duplicates
		if (formData.has('data')) {
			formData.delete('data');
		}
		formData.append('data', JSON.stringify(data));

		// Debug FormData contents
		// console.log('=== FormData Debug ===');
		// console.log('FormData keys and values:');
		// for (let [key, value] of formData.entries()) {
		// 	if (value instanceof File) {
		// 		console.log(`${key}:`, {
		// 			name: value.name,
		// 			size: value.size,
		// 			type: value.type,
		// 			lastModified: value.lastModified
		// 		});
		// 	} else {
		// 		console.log(`${key}:`, value);
		// 	}
		// }
		// console.log('=== End FormData Debug ===');
	}
	async function handleSave() {
		handleAppendingFormData();
		try {
			const res = await save_function(formData);
			success = true;
			await invalidateAll().then(() => {
				goto(save_url_path, { invalidateAll: true });
			});
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';

			failed = true;
		}
	}

	async function handleSaveAndContinue() {
		handleAppendingFormData();
		try {
			const res = await save_function(formData);
			success = true;
			// try {
			// make the save_and_continue_url_path work actually on creating object
			// }
			await invalidateAll().then(() => {
				goto(save_and_continue_url_path, { invalidateAll: true });
			});
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';
			failed = true;
		}
	}

	async function handleSaveAndNew() {
		handleAppendingFormData();
		try {
			const res = await save_function(formData);
			success = true;
			await invalidateAll().then(() => {
				goto(save_and_new_url_path, { invalidateAll: true });
			});
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';

			failed = true;
		}
	}
	async function handleDelete() {
		if (id) {
			delete_function(id);
			await invalidateAll().then(() => {
				goto(save_url_path, { invalidateAll: true });
			});
		}
	}

	async function handleWebhookPost() {
		try {
			let res = await webhook_func(id);
			success = true;
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';
			failed = true;
		}
	}
</script>

<Footer class="!bg-[#364153] m-4">
	<div class="flex gap-3">
		{#if delete_function}
			<Button color="red" onclick={() => (defaultModal = true)}>Delete</Button>
			<Modal title="Delete" form bind:open={defaultModal}>
				<span>Are you really willing to delete this {action_name.replace('Episodes', 'Episode')}?</span>

				{#snippet footer()}
					<Button type="submit" onclick={handleDelete} value="decline" color="red">Yes</Button>
					<Button
						type="submit"
						onclick={() => {
							defaultModal = false;
						}}
						value="success"
						color="alternative">No</Button
					>
				{/snippet}
			</Modal>
		{/if}
		{#if can_read_logs}
			<Button
				color="green"
				onclick={() => {
					goto(`/admin/administration/logs/${action_name.toLowerCase()}/${data.id.id.String}`);
				}}>Logs</Button
			>
		{/if}
	</div>
	<div class="flex flex-col fixed right-5 bottom-5 gap-3">
		{#if success}
			<Toast>
				{#snippet icon()}
					<FireOutline class="text-primary-500 bg-primary-100 dark:bg-primary-800 dark:text-primary-200 h-6 w-6" />
				{/snippet}
				{action_name} saved successfully.
			</Toast>
		{/if}
		{#if failed}
			<Toast
				onclose={() => {
					failed = false;
				}}
				color="red"
			>
				{#snippet icon()}
					<CloseCircleSolid class="text-red-500 bg-red-100 dark:bg-red-800 dark:text-red-200 h-6 w-6" />
				{/snippet}
				{message_error}.
			</Toast>
		{/if}
	</div>
	<FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
		<div class="flex gap-3">
			{#if webhook_func && id}
				<Button color="teal" onclick={handleWebhookPost}>Webhook</Button>
			{/if}
			<Button color="blue" onclick={handleSave}>Save</Button>
			<Button color="blue" onclick={handleSaveAndContinue}>Save and continue editing</Button>
			<Button color="blue" onclick={handleSaveAndNew}>Save and new</Button>
		</div>
	</FooterLinkGroup>
</Footer>
