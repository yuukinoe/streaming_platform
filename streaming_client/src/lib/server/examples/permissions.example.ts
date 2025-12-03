import { hasPermission, can, canAll, canAny, hasAllPermissions, hasAnyPermission, checkPermissions } from '../../permissions.ts';
import type { UserModelSuperUser } from '../types.ts';

// Example usage of the permission module

/**
 * Example 1: Basic permission check
 */
function exampleBasicCheck(user: UserModelSuperUser) {
	// Check if user can read news
	const canReadNews = hasPermission(user, 'news', 'read');
	
	// Check if user can edit news
	const canEditNews = hasPermission(user, 'news', 'edit');
	
	// Check if user can add news
	const canAddNews = hasPermission(user, 'news', 'add');
	
	return { canReadNews, canEditNews, canAddNews };
}

/**
 * Example 2: Using the convenience 'can' function
 */
function exampleCanFunction(user: UserModelSuperUser) {
	// Check single permission using string format
	const canReadNews = can(user, 'news:read');
	const canEditNews = can(user, 'news:edit');
	
	// Check multiple permissions (all must be true)
	const canManageNews = canAll(user, ['news:read', 'news:edit', 'news:add']);
	
	// Check multiple permissions (any can be true)
	const canInteractWithNews = canAny(user, ['news:read', 'news:edit']);
	
	return { canReadNews, canEditNews, canManageNews, canInteractWithNews };
}

/**
 * Example 3: Multiple permissions for same resource
 */
function exampleMultiplePermissions(user: UserModelSuperUser) {
	// Check if user has all specified permissions for news
	const canFullyManageNews = hasAllPermissions(user, 'news', ['read', 'edit', 'add', 'delete']);
	
	// Check if user has any of the specified permissions for news
	const canInteractWithNews = hasAnyPermission(user, 'news', ['read', 'edit']);
	
	return { canFullyManageNews, canInteractWithNews };
}

/**
 * Example 4: Complex permission checks across multiple resources
 */
function exampleComplexChecks(user: UserModelSuperUser) {
	// Check multiple permission requirements
	const canManageContent = checkPermissions(user, [
		{ resource: 'news', action: 'read' },
		{ resource: 'news_categories', action: 'read' },
		{ resource: 'users', action: 'read' }
	]);
	
	return { canManageContent };
}

/**
 * Example 5: Conditional logic based on permissions
 */
function exampleConditionalLogic(user: UserModelSuperUser) {
	if (can(user, 'news:read')) {
		// User can read news
		console.log('User can read news');
	}
	
	if (canAll(user, ['news:read', 'news:edit'])) {
		// User can both read and edit news
		console.log('User can read and edit news');
	}
	
	if (canAny(user, ['news:add', 'news:edit'])) {
		// User can either add or edit news
		console.log('User can add or edit news');
	}
}

/**
 * Example 6: Server-side route protection
 */
function exampleRouteProtection(user: UserModelSuperUser) {
	// Check if user can access admin panel
	const canAccessAdmin = canAny(user, [
		'users:read',
		'news:read',
		'news_categories:read'
	]);
	
	// Check if user can manage users
	const canManageUsers = canAll(user, [
		'users:read',
		'users:edit'
	]);
	
	return { canAccessAdmin, canManageUsers };
}

/**
 * Example 7: UI component permission checks
 */
function exampleUIComponentChecks(user: UserModelSuperUser) {
	// Show/hide buttons based on permissions
	const showAddNewsButton = can(user, 'news:add');
	const showEditNewsButton = can(user, 'news:edit');
	const showDeleteNewsButton = can(user, 'news:delete');
	
	// Enable/disable form fields based on permissions
	const canEditNewsFields = canAll(user, ['news:read', 'news:edit']);
	
	return {
		showAddNewsButton,
		showEditNewsButton,
		showDeleteNewsButton,
		canEditNewsFields
	};
}

export {
	exampleBasicCheck,
	exampleCanFunction,
	exampleMultiplePermissions,
	exampleComplexChecks,
	exampleConditionalLogic,
	exampleRouteProtection,
	exampleUIComponentChecks
}; 