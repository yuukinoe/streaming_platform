import type { Handle } from '@sveltejs/kit';
import { loadConfig } from '$lib/server/+server.ts';

const config = loadConfig();
const apiUrl = config.app.backend_url;
const DEBUG = config.debug;

// Define your exact proxy routes - same as in vite.config.js
const proxyRoutes = {
    '/server/logged_in': '/logged_in',
    '/server/login': '/login',
    '/server/patch_role_secured': '/patch_role_secured',
    '/server/patch_anime_secured': '/patch_anime_secured',
    '/server/patch_user_secured': '/patch_user_secured',
    '/server/patch_episode_secured': '/patch_episode_secured',
    '/server/patch_user_profile_secured': '/patch_user_profile_secured',
    '/server/patch_change_password_secured': '/patch_change_password_secured',
    '/server/patch_change_password_secured_as_superuser': '/patch_change_password_secured_as_superuser',
    '/server/register': '/register',
    '/server/add_anime': '/add_anime',
    '/server/logout': '/logout',
    '/server/add_episode': '/add_episode',
    '/server/add_role': '/add_role',
    '/server/episode_webhook': '/episode_webhook',
    '/server/update_anime_background_position': '/update_anime_background_position',
    '/server/create_news_category': '/create_news_category',
    '/server/create_news': '/create_news',
    '/server/patch_news_category_secured': '/patch_news_category_secured',
    '/server/patch_news_secured': '/patch_news_secured',
    '/server/delete_anime': '/delete_anime',
    '/server/delete_episode': '/delete_episode',
    '/server/delete_user': '/delete_user'
};

export const handle: Handle = async ({ event, resolve }) => {
    const url = event.url;
    const pathname = url.pathname;

    // Handle exact proxy routes
    for (const [frontendPath, backendPath] of Object.entries(proxyRoutes)) {
        if (pathname === frontendPath || pathname.startsWith(frontendPath + '/')) {
            const targetPath = pathname.replace(frontendPath, backendPath);
            const targetUrl = `${apiUrl}${targetPath}${url.search}`;

            try {
                // Create headers and ensure we forward the original origin
                const headers = new Headers(event.request.headers);
                headers.delete('content-length');
                if (!headers.has('Origin')) {
                    if (DEBUG) {
                        console.log('Origin', config.app.domain);
                    }
                    headers.set('Origin', config.app.domain);
                }

                // Prepare fetch options
                const fetchOptions: any = {
                    method: event.request.method,
                    headers: headers,
                };

                // Add body and duplex option for requests with body
                if (event.request.body && event.request.method !== 'GET' && event.request.method !== 'HEAD') {
                    fetchOptions.body = event.request.body;
                    fetchOptions.duplex = 'half';
                }

                const response = await fetch(targetUrl, fetchOptions);

                return new Response(response.body, {
                    status: response.status,
                    statusText: response.statusText,
                    headers: response.headers
                });
            } catch (error) {
                console.error(`Proxy error for ${pathname}:`, error);
                return new Response('Proxy Error', { status: 500 });
            }
        }
    }

    // Handle special cases with regex patterns

    // Handle /server/episodes/:anime/:episode
    if (pathname.startsWith('/server/episodes/')) {
        const match = pathname.match(/^\/server\/episodes\/([^/]+)\/([^/]+)/);
        if (match) {
            const [, anime, episode] = match;
            const targetPath = `/episodes/${anime}/${episode}`;
            const targetUrl = `${apiUrl}${targetPath}${url.search}`;

            try {
                // Create headers and ensure we forward the original origin
                const headers = new Headers(event.request.headers);
                headers.delete('content-length');
                if (!headers.has('Origin')) {
                    if (DEBUG) {
                        console.log('Origin', config.app.domain);
                    }
                    headers.set('Origin', config.app.domain);
                }

                // Prepare fetch options
                const fetchOptions: any = {
                    method: event.request.method,
                    headers: headers,
                };

                // Add body and duplex option for requests with body
                if (event.request.body && event.request.method !== 'GET' && event.request.method !== 'HEAD') {
                    fetchOptions.body = event.request.body;
                    fetchOptions.duplex = 'half';
                }

                const response = await fetch(targetUrl, fetchOptions);

                return new Response(response.body, {
                    status: response.status,
                    statusText: response.statusText,
                    headers: response.headers
                });
            } catch (error) {
                console.error(`Proxy error for episodes:`, error);
                return new Response('Proxy Error', { status: 500 });
            }
        }
    }

    // Handle /server/anime/:anime_id
    if (pathname.startsWith('/server/anime/')) {
        const match = pathname.match(/^\/server\/anime\/([^/]+)/);
        if (match) {
            const [, animeId] = match;
            const targetPath = `/anime/${animeId}`;
            const targetUrl = `${apiUrl}${targetPath}${url.search}`;

            try {
                const response = await fetch(targetUrl, {
                    method: event.request.method,
                    headers: event.request.headers,
                    body: event.request.body
                });

                return new Response(response.body, {
                    status: response.status,
                    statusText: response.statusText,
                    headers: response.headers
                });
            } catch (error) {
                console.error(`Proxy error for anime:`, error);
                return new Response('Proxy Error', { status: 500 });
            }
        }
    }

    // Handle /media routes (keep path as is)
    if (pathname.startsWith('/media/')) {
        const targetUrl = `${apiUrl}${pathname}${url.search}`;

        try {
            // Create headers and ensure we forward the original origin
            const headers = new Headers(event.request.headers);
            headers.delete('content-length');
            if (!headers.has('Origin')) {
                if (DEBUG) {
                    console.log('Origin', config.app.domain);
                }
                headers.set('Origin', config.app.domain);
            }

            // Prepare fetch options
            const fetchOptions: any = {
                method: event.request.method,
                headers: headers,
            };

            // Add body and duplex option for requests with body
            if (event.request.body && event.request.method !== 'GET' && event.request.method !== 'HEAD') {
                fetchOptions.body = event.request.body;
                fetchOptions.duplex = 'half';
            }

            const response = await fetch(targetUrl, fetchOptions);

            return new Response(response.body, {
                status: response.status,
                statusText: response.statusText,
                headers: response.headers
            });
        } catch (error) {
            console.error('Media proxy error:', error);
            return new Response('Media Proxy Error', { status: 500 });
        }
    }

    // Continue with normal SvelteKit handling for all other routes
    return resolve(event);
};