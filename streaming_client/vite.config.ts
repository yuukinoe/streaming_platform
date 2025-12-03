import tailwindcss from '@tailwindcss/vite';
import devtoolsJson from 'vite-plugin-devtools-json';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { loadConfig } from './src/lib/server/+server.ts';

const config = loadConfig();
const apiUrl = config.app.backend_url;

// export const apiUrl: string = 'http://100.80.74.102:8001';


const DEBUG = config.debug;

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), devtoolsJson()],
	server: {
		allowedHosts: true,
		proxy: {
			'/server/logged_in': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/logged_in/, '/logged_in')
			},
			'/server/login': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/login/, '/login')
			},
			'/media': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/media/, '/media') // zostawiamy tak jak jest
			},
			'/server/patch_role_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_role_secured/, '/patch_role_secured')
			},
			'/server/patch_anime_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_anime_secured/, '/patch_anime_secured')
			},
			'/server/patch_user_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_user_secured/, '/patch_user_secured')
			},
			'/server/patch_episode_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_episode_secured/, '/patch_episode_secured')
			},
			'/server/patch_user_profile_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_user_profile_secured/, '/patch_user_profile_secured')
			},
			'/server/patch_change_password_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_change_password_secured/, '/patch_change_password_secured')
			},
			'/server/patch_change_password_secured_as_superuser': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_change_password_secured_as_superuser/, '/patch_change_password_secured_as_superuser')
			},
			'/server/register': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/register/, '/register')
			},
			'/server/add_anime': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/add_anime/, '/add_anime')
			},
			'/server/logout': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/logout/, '/logout')
			},
			'/server/add_episode': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/add_episode/, '/add_episode')
			},
			'/server/add_role': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/add_role/, '/add_role')
			},
			'/server/episode_webhook': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/episode_webhook/, '/episode_webhook')
			},
			'/server/episodes': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => {
					return path.replace(/^\/server\/episodes\/([^/]+)\/([^/]+)/, '/episodes/$1/$2');
				}
			},
			'/server/anime': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/anime\/([^/]+)/, '/anime/$1')
			},
			'/server/update_anime_background_position': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/update_anime_background_position/, '/update_anime_background_position')
			},
			'/server/create_news_category': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/create_news_category/, '/create_news_category')
			},
			'/server/create_news': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/create_news/, '/create_news')
			},
			'/server/patch_news_category_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_news_category_secured/, '/patch_news_category_secured')
			},
			'/server/patch_news_secured': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/patch_news_secured/, '/patch_news_secured')
			},
			'/server/delete_anime': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/delete_anime/, '/delete_anime')
			},
			'/server/delete_episode': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/delete_episode/, '/delete_episode')
			},
			'/server/delete_user': {
				target: apiUrl,
				changeOrigin: true,
				secure: DEBUG ? false : true,
				rewrite: (path) => path.replace(/^\/server\/delete_user/, '/delete_user')
			}
		}
	}
});
// #[patch("/patch_news_category_secured/<id>", data = "<form>")]
// #[patch("/patch_news_secured/<id>", data = "<form>")]
