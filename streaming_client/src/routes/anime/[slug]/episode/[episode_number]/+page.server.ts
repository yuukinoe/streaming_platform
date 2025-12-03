import type { PageServerLoad } from './$types.ts';
import { analyticsEpisode, analyticsAnime } from '$lib/server/post.ts';

export const load: PageServerLoad = async ({ params, parent }) => {
    try {
        await analyticsAnime(params.slug);
        await analyticsEpisode(params.slug, params.episode_number);
    } catch {
        console.log('Error in analyticsEpisode or analyticsAnime');
    }
    const { episodes } = await parent();
    return {
        currentEpisode: episodes.find((e) => e.anime.slug === params.slug && e.episode.toString() === params.episode_number)
    };
};
