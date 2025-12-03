<script lang="ts">
	import AddButtons from '$lib/components/AddButtons.svelte';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { createUser } from '$lib/data/admin/post.ts';
	// @ts-ignore
	import { Label, Input, Footer, FooterCopyright, FooterLinkGroup, FooterLink, Button } from 'flowbite-svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	import type { PageProps } from './$types.js';
	let { data }: PageProps = $props();
	let userData = $state({
		username: '',
		password: ''
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'users', 'add')}
	<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> Add user </span>
			</div>
			<div>
				<span class="text-2xl"> Add user to the database </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> Just fill in the details below to create a new user. </span>
			</div>
		</div>
		<div class="flex flex-col gap-5">
			<div class="flex flex-col">
				<Label for="username-input" class="mb-2 block">Username</Label>
				<Input bind:value={userData.username} id="username-input" placeholder="Username..." />
			</div>
			<div class="flex flex-col">
				<Label for="password-input" class="mb-2 block">Password</Label>
				<Input bind:value={userData.password} id="password-input" type="password" placeholder="Password..." />
			</div>
		</div>
		<AddButtons save_url_path="/admin/administration/users" save_and_continue_url_path="/admin/administration/users" save_and_new_url_path="/admin/administration/users/add" data={userData} save_function={createUser} />
	</div>
{:else}
	<InsufficientPermissions />
{/if}
