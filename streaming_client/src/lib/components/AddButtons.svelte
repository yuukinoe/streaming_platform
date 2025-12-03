<script lang="ts">
	// @ts-ignore
	import { Button, Footer, FooterLinkGroup, Toast } from 'flowbite-svelte';
	import { goto } from '$app/navigation';
	import { CloseCircleSolid, FireOutline } from 'flowbite-svelte-icons';

	const { save_url_path, save_and_continue_url_path, save_and_new_url_path, save_function, data } = $props<{
		save_url_path: string;
		save_and_continue_url_path: string;
		save_and_new_url_path: string;
		save_function: (data: any) => Promise<any>;
		data: any;
	}>();

	let message_error = $state('');
	let success = $state(false);
	let failed = $state(false);

	async function handleSave() {
		try {
			await save_function(data);
			success = true;
			await goto(save_url_path);
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';

			failed = true;
		}
	}

	async function handleSaveAndContinue() {
		try {
			await save_function(data);
			success = true;
			await goto(save_and_continue_url_path);
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';

			failed = true;
		}
	}

	async function handleSaveAndNew() {
		try {
			await save_function(data);
			success = true;
			await goto(save_and_new_url_path);
		} catch (err) {
			message_error = err instanceof Error ? err.message : 'Unknown error';

			failed = true;
		}
	}
</script>

<Footer class="!bg-[#364153]">
	<div></div>
	<FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
		<div class="flex gap-3">
			<Button color="blue" onclick={handleSave}>Save</Button>
			<Button color="blue" onclick={handleSaveAndContinue}>Save and continue editing</Button>
			<Button color="blue" onclick={handleSaveAndNew}>Save and new</Button>
		</div>

		<div class="flex flex-col fixed right-5 bottom-5 gap-3">
			{#if success}
				<Toast>
					{#snippet icon()}
						<FireOutline class="text-primary-500 bg-primary-100 dark:bg-primary-800 dark:text-primary-200 h-6 w-6" />
					{/snippet}
					User created successfully.
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
	</FooterLinkGroup>
</Footer>
