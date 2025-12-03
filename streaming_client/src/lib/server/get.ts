import type { Anime, EpisodesPublic, UserModelSuperUser, UserPublic, UserResponse, LogsPublic, Role, TagOrStudio, NewsCategoryPublic, NewsPublic, AnalyticsAnime, AnalyticsOverall } from './types.ts';
// import { apiUrl} from '../../../vite.config.ts'
// import { apiUrl } from '$lib/index.ts';
import { loadConfig } from '$lib/server/+server.ts';

const config = loadConfig();
const apiUrl = config.app.backend_url;

export const getAnimeList = async () => {
	const response = await fetch(`${apiUrl}/anime`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`${response.status}`);
	}

	return (await response.json()) as Anime[];
};

export const getEpisodeList = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/episodes`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		throw new Error(`${response.status}`);
	}

	return (await response.json()) as EpisodesPublic[]; // Replace with actual type when defined
};

export const getUsers = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/users`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});
	if (!response.ok) {
		if (response.status !== 200) {
			return (await response.json()) as UserModelSuperUser[];
		}
	}

	return (await response.json()) as UserModelSuperUser[];
};

export const getIsLoggedIn = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	const response = await fetch(`/server/logged_in`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});
	if (!response.ok) {
		if (config.debug) {
			console.log(response.status);
			console.log(response);
			console.log(response.statusText);
		}

		if (response.status !== 200) {
			return (await response.json()) as UserResponse;
		}
	}

	return (await response.json()) as UserResponse;
};

export const getLogs = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/logs`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},

	});

	if (!response.ok) {
		throw new Error('Failed to fetch logs');
	}

	return (await response.json()) as LogsPublic[];
};
export const getRoles = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/get_roles`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		if (response.status === 401) {
			return null;
		}
		throw new Error('Failed to fetch roles');
	}

	return (await response.json()) as Role[];
};


export const getPermissionsOptions = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/permissions_options`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		if (response.status === 401) {
			return null;
		}
		throw new Error('Failed to fetch permissions options');
	}

	return (await response.json()) as [];
};

export const getTagsList = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	const response = await fetch(`${apiUrl}/tags`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error('Failed to fetch tags list');
	}

	return (await response.json()) as TagOrStudio[];
};

export const getStudiosList = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	const response = await fetch(`${apiUrl}/studios`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error('Failed to fetch studios list');
	}

	return (await response.json()) as TagOrStudio[];
};

export const getNewsCategories = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/news_categories`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		throw new Error('Failed to fetch news categories');
	}

	return (await response.json()) as NewsCategoryPublic[];
};

export const getNews = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	const response = await fetch(`${apiUrl}/news`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});

	if (!response.ok) {
		throw new Error('Failed to fetch news');
	}

	return (await response.json()) as NewsPublic[];
};

export const getAnimeAnalytics = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/analytics_anime`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		throw new Error('Failed to fetch anime analytics');
	}

	return (await response.json()) as AnalyticsAnime[];
};

export const getOverallAnalytics = async ({ fetch, sessionId }: { fetch: typeof globalThis.fetch, sessionId: string }) => {
	const response = await fetch(`${apiUrl}/analytics_overall`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		},
	});

	if (!response.ok) {
		throw new Error('Failed to fetch overall analytics');
	}

	return (await response.json()) as AnalyticsOverall[];
};