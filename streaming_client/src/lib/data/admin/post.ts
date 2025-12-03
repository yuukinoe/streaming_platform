import type { Anime, RResponse } from '$lib/types.js';

export const createUser = async (user: { username: string; password: string }) => {
	const response = await fetch(`/server/register`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(user)
	});
	let default_error_message = 'Failed to create user';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const addAnime = async (anime: { message: string }) => {
	const response = await fetch(`/server/add_anime`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(anime)
	});

	let default_error_message = 'Failed to add anime';

	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const loginUser = async (user: { username: string; password: string }) => {
	// console.log('Logging in user:', user);
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

export const logoutUser = async () => {
	const response = await fetch(`/server/logout`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to logout user');
	}

	return (await response.json()) as RResponse;
};

export const addEpisode = async (formData: FormData) => {
	const response = await fetch(`/server/add_episode`, {
		method: 'POST',
		body: formData,
		credentials: 'include'
	});
	let default_error_message = 'Failed to add episode';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const addRole = async (formData: FormData) => {
	const response = await fetch(`/server/add_role`, {
		method: 'POST',
		body: formData,
		credentials: 'include'
	});
	let default_error_message = 'Failed to add role';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}
	return (await response.json()) as RResponse;
};

export const episodeWebhook = async (id: string) => {
	const response = await fetch(`/server/episode_webhook/${id}`, {
		method: 'POST',
		credentials: 'include'
	});

	let default_error_message = 'Failed to post webhook';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const addCategory = async (formData: FormData) => {
	const response = await fetch(`/server/create_news_category`, {
		method: 'POST',
		body: formData,
		credentials: 'include'
	});

	let default_error_message = 'Failed to add category';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}
	return (await response.json()) as RResponse;
};

export const addNews = async (formData: FormData) => {
	const response = await fetch(`/server/create_news`, {
		method: 'POST',
		body: formData,
		credentials: 'include'
	});
	let default_error_message = 'Failed to add news';
	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}
	return (await response.json()) as RResponse;
};

