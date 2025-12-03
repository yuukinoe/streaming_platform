<script lang="ts">
	// @ts-ignore
	import { Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Search, PaginationNav, Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import type { NewsCategoryPublic } from '$lib/server/types.ts';
	import { goto } from '$app/navigation';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let categoriesList = $state([]) as NewsCategoryPublic[];

	let querySearch = $state('');
	let currentPage = $state(1);
	let pageSize = $derived(querySearch ? 10 : 5);

	let searchedCategoriesList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return categoriesList;
		}
		return categoriesList.filter((category) => category.name.toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedCategoriesList.length / pageSize));
	let paginatedCategoriesList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		const end = start + pageSize;
		return searchedCategoriesList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	function reverseSort() {
		categoriesList.reverse();
	}

	let { data } = $props();
	onMount(async () => {
		if (data?.newsCategories) {
			categoriesList = data.newsCategories;
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'news_categories', 'read')}
	<div class="flex flex-col gap-5 m-2">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> News Categories </span>
			</div>
			<div>
				<span class="text-2xl"> Create, edit, delete news categories. </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> Manage news categories for organizing articles. </span>
			</div>
		</div>
		<Search bind:value={querySearch} color="blue" placeholder="Search" />
		<div class="w-full grid grid-cols-3 items-center">
			<div></div>
			<div class="flex justify-center items-center">
				<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
			</div>
			<div class="flex justify-end items-center">
				<a href="/admin/administration/categoryarticles/add" class="hover:underline justify-self-end">
					<Button color="light" class="h-[40px]">Add category</Button>
				</a>
			</div>
		</div>
		<div>
			<Table>
				<TableHead>
					<TableHeadCell>Category ID</TableHeadCell>
					<TableHeadCell>Name</TableHeadCell>
					<TableHeadCell>Allowed Everyone</TableHeadCell>
					<TableHeadCell>Visibility</TableHeadCell>
					<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each paginatedCategoriesList as category (category.id)}
						<TableBodyRow
							class="cursor-pointer hover:bg-[#192230] transition-all"
							onclick={() => {
								goto(`/admin/administration/categoryarticles/${category.id.id.String}`);
							}}
						>
							<TableBodyCell>{category.id.id.String}</TableBodyCell>
							<TableBodyCell>
								{category.name}
							</TableBodyCell>
							<TableBodyCell>{category.allowed_everyone}</TableBodyCell>
							<TableBodyCell>{category.visible}</TableBodyCell>
							<TableBodyCell>{category.date ? `${new Date(category.date).toLocaleString()}` : 'no data'}</TableBodyCell>
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
