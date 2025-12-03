import type { PageServerLoad } from './$types.ts';
import { analyticsAnime } from '$lib/server/post.ts';
import type { Episodes, EpisodesPublic } from '$lib/server/types.ts';


export const load: PageServerLoad = async ({ params, parent }) => {
    try {
        await analyticsAnime(params.slug);
    } catch {
        console.log('Error in analyticsAnime');
    }
    let data = await parent();

    let episodes = data.episodes.filter((e) => e.anime.slug === params.slug).sort((a, b) => a.episode - b.episode)
    if (data.isLoggedIn.status == 200) {
        episodes = episodes as Episodes[]
    } else {
        episodes = episodes as EpisodesPublic[]
    }


    return {
        currentAnime: data.anime.find((a) => a.slug === params.slug),
        episodesForAnime: episodes
    }
};
