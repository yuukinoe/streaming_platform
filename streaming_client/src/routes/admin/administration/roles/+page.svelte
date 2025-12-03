<script lang="ts">
	// @ts-ignore
	import { Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Search, PaginationNav, Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import type { Role } from '$lib/types.ts';
	import { goto } from '$app/navigation';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let rolesList = $state([]) as Role[];

	let querySearch = $state('');
	let currentPage = $state(1);
	let pageSize = $derived(querySearch ? 10 : 5);

	let searchedRolesList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return rolesList;
		}
		return rolesList.filter((role) => role.name.toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedRolesList.length / pageSize));
	let paginatedRolesList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		const end = start + pageSize;
		return searchedRolesList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	function reverseSort() {
		rolesList.reverse();
	}

	let { data } = $props();
	onMount(async () => {
		if (data?.roles) {
			rolesList = data.roles;
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'roles', 'read')}
	<div class="flex flex-col gap-5 m-2">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> Roles </span>
			</div>
			<div>
				<span class="text-2xl"> Create, edit, delete roles. </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> To apply permissions to a user u should create a role. </span>
			</div>
		</div>
		<Search bind:value={querySearch} color="blue" placeholder="Search" />
		<div class="w-full grid grid-cols-3 items-center">
			<div></div>
			<div class="flex justify-center items-center">
				<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
			</div>
			<div class="flex justify-end items-center">
				<a href="/admin/administration/roles/add" class="hover:underline justify-self-end">
					<Button color="light" class="h-[40px]">Add role</Button>
				</a>
			</div>
		</div>
		<div>
			<Table>
				<TableHead>
					<TableHeadCell>Role ID</TableHeadCell>
					<TableHeadCell>Name</TableHeadCell>
					<TableHeadCell>Administrative role</TableHeadCell>
					<TableHeadCell>Visibility</TableHeadCell>
					<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each paginatedRolesList as role (role.id)}
						<TableBodyRow
							class="cursor-pointer hover:bg-[#192230] transition-all"
							onclick={() => {
								goto(`/admin/administration/roles/${role.id.id.String}`);
							}}
						>
							<TableBodyCell>{role.id.id.String}</TableBodyCell>
							<TableBodyCell>
								{role.name}
							</TableBodyCell>
							<TableBodyCell>{role.administrative_role}</TableBodyCell>
							<TableBodyCell>{role.visible}</TableBodyCell>
							<TableBodyCell>{role.date ? `${new Date(role.date).toLocaleString()}` : 'no data'}</TableBodyCell>
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
