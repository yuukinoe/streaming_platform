import { 
	checkPermission, 
	hasUserPermission, 
	hasAnyPermission, 
	hasAllPermissions, 
	isSuperUser, 
	hasRoles, 
	getUserPermissions 
} from '../../checkPermission.ts';
import type { UserModelSuperUser } from '../types.ts';

// Example usage of the simple permission checker

/**
 * Example 1: Simple permission check
 */
export function exampleSimpleCheck(user: UserModelSuperUser) {
	// Check if user can read news
	const canReadNews = checkPermission(user, 'news:read');
	console.log('Can read news:', canReadNews);
	
	// Check if user can edit news
	const canEditNews = checkPermission(user, 'news:edit');
	console.log('Can edit news:', canEditNews);
	
	return { canReadNews, canEditNews };
}

/**
 * Example 2: Using resource and action separately
 */
export function exampleResourceAction(user: UserModelSuperUser) {
	// Check if user can read news
	const canReadNews = hasUserPermission(user, 'news', 'read');
	
	// Check if user can add news
	const canAddNews = hasUserPermission(user, 'news', 'add');
	
	// Check if user can delete news
	const canDeleteNews = hasUserPermission(user, 'news', 'delete');
	
	return { canReadNews, canAddNews, canDeleteNews };
}

/**
 * Example 3: Check multiple permissions (ANY)
 */
export function exampleAnyPermission(user: UserModelSuperUser) {
	// Check if user has ANY news permission
	const hasAnyNewsPermission = hasAnyPermission(user, [
		'news:read',
		'news:edit',
		'news:add',
		'news:delete'
	]);
	
	console.log('Has any news permission:', hasAnyNewsPermission);
	return hasAnyNewsPermission;
}

/**
 * Example 4: Check multiple permissions (ALL)
 */
export function exampleAllPermissions(user: UserModelSuperUser) {
	// Check if user has ALL news management permissions
	const hasAllNewsPermissions = hasAllPermissions(user, [
		'news:read',
		'news:edit',
		'news:add'
	]);
	
	console.log('Has all news permissions:', hasAllNewsPermissions);
	return hasAllNewsPermissions;
}

/**
 * Example 5: Check user status
 */
export function exampleUserStatus(user: UserModelSuperUser) {
	const isSuper = isSuperUser(user);
	const hasUserRoles = hasRoles(user);
	
	console.log('Is superuser:', isSuper);
	console.log('Has roles:', hasUserRoles);
	
	return { isSuper, hasUserRoles };
}

/**
 * Example 6: Get all user permissions
 */
export function exampleGetAllPermissions(user: UserModelSuperUser) {
	const allPermissions = getUserPermissions(user);
	console.log('All user permissions:', allPermissions);
	
	// Example output:
	// {
	//   "anime": ["read"],
	//   "news": ["read", "edit"],
	//   "users": ["read"]
	// }
	
	return allPermissions;
}

/**
 * Example 7: Conditional UI logic
 */
export function exampleConditionalUI(user: UserModelSuperUser) {
	const showNewsSection = checkPermission(user, 'news:read');
	const showAddNewsButton = checkPermission(user, 'news:add');
	const showEditNewsButton = checkPermission(user, 'news:edit');
	const showDeleteNewsButton = checkPermission(user, 'news:delete');
	
	return {
		showNewsSection,
		showAddNewsButton,
		showEditNewsButton,
		showDeleteNewsButton
	};
}

/**
 * Example 8: Page access control
 */
export function examplePageAccess(user: UserModelSuperUser) {
	// Check if user can access admin panel
	const canAccessAdmin = hasAnyPermission(user, [
		'users:read',
		'news:read',
		'anime:read',
		'episodes:read'
	]);
	
	// Check if user can access super admin features
	const canAccessSuperAdmin = hasAllPermissions(user, [
		'users:read',
		'users:edit',
		'users:add',
		'users:delete'
	]);
	
	return { canAccessAdmin, canAccessSuperAdmin };
}

/**
 * Example 9: In a Svelte component
 */
export function exampleSvelteComponent(user: UserModelSuperUser) {
	// These would be used in a Svelte component
	const permissions = {
		canReadNews: checkPermission(user, 'news:read'),
		canEditNews: checkPermission(user, 'news:edit'),
		canAddNews: checkPermission(user, 'news:add'),
		canDeleteNews: checkPermission(user, 'news:delete'),
		isAdmin: isSuperUser(user)
	};
	
	return permissions;
}

/**
 * Example 10: Server-side validation
 */
export function exampleServerValidation(user: UserModelSuperUser, action: string) {
	switch (action) {
		case 'read_news':
			return checkPermission(user, 'news:read');
		case 'edit_news':
			return checkPermission(user, 'news:edit');
		case 'add_news':
			return checkPermission(user, 'news:add');
		case 'delete_news':
			return checkPermission(user, 'news:delete');
		case 'manage_users':
			return hasAllPermissions(user, ['users:read', 'users:edit']);
		default:
			return false;
	}
} 