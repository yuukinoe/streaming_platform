import type { UserModelSuperUser } from './server/types.js';
import { hasPermission, can } from './permissions.js';

/**
 * Simple permission checker - returns true if user has permission, false otherwise
 * @param user - The user object
 * @param permission - Permission string in format "resource:action" (e.g., "news:read")
 * @returns boolean - true if user has permission, false otherwise
 */
export function checkPermission(user: UserModelSuperUser, permission: string): boolean {
	return can(user, permission);
}

/**
 * Check if user has permission for a specific resource and action
 * @param user - The user object
 * @param resource - The resource (e.g., "news", "users", "anime")
 * @param action - The action (e.g., "read", "edit", "add", "delete")
 * @returns boolean - true if user has permission, false otherwise
 */
export function hasUserPermission(
	user: UserModelSuperUser, 
	resource: string, 
	action: 'add' | 'read' | 'edit' | 'self_action' | 'post_requests' | 'delete'
): boolean {
	return hasPermission(user, resource, action);
}

/**
 * Check if user has ANY of the specified permissions
 * @param user - The user object
 * @param permissions - Array of permission strings (e.g., ["news:read", "news:edit"])
 * @returns boolean - true if user has ANY permission, false otherwise
 */
export function hasAnyPermission(user: UserModelSuperUser, permissions: string[]): boolean {
	return permissions.some(permission => can(user, permission));
}

/**
 * Check if user has ALL of the specified permissions
 * @param user - The user object
 * @param permissions - Array of permission strings (e.g., ["news:read", "news:edit"])
 * @returns boolean - true if user has ALL permissions, false otherwise
 */
export function hasAllPermissions(user: UserModelSuperUser, permissions: string[]): boolean {
	return permissions.every(permission => can(user, permission));
}

/**
 * Check if user is a superuser
 * @param user - The user object
 * @returns boolean - true if user is superuser, false otherwise
 */
export function isSuperUser(user: UserModelSuperUser): boolean {
	return user.is_superuser;
}

/**
 * Check if user has any roles
 * @param user - The user object
 * @returns boolean - true if user has roles, false otherwise
 */
export function hasRoles(user: UserModelSuperUser): boolean {
	return Array.isArray(user.roles) && user.roles.length > 0;
}

/**
 * Get all permissions for a user as a simple object
 * @param user - The user object
 * @returns object with resource names as keys and arrays of actions as values
 */
export function getUserPermissions(user: UserModelSuperUser): Record<string, string[]> {
	const permissions: Record<string, string[]> = {};
	
	if (user.is_superuser) {
		return { '*': ['add', 'read', 'edit', 'self_action', 'post_requests', 'delete'] };
	}
	
	if (user.roles) {
		user.roles.forEach(role => {
			if (role.permissions && typeof role.permissions === 'object') {
				Object.entries(role.permissions).forEach(([resource, flags]) => {
					if (!permissions[resource]) {
						permissions[resource] = [];
					}
					
					if (flags && typeof flags === 'object') {
						Object.entries(flags).forEach(([action, hasPermission]) => {
							if (hasPermission && action in flags) {
								if (!permissions[resource].includes(action)) {
									permissions[resource].push(action);
								}
							}
						});
					}
				});
			}
		});
	}
	
	return permissions;
} 