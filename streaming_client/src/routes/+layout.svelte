<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	// @ts-ignore
	import { Footer, FooterCopyright, FooterLinkGroup, FooterLink } from 'flowbite-svelte';

	import { page } from '$app/state';
	import { ClapperboardPlayOutline, DiscordSolid, FacebookSolid, HomeOutline, InfoCircleOutline, InstagramSolid, NewspaperOutline } from 'flowbite-svelte-icons';
	import { groupCoffeeLink, groupDiscord, groupFacebook, groupInstagram, groupName } from '$lib/index.ts';
	import Coffee from 'phosphor-svelte/lib/Coffee';
	import UsersThree from 'phosphor-svelte/lib/UsersThree';
	import Megaphone from 'phosphor-svelte/lib/Megaphone';

	let { children } = $props();
	// Mobile navbar scroll behavior
	let lastScrollY = 0;
	let showMobileNavbar = $state(true);
	const scrollSensitivity = 5; // Pixels to scroll before triggering hide/show

	onMount(() => {
		const handleScroll = () => {
			const currentScrollY = window.scrollY;
			const scrollDifference = Math.abs(currentScrollY - lastScrollY);

			// Only trigger if scroll difference is greater than sensitivity
			if (scrollDifference > scrollSensitivity) {
				if (currentScrollY > lastScrollY && currentScrollY > 100) {
					// Scrolling down and past threshold - hide navbar
					showMobileNavbar = false;
				} else if (currentScrollY < lastScrollY) {
					// Scrolling up - show navbar
					showMobileNavbar = true;
				}
				lastScrollY = currentScrollY;
			}
		};

		window.addEventListener('scroll', handleScroll, { passive: true });

		return () => {
			window.removeEventListener('scroll', handleScroll);
		};
	});
</script>

<!--<a href="/">Home</a>-->
<!--<a href="/admin">Admin</a>-->
<svelte:head>
	<link rel="preload" href="/background_2.png" as="image" />
</svelte:head>
{#if page.url.pathname.startsWith('/admin')}
	{@render children()}
{:else}
	<div class="min-h-screen bg-[#1a1919] relative z-1">
		<div class="absolute top-0 left-0 w-full h-full z-[-1] overflow-hidden">
			<div class="w-full h-full">
				<div class="w-full h-full bg-[url('/background_2.png')] bg-cover"></div>
			</div>
		</div>
		<div class="min-h-screen flex justify-center w-full">
			<div class="min-h-screen w-full flex items-center flex-col max-w-[1440px] bg-[#1f1f1f] relative z-10">
				<div class="absolute top-0 left-0 w-full h-full z-[-1] overflow-hidden">
					<div class="w-full h-full">
						<div class="w-full h-full bg-[url('/background_2.png')] bg-cover"></div>
					</div>
				</div>
				<div class="min-h-screen max-w-[1440px] w-full flex items-center flex-col relative">
					<div class="min-h-20 max-h-20 w-full grid sm:grid-cols-3 items-center text-white font-roboto">
						<div class="flex justify-center sm:justify-start xl:justify-center items-center w-full sm:pl-2 lg:pl-20 xl:pl-0">
							<a href="/" class="flex gap-5 items-center font-roboto">
								<img src="/logo.webp" class="h-15 w-15" alt="logo" />
								<span class="text-2xl font-light block sm:hidden">Teacup<span class="font-bold">Subs</span></span>
							</a>
						</div>
						<div class="sm:flex hidden justify-center items-center gap-10 md:gap-20 sm:text-2xl xl:text-3xl text-nowrap w-full ">
							<span class={`${page.url.pathname != '/' ? 'text-[#B3B3B3]' : 'text-white'}`}>
								<a href="/">Strona Główna</a>
							</span>
							<span class={`${!page.url.pathname.includes('/anime') ? 'text-[#B3B3B3]' : 'text-white'}`}>
								<a href="/anime"> Anime </a>
							</span>
							<span class={`${!page.url.pathname.includes('/news') ? 'text-[#B3B3B3]' : 'text-white'}`}>
								<a href="/news"> Ogłoszenia </a>
							</span>
							<span class={`${!page.url.pathname.includes('/about') ? 'text-[#B3B3B3]' : 'text-white'}`}>
								<a href="/about"> O nas </a>
							</span>
						</div>
						<div class="flex justify-center items-center sm:hidden">
							<!-- <span>BTN</span> -->
						</div>
					</div>
					{@render children()}
					<div
						class="fixed flex sm:hidden justify-center items-center bottom-10 p-10 h-20 bg-[#1f1f1f] shadow-around-outset border-[#1a1919] border-2 text-[12px] min-[390px]:text-sm text-white font-roboto rounded-full z-20 transition-all duration-300 ease-in-out"
						class:translate-y-0={showMobileNavbar}
						class:translate-y-24={!showMobileNavbar}
						class:opacity-100={showMobileNavbar}
						class:opacity-0={!showMobileNavbar}
					>
						<div class="grid grid-cols-4 gap-1 min-[410px]:text-sm text-[12px] font-roboto">
							<div class="flex justify-center items-center">
								<a href="/">
									<div class="flex flex-col items-center">
										<HomeOutline class="h-8 w-8" />
										<span>Główna</span>
									</div>
								</a>
							</div>
							<div class="flex justify-center items-center">
								<a href="/anime">
									<div class="flex flex-col items-center">
										<ClapperboardPlayOutline class="h-8 w-8" />
										<span>Anime</span>
									</div>
								</a>
							</div>
							<div class="flex justify-center items-center">
								<a href="/news">
									<div class="flex flex-col items-center">
										<Megaphone class="h-8 w-8" />
										<span>Ogłoszenia</span>
									</div>
								</a>
							</div>
							<div class="flex justify-center items-center">
								<a href="/about">
									<div class="flex flex-col items-center">
										<UsersThree class="h-8 w-8" />
										<span>O nas</span>
									</div>
								</a>
							</div>
						</div>
					</div>
				</div>
				<div class="w-full flex justify-center items-center !bg-[#ffffff00]">
					<Footer class="!bg-[#ffffff00] text-white font-roboto max-w-[1440px] w-full rounded-none">
						<div class="flex gap-5">
							<img src="/logo.webp" alt={`${groupName} Logo`} class="h-15 w-15 sm:h-20 sm:w-20" />
							<div class="flex flex-col justify-center">
								<span>{groupName}</span>
								<span>Wszelkie prawa zastrzeżone</span>
							</div>
						</div>
						<FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
							<FooterLink href={groupFacebook}><FacebookSolid class="h-5 w-5 sm:h-10 sm:w-10" /></FooterLink>
							<FooterLink href={groupInstagram}><InstagramSolid class="h-5 w-5 sm:h-10 sm:w-10" /></FooterLink>
							<FooterLink href={groupDiscord}><DiscordSolid class="h-5 w-5 sm:h-10 sm:w-10" /></FooterLink>
							<FooterLink href={groupCoffeeLink}><Coffee class="h-5 w-5 sm:h-10 sm:w-10" /></FooterLink>
						</FooterLinkGroup>
					</Footer>
				</div>
			</div>
		</div>
	</div>
{/if}
