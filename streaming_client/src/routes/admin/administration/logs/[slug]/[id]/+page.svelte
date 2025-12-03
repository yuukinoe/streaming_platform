<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.ts';
	import { type LogsPublic, type LogsTable, Dummy } from '$lib/types.ts';
	import LogsList from '$lib/components/LogsList.svelte';
	import Loading from '$lib/data/client/components/Loading.svelte';
	let isLoading = $state(true);
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
	let logs = $state([]) as LogsTable[];
	onMount(async () => {
		let object_id = data?.id;
		let object = data?.slug;
		if (object_id && object) {
			// find list of objects where object_id == object_id and object == object
			const foundLogs = data.logs?.filter((log) => log?.object_id === object_id && log?.object === object);
			logs = foundLogs
				? foundLogs.map((log: LogsPublic) => ({
						id: log.id.id.String,
						object: log.object,
						user: log.user_id.name,
						action: log.action.length > 109 ? log.action.slice(0, 109) + '…' : log.action,
						date: new Date(log.date).toLocaleString()
					}))
				: [];
		}
		isLoading = false;
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'logs', 'read')}
	{#if isLoading}
		<div class="w-full h-full">
			<Loading />
		</div>
	{:else}
		<LogsList logsList={logs} />
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
