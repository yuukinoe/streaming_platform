<script lang="ts">
	import SaveAddRole from '$lib/components/+SaveAddRole.svelte';
	import { addRole } from '$lib/data/admin/post.ts';
	import type { PageProps } from './$types.js';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
</script>

{#if hasPermission(data.isLoggedIn.message, 'roles', 'add')}
	<div>
		<SaveAddRole permissionoptions={data.permissionsOptions as []} save_fn={addRole} />
	</div>
{:else}
	<InsufficientPermissions />
{/if}
