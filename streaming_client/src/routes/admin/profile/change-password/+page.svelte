<script lang="ts">
	// @ts-ignore
	import { Input, Label, Button, Alert } from 'flowbite-svelte';
	import { goto } from '$app/navigation';
	import { patchUserPasswordByHimself } from '$lib/data/admin/patch.ts';
	let passwordData = $state({
		old_password: '',
		new_password: ''
	});

	let changePasswordError = $state(false);

	let formData = new FormData();
	const changePassword = async () => {
		const passwordDataC = {
			old_password: passwordData.old_password,
			new_password: passwordData.new_password
		};
		if (formData.has('data')) {
			formData.delete('data');
		}
		formData.append('data', JSON.stringify(passwordDataC));
		await patchUserPasswordByHimself(formData).then((response) => {
			if (response) {
				goto('/admin/profile');
			} else {
				changePasswordError = true;
			}
		});
		return;
	};
</script>

<div class="flex items-center justify-center bg-[#1e1e1e]">
	<div class="p-3 bg-[#1e2939] rounded-lg shadow-lg w-96 flex flex-col gap-5">
		<div>
			{#if changePasswordError}
				<Alert color="red">
					<span class="font-medium">Wrong old or new password!</span>
					Provided passwords do not match your password in the database. Please try again.
				</Alert>
			{/if}
		</div>
		<div>
			<Label for="oldpassword" class="mb-2">Old password</Label>
			<Input bind:value={passwordData.old_password} minlength="8" type="password" id="oldpassword" placeholder="Old password..." required />
		</div>
		<div>
			<Label for="newpassword" class="mb-2">New password</Label>
			<Input
				bind:value={passwordData.new_password}
				minlength="8"
				type="password"
				id="newpassword"
				placeholder="New password..."
				onkeydown={(e: KeyboardEvent) => {
					if (e.key === 'Enter') {
						changePassword();
					}
				}}
				required
			/>
		</div>
		<div class="flex justify-end gap-3">
			<!-- <Button
					color="yellow"
					onclick={debugPasswordData}
				>
					Debug
				</Button> -->
			<Button
				color="blue"
				class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
				onclick={() => {
					changePassword();
				}}
			>
				Change password
			</Button>
		</div>
	</div>
</div>
