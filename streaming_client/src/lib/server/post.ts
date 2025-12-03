import { loadConfig } from '$lib/server/+server.ts';

const apiUrl = loadConfig().app.backend_url;


export const loginUser = async (user: { username: string; password: string }) => {
	const response = await fetch(`/server/login`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			username: user.username,
			password: user.password
		})
	});
	if (!response.ok) return false;
	return true;
};


export const analyticsAnime = async (slug: string) => {
	const response = await fetch(`${apiUrl}/analytics_anime/${slug}`, {
		method: 'POST',
		credentials: 'include'
	});
	if (!response.ok) return false;
	return true;
};

export const analyticsEpisode = async (slug: string, number: string) => {
	const response = await fetch(`${apiUrl}/analytics_episode/${slug}/${number}`, {
		method: 'POST',
		credentials: 'include'
	});
	if (!response.ok) return false;
	return true;
};

export const analyticsMainPage = async () => {
	const response = await fetch(`${apiUrl}/analytics_main_page`, {
		method: 'POST',
		credentials: 'include'
	});
	if (!response.ok) return false;
	return true;
};