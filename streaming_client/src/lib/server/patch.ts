import { apiUrl } from "$lib/index.ts";

export async function patchRole(formData: FormData) {
	try {
		const response = await fetch(`${apiUrl}/patch_role_secured`, {
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