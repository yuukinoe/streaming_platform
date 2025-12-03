// import { apiUrl } from '$lib/index.ts';
// import type { Anime, EpisodesPublic, UserPublic } from '$lib/data/client/types.ts';

// export const getAnimeList = async () => {
// 	const response = await fetch(`${apiUrl}/anime`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error(`${response.status}`);
// 	}

// 	return (await response.json()) as Anime[];
// };

// export const getEpisodeList = async () => {
// 	const response = await fetch(`${apiUrl}/episodes_public`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error(`${response.status}`);
// 	}

// 	return (await response.json()) as EpisodesPublic[]; // Replace with actual type when defined
// };

// export const getAnimeBySlug = async (slug: string) => {
// 	const response = await fetch(`${apiUrl}/anime/${slug}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error(`${response.status}`);
// 	}

// 	return (await response.json()) as Anime;
// };

// export const getEpisodesByAnimeSlug = async (slug: string) => {
// 	const response = await fetch(`${apiUrl}/episodes_by_anime/${slug}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error(`${response.status}`);
// 	}

// 	return (await response.json()) as EpisodesPublic[];
// };

// export const getEpisodeBySlug = async (slug: string, episode_number: string) => {
// 	const response = await fetch(`${apiUrl}/episodes_public/${slug}/${episode_number}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error(`${response.status}`);
// 	}

// 	return (await response.json()) as EpisodesPublic;
// };
// export const getUsers = async () => {
// 	const response = await fetch(`${apiUrl}/users_active`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch users');
// 	}

// 	return (await response.json()) as UserPublic[];
// };
