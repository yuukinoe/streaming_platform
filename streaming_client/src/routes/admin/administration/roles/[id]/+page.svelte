<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	import type { Role } from '$lib/types.js';
	import { patchRole } from '$lib/data/admin/patch.ts';
	import SaveAddRole from '$lib/components/+SaveAddRole.svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	let { data }: PageProps = $props();
	let role = $state<Role>() as Role;
	onMount(async () => {
		const foundRole = data.roles?.find((role: Role) => role.id.id.String === data.id);
		if (foundRole) {
			role = foundRole;
		} else {
			console.error(`Role with id ${data.id} not found.`);
		}
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'roles', 'edit')}
	<div>
		<SaveAddRole permissionoptions={data.permissionsOptions as []} {role} save_fn={patchRole} />
	</div>
{:else}
	<InsufficientPermissions />
{/if}
