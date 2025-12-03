import type { Actions } from './$types.ts';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	login: async ({ request, cookies, fetch }) => {
		const formData = await request.formData();
		const username = formData.get('username');
		const password = formData.get('password');

		const res = await fetch('/server/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password }),
			credentials: 'include'
		});

		if (!res.ok) {
			return fail(401, { error: 'Invalid credentials' });
		}

		throw redirect(302, '/admin');
	}
};
