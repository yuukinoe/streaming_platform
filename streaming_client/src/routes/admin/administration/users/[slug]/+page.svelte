<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	// @ts-ignore
	import { PaginationNav, Avatar, Checkbox, Search, Button, GradientButton, Fileupload, Helper, FloatingLabelInput, Textarea, Toggle, Datepicker, Label, MultiSelect, Badge } from 'flowbite-svelte';
	import { CirclePlusSolid } from 'flowbite-svelte-icons';

	import type { Role, UserModelSuperUser } from '$lib/types.ts';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { patchUserData } from '$lib/data/admin/patch.ts';
	import { deleteUser } from '$lib/data/admin/delete.ts';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import InsufficientPermissions from '$lib/components/insufficient-permissions/+page.svelte';
	import { hasPermission } from '$lib/permissions.js';
	import { formatDateToISOWithNanoseconds, formatToRustDatetime } from '$lib/utils.ts';
	let { data }: PageProps = $props();
	let user = $state({}) as UserModelSuperUser;
	let roles = $state() as Role[];
	let isLoading = $state(true);
	let isImageUploaded = $state(false);
	let uploadedFile: File | null = null;
	let avatarSrc = $state('');
	let formData = new FormData();
	let created_at = $state(new Date());
	let updated_at = $state(new Date());
	let rolesSelect = $state([]) as RolesSelect[];
	let selectedRoles = $state([]) as string[];
	let colors = ['indigo', 'blue', 'green', 'yellow', 'red', 'purple', 'pink'];

	type RolesSelect = {
		name: string;
		value: string;
	};

	function filterSelectedItems(items: string[]) {
		return roles.filter((role) => items.includes(role.id.id.String));
	}

	function handleFileUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target?.files?.[0];
		if (file) {
			uploadedFile = file;

			if (formData.has('image')) {
				formData.delete('image');
			}
			formData.append('image', file);

			const reader = new FileReader();
			reader.onload = (e) => {
				avatarSrc = e.target?.result as string;
				isImageUploaded = true;
			};
			reader.readAsDataURL(file);
		}
	}

	onMount(async () => {
		if (data.isLoggedIn.message.is_superuser) {
			if (data.usersSuperUserView) {
				user = data.usersSuperUserView.find((u) => u.slug === data.slug) as UserModelSuperUser;
				// console.log(user.created_at);
				created_at = new Date(user.created_at);
				updated_at = new Date(user.updated_at);
				avatarSrc = user.avatar && user.avatar !== '' ? `/${user.avatar}` : '/unknown_user.png';
				if (user !== null && user !== undefined) {
					if (user.roles) {
						user.roles.forEach((role) => {
							selectedRoles.push(`${role.id.id.String}`);
						});
					}
				} else {
					// @ts-ignore
					user!.roles = null;
				}

				if (data.roles) {
					roles = data.roles as Role[];
					data.roles?.forEach((role) => {
						rolesSelect.push({
							name: role.name,
							value: role.id.id.String
						});
					});
				}
			}
		}
		isLoading = false;
	});

	const user_created_at = $derived(() => {
		if (!created_at) return '';
		return formatDateToISOWithNanoseconds(created_at);
	});
	$effect(() => {
		let created_at_rust = user_created_at();
		user.created_at = created_at_rust;
	});
</script>

{#if hasPermission(data.isLoggedIn.message, 'users', 'edit')}
	{#if isLoading}
		<div class="w-full h-full">
			<Loading />
		</div>
	{:else}
		<div class="flex flex-col gap-2 h-full overflow-hidden">
			<div class="flex gap-7 p-4 justify-around">
				<div class="flex flex-col items-center min-w-56">
					<Avatar src={avatarSrc} alt={user.name} class="rounded-b-none w-56! h-56!" cornerStyle="rounded"></Avatar>
					<Fileupload size="sm" id="avatar_upload" class="mb-2 rounded-t-none" onchange={handleFileUpload} accept="image/*" />
					<Helper>SVG, PNG, JPG, WEBP or GIF.</Helper>
				</div>
				<div class="flex flex-col w-full">
					<div class="flex w-full gap-3">
						<FloatingLabelInput
							variant="filled"
							class="w-full"
							id="username"
							value={user.name}
							name="username"
							type="text"
							onchange={(e: Event) => {
								user.name = (e.target as HTMLInputElement).value;
							}}>Username</FloatingLabelInput
						>
						<FloatingLabelInput
							variant="filled"
							class="w-full"
							id="email"
							value={user.email}
							name="email"
							type="email"
							onchange={(e: Event) => {
								user.email = (e.target as HTMLInputElement).value;
							}}>E-mail</FloatingLabelInput
						>
						<div class="relative w-full">
							<FloatingLabelInput variant="filled" class="w-full cursor-not-allowed opacity-50" id="password" value={user.password} name="password" disabled type="text">Hashed password</FloatingLabelInput>
							<Button class="absolute top-0 right-0 h-full rounded-b-none rounded-tl-none" color="blue" size="sm" href={user.id ? `/admin/administration/users/${user.id.id.String}/change-password` : '#'}>Change password</Button>
						</div>
					</div>
					<div class="flex w-full gap-3 mt-3 [&>div]:w-full">
						<Textarea
							clearable
							class="w-full resize-none"
							id="textarea-clearable"
							placeholder="Description"
							rows={9}
							name="description"
							value={user.description}
							onchange={(e: Event) => {
								user.description = (e.target as HTMLTextAreaElement).value;
							}}
						/>
					</div>
				</div>
			</div>
			<div class="grid grid-cols-3 p-4">
				<div class="flex flex-col gap-2 w-full">
					<Label for="created_at">Translator</Label>
					<Toggle
						name="translator"
						color="orange"
						checked={user.translator}
						onchange={(e: Event) => {
							user.translator = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="proofreader">Proofreader</Label>
					<Toggle
						name="proofreader"
						color="yellow"
						checked={user.proofreader}
						onchange={(e: Event) => {
							user.proofreader = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="uploader">Uploader</Label>
					<Toggle
						name="uploader"
						color="teal"
						checked={user.uploader}
						onchange={(e: Event) => {
							user.uploader = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="typesetter">Typesetter</Label>
					<Toggle
						name="typesetter"
						color="green"
						checked={user.editor}
						onchange={(e: Event) => {
							user.editor = (e.target as HTMLInputElement).checked;
						}}
					/>
				</div>
				<div class="flex flex-col gap-2 w-full">
					<Label for="superuser">Superuser</Label>
					<Toggle
						name="superuser"
						color="teal"
						checked={user.is_superuser}
						onchange={(e: Event) => {
							user.is_superuser = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="staff">Staff</Label>
					<Toggle
						name="staff"
						color="blue"
						checked={user.is_staff}
						onchange={(e: Event) => {
							user.is_staff = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="active">Active</Label>
					<Toggle
						name="active"
						color="green"
						checked={user.is_active}
						onchange={(e: Event) => {
							user.is_active = (e.target as HTMLInputElement).checked;
						}}
					/>
					<Label for="banned">Banned</Label>
					<Toggle
						name="banned"
						color="red"
						checked={user.is_banned}
						onchange={(e: Event) => {
							user.is_banned = (e.target as HTMLInputElement).checked;
						}}
					/>
				</div>
				<div class="flex flex-col gap-1 w-full">
					<MultiSelect
						items={rolesSelect}
						value={selectedRoles}
						onchange={(e: CustomEvent<string[]>) => {
							if (e.currentTarget) {
								const value = (e.currentTarget as HTMLInputElement).value;
								const selected = Array.isArray(value)
									? value
									: value
											.split(',')
											.map((v) => v.trim())
											.filter(Boolean);
								user.roles = filterSelectedItems(selected);
							}
						}}
					>
						{#snippet children({ item, clear }: { item: Role; clear: void })}
							<Badge color={colors[Math.floor(Math.random() * colors.length)]} dismissable params={{ duration: 100 }} onclose={clear}>
								{item.name}
							</Badge>
						{/snippet}
					</MultiSelect>

					<Label for="created_at">Created at</Label>
					<Datepicker
						id="CreatedAt"
						label="Created at"
						placeholder="Select date"
						bind:value={created_at}
						onchange={(e: Event) => {
							const value = (e.target as HTMLInputElement).value; // "2025-08-05T17:30"
							const rustFormat = value.replace('T', ' ') + ':00'; // "2025-08-05 17:30:00"
							// console.log(rustFormat);
							user.created_at = rustFormat;
						}}
						name="created_at"
					/>
					<Label for="updated_at">Updated at</Label>
					<Datepicker id="UpdatedAt" label="Updated at" placeholder="Select date" value={updated_at} name="updated_at" />
					<Label for="published">Published</Label>
					<Toggle
						name="published"
						color="purple"
						checked={user.published}
						onchange={(e: Event) => {
							user.published = (e.target as HTMLInputElement).checked;
						}}
					/>
				</div>
			</div>
			{#if user && user.id && user.id.id && user.id.id.String}
				<SaveButtons action_name="Users" save_url_path={`/admin/administration/users`} save_and_continue_url_path={`/admin/administration/users/${data.slug}`} save_and_new_url_path="/admin/administration/users/add" slug={data.slug} id={user.id.id.String} {formData} data={user} save_function={patchUserData} delete_function={deleteUser} />
			{/if}
		</div>
	{/if}
{:else}
	<InsufficientPermissions />
{/if}
