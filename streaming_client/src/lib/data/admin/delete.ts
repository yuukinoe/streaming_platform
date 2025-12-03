import type { RResponse } from '$lib/types.ts';

export const deleteAnime = async (id: string) => {
	const response = await fetch(`/server/delete_anime/${id}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	let default_error_message = 'Failed to delete anime';

	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const deleteEpisode = async (id: string) => {
	const response = await fetch(`/server/delete_episode/${id}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	let default_error_message = 'Failed to delete episode';

	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};

export const deleteUser = async (id: string) => {
	const response = await fetch(`/server/delete_user/${id}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	let default_error_message = 'Failed to delete user';

	if (!response.ok) {
		let error_message = (await response.json()) as RResponse;
		const message = typeof error_message.message === 'string' ? error_message.message : default_error_message;
		throw new Error(message);
	}

	return (await response.json()) as RResponse;
};
