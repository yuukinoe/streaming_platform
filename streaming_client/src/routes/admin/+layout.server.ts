import { getAnimeList, getIsLoggedIn, getLogs, getUsers, getRoles, getPermissionsOptions, getTagsList, getStudiosList, getNewsCategories, getNews, getAnimeAnalytics, getOverallAnalytics } from '$lib/server/get.ts';
import type { Anime, EpisodesPublic, LogsPublic, NewsCategoryPublic, NewsPublic, Role, UserPublic } from '$lib/server/types.ts';
import type { LayoutServerLoad } from './$types.ts';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ fetch, parent, cookies, url }) => {
	const { isLoggedIn } = await parent();
	let user = isLoggedIn;
	let logs: LogsPublic[] = [];
	let roles: Role[] = [];
	const sessionId = cookies.get('session_id');
	let permissionsOptions: [] = [];
	
	// Check if user is trying to access login page
	const isLoginPage = url.pathname === '/admin/login';
	
	// If user is not logged in (status !== 200) and not on login page, redirect to login
	if (user.status !== 200 && !isLoginPage) {
		throw redirect(302, '/admin/login');
	}
	
	// If user is logged in (status === 200) and trying to access login page, redirect to admin
	if (user.status === 200 && isLoginPage) {
		throw redirect(302, '/admin');
	}
	
	const canReadRoles = user.message.roles?.some((role: any) =>
		role.permissions?.roles?.read === true
	) || user.message.is_superuser;
	const canReadLogs = user.message.roles?.some((role: any) =>
		role.permissions?.logs?.read === true
	) || user.message.is_superuser;

	if (user.status === 200) {
		if (canReadRoles) {
			roles = (await getRoles({ fetch, sessionId: sessionId! })) as Role[];
			permissionsOptions = await getPermissionsOptions({ fetch, sessionId: sessionId! }) as [];
		}
		if (canReadLogs) {
			logs = await getLogs({ fetch, sessionId: sessionId! });
		}

		return {
			isLoggedIn: user,
			animeAnalytics: await getAnimeAnalytics({ fetch, sessionId: sessionId! }),
			overallAnalytics: await getOverallAnalytics({ fetch, sessionId: sessionId! }),
			usersSuperUserView: await getUsers({ fetch, sessionId: sessionId! }),
			logs: logs.length > 0 ? logs : null,
			roles: roles.length > 0 ? roles : null,
			permissionsOptions: permissionsOptions,
			tags: await getTagsList({ fetch }),
			studios: await getStudiosList({ fetch }),
		};
	}

	return {
		isLoggedIn: user
	};
};
