<script lang="ts">
	// @ts-ignore
	import { MultiSelect, Badge, Avatar, Fileupload, Helper, FloatingLabelInput, Toggle, Label, Input, Button, ButtonGroup } from 'flowbite-svelte';
	import { Dummy, type PermissionsMap, type Role, type PermissionFlags } from '$lib/types.ts';
	import { onMount } from 'svelte';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { MinusOutline, PlusOutline, UserGraduateOutline } from 'flowbite-svelte-icons';
	// import type { PageProps } from './$types.js';
	
	let { role: initialRoleData, save_fn, permissionoptions } = $props<{
		role?: Role;
		save_fn: (formData: FormData) => Promise<any>;
		permissionoptions: string[];
	}>();
	let role = $state<Role>(Dummy.Role()) as Role;

	let permissionsFlags = Object.keys(Dummy.PermissionFlags());
	let permissionOptions = $state([]) as [];
	let colors = ['indigo', 'blue', 'green', 'yellow', 'red', 'purple', 'pink'];
	let iconSrc = $state('');
	let isImageUploaded = $state(false);
	let uploadedFile: File | null = null;

	let formData = new FormData();
	type Permission = {
		name: string;
		permission: string;
		flag: string;
		value: string;
	};
	let permissions = $state([] as Permission[]);
	let selectedPermissionSelect = $state([] as string[]);
	let initialized = false;
	$effect(() => {
		if (!initialized && initialRoleData) {
			initialized = true;
			role = initialRoleData;

			const result: string[] = [];

			for (const [moduleName, flags] of Object.entries(role.permissions)) {
				for (const [flagName, value] of Object.entries(flags)) {
					if (value) {
						result.push(`${moduleName} | PERMISSION ${flagName}`.toLocaleUpperCase());
					}
				}
			}

			selectedPermissionSelect = result;
		}
	});

	$effect(() => {
		iconSrc = role.icon && role.icon !== '' ? `/${role.icon}` : '/unknown_user.png';
		role.date = role.date && role.date == '' ? role.date : null;
	});

	$effect(() => {
		const selectedObjects = permissions.filter((p) => selectedPermissionSelect.includes(p.value));
		const agg = new Map<string, PermissionFlags>();
		for (const perm of selectedObjects) {
			if (!perm.permission || !perm.flag) continue;
			if (!agg.has(perm.permission)) {
				agg.set(perm.permission, { ...Dummy.PermissionFlags() });
			}
			const flags = agg.get(perm.permission)!;
			flags[perm.flag as keyof PermissionFlags] = true;
		}
		const result: PermissionsMap = {};
		for (const [perm, flags] of agg.entries()) {
			result[perm] = flags;
		}
		const newVal = [result];

		role.permissions = result;
	});
	function handleFileUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			uploadedFile = file;

			if (formData.has('icon')) {
				formData.delete('icon');
			}
			formData.append('icon', file);

			const reader = new FileReader();
			reader.onload = (e) => {
				iconSrc = e.target?.result as string;
				isImageUploaded = true;
			};
			reader.readAsDataURL(file);
		}
	}
	onMount(async () => {
		const permissionsFetch = permissionoptions;
			if (permissionsFetch?.length !== 0) {
				permissionOptions = permissionsFetch as [];
				if (permissionsFetch) {
					for (const perm of permissionsFetch) {
						for (const permflag of permissionsFlags) {
							permissions.push({
								name: `${perm} | PERMISSION ${permflag}`.toLocaleUpperCase(),
								permission: `${perm}`,
								flag: `${permflag}`,
								value: `${perm} | PERMISSION ${permflag}`.toLocaleUpperCase()
							});
						}
					}
				}
			}

	});
</script>

