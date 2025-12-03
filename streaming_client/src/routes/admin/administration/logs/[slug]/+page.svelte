<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import type { LogsPublic } from '$lib/types.ts';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
	let log = $state<LogsPublic | null>(null);
	onMount(async () => {
		const id = data?.slug;
		if (id && Array.isArray(data?.logs)) {
			const foundLog = data.logs.find((log) => log?.id?.id?.String === id);
			log = foundLog ?? null;
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'logs', 'read')}
	<div class="flex flex-col items-center justify-center h-full">
		<h1 class="text-2xl font-bold mb-4 text-white">Log Details</h1>
		{#if log && log.id}
			<div class="bg-[#1e2939] shadow-md rounded-lg p-6 w-full flex flex-col gap-3 max-w-3xl text-white">
				<p class="flex flex-col"><strong>Object:</strong> {log.object}</p>
				<p class="flex flex-col"><strong>Object ID:</strong> {log.object_id}</p>
				<p class="flex flex-col">
					<strong>Description:</strong>
					{log.description || 'No description'}
				</p>
				<p class="flex flex-col">
					<strong>User:</strong>
					<a href={`/admin/administration/users/${log.user_id?.slug || 'unknown'}`} class="text-blue-500 hover:underline">
						{log.user_id?.name || 'Unknown'} (User ID: {log.user_id?.id?.id?.String || 'No ID'})
					</a>
				</p>
				<p class="flex flex-col"><strong>Action:</strong> {log.action}</p>
				<p class="flex flex-col"><strong>Date:</strong> {new Date(log.date).toLocaleString()}</p>
			</div>
		{:else}
			<div class="bg-gray-100 rounded-lg p-6 w-full max-w-3xl">
				<p class="text-gray-600">Loading log details...</p>
			</div>
		{/if}
	</div>
{:else}
	<InsufficientPermissions />
{/if}
