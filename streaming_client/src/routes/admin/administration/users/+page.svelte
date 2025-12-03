<script lang="ts">
	import type { UserPublic } from '$lib/types.ts';
	// @ts-ignore
	import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button, PaginationNav, Avatar, Checkbox, Search } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();

	let usersList = $state([]) as UserPublic[];

	let querySearch = $state('');
	let currentPage = $state(1);
	let pageSize = $derived(querySearch ? 10 : 5);

	let searchedUsersList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return usersList;
		}
		return usersList.filter((user) => user.name.toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedUsersList.length / pageSize));
	let paginatedUsersList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		const end = start + pageSize;
		return searchedUsersList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	function reverseSort() {
		usersList.reverse();
	}

	onMount(async () => {
		usersList = data.users;
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'users', 'read')}
<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
	<div class="flex flex-col text-white">
		<div>
			<span class="text-lg text-[#999999]"> Users </span>
		</div>
		<div>
			<span class="text-2xl"> Manage users in the database </span>
		</div>
		<div>
			<span class="text-lg text-[#999999]"> Register new users, edit existing ones, and manage their permissions. </span>
		</div>
	</div>
	<Search bind:value={querySearch} color="blue" placeholder="Search" />
	<div class="w-full grid grid-cols-3 items-center">
		<!--		<div class="grid grid-cols-2 items-center">-->
		<div></div>
		<div class="flex justify-center items-center">
			<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
		</div>
		<div class="flex justify-end items-center">
			<a href="/admin/administration/users/add" class="hover:underline justify-self-end">
				<Button color="light" class="h-[40px]">Add user</Button>
			</a>
		</div>
	</div>
	<div>
		<Table>
			<TableHead>
				<TableHeadCell>Avatar</TableHeadCell>
				<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>Username</TableHeadCell>
				<TableHeadCell>Translator</TableHeadCell>
				<TableHeadCell>Proofreader</TableHeadCell>
				<TableHeadCell>Uploader</TableHeadCell>
				<TableHeadCell>Typesetter</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each paginatedUsersList as user (user.id)}
					<TableBodyRow>
						<TableBodyCell>
							<a href={`/admin/administration/users/${user.slug}`} class="hover:underline">
								<Avatar src={user.avatar && user.avatar !== '' ? `/${user.avatar}` : '/unknown_user.png'} alt={user.name} size="md" />
							</a>
						</TableBodyCell>
						<TableBodyCell>
							<a href={`/admin/administration/users/${user.slug}`} class="hover:underline">
								{user.name}
							</a>
						</TableBodyCell>
						<TableBodyCell>
							<Checkbox color="teal" disabled bind:checked={user.translator}></Checkbox>
						</TableBodyCell>
						<TableBodyCell>
							<Checkbox color="teal" disabled bind:checked={user.proofreader}></Checkbox>
						</TableBodyCell>
						<TableBodyCell>
							<Checkbox color="teal" disabled bind:checked={user.uploader}></Checkbox>
						</TableBodyCell>
						<TableBodyCell>
							<Checkbox color="teal" disabled bind:checked={user.editor}></Checkbox>
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
	<div class="w-full flex justify-center">
			<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
		</div>
	</div>
{:else}
	<InsufficientPermissions />
{/if}
