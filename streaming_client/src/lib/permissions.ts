import type { UserModelSuperUser, PermissionFlags } from './server/types.js';

export type PermissionAction = 'add' | 'read' | 'edit' | 'self_action' | 'post_requests' | 'delete';

export interface PermissionCheck {
    resource: string;
    action: PermissionAction;
}

/**
 * Checks if a user has a specific permission for a resource
 * @param user - The user object with roles and superuser status
 * @param resource - The resource to check permissions for (e.g., 'news', 'news_categories')
 * @param action - The action to check (e.g., 'read', 'edit', 'add')
 * @returns boolean indicating if the user has the permission
 */
export function hasPermission(
    user: UserModelSuperUser,
    resource: string,
    action: PermissionAction
): boolean {
    // Superusers have all permissions
    if (user.is_superuser) {
        return true;
    }

    // Check if user has roles
    if (!user.roles || user.roles.length === 0) {
        return false;
    }

    // Check if any role has the required permission
    return user.roles.some((role) => {
        if (!role.permissions || typeof role.permissions !== 'object') {
            return false;
        }
        const permissions = role.permissions[resource];
        return permissions && typeof permissions === 'object' && permissions[action] === true;
    });
}

/**
 * Checks if a user has multiple permissions for a resource
 * @param user - The user object with roles and superuser status
 * @param resource - The resource to check permissions for
 * @param actions - Array of actions to check
 * @returns boolean indicating if the user has all the permissions
 */
export function hasAllPermissions(
    user: UserModelSuperUser,
    resource: string,
    actions: PermissionAction[]
): boolean {
    return actions.every((action) => hasPermission(user, resource, action));
}

/**
 * Checks if a user has any of the specified permissions for a resource
 * @param user - The user object with roles and superuser status
 * @param resource - The resource to check permissions for
 * @param actions - Array of actions to check
 * @returns boolean indicating if the user has at least one of the permissions
 */
export function hasAnyPermission(
    user: UserModelSuperUser,
    resource: string,
    actions: PermissionAction[]
): boolean {
    return actions.some((action) => hasPermission(user, resource, action));
}

/**
 * Checks multiple permission requirements
 * @param user - The user object with roles and superuser status
 * @param checks - Array of permission checks to perform
 * @returns boolean indicating if the user has all the required permissions
 */
export function checkPermissions(
    user: UserModelSuperUser,
    checks: PermissionCheck[]
): boolean {
    return checks.every((check) => hasPermission(user, check.resource, check.action));
}

/**
 * Gets all permissions for a user across all their roles
 * @param user - The user object with roles and superuser status
 * @returns Record of resource -> actions mapping
 */
export function getUserPermissions(user: UserModelSuperUser): Record<string, PermissionAction[]> {
    const permissions: Record<string, Set<PermissionAction>> = {};

    // Superusers have all permissions for all resources
    if (user.is_superuser) {
        // This is a simplified approach - in practice you might want to define all possible resources
        return {
            '*': ['add', 'read', 'edit', 'self_action', 'post_requests', 'delete']
        };
    }

    // Aggregate permissions from all roles
    if (user.roles) {
        user.roles.forEach((role) => {
            if (role.permissions && typeof role.permissions === 'object') {
                Object.entries(role.permissions).forEach(([resource, flags]) => {
                    if (!permissions[resource]) {
                        permissions[resource] = new Set();
                    }

                    // Add each permission flag that is true
                    if (flags && typeof flags === 'object') {
                        Object.entries(flags).forEach(([action, hasPermission]) => {
                            if (hasPermission && action in flags) {
                                permissions[resource].add(action as PermissionAction);
                            }
                        });
                    }
                });
            }
        });
    }

    // Convert Sets to arrays
    const result: Record<string, PermissionAction[]> = {};
    Object.entries(permissions).forEach(([resource, actions]) => {
        result[resource] = Array.from(actions);
    });

    return result;
}

/**
 * Checks if a user can perform a specific action on a resource
 * This is a convenience function that combines resource and action into a single string
 * @param user - The user object with roles and superuser status
 * @param permission - Permission string in format "resource:action" (e.g., "news:read")
 * @returns boolean indicating if the user has the permission
 */
export function can(user: UserModelSuperUser, permission: string): boolean {
    const [resource, action] = permission.split(':');
    if (!resource || !action) {
        return false;
    }
    return hasPermission(user, resource, action as PermissionAction);
}

/**
 * Checks if a user can perform multiple actions
 * @param user - The user object with roles and superuser status
 * @param permissions - Array of permission strings in format "resource:action"
 * @returns boolean indicating if the user has all the permissions
 */
export function canAll(user: UserModelSuperUser, permissions: string[]): boolean {
    return permissions.every((permission) => can(user, permission));
}

/**
 * Checks if a user can perform any of the specified actions
 * @param user - The user object with roles and superuser status
 * @param permissions - Array of permission strings in format "resource:action"
 * @returns boolean indicating if the user has at least one of the permissions
 */
export function canAny(user: UserModelSuperUser, permissions: string[]): boolean {
    return permissions.some((permission) => can(user, permission));
} 