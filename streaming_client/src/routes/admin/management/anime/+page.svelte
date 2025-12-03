<script lang="ts">
	// @ts-ignore
	import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button, Checkbox, Search, PaginationNav } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { type AnimeList } from '$lib/types.ts';
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let querySearch = $state('');
	let currentPage = $state(1);
	let pageSize = $derived(querySearch ? 10 : 5);

	let animeList = $state<AnimeList>([]);

	let searchedAnimeList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return animeList;
		}
		return animeList.filter((anime) => anime.title.toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedAnimeList.length / pageSize));
	let paginatedAnimeList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		const end = start + pageSize;
		return searchedAnimeList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	let { data }: PageProps = $props();

	onMount(async () => {
		animeList = data.anime;
	});

	function reverseSort() {
		animeList.reverse();
	}
</script>

{#if hasPermission(data.isLoggedIn.message, 'anime', 'read')}
	<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
		<div class="flex flex-col text-white">
			<div>
				<span class="text-lg text-[#999999]"> Anime </span>
			</div>
			<div>
				<span class="text-2xl"> Query database in search of anime </span>
			</div>
			<div>
				<span class="text-lg text-[#999999]"> There are filters, search, and all the anime u have access to. </span>
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
				<a href="/admin/management/anime/add" class="hover:underline justify-self-end">
					<Button color="light" class="h-[40px]">Add anime</Button>
				</a>
			</div>
			<!--		</div>-->
		</div>
		<div>
			<Table>
				<TableHead>
					<TableHeadCell class="!max-w-[67.5px] ">Thumbnail</TableHeadCell>
					<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>Title</TableHeadCell>
					<TableHeadCell>In progress</TableHeadCell>
					<TableHeadCell>Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each paginatedAnimeList as anime (anime.id)}
						<TableBodyRow
							class="hover:bg-[#ffffff05] cursor-pointer transition-all"
							onclick={() => {
								goto(`/admin/management/anime/${anime.slug}`);
							}}
						>
							<TableBodyCell class="!max-w-[67.5px] "><img src={`/${anime.image}`} alt={anime.title} class="h-30 aspect-[9/16] !max-w-[67.5px]" /></TableBodyCell>
							<TableBodyCell>
								<a href={`/admin/management/anime/${anime.slug}`} class="hover:underline">
									{anime.title}
								</a>
							</TableBodyCell>
							<TableBodyCell>
								<Checkbox disabled bind:checked={anime.in_progress}></Checkbox>
							</TableBodyCell>
							<TableBodyCell>{anime.date}</TableBodyCell>
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
