import { getSeasonDates, stripHTML } from "./server_utils.ts";
import type { Anime, CarouselItem, EpisodesPublic, NewsPublic } from "./types.ts";

export const server_main_page = async (anime: Anime[], episodes: EpisodesPublic[], news: NewsPublic[]) => {
    try {
        let newsImages: CarouselItem[] = [];
        let images: CarouselItem[] = episodes
            .slice()
            .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
            .slice(0, 4)
            .map((episode) => ({
                alt: episode.title,
                src: `/${episode.image}`,
                title: episode.title,
                anime_title: episode.anime.title,
                anime_slug: episode.anime.slug,
                episode_number: episode.episode,
                episode_description: episode.description,
                item_type: 'episode'
            }));
        newsImages.push(
            ...news
                .filter((newsItem) => newsItem.pinned)
                .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
                .slice(0, 2)
                .map((newsItem) => ({
                    alt: newsItem.text_header,
                    src: `/${newsItem.image}`,
                    title: newsItem.text_header,
                    anime_title: newsItem.text_header,
                    anime_slug: newsItem.slug,
                    episode_number: 0,
                    episode_description: stripHTML(newsItem.description),
                    item_type: 'news'
                }))
        );

        if (newsImages.length < 4) {
            const needed = 4 - newsImages.length;
            const episodeImagesToAdd = images.filter((epImg) => !newsImages.some((newsImg) => newsImg.src === epImg.src && newsImg.item_type === epImg.item_type)).slice(0, needed);
            newsImages.push(...episodeImagesToAdd);
        } else if (newsImages.length > 4) {
            newsImages = newsImages.slice(0, 4);
        }

        let latestEpisodes: EpisodesPublic[] = episodes
            .slice()
            .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
            .slice(0, 10);
        let [startDate, endDate] = getSeasonDates();
        let seasonAnimeList: Anime[] = anime.filter((anime) => {
            let airedDate: Date | null = null;

            if (typeof anime.aired === 'string') {
                airedDate = new Date(anime.aired);
            }
            if (!airedDate || isNaN(airedDate.getTime())) return false;

            return airedDate >= startDate && airedDate <= endDate;
        });
        return {
            newsImages,
            images,
            latestEpisodes,
            seasonAnimeList
        };
    } catch (error) {
        console.error(error);
        return {
            newsImages: [],
            images: [],
            latestEpisodes: [],
            seasonAnimeList: []
        };
    }
}