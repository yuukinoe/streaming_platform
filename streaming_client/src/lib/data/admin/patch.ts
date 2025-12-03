
export async function patchAnimeData(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_anime_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving anime data:', error);
	}
}

export async function patchUserData(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_user_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving user data:', error);
	}
}

export async function patchEpisodeData(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_episode_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving episode data:', error);
	}
}

export async function patchUserProfile(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_user_profile_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving user profile data:', error);
	}
}

export async function patchUserPasswordByHimself(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_change_password_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving user password:', error);
	}
}

export async function patchUserPasswordByAdmin(formData: FormData, id: string) {
	try {
		const response = await fetch(`/server/patch_change_password_secured_as_superuser/${id}`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving user password by admin:', error);
	}
}

export async function patchRole(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_role_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});

		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving role by admin:', error);
	}
}
export async function patchNewsCategory(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_news_category_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});
		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving news category:', error);
	}
};

export async function patchNews(formData: FormData) {
	try {
		const response = await fetch(`/server/patch_news_secured`, {
			method: 'PATCH',
			body: formData,
			credentials: 'include'
		});
		if (response.ok) {
			const result = await response.json();
			return response;
		} else {
			console.error('Save failed:', response.statusText);
		}
	} catch (error) {
		console.error('Error saving news:', error);
	}
};