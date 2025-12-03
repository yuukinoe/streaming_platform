<script lang="ts">
	// @ts-ignore
	import { Input, Label, Button, Alert } from 'flowbite-svelte';
	import { goto } from '$app/navigation';
	import { loginUser } from '$lib/data/admin/post.ts';

	let loginError = $state(false);
	// function debugUserData() {
	// 	console.log('Username value:', userData.username);
	// 	console.log('Password value:', userData.password ? '***' : 'empty');
	// 	console.log('Username length:', userData.username.length);
	// 	console.log('Password length:', userData.password.length);
	// }
	let { data } = $props();

	let userData = $state({
		username: '',
		password: ''
	});
	const login = async () => {
		const loginData = {
			username: userData.username,
			password: userData.password
		};
		await loginUser(loginData).then((response) => {
			if (response) {
				window.location.href = '/admin';
			} else {
				loginError = true;
			}
		});
		return;
	};
</script>

<div class="flex items-center justify-center bg-[#1e1e1e] min-h-screen">
	<!-- <form method="POST" use:enhance> -->
	<div class="p-3 bg-[#1e2939] rounded-lg shadow-lg w-96 flex flex-col gap-5">
		<div>
			{#if loginError}
				<Alert color="red">
					<span class="font-medium">Wrong login credentials!</span>
					Provided credentials do not match any user in the database. Please try again.
				</Alert>
			{/if}
		</div>
		<div>
			<Label for="username" class="mb-2">Username</Label>
			<Input bind:value={userData.username} type="text" id="username" placeholder="Username..." required />
		</div>
		<div>
			<Label for="password" class="mb-2">Password</Label>
			<Input
				bind:value={userData.password}
				type="password"
				id="password"
				placeholder="Password..."
				onkeydown={(e: KeyboardEvent) => {
					if (e.key === 'Enter') {
						login();
					}
				}}
				required
			/>
		</div>
		<div class="flex justify-end gap-3">
			<!-- <Button
					color="yellow"
					onclick={debugUserData}
				>
					Debug
				</Button> -->
			<Button
				color="blue"
				class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
				onclick={() => {
					login();
				}}
			>
				Login
			</Button>
		</div>
	</div>
	<!-- </form> -->
</div>