<div>
	<div class="flex gap-2 h-full">
		<div class="flex gap-7 p-4 justify-around">
			<div class="flex flex-col items-center min-w-56">
				<Avatar src={iconSrc} alt={role.name} class="rounded-b-none w-56! h-56!" cornerStyle="rounded"></Avatar>
				<Fileupload size="sm" id="avatar_upload" class="mb-2 rounded-t-none" onchange={handleFileUpload} accept="image/*" />
				<Helper>SVG, PNG, JPG, WEBP or GIF.</Helper>
			</div>
		</div>

		<div class="flex flex-col w-full p-4 gap-2">
			<FloatingLabelInput
				variant="filled"
				class="w-full"
				id="role_name"
				value={role.name}
				name="role_name"
				type="text"
				onchange={(e: Event) => {
					role.name = (e.target as HTMLInputElement).value;
				}}>Role name</FloatingLabelInput
			>
			<MultiSelect items={permissions} bind:value={selectedPermissionSelect}>
				{#snippet children({ item, clear }: { item: Permission; clear: void })}
					<Badge color={colors[Math.floor(Math.random() * colors.length)]} dismissable params={{ duration: 100 }} onclose={clear}>
						{item.name}
					</Badge>
				{/snippet}
			</MultiSelect>
			<div class="flex gap-10">
				<div class="flex flex-col">
					<Label for="administrative_role">Administrative role</Label>
					<Toggle
						name="administrative_role"
						color="teal"
						checked={role.administrative_role}
						onchange={(e: Event) => {
							role.administrative_role = (e.target as HTMLInputElement).checked;
						}}
					/>
				</div>
				<div class="flex flex-col">
					<Label for="visibility">Visibility</Label>
					<Toggle
						name="visibility"
						color="yellow"
						checked={role.visible}
						onchange={(e: Event) => {
							role.visible = (e.target as HTMLInputElement).checked;
						}}
					/>
				</div>

				<div class="flex flex-col">
					<Label class="pb-2">Hierarchy</Label>
					<div>
						<ButtonGroup class="w-full">
							<Button
								type="button"
								id="decrement-button"
								onclick={() => {
									if (role.hierarchy > 0) role.hierarchy -= 1;
								}}
								class="h-11 p-3"
							>
								<MinusOutline />
							</Button>
							<Input
								min="0"
								bind:value={role.hierarchy}
								type="number"
								id="quantity-input"
								aria-describedby="helper-text-explanation"
								placeholder=" "
								required
								class="h-11 w-full pb-6 text-center appearance-none"
								onchange={(e: Event) => {
									const value = parseInt((e.target as HTMLInputElement).value, 10);
									if (!isNaN(value)) {
										role.hierarchy = value;
									}
								}}
							/>
							<div class="absolute start-1/2 bottom-0 flex -translate-x-1/2 items-center space-x-1 text-xs text-gray-400 rtl:translate-x-1/2 rtl:space-x-reverse">
								<UserGraduateOutline class="h-4 w-4" />
								<span>Hierarchy</span>
							</div>
							<Button type="button" id="increment-button" onclick={() => (role.hierarchy += 1)} class="h-11 p-3">
								<PlusOutline />
							</Button>
						</ButtonGroup>
					</div>
				</div>

				<div class="flex flex-col">
					<Label for="color-picker" class="pb-2">Role color</Label>
					<Input
						type="color"
						id="color-picker"
						class="h-12 w-15"
						value={role.color || '#ffffff'}
						onchange={(e: Event) => {
							role.color = (e.target as HTMLInputElement).value;
						}}
					/>
				</div>
			</div>
			<!-- 
			<button
				onclick={() => {
					console.log(role);
				}}>TEST</button
			> -->
		</div>
	</div>
	<SaveButtons
		action_name="Role"
		save_url_path={`/admin/administration/roles`}
		save_and_continue_url_path={`/admin/administration/roles/${role.id.id.String}`}
		save_and_new_url_path="/admin/administration/roles/add"
		slug={role.id.id.String}
		{formData}
		data={role}
		save_function={save_fn}
		delete_function={() => {
			console.log('Delete function called');
			// Call your delete function here
		}}
	/>
</div>
