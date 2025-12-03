<script lang="ts">
	import LogsList from '$lib/components/LogsList.svelte';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import type { LogsPublic, LogsTable } from '$lib/types.ts';
	import { onMount } from 'svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let logsList: LogsTable[] = $state([]);
	let isLoading = $state(true);
	let { data } = $props();
	onMount(async () => {
		if (data?.logs && Array.isArray(data.logs)) {
			for (const log of data.logs) {
				logsList.push({
					id: log.id.id.String,
					object: log.object,
					user: (log as any).user_id && (log as any).user_id.name ? (log as any).user_id.name : 'No data',
					action: log.action.length > 109 ? log.action.slice(0, 109) + '…' : log.action,
					date: new Date(log.date).toLocaleString()
				});
			}
			isLoading = false;
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'logs', 'read')}
	{#if isLoading}
		<div class="w-full h-full">
			<Loading />
		</div>
	{:else}
		<LogsList {logsList} />
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
