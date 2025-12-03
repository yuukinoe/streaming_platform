<script lang="ts">
	// @ts-ignore
	import { Sidebar, SidebarGroup, SidebarItem, SidebarDropdownWrapper, SidebarButton, uiHelpers, Input, Label, Button, Alert } from 'flowbite-svelte';
	import { ChartOutline, GridSolid, AdjustmentsHorizontalOutline, RectangleListOutline, EditSolid, UserSolid } from 'flowbite-svelte-icons';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { redirect } from '@sveltejs/kit';
	import { patchUserPasswordByAdmin, patchUserPasswordByHimself } from '$lib/data/admin/patch.ts';
	import type { PageProps } from './$types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let newPassword = $state('');

	let changePasswordError = $state(false);
	let { data }: PageProps = $props();
	let formData = new FormData();
	const changePassword = async () => {
		const password = newPassword;
		if (formData.has('data')) {
			formData.delete('data');
		}
		formData.append('data', newPassword);
		await patchUserPasswordByAdmin(formData, data.id).then((response) => {
			if (response) {
				history.back();
			} else {
				changePasswordError = true;
			}
		});
		return;
	};
</script>

{#if hasPermission(data.isLoggedIn.message, 'users', 'edit')}
	<div class="flex items-center justify-center bg-[#1e1e1e]">
		<div class="p-3 bg-[#1e2939] rounded-lg shadow-lg w-96 flex flex-col gap-5">
			<div>
				{#if changePasswordError}
					<Alert color="red">
						<span class="font-medium">Something went wrong!</span>
						No clue what happend but u are welcomed to try again.
					</Alert>
				{/if}
			</div>
			<div>
				<Label for="newpassword" class="mb-2">New password</Label>
				<Input
					bind:value={newPassword}
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
{:else}
	<InsufficientPermissions />
{/if}
