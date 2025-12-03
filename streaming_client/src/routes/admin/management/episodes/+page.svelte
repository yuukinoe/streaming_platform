<script lang="ts">
	// @ts-ignore
	import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button, Checkbox, Search, PaginationNav } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { type Episodes, Dummy } from '$lib/types.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	import type { PageProps } from './$types.js';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import { goto } from '$app/navigation';
	let { data }: PageProps = $props();
	let querySearch = $state('');
	let isLoading = $state(true);
	let currentPage = $state(1);

	let pageSize = $derived(querySearch ? 10 : 5);

	let episodesList = $state<Episodes[]>([]);

	// Sort state tracking
	let currentSortField = $state<string | null>(null);
	let isAscending = $state(true);

	let searchedEpisodesList = $derived.by(() => {
		if (querySearch.trim() === '') {
			return episodesList;
		}

		return episodesList.filter((episodes) => (episodes.anime.title ?? '').toLowerCase().includes(querySearch.toLowerCase()));
	});

	let totalPages = $derived(Math.ceil(searchedEpisodesList.length / pageSize));

	let paginatedEpisodesList = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;

		const end = start + pageSize;

		return searchedEpisodesList.slice(start, end);
	});

	function handlePageChange(newPage: number) {
		currentPage = newPage;
	}

	onMount(async () => {
		episodesList = data.episodes as Episodes[];

		if (episodesList.length === 0) {
			let dummyData = Dummy.Episodes();

			dummyData.title = 'No';

			dummyData.anime.title = 'data to display.';

			episodesList = [dummyData];
		}
		isLoading = false;
	});

	function reverseSort() {
		if (currentSortField === 'thumbnail') {
			isAscending = !isAscending;
		} else {
			currentSortField = 'thumbnail';
			isAscending = true;
		}
		episodesList.reverse();
	}

	function sortByDate() {
		if (currentSortField === 'date') {
			isAscending = !isAscending;
		} else {
			currentSortField = 'date';
			isAscending = true;
		}

		episodesList.sort((a, b) => {
			const dateA = new Date(a.date ?? '').getTime();
			const dateB = new Date(b.date ?? '').getTime();
			return isAscending ? dateA - dateB : dateB - dateA;
		});
	}

	function sortByEpisode() {
		if (currentSortField === 'episode') {
			isAscending = !isAscending;
		} else {
			currentSortField = 'episode';
			isAscending = true;
		}

		episodesList.sort((a, b) => {
			return isAscending ? a.episode - b.episode : b.episode - a.episode;
		});
	}

	function sortByTitle() {
		if (currentSortField === 'title') {
			isAscending = !isAscending;
		} else {
			currentSortField = 'title';
			isAscending = true;
		}

		episodesList.sort((a, b) => {
			const comparison = (a.title ?? '').localeCompare(b.title ?? '');
			return isAscending ? comparison : -comparison;
		});
	}

	function sortByAnime() {
		if (currentSortField === 'anime') {
			isAscending = !isAscending;
		} else {
			currentSortField = 'anime';
			isAscending = true;
		}

		episodesList.sort((a, b) => {
			const comparison = (a.anime.title ?? '').localeCompare(b.anime.title ?? '');
			return isAscending ? comparison : -comparison;
		});
	}
</script>

{#if hasPermission(data.isLoggedIn.message, 'episodes', 'read')}
	{#if isLoading}
		<div class="w-full h-full">
			<Loading />
		</div>
	{:else}
		<div class="m-2 flex flex-col gap-5 h-full overflow-hidden">
			<div class="flex w-full justify-between">
				<div class="flex flex-col text-white">
					<div>
						<span class="text-lg text-[#999999]"> Episodes </span>
					</div>
					<div>
						<span class="text-2xl"> Query database in search of episodes </span>
					</div>
					<div>
						<span class="text-lg text-[#999999]"> There are filters, search, and all the episodes u have access to. </span>
					</div>
				</div>
				<!-- <div class="flex flex-col justify-end">
					<a href="/admin/management/episodes/add" class="hover:underline justify-self-end">
						<Button color="light" class="h-[40px]">Add episodes</Button>
					</a>
				</div> -->
			</div>
			<!-- <Table items={episodesDataTableList} dataTableOptions={filterOptions} /> -->
			<Search bind:value={querySearch} color="blue" placeholder="Search" />
			<div class="w-full grid grid-cols-3 items-center">
				<div></div>
				<div class="flex justify-center items-center">
					<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
				</div>
				<div class="flex justify-end items-center">
					<a href="/admin/management/episodes/add" class="hover:underline justify-self-end">
						<Button color="light" class="h-[40px]">Add episodes</Button>
					</a>
				</div>
			</div>
			<div>
				<Table>
					<TableHead>
						<TableHeadCell class="!max-w-[213.5px] hover:bg-[#2F3848] transition-all cursor-pointer" onclick={reverseSort}>
							Thumbnail {currentSortField === 'thumbnail' ? (isAscending ? '↑' : '↓') : ''}
						</TableHeadCell>
						<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={sortByEpisode}>
							Episode {currentSortField === 'episode' ? (isAscending ? '↑' : '↓') : ''}
						</TableHeadCell>
						<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={sortByAnime}>
							Anime {currentSortField === 'anime' ? (isAscending ? '↑' : '↓') : ''}
						</TableHeadCell>
						<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={sortByTitle}>
							Title {currentSortField === 'title' ? (isAscending ? '↑' : '↓') : ''}
						</TableHeadCell>
						<TableHeadCell class="hover:bg-[#2F3848] transition-all cursor-pointer" onclick={sortByDate}>
							Date {currentSortField === 'date' ? (isAscending ? '↑' : '↓') : ''}
						</TableHeadCell>
					</TableHead>
					<TableBody>
						{#each paginatedEpisodesList as episodes (episodes.id)}
							<TableBodyRow
								class="hover:bg-[#ffffff05] cursor-pointer transition-all"
								onclick={() => {
									goto(`/admin/management/episodes/${episodes.anime.slug}/${episodes.episode}`, {
										replaceState: true
									});
								}}
							>
								<a href={`/admin/management/episodes/${episodes.anime.slug}/${episodes.episode}`}>
									<TableBodyCell class="!max-w-[213.5px] "><img src={`${episodes.image ? `/${episodes.image}` : '/404_1920x1080.webp'}`} alt={episodes.title} class="max-h-30 aspect-[16/9]" /></TableBodyCell>
								</a>
								<TableBodyCell>
									{episodes.episode}
								</TableBodyCell>
								<TableBodyCell>
									<!-- <Checkbox disabled bind:checked={episodes.in_progress}></Checkbox> -->
									<a href={`/admin/management/anime/${episodes.anime.slug}`} class="hover:underline">
										{episodes.anime.title}
									</a>
								</TableBodyCell>
								<TableBodyCell>
									<a href={`/admin/management/episodes/${episodes.anime.slug}/${episodes.episode}`} class="hover:underline">
										{episodes.title ? episodes.title : 'null'}
									</a>
								</TableBodyCell>
								<TableBodyCell>{episodes.date ? new Date(episodes.date).toLocaleString() : `${episodes.date}`}</TableBodyCell>
							</TableBodyRow>
						{/each}
					</TableBody>
				</Table>
			</div>

			<div class="w-full grid grid-cols-3 items-center">
				<div></div>
				<div class="flex justify-center items-center">
					<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange} size="large" />
				</div>
				<div class="flex justify-end items-center">
					<span class="text-[#bdbdbd] text-xs">Total episodes: {episodesList.length}</span>
				</div>
			</div>
		</div>
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
