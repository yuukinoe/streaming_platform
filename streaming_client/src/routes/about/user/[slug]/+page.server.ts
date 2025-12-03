import type { PageServerLoad } from './$types.ts';

export const load: PageServerLoad = async ({ parent, params }) => {
    const { users } = await parent();
    return {
        currentUser: users.find((user) => user.slug === params.slug)
    }
};
