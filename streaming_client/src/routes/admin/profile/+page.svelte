<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types.js';
	// @ts-ignore
	import { PaginationNav, Avatar, Checkbox, Search, Button, GradientButton, Fileupload, Helper, FloatingLabelInput, Textarea, Toggle, Datepicker, Label, Footer, FooterLinkGroup, Toast } from 'flowbite-svelte';
	import { CirclePlusSolid, CloseCircleSolid, FireOutline } from 'flowbite-svelte-icons';

	import type { UserProfile } from '$lib/types.ts';
	import SaveButtons from '$lib/components/SaveButtons.svelte';
	import { patchUserData, patchUserProfile } from '$lib/data/admin/patch.ts';
	let success = $state(false);
	let failed = $state(false);

	let { data }: PageProps = $props();
	let user = $state({}) as UserProfile;
	let isImageUploaded = $state(false);
	let uploadedFile: File | null = null;
	let avatarSrc = $state('');
	let formData = new FormData();
	let created_at = $state(new Date());
	let updated_at = $state(new Date());
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
		user = data.isLoggedIn.message as UserProfile;
		created_at = new Date(user.created_at);
		updated_at = new Date(user.updated_at);
		avatarSrc = user.avatar && user.avatar !== '' ? (user.avatar.includes('data:image/') ? user.avatar : `/${user.avatar}`) : '/unknown_user.png';
	});

	function handleSave() {
		if (formData.has('data')) {
			formData.delete('data');
		}
		formData.append('data', JSON.stringify(user));
		patchUserProfile(formData).then((res: any) => {
			if (res) {
				data.isLoggedIn.message.avatar = avatarSrc;
				data.isLoggedIn.message.name = user.name;
				data.isLoggedIn.message.description = user.description;
				data.isLoggedIn.message.email = user.email;

				success = true;
			} else {
				failed = true;
			}
		});
	}
</script>

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
					<Button class="absolute top-0 right-0 h-full rounded-b-none rounded-tl-none" color="blue" size="sm" href={`/admin/profile/change-password`}>Change password</Button>
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
	<div class="grid grid-cols-1 p-4">
		<div class="flex flex-col gap-2 w-full">
			<Label for="created_at">Created at</Label>
			<Datepicker id="CreatedAt" label="Created at" placeholder="Select date" value={created_at} name="created_at" disabled />
			<Label for="updated_at">Updated at</Label>
			<Datepicker id="UpdatedAt" label="Updated at" placeholder="Select date" value={updated_at} name="updated_at" disabled />
		</div>
	</div>
	<Footer class="!bg-[#364153] m-4">
		<div class="flex gap-3"></div>
		<div class="flex flex-col fixed right-5 bottom-5 gap-3">
			{#if success}
				<Toast>
					{#snippet icon()}
						<FireOutline class="text-primary-500 bg-primary-100 dark:bg-primary-800 dark:text-primary-200 h-6 w-6" />
					{/snippet}
					User profile saved successfully.
				</Toast>
			{/if}
			{#if failed}
				<Toast color="red">
					{#snippet icon()}
						<CloseCircleSolid class="text-red-500 bg-red-100 dark:bg-red-800 dark:text-red-200 h-6 w-6" />
					{/snippet}
					Failed to save user profile. Please try again.
				</Toast>
			{/if}
		</div>
		<FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
			<div class="flex gap-3">
				<Button color="blue" onclick={handleSave}>Save</Button>
			</div>
		</FooterLinkGroup>
	</Footer>
</div>
