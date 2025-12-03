import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types.ts';

export const load: PageLoad = ({ params }) => {
	return { slug: params.slug, id: params.id };
};
