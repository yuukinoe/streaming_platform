import type { PageLoad } from './$types.js';

export const load: PageLoad = ({ params, data }) => {
    return { ...data, slug: params.slug };
};