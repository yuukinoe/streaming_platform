import { redirect } from '@sveltejs/kit';
import type { UserModelSuperUser } from './server/types.js';
import { hasPermission, can } from './permissions.js';

export type PermissionRequirement = {
	resource: string;
	action: 'add' | 'read' | 'edit' | 'self_action' | 'post_requests' | 'delete';
};

export type PermissionGuardOptions = {
	user: UserModelSuperUser;
	requiredPermissions: PermissionRequirement[];
	redirectTo?: string;
	anyPermission?: boolean; // If true, user needs ANY permission. If false, user needs ALL permissions
};

/**
 * Guards a page based on user permissions
 * @param options - Permission guard options
 * @throws redirect if user doesn't have required permissions
 */
export function guardPage(options: PermissionGuardOptions): void {
	const { user, requiredPermissions, redirectTo = '/', anyPermission = false } = options;

	// Check if user has required permissions
	let hasAccess = false;

	if (anyPermission) {
		// User needs ANY of the required permissions
		hasAccess = requiredPermissions.some(({ resource, action }) =>
			hasPermission(user, resource, action)
		);
	} else {
		// User needs ALL of the required permissions
		hasAccess = requiredPermissions.every(({ resource, action }) =>
			hasPermission(user, resource, action)
		);
	}

	if (!hasAccess) {
		throw redirect(302, redirectTo);
	}
}

/**
 * Guards a page using string-based permissions (e.g., 'news:read')
 * @param options - Permission guard options with string permissions
 * @throws redirect if user doesn't have required permissions
 */
export function guardPageWithStrings(options: {
	user: UserModelSuperUser;
	requiredPermissions: string[];
	redirectTo?: string;
	anyPermission?: boolean;
}): void {
	const { user, requiredPermissions, redirectTo = '/', anyPermission = false } = options;

	// Check if user has required permissions
	let hasAccess = false;

	if (anyPermission) {
		// User needs ANY of the required permissions
		hasAccess = requiredPermissions.some((permission) => can(user, permission));
	} else {
		// User needs ALL of the required permissions
		hasAccess = requiredPermissions.every((permission) => can(user, permission));
	}

	if (!hasAccess) {
		throw redirect(302, redirectTo);
	}
}

/**
 * Checks if user has access without redirecting
 * @param options - Permission guard options
 * @returns boolean indicating if user has access
 */
export function checkPageAccess(options: PermissionGuardOptions): boolean {
	const { user, requiredPermissions, anyPermission = false } = options;

	if (anyPermission) {
		return requiredPermissions.some(({ resource, action }) =>
			hasPermission(user, resource, action)
		);
	} else {
		return requiredPermissions.every(({ resource, action }) =>
			hasPermission(user, resource, action)
		);
	}
}

/**
 * Checks if user has access using string-based permissions
 * @param options - Permission guard options with string permissions
 * @returns boolean indicating if user has access
 */
export function checkPageAccessWithStrings(options: {
	user: UserModelSuperUser;
	requiredPermissions: string[];
	anyPermission?: boolean;
}): boolean {
	const { user, requiredPermissions, anyPermission = false } = options;

	if (anyPermission) {
		return requiredPermissions.some((permission) => can(user, permission));
	} else {
		return requiredPermissions.every((permission) => can(user, permission));
	}
} 