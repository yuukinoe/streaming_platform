import { getNews } from '$lib/server/get.ts';

export const load = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	try {
		let news = await getNews({ fetch });
		const sortedNews = [...news].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
		return {
			news: sortedNews
		};
	} catch (error) {
		console.error('Error loading news:', error);
		return {
			news: []
		};
	}
};
