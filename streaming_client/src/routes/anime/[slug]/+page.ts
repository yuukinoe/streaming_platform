import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types.js';

export const load: PageLoad = async ({ params, data }) => {
    return { ...data, slug: params.slug };
};