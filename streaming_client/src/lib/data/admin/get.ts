// import { apiUrl } from '$lib/index.ts';
// import type {
// 	Anime,
// 	AnimeList,
// 	Episodes,
// 	LogsPublic,
// 	Role,
// 	RResponse,
// 	TagOrStudio,
// 	UserModelSuperUser,
// 	UserProfile,
// 	UserPublic
// } from '$lib/types.js';

import type { Anime, Episodes } from "$lib/types.ts";

// export const getAnimeList = async () => {
// 	const response = await fetch(`${apiUrl}/anime`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch anime list');
// 	}

// 	return (await response.json()) as AnimeList;
// };
// export const getAnimeBySlug = async (slug: string) => {
// 	const response = await fetch(`${apiUrl}/anime/${slug}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch anime by slug');
// 	}

// 	return (await response.json()) as Anime;
// };

// export const getTagsList = async () => {
// 	const response = await fetch(`${apiUrl}/tags`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch tags list');
// 	}

// 	return (await response.json()) as TagOrStudio[];
// };

// export const getStudiosList = async () => {
// 	const response = await fetch(`${apiUrl}/studios`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		}
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch studios list');
// 	}

// 	return (await response.json()) as TagOrStudio[];
// };

// // export const getIsLoggedIn = async () => {
// // 	const response = await fetch(`${apiUrl}/logged_in`, {
// // 		method: 'GET',
// // 		headers: {
// // 			'Content-Type': 'application/json'
// // 		},
// // 		credentials: 'include'
// // 	});
// // 	if (!response.ok) {
// // 		if (response.status === 401) {
// // 			return false;
// // 		}
// // 		throw new Error('Failed to check login status');
// // 	}

// // 	return (await response.json()) as UserModelSuperUser;
// // };

// export const getUsers = async () => {
// 	const response = await fetch(`${apiUrl}/users`, {
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

// export const getUserBySlugSecured = async (slug: string) => {
// 	const response = await fetch(`${apiUrl}/users/secured/${slug}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		if (response.status === 401) {
// 			return null;
// 		}
// 		throw new Error('Failed to fetch user by slug');
// 	}

// 	return (await response.json()) as UserModelSuperUser;
// };

// export const getLogByObjectAndObjectID = async (object: string, object_id: string) => {
// 	const response = await fetch(`${apiUrl}/logs/${object}/${object_id}/view`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch log by ID');
// 	}

// 	return (await response.json()) as LogsPublic[];
// };

// export const getLogs = async () => {
// 	const response = await fetch(`${apiUrl}/logs`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch logs');
// 	}

// 	return (await response.json()) as LogsPublic[];
// };

// export const getLogByID = async (id: string) => {
// 	const response = await fetch(`${apiUrl}/logs/${id}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});
// 	if (!response.ok) {
// 		throw new Error('Failed to fetch log by ID');
// 	}

// 	return (await response.json()) as LogsPublic;
// };

// export const getEpisodesList = async () => {
// 	const response = await fetch(`${apiUrl}/episodes_secured`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch episodes list');
// 	}

// 	return (await response.json()) as Episodes[];
// };

// export const getEpisodeBySlug = async (slug: string, episode_number: string) => {
// 	const response = await fetch(`${apiUrl}/episodes_secured/${slug}/${episode_number}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		throw new Error('Failed to fetch episode by slug');
// 	}

// 	return (await response.json()) as Episodes;
// };

// export const getUserProfile = async () => {
// 	const response = await fetch(`${apiUrl}/user_profile_secured`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		if (response.status === 401) {
// 			return null;
// 		}
// 		throw new Error('Failed to fetch user profile');
// 	}

// 	return (await response.json()) as UserProfile;
// };

// export const getRoles = async () => {
// 	const response = await fetch(`${apiUrl}/get_roles`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		if (response.status === 401) {
// 			return null;
// 		}
// 		throw new Error('Failed to fetch roles');
// 	}

// 	return (await response.json()) as Role[];
// };



// export const getRoleByID = async (id: string) => {
// 	const response = await fetch(`${apiUrl}/get_role_by_id_secured/${id}`, {
// 		method: 'GET',
// 		headers: {
// 			'Content-Type': 'application/json'
// 		},
// 		credentials: 'include'
// 	});

// 	if (!response.ok) {
// 		if (response.status === 401) {
// 			return null;
// 		}
// 		throw new Error('Failed to fetch roles');
// 	}

// 	return (await response.json()) as Role;
// };


export const getEpisodeBySlugAndNumber = async (slug: string, number: string) => {
	const response = await fetch(`/server/episodes/${slug}/${number}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		if (response.status === 404) {
			return null;
		}
		throw new Error('Failed to fetch episode by slug and number');
	}

	return (await response.json()) as Episodes;
}

export const getAnimeBySlug = async (slug: string) => {
	const response = await fetch(`/server/anime/${slug}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		if (response.status === 404) {
			return null;
		}
		throw new Error('Failed to fetch anime by slug');
	}

	return (await response.json()) as Anime;
}