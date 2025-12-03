import { guardPage, guardPageWithStrings, checkPageAccess } from '../../permissionGuard.ts';
import type { UserModelSuperUser } from '../types.ts';

// Example usage in page.server.ts files

/**
 * Example 1: Simple single permission check
 */
export function exampleSimplePermission(user: UserModelSuperUser) {
	// User needs 'news:read' permission
	guardPageWithStrings({
		user,
		requiredPermissions: ['news:read'],
		redirectTo: '/insufficient-permissions'
	});
}

/**
 * Example 2: Multiple permissions (ALL required)
 */
export function exampleMultiplePermissionsAll(user: UserModelSuperUser) {
	// User needs ALL permissions: news:read AND news:edit AND news:add
	guardPageWithStrings({
		user,
		requiredPermissions: ['news:read', 'news:edit', 'news:add'],
		redirectTo: '/insufficient-permissions',
		anyPermission: false // Default - user needs ALL permissions
	});
}

/**
 * Example 3: Multiple permissions (ANY required)
 */
export function exampleMultiplePermissionsAny(user: UserModelSuperUser) {
	// User needs ANY permission: news:read OR news:edit OR news:add
	guardPageWithStrings({
		user,
		requiredPermissions: ['news:read', 'news:edit', 'news:add'],
		redirectTo: '/insufficient-permissions',
		anyPermission: true // User needs ANY permission
	});
}

/**
 * Example 4: Using object-based permissions
 */
export function exampleObjectPermissions(user: UserModelSuperUser) {
	// User needs news:read AND users:read permissions
	guardPage({
		user,
		requiredPermissions: [
			{ resource: 'news', action: 'read' },
			{ resource: 'users', action: 'read' }
		],
		redirectTo: '/insufficient-permissions'
	});
}

/**
 * Example 5: Check permissions without redirecting
 */
export function exampleCheckWithoutRedirect(user: UserModelSuperUser) {
	// Check if user has access without redirecting
	const hasAccess = checkPageAccess({
		user,
		requiredPermissions: [
			{ resource: 'news', action: 'read' },
			{ resource: 'news', action: 'edit' }
		]
	});

	if (hasAccess) {
		// User has permission, proceed with page logic
		console.log('User has access');
	} else {
		// User doesn't have permission, show alternative content
		console.log('User does not have access');
	}
}

/**
 * Example 6: Admin panel access check
 */
export function exampleAdminAccess(user: UserModelSuperUser) {
	// User needs ANY admin permission
	guardPageWithStrings({
		user,
		requiredPermissions: [
			'users:read',
			'news:read',
			'anime:read',
			'episodes:read'
		],
		redirectTo: '/insufficient-permissions',
		anyPermission: true // User needs ANY of these permissions
	});
}

/**
 * Example 7: Super admin access check
 */
export function exampleSuperAdminAccess(user: UserModelSuperUser) {
	// User needs ALL admin permissions
	guardPageWithStrings({
		user,
		requiredPermissions: [
			'users:read',
			'users:edit',
			'users:add',
			'users:delete'
		],
		redirectTo: '/insufficient-permissions',
		anyPermission: false // User needs ALL permissions
	});
}

/**
 * Example 8: Conditional page logic based on permissions
 */
export function exampleConditionalLogic(user: UserModelSuperUser) {
	// Check different permission levels
	const canReadNews = checkPageAccess({
		user,
		requiredPermissions: [{ resource: 'news', action: 'read' }]
	});

	const canEditNews = checkPageAccess({
		user,
		requiredPermissions: [{ resource: 'news', action: 'edit' }]
	});

	const canAddNews = checkPageAccess({
		user,
		requiredPermissions: [{ resource: 'news', action: 'add' }]
	});

	// Return different data based on permissions
	return {
		canReadNews,
		canEditNews,
		canAddNews,
		showEditButton: canEditNews,
		showAddButton: canAddNews
	};
}

/**
 * Example 9: Page server load function with permission guard
 */
export async function examplePageServerLoad(user: UserModelSuperUser) {
	// Guard the page first
	guardPageWithStrings({
		user,
		requiredPermissions: ['news:read'],
		redirectTo: '/insufficient-permissions'
	});

	// If we get here, user has permission
	// Fetch data for the page
	const newsData = await fetchNewsData(); // Your data fetching logic

	return {
		newsData
	};
}

// Mock function for example
function fetchNewsData() {
	return Promise.resolve([]);
} 