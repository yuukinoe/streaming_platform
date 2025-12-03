<script lang="ts">
	// @ts-ignore
	import { Avatar, Tooltip } from 'flowbite-svelte';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import NotFound from '$lib/data/client/components/404.svelte';
	import ServerError from '$lib/data/client/components/500.svelte';
	import { onMount } from 'svelte';
	import type { Member, UserPublic } from '$lib/data/client/types.ts';
	import { apiUrl, groupCoffeeLink } from '$lib/index.ts';
	import type { PageProps } from './$types.js';
	import { hexToRGB } from '$lib/utils.ts';
	import Coffee from 'phosphor-svelte/lib/Coffee';
	const { data }: PageProps = $props();

	let isLoading = $state(true);
	let is404 = $state(false);
	let is500 = $state(false);
	let isMembersTabOpen = $state(true);
	let users = $state([]) as Member[];

	// Szerokość okna (globalnie)
	let width = $state(0);

	// Aktualizacja przy resize
	$effect(() => {
		const update = () => {
			width = window.innerWidth;
		};
		update();
		window.addEventListener('resize', update);
		return () => window.removeEventListener('resize', update);
	});

	// Breakpoint na podstawie szerokości
	const breakpoint = $derived(() => {
		// if (width >= 1280) return 'xl';
		// if (width >= 1024) return 'lg';
		// if (width >= 768) return 'md';
		// if (width >= 640) return 'sm';
		if (width >= 430) return 'base';
		return 'sm';
	});

	// Funkcja do generowania gradientu dla danego koloru
	function generateGradient(color: string, breakpoint: 'xs' | 'sm' | 'base' | 'md' | 'lg' | 'xl'): string {
		const stopMap: Record<typeof breakpoint, number> = {
			xs: 10,
			sm: 15,
			base: 20,
			md: 25,
			lg: 30,
			xl: 35
		};
		const stop = stopMap[breakpoint];
		return `linear-gradient(180deg, #1f1f1f00 0%, #1f1f1f00 ${stop}%, ${color} ${stop}%, ${color} 100%)`;
	}

	onMount(async () => {
		try {
			data.users.forEach((user: UserPublic) => {
				if (user.slug === 'byly-czlonek') {
					// console.log(user);
				}
				users.push({
					id: user.id,
					name: user.name,
					avatar: user.avatar,
					description: user.description,
					translator: user.translator,
					proofreader: user.proofreader,
					uploader: user.uploader,
					editor: user.editor,
					roles: user.roles,
					is_active: user.is_active,
					administrative_role: Array.isArray(user.roles) ? user.roles.find((role) => role.administrative_role) : undefined,
					slug: user.slug
				} as Member);
			});

			users = users.sort((a, b) => {
				if (a.id.id.String === 'bd39ismp65vkeq9g0l37') {
					return -1;
				}
				if (b.id.id.String === 'bd39ismp65vkeq9g0l37') {
					return 1;
				}

				if (!a.roles || a.roles.length === 0) {
					return 1;
				}
				if (!b.roles || b.roles.length === 0) {
					return -1;
				}

				const aMinHierarchy = Math.min(...a.roles.map((role) => role.hierarchy));
				const bMinHierarchy = Math.min(...b.roles.map((role) => role.hierarchy));

				return aMinHierarchy - bMinHierarchy;
			});

			// Move the special user to second position
			const specialUserIndex = users.findIndex((user) => user.id.id.String === 'bd39ismp65vkeq9g0l37');
			if (specialUserIndex !== -1 && specialUserIndex !== 1) {
				const specialUser = users.splice(specialUserIndex, 1)[0];
				users.splice(1, 0, specialUser);
			}

			isLoading = false;
		} catch (error) {
			if (error instanceof Error) {
				if (error.message.includes('404')) {
					is404 = true;
				} else if (error.message.includes('500')) {
					is500 = true;
				} else {
					console.error('Unexpected error:', error);
				}
			}
		}
	});
</script>

