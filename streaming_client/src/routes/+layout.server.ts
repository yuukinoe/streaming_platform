import { getAnimeList, getEpisodeList, getIsLoggedIn, getNews, getNewsCategories, getUsers } from '$lib/server/get.ts';
import type { Anime, Episodes, EpisodesPublic, NewsCategoryPublic, NewsPublic, UserPublic } from '$lib/server/types.ts';
import type { LayoutServerLoad } from './$types.js';
import { hasPermission } from '$lib/permissions.js';
import { analyticsMainPage } from '$lib/server/post.ts';
import { server_main_page } from '$lib/server/main_page.ts';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
	try {
		await analyticsMainPage();
	} catch {
		console.log('Error in analyticsMainPage');
	}
	const sessionId = cookies.get('session_id');
	let user = await getIsLoggedIn({ fetch });
	let newsCategories: NewsCategoryPublic[] | null = null;
	let news: NewsPublic[] | null = null;

	const canReadNewsCategories = hasPermission(user.message, 'news_categories', 'read');

	if (canReadNewsCategories) {
		newsCategories = (await getNewsCategories({ fetch, sessionId: sessionId! })) as NewsCategoryPublic[];
	}

	news = (await getNews({ fetch })) as NewsPublic[];

	const sortedNews = [...news].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());

	const anime = (await getAnimeList()) as Anime[];
	const episodes = hasPermission(user.message, 'episodes', 'read') ? ((await getEpisodeList({ fetch, sessionId: sessionId! })) as EpisodesPublic[]) : ((await getEpisodeList({ fetch, sessionId: sessionId! })) as Episodes[]);
	return {
		news: sortedNews,
		mainPage: await server_main_page(anime, episodes as EpisodesPublic[], news),
		anime: anime,
		filteredAnime: anime,
		isLoggedIn: user,
		episodes: episodes,
		users: (await getUsers({ fetch, sessionId: sessionId! })) as UserPublic[],
		newsCategories: newsCategories
	};
};
