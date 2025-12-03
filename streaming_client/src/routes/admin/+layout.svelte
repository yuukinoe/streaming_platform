<script lang="ts">
	// @ts-ignore
	import { Sidebar, SidebarGroup, SidebarItem, SidebarDropdownWrapper, SidebarButton, uiHelpers, Input, Label, Button, Alert } from 'flowbite-svelte';
	import { ChartOutline, GridSolid, AdjustmentsHorizontalOutline, RectangleListOutline, EditSolid, UserSolid, ClapperboardPlayOutline, NewspaperOutline, UsersGroupOutline, VideoCameraOutline, AwardOutline, NewspaperSolid } from 'flowbite-svelte-icons';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { logoutUser } from '$lib/data/admin/post.ts';
	import { hasPermission } from '$lib/permissions.js';
	// import { PlusPlaceholder } from "flowbite-svelte-icons"
	let activeUrl = $state(page.url.pathname);
	// import PlusPlaceholder from '../../utils/PlusPlaceholder.svelte';

	let { data, children } = $props();

	const spanClass = 'flex-1 ms-3 whitespace-nowrap';
	const SidebarUi = uiHelpers();
	let isOpen = $state(false);
	const closeSidebar = SidebarUi.close;

	let loggedIn = $state(false);
	onMount(async () => {
		let logginData = data.isLoggedIn;
		if (logginData.status !== 200) {
			loggedIn = false;
		} else {
			loggedIn = true;
			setTimeout(() => {
				const sidebar = document.querySelector('.sidebar-with-user-info');
				if (sidebar) {
					// Check if user info already exists
					if (!sidebar.querySelector('.user-info-injected')) {
						const userInfoDiv = document.createElement('div');
						userInfoDiv.className = 'bg-black text-white p-4 font-mono border-b-2 border-[#313131] user-info-injected flex flex-col items-center';
						userInfoDiv.innerHTML = `
							Logged in as<br>
							<span class="text-white font-mono">${data.isLoggedIn.message.name}${data.isLoggedIn.message.is_superuser ? ' (Superuser)' : ''}</span>
						`;

						// Insert as the first child of the sidebar
						sidebar.insertBefore(userInfoDiv, sidebar.firstChild);
					}
				}
			}, 100);
		}
	});

	const logout = async () => {
		await logoutUser();
		window.location.href = '/admin/login';
	};

	function toggleSidebar() {
		SidebarUi.toggle();
		// isOpen = true;
	}
	$effect(() => {
		isOpen = SidebarUi.isOpen;
		activeUrl = page.url.pathname;
	});
</script>

{#if data.isLoggedIn.status !== 200}
	{@render children()}
{:else}
	<div class="bg-[#1e1e1e] min-h-screen">
		<div class="border-b-[#313131] border-b-2 shadow-xl relative z-50">
			<div class="h-20 flex justify-between items-center mx-10">
				<div class="flex gap-3">
					<div class="flex items-cente md:hidden">
						<SidebarButton onclick={toggleSidebar} class="!block" />
					</div>
					<div class="flex items-center gap-4">
						<a href="/admin">
							<img alt="logo" src="/logo.png" class="h-10" />
						</a>
						<!-- <span class="text-white font-mono">Panel administracyjny {groupName}</span> -->
					</div>
				</div>
				<div class="flex items-center gap-4">
					<Button color="red" class="cursor-pointer" onclick={logout}>Logout</Button>
				</div>
			</div>
		</div>

		<div class="relative">
			<Sidebar {activeUrl} backdrop={false} {isOpen} {closeSidebar} params={{ x: -50, duration: 50 }} position="absolute" activeClass="" nonActiveClass="" class="z-40 h-full min-h-screen sidebar-with-user-info border-r-2 border-[#313131] bg-[#1e2939] text-white">
				<SidebarGroup class="">
					{#if hasPermission(data.isLoggedIn.message, 'users', 'read') || hasPermission(data.isLoggedIn.message, 'logs', 'read') || hasPermission(data.isLoggedIn.message, 'roles', 'read') || hasPermission(data.isLoggedIn.message, 'news_categories', 'read')}
						<SidebarDropdownWrapper label="Administration" btnClass="p-2">
							{#snippet icon()}
								<AdjustmentsHorizontalOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
							{/snippet}
							{#if hasPermission(data.isLoggedIn.message, 'users', 'read')}
								<SidebarItem label="Users" href="/admin/administration/users">
									{#snippet icon()}
										<UsersGroupOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
							{#if hasPermission(data.isLoggedIn.message, 'logs', 'read')}
								<SidebarItem label="Logs" href="/admin/administration/logs">
									{#snippet icon()}
										<NewspaperOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
							{#if hasPermission(data.isLoggedIn.message, 'roles', 'read')}
								<SidebarItem label="Roles" href="/admin/administration/roles">
									{#snippet icon()}
										<AwardOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
							{#if hasPermission(data.isLoggedIn.message, 'news_categories', 'read')}
								<SidebarItem label="News category" href="/admin/administration/categoryarticles">
									{#snippet icon()}
										<NewspaperSolid class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
						</SidebarDropdownWrapper>
					{/if}
					{#if hasPermission(data.isLoggedIn.message, 'anime', 'read') || hasPermission(data.isLoggedIn.message, 'episodes', 'read') || hasPermission(data.isLoggedIn.message, 'news', 'read')}
						<SidebarDropdownWrapper label="Management" btnClass="p-2">
							{#snippet icon()}
								<EditSolid class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
							{/snippet}
							{#if hasPermission(data.isLoggedIn.message, 'anime', 'read')}
								<SidebarItem label="Anime" href="/admin/management/anime">
									{#snippet icon()}
										<ClapperboardPlayOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
							{#if hasPermission(data.isLoggedIn.message, 'episodes', 'read')}
								<SidebarItem label="Episodes" href="/admin/management/episodes">
									{#snippet icon()}
										<VideoCameraOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
							{#if hasPermission(data.isLoggedIn.message, 'news', 'read')}
								<SidebarItem label="News" href="/admin/management/news">
									{#snippet icon()}
										<NewspaperOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
									{/snippet}
								</SidebarItem>
							{/if}
						</SidebarDropdownWrapper>
					{/if}

					<SidebarItem label="Profile" href="/admin/profile">
						{#snippet icon()}
							<UserSolid class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
						{/snippet}
					</SidebarItem>
				</SidebarGroup>
			</Sidebar>
			<div class="min-h-screen overflow-auto px-4 md:ml-64 shadow-content">
				<div class="rounded-lg border-2 mt-5 border-dashed border-gray-200 p-4 dark:border-gray-700 h-full">
					<div>
						{@render children()}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