<svelte:head>
	<title>teacup - O nas</title>
	<meta property="og:title" content="teacup - O nas" />
	<meta property="og:description" content="teacup - oglądaj i ciesz się każdą chwilą." />
	<meta property="og:url" content="https://teacup.pl/about" />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="teacup" />
	<meta property="og:locale" content="pl_PL" />
</svelte:head>

<div class="w-full h-full">
	{#if is500}
		<ServerError />
	{:else if is404}
		<NotFound />
	{:else if isLoading}
		<Loading />
	{:else}
		<div class="flex flex-col gap-10 [&>div]:[&>div]:mx-3 [&>div]:[&>div]:sm:mx-10 [&>div]:[&>div]:flex [&>div]:[&>div]:flex-col [&>div]:[&>div]:gap-8">
			<div class="bg-[#0E131CA6] mix-blend-plus-lighter h-15 sm:flex items-center hidden">
				<div>
					<div class="text-[#EEEDF0B2] font-roboto font-[400] flex gap-1">
						<div class="text-[20px]">O grupie</div>
					</div>
				</div>
			</div>
			<div>
				<div class="pb-20">
					<!-- <div class="flex items-center justify-center text-[#EEEDF0] font-inter font-semibold">
						<div class="grid grid-cols-2 [&>button]:cursor-pointer">
							<button class={`p-3 pl-5 transition-all rounded-l-full ${isMembersTabOpen ? 'bg-[#832121]' : 'bg-[#2f2f2f]'}`} onclick={() => (isMembersTabOpen = true)}>
								<span> Członkowie </span>
							</button>
							<button class={`p-3 pr-5 transition-all rounded-r-full  ${isMembersTabOpen ? 'bg-[#2f2f2f]' : 'bg-[#832121]'}`} onclick={() => (isMembersTabOpen = false)}>
								<span> Informacje </span>
							</button>
						</div>
					</div> -->

					<div class="flex flex-col gap-3 text-white bg-[#0E131CA6] mix-blend-plus-lighter p-10 rounded-2xl font-roboto">
						<div class="flex justify-between">
							<span class="text-4xl font-bold">O nas</span>
						</div>
						<div class="flex flex-col gap-2">
							<span class="text-xl">
								Nietypowa mieszanka ludzi z całkowicie odmiennych części Polski - od bezrobotnych nastolatków, przez zabieganych studentów, aż po (teoretycznie) dorosłych z poważnymi pracami. Szczerze mówiąc różni nas praktycznie wszystko, ale miłość do anime zebrała nas razem, byśmy mogli dołożyć własną cegiełkę dla tej społeczności. Chcemy
								dostarczać wam odcinków na najwyższym poziomie i przy tym dobrze się bawić. Dzięki cudownej atmosferze na grupie zawarcie nowych znajomości przychodzi samo, a wspierając się nawzajem ciągle stajemy się lepsi.
							</span>
							<div class="flex w-full justify-end">
								<a target="_blank" href={groupCoffeeLink} class="p-1 border-4 rounded-3xl px-5 flex items-center gap-3 max-w-[190px] hover:rounded-md transition-all" style={`background: ${hexToRGB('#50bc54')}; border-color: #50bc54;`}>
									<Coffee class="h-7 w-7" />
									<span class="text-white font-medium font-roboto text-sm">Postaw kawkę!</span>
								</a>
							</div>
						</div>
					</div>
					{#if isMembersTabOpen}
						<div class="flex gap-3 items-center text-white font-roboto">
							<div class="border-l-6 border-l-red-700 h-10"></div>
							<span class="text-4xl select-none">CZŁONKOWIE</span>
						</div>
						<div>
							<div class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 justify-around w-full gap-10">
								{#each users as user (user.id.id.String)}
									{#if user}
										{#if user.is_active}
											<a href={`/about/user/${user.slug}`} class={`pb-10 rounded-b-3xl mix-blend-plus-lighter hover:scale-101 transition-all`} style={`background: ${generateGradient('#171f2d99', breakpoint())};`}>
												<div class="flex flex-col gap-5">
													<div class="flex justify-center items-center gap-3 px-5">
														<img src={`${user.avatar ? `/${user.avatar}` : `/unknown_user.png`}`} alt={user.name} class="rounded-full w-full lg:max-w-[80%] aspect-square" />
													</div>
													<div class="flex justify-center items-center text-center font-inder">
														<div class="flex flex-col gap-5">
															<div class="flex flex-col gap-1">
																<span class="text-[#EEEDF0] text-lg lg:text-2xl xl:text-3xl font-bold">
																	{user.name}
																</span>
																<span class="text-[#EEEDF0CC] text-sm sm:text-base md:text-lg lg:text-xl">
																	{#if user.administrative_role}
																		{user.administrative_role.name}
																	{/if}
																</span>
															</div>
															<div class="flex flex-col gap-3">
																<!-- <span
															class="text-[#EEEDF0CC] text-xs md:text-sm lg:text-base leading-[1.2rem]"
														>
															{#if user.roles}
																{#each user.roles
																	.filter((r) => !r.administrative_role)
																	.slice(0, 4) as _, index (index)}
																	{#if index % 2 === 0}
																		{user.roles.filter((r) => !r.administrative_role).slice(0, 4)[
																			index
																		]?.name}
																		{#if user.roles
																			.filter((r) => !r.administrative_role)
																			.slice(0, 4)[index + 1]}
																			/ {user.roles
																				.filter((r) => !r.administrative_role)
																				.slice(0, 4)[index + 1].name}
																		{/if}
																		<br />
																	{/if}
																{/each}
															{/if}
														</span> -->

																<div class="">
																	{#if user.roles}
																		<div class="grid grid-cols-2 gap-2">
																			{#each user.roles.filter((r) => !r.administrative_role).slice(0, 4) as role, index (index)}
																				<button class="rounded-full border-5 flex justify-center items-center aspect-square p-3 hover:rotate-[20deg] hover:bg-[#ffffff26] transition-all" style={`background: ${hexToRGB(role.color)}; border-color: ${role.color}`}>
																					<img alt="" draggable="false" class="sm:w-13 sm:h-13 min-[330px]:w-8 min-[330px]:h-8 h-3 w-3 select-none" src={`/${role.icon}`} />
																				</button>
																				<Tooltip>{role.name}</Tooltip>
																			{/each}
																		</div>
																	{/if}
																</div>

																<!-- <div class="">
															{#if user.roles}
																<div class="grid grid-cols-2 gap-2">
																	{#each user.roles.filter((r) => !r.administrative_role).slice(0, 4) as _, index (index)}
																		{#if index % 2 === 0}
																			<button class="rounded-full border-3 flex justify-center items-center aspect-square p-3 hover:rotate-[20deg] hover:bg-[#ffffff26] transition-all" style={`background: ${hexToRGB(user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index]?.color)}; border-color: ${user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index]?.color}`}>
																				<img alt="" draggable="false" class="sm:w-15 sm:h-15 min-[330px]:w-10 min-[330px]:h-10 h-5 w-5 select-none" src={`/${user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index]?.icon}`} />
																			</button>
																			<Tooltip>{user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index]?.name}</Tooltip>
																			{#if user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index + 1]}
																				<button class="rounded-full border-3 flex justify-center items-center aspect-square p-3 hover:rotate-[-20deg] hover:bg-[#ffffff26] transition-all" style={`background: ${hexToRGB(user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index + 1].color)}; border-color: ${user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index + 1].color}`}>
																					<img alt="" draggable="false" class="sm:w-15 sm:h-15 min-[330px]:w-10 min-[330px]:h-10 h-5 w-5" src={`/${user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index + 1].icon}`} />
																				</button>
																				<Tooltip>{user.roles.filter((r) => !r.administrative_role).slice(0, 4)[index + 1].name}</Tooltip>
																			{/if}
																		{/if}
																	{/each}
																</div>
															{/if}
														</div> -->
															</div>
														</div>
													</div>
												</div>
											</a>
										{/if}
									{/if}
								{/each}
							</div>
						</div>
					{:else}
						<div class="flex gap-3 items-center text-white font-roboto">
							<div class="border-l-6 border-l-red-700 h-10"></div>
							<span class="text-4xl">INFORMACJE</span>
						</div>
						<!-- <PlayOutline /> -->
						<div class="items-center justify-center flex font-inder">
							<div class="flex flex-col gap-[7rem]">
								<div class="flex gap-3">
									<div class="flex items-center justify-center">
										<img src={`/star.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:sm:text-2xl text-lg">Kim jesteśmy?</span>
										<span class="text-[#EEEDF0CC] text-sm sm:text-base">Jesteśmy grupą pasjonatów anime, którzy poświęcają swój wolny czas na tworzenie wysokiej jakości polskich napisów, aby fani mogli cieszyć się swoimi ulubionymi seriami.</span>
									</div>
								</div>
								<div class="flex gap-3 flex-row-reverse text-right">
									<div class="flex items-center justify-center">
										<img src={`/people.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full h-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:text-2xl text-lg">Jaką mamy ekipę?</span>
										<span class="text-[#EEEDF0CC] sm:text-base text-sm">W naszej grupie znajdują się osoby o różnorodnych zainteresowaniach i umiejętnościach. Każdy ma swoje ulubione gatunki i serie, co pozwala nam na tłumaczenie szerokiego wachlarza tytułów. </span>
									</div>
								</div>

								<div class="flex gap-3">
									<div class="flex items-center justify-center">
										<img src={`/medal.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full h-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:text-2xl text-lg">Co jest celem grupy?</span>
										<span class="text-[#EEEDF0CC] sm:text-base text-sm">W naszej grupie znajdują się osoby o różnorodnych zainteresowaniach i umiejętnościach. Każdy ma swoje ulubione gatunki i serie, co pozwala nam na tłumaczenie szerokiego wachlarza tytułów. </span>
									</div>
								</div>

								<div class="flex gap-3 flex-row-reverse text-right">
									<div class="flex items-center justify-center">
										<img src={`/chat.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full h-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:text-2xl text-lg">Jaki mamy stosunek do naszej społeczności?</span>
										<span class="text-[#EEEDF0CC] sm:text-base text-sm">Jesteśmy otwarci na sugestie i prośby fanów, co pozwala nam lepiej dopasowywać się do oczekiwań naszej społeczności. </span>
									</div>
								</div>

								<div class="flex gap-3">
									<div class="flex items-center justify-center">
										<img src={`/magnifier.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full h-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:text-2xl text-lg">Jak tworzymy odcinki?</span>
										<div class="flex flex-col text-[#EEEDF0CC] sm:text-base text-sm">
											<div class="flex">
												<span>1.</span>
												<span>Tłumaczenie: Surowe napisy angielskie są przekładane na język polski przez naszych tłumaczy.</span>
											</div>
											<div class="flex">
												<span>2.</span>
												<span>Korekta: Korektorzy sprawdzają każdą linijkę, aby wyeliminować błędy i poprawić styl.</span>
											</div>
											<div class="flex">
												<span>3.</span>
												<span>Stylizacja: Dbamy o wygląd napisów - dobór czcionek, kolorów i formatowania, aby były estetyczne i czytelne.</span>
											</div>
										</div>
									</div>
								</div>

								<div class="flex gap-3 flex-row-reverse text-right">
									<div class="flex items-center justify-center">
										<img src={`/heart.webp`} alt="star" class="max-w-[150px] max-h-[150px] min-w-full h-full" />
									</div>
									<div class="min-h-full min-w-[3px] w-[3px] bg-[#555555]"></div>
									<div class="flex flex-col text-white max-w-[400px] justify-center gap-2">
										<span class="sm:text-2xl text-lg">Znasz naszą misję?</span>
										<span class="text-[#EEEDF0CC] sm:text-base text-sm">Chcemy nadal dzielić się naszą miłością do anime z polskimi fanami, tłumacząc i dostarczając napisy do kolejnych sezonów ulubionych serii. </span>
									</div>
								</div>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>
