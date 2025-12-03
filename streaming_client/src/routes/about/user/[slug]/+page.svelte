<script lang="ts">
	// @ts-ignore
	import { Indicator } from 'flowbite-svelte';
	import Loading from '$lib/data/client/components/Loading.svelte';
	import type { Anime, PublicRole, UserPublic } from '$lib/data/client/types.ts';
	import type { PageProps } from './$types.js';
	import { onMount } from 'svelte';
	import { hexToRGB } from '$lib/utils.ts';
	let { data }: PageProps = $props();
	let userData = $state() as UserPublic;
	let isLoading = $state(true);
	let avatarBackgroundColor = $state() as string;
	let animeCount = $state(0);
	let episodesCount = $state(0);
	let currentAnimeProjects = $state<Anime[]>([]);

	interface AnimeWithRolesInIt {
		anime: Anime;
		roles: string[];
	}

	const animeWithRolesInIt = $state<AnimeWithRolesInIt[]>([]);
	let roles = $state<PublicRole[]>([]);
	onMount(async () => {
		userData = data.currentUser as UserPublic;
		avatarBackgroundColor = userData.roles && userData.roles.length > 0 ? userData.roles[0].color : '#fff';

		// animeCount = data.episodes.filter((episode) => {
		//     episode.translators.some((translator) => translator.user_id === userData.id)
		//     episode.proofreaders?.includes()
		// });

		roles = (data.currentUser?.roles || []).sort((a, b) => a.hierarchy - b.hierarchy);

		// Populate animeWithRolesInIt array
		data.anime.forEach((anime) => {
			const episodes = data.episodes.filter((episode) => episode.anime.id.id.String === anime.id.id.String);
			const userRoles: string[] = [];

			// Check if user worked on any episodes of this anime
			const hasWorkedOnAnime = episodes.some((episode) => {
				let hasRole = false;

				if (episode.translators?.some((translator) => translator.id.id.String === userData.id.id.String)) {
					userRoles.push('Tłumacz');
					hasRole = true;
				}
				if (episode.proofreaders?.some((proofreader) => proofreader.id.id.String === userData.id.id.String)) {
					userRoles.push('Korektor');
					hasRole = true;
				}
				if (episode.uploaders?.some((uploader) => uploader.id.id.String === userData.id.id.String)) {
					userRoles.push('Uploader');
					hasRole = true;
				}
				if (episode.typesetters?.some((editor) => editor.id.id.String === userData.id.id.String)) {
					userRoles.push('Typesetter');
					hasRole = true;
				}

				return hasRole;
			});

			if (hasWorkedOnAnime) {
				// Remove duplicates from roles array
				const uniqueRoles = [...new Set(userRoles)];
				animeWithRolesInIt.push({
					anime: anime,
					roles: uniqueRoles
				});
			}
		});

		episodesCount = data.episodes.reduce((acc, episode) => {
			if (episode.translators?.some((translator) => translator.id.id.String === userData.id.id.String)) {
				return acc + 1;
			} else if (episode.proofreaders?.some((proofreader) => proofreader.id.id.String === userData.id.id.String)) {
				return acc + 1;
			} else if (episode.uploaders?.some((uploader) => uploader.id.id.String === userData.id.id.String)) {
				return acc + 1;
			} else if (episode.typesetters?.some((editor) => editor.id.id.String === userData.id.id.String)) {
				return acc + 1;
			}

			return acc;
		}, 0);

		animeCount = data.anime.reduce((acc, anime) => {
			const episodes = data.episodes.filter((episode) => episode.anime.id.id.String === anime.id.id.String);

			if (episodes.some((episode) => episode.translators?.some((translator) => translator.id.id.String === userData.id.id.String))) {
				if (anime.in_progress) {
					currentAnimeProjects.push(anime);
				}
				return acc + 1;
			} else if (episodes.some((episode) => episode.proofreaders?.some((proofreader) => proofreader.id.id.String === userData.id.id.String))) {
				if (anime.in_progress) {
					currentAnimeProjects.push(anime);
				}
				return acc + 1;
			} else if (episodes.some((episode) => episode.uploaders?.some((uploader) => uploader.id.id.String === userData.id.id.String))) {
				if (anime.in_progress) {
					currentAnimeProjects.push(anime);
				}
				return acc + 1;
			} else if (episodes.some((episode) => episode.typesetters?.some((editor) => editor.id.id.String === userData.id.id.String))) {
				if (anime.in_progress) {
					currentAnimeProjects.push(anime);
				}
				return acc + 1;
			}

			return acc;
		}, 0);

		// console.log(animeCount);
		// console.log(currentAnimeProjects);
		// console.log(animeWithRolesInIt);

		isLoading = false;
	});
</script>

<svelte:head>
	<title>
		teacup - {data.currentUser?.name || 'Użytkownik'}"
	</title>
	<meta property="og:title" content="teacup - {data.currentUser?.name || 'Użytkownik'}" />
	<meta property="og:description" content={data.currentUser?.description || 'Kto wie.'} />
	<meta property="og:url" content="https://teacup.pl/about/user/{data.currentUser?.slug || 'unknown'}" />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="teacup" />
	<meta property="og:locale" content="pl_PL" />
</svelte:head>

{#if isLoading}
	<div class="w-full flex justify-center items-center">
		<Loading />
	</div>
{:else}
	<div class="flex flex-col gap-10 w-full mt-20 mb-20">
		<div class="w-full">
			<div class="sm:mx-10 mx-3 flex flex-col gap-6 items-start">
				<div class="grid grid-cols-1 md:grid-cols-2 items-start w-full gap-4">
					<div class="w-full flex flex-col gap-3 h-full">
						<div class="flex gap-5 w-full">
							<div class={`p-1.5 rounded-full max-h-[150px] max-w-[150px] w-full h-full  aspect-square brightness-125`} style={`background: ${avatarBackgroundColor}; box-shadow: 0px 0px 17px 0px ${avatarBackgroundColor}; -webkit-box-shadow: 0px 0px 17px 0px ${avatarBackgroundColor}; -moz-box-shadow: 0px 0px 17px 0px ${avatarBackgroundColor};`}>
								<!-- <img draggable="false" src={`${userData.avatar ? `/${userData.avatar}` : `/unknown_user.png`}  `} alt="avatar" class="w-full rounded-full brightness-75" /> -->
								<div class="w-full rounded-full brightness-75 aspect-square" style={`background-image: url(${userData.avatar ? `/${userData.avatar}` : `/unknown_user.png`}); background-size: cover; background-position: center;`}></div>
							</div>

							<div class="p-5 px-8 flex flex-col gap-1 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl">
								<div>
									<span class="text-white font-bold text-2xl font-roboto">{userData.name}</span>
								</div>
								{#if userData.is_active}
									<div class="flex items-center gap-2 text-xs">
										<Indicator class="w-[5px] h-[5px]" color="green" />
										<span class="font-semibold font-roboto text-[#cecece]"> Aktywny członek</span>
									</div>
								{:else}
									<div class="flex items-center gap-2 text-xs">
										<Indicator class="w-[5px] h-[5px]" color="red" />
										<span class="font-semibold font-roboto text-[#cecece]"> Nieaktywny członek</span>
									</div>
								{/if}
							</div>
						</div>
						<div class="p-5 px-8 flex flex-col gap-1 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl h-full">
							<span class="font-roboto font-regular text-[15px] text-white">
								{userData.description ? userData.description : 'Tutaj będzie opis..'}
							</span>
						</div>
					</div>
					<div class="flex flex-col items-stretch gap-3 h-full">
						<div class="p-5 px-8 flex flex-col gap-1 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl">
							<span class="text-white font-bold text-[20px] font-roboto">Role</span>
							<div class="flex gap-2 flex-wrap max-[330px]:flex-col w-full">
								{#each roles as role}
									<div class="p-1 border-2 rounded-3xl px-5 flex items-center gap-3" style={`background: ${hexToRGB(role.color)}; border-color: ${role.color};`}>
										<img draggable="false" src={`/${role.icon}`} alt="role" class="max-w-[20px] max-h-[20px] select-none" />
										<span class="text-white font-medium font-roboto text-xs select-none">{role.name}</span>
									</div>
								{/each}
							</div>
						</div>
						<div class="p-5 px-8 flex flex-col gap-5 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl h-full">
							<span class="text-white font-bold text-[20px] font-roboto">Statystyki</span>
							<div class="flex justify-around text-center">
								<div class="flex flex-col">
									<span class="text-white font-bold text-[32px] font-roboto">{animeCount}</span>
									<span class="text-[#6F737C] font-medium text-[16px] font-roboto">Ukończonych serii</span>
								</div>
								<div class="flex flex-col">
									<span class="text-white font-bold text-[32px] font-roboto">{episodesCount}</span>
									<span class="text-[#6F737C] font-medium text-[16px] font-roboto">Zrobionych odcinków</span>
								</div>
								<div class="flex flex-col">
									<span class="text-white font-bold text-[32px] font-roboto">{userData.created_at ? new Date(userData.created_at).getFullYear() : 'Kto wie'}</span>
									<span class="text-[#6F737C] font-medium text-[16px] font-roboto">Jest z nami od</span>
								</div>
							</div>
						</div>
					</div>
				</div>
				<div class="w-full flex flex-col gap-6">
					{#if currentAnimeProjects.length > 0}
						<div class="p-5 px-8 flex flex-col gap-5 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl h-full">
							<div class="flex items-center gap-[10px]">
								<img src="/fat_arrow_right.svg" alt="arrow" class="min-w-6 min-h-6" />
								<span class="text-white text-xl font-roboto font-bold">Aktualne projekty</span>
							</div>
							<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
								{#each animeWithRolesInIt as animeWithRoles}
									{#if animeWithRoles.anime.in_progress}
										<a
											href="/anime/{animeWithRoles.anime.slug}"
											class="!bg-cover bg-no-repeat w-full rounded-xl px-3 py-2 flex justify-between hover:brightness-125 transition-all"
											style={`background: linear-gradient(0deg, rgba(25, 30, 39, 0.69) 0%, rgba(25, 30, 39, 0.69) 100%), url('/${animeWithRoles.anime.image}'); background-position-y: ${animeWithRoles.anime.background_position + 5}% ;`}
										>
											<div class="flex flex-col gap-1">
												<span class="font-roboto text-white font-semibold text-base line-clamp-1">{animeWithRoles.anime.title}</span>

												<span class="font-roboto text-[#b2b2b2] font-regular text-xs line-clamp-1">{animeWithRoles.roles.join(' • ')} • Odc 1-{animeWithRoles.anime.episodes}</span>
											</div>
											<div class="flex items-center justify-center">
												<img src="/arrow_right.svg" alt="arrow" class="min-w-6 min-h-6" />
											</div>
										</a>
									{/if}
								{/each}
							</div>
						</div>
					{/if}
					{#if animeWithRolesInIt.length > 0}
						<div class="p-5 px-8 flex flex-col gap-5 bg-[#0E131CA6] mix-blend-plus-lighter w-full rounded-2xl h-full">
							<div class="flex items-center gap-[10px]">
								<img src="/fat_arrow_right.svg" alt="arrow" class="min-w-6 min-h-6" />
								<span class="text-white text-xl font-roboto font-bold">Wszystkie projekty</span>
							</div>
							<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
								{#each animeWithRolesInIt as animeWithRoles}
									<a
										href="/anime/{animeWithRoles.anime.slug}"
										class="!bg-cover bg-no-repeat w-full rounded-xl px-3 py-2 flex justify-between hover:brightness-125 transition-all"
										style={`background: linear-gradient(0deg, rgba(25, 30, 39, 0.69) 0%, rgba(25, 30, 39, 0.69) 100%), url('/${animeWithRoles.anime.image}'); background-position-y: ${animeWithRoles.anime.background_position + 5}% ;`}
									>
										<div class="flex flex-col gap-1">
											<span class="font-roboto text-white font-semibold text-base line-clamp-1">{animeWithRoles.anime.title}</span>

											<span class="font-roboto text-[#b2b2b2] font-regular text-xs line-clamp-1">{animeWithRoles.roles.join(' • ')} • Odc 1-{animeWithRoles.anime.episodes}</span>
										</div>
										<div class="flex items-center justify-center">
											<img src="/arrow_right.svg" alt="arrow" class="min-w-6 min-h-6" />
										</div>
									</a>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
