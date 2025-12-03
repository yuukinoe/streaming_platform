<script lang="ts">
	// @ts-ignore
	import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button, Search, PaginationNav } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types.js';
	import type { NewsPublic } from '$lib/server/types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let querySearch = $state('');
	let currentPage = $state(1);
	let pageSize = $derived(querySearch ? 10 : 5);

	let newsList = $state<NewsPublic[]>([]);

	let searchedNewsList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return newsList;
		}
		return newsList.filter((news) => news.text_header.toLowerCase().includes(querySearch.toLowerCase()) || news.description.toLowerCase().includes(querySearch.toLowerCase()) || news.category.name.toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedNewsList.length / pageSize));
	let paginatedNewsList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		const end = start + pageSize;
		return searchedNewsList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	let { data }: PageProps = $props();

	onMount(async () => {
		newsList = data.news || [];
	});

	function reverseSort() {
		newsList.reverse();
	}
</script>

{#if hasPermission(data.isLoggedIn.message, 'news', 'read')}
	<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> News </span>
			</div>
			<div>
				<span class="text-2xl"> Query database in search of news </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> There are filters, search, and all the news you have access to. </span>
			</div>
		</div>
		<Search bind:value={querySearch} color="blue" placeholder="Search" />
		<div class="w-full grid grid-cols-3 items-center">
			<div></div>
			<div class="flex justify-center items-center">
				<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
			</div>
			<div class="flex justify-end items-center">
				<a href="/admin/management/news/add" class="hover:underline justify-self-end">
					<Button color="light" class="h-[40px]">Add news</Button>
				</a>
			</div>
		</div>
		<div>
			<Table>
				<TableHead>
					<TableHeadCell class="!max-w-[67.5px] ">Banner</TableHeadCell>
					<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>Title</TableHeadCell>
					<TableHeadCell>Category</TableHeadCell>
					<TableHeadCell>Pinned</TableHeadCell>
					<TableHeadCell>Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each paginatedNewsList as news (news.id.id.String)}
						<TableBodyRow
							class="hover:bg-[#ffffff05] cursor-pointer transition-all"
							onclick={() => {
								goto(`/admin/management/news/${news.slug}`);
							}}
						>
							<TableBodyCell class="!max-w-[67.5px] ">
								<img src={`/${news.image}`} alt={news.text_header} class="h-30 aspect-[9/16] !max-w-[67.5px]" />
							</TableBodyCell>
							<TableBodyCell>
								<a href={`/admin/management/news/${news.slug}`} class="hover:underline">
									{news.text_header}
								</a>
							</TableBodyCell>
							<TableBodyCell>
								<span class="px-2 py-1 bg-[#2F3848] rounded text-sm">
									{news.category.name}
								</span>
							</TableBodyCell>
							<TableBodyCell>
								{#if news.pinned}
									<span class="text-yellow-400">★</span>
								{/if}
							</TableBodyCell>
							<TableBodyCell>{news.date}</TableBodyCell>
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
