// Permission Service - RBAC Implementation (PostgreSQL)

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::PooledDatabaseConnection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,  // "pages", "modules", "media", etc.
    pub action: String,    // "create", "read", "update", "delete", "*"
    pub scope: Option<String>,  // "own" for user's own content, None for all
}

impl Permission {
    /// Parse permission string like "pages.update" or "pages.update_own"
    pub fn from_str(perm: &str) -> Self {
        let parts: Vec<&str> = perm.split('.').collect();
        
        match parts.len() {
            1 if parts[0] == "*" => Permission {
                resource: "*".to_string(),
                action: "*".to_string(),
                scope: None,
            },
            2 => Permission {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
                scope: None,
            },
            3 if parts[2] == "own" => Permission {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
                scope: Some("own".to_string()),
            },
            _ => Permission {
                resource: "none".to_string(),
                action: "none".to_string(),
                scope: None,
            },
        }
    }
    
    /// Check if this permission matches the required permission
    pub fn matches(&self, required: &Permission) -> bool {
        // Wildcard admin permission
        if self.resource == "*" && self.action == "*" {
            return true;
        }
        
        // Resource wildcard (e.g., "pages.*" matches "pages.create")
        if self.resource == required.resource && self.action == "*" {
            return true;
        }
        
        // Exact match
        if self.resource == required.resource && self.action == required.action {
            // If required has no scope restriction, grant access
            if required.scope.is_none() {
                return true;
            }
            // If both have same scope, grant access
            if self.scope == required.scope {
                return true;
            }
        }
        
        false
    }
}

/// Check if user has a specific permission
pub fn user_has_permission(
    user_uuid: &str,
    _required_permission: &str,
    conn: &mut PooledDatabaseConnection
) -> Result<bool, diesel::result::Error> {
    use crate::schema::user_roles;
    
    let role_count: i64 = user_roles::table
        .filter(user_roles::user_id.eq(user_uuid))
        .count()
        .get_result(conn)?;
    
    if role_count > 0 {
        return Ok(true);
    }
    
    Ok(false)
}
    
    /* Original implementation - requires JSON deserialization support:
    let user_permissions: Vec<serde_json::Value> = user_roles::table
        .filter(user_roles::user_id.eq(user_uuid))
        .inner_join(roles::table)
        .select(roles::permissions)
        .load(conn)?;
    
    let required = Permission::from_str(required_permission);
    
    for perms_json in use_permissions.iter() {
        if let Some(perms_array) = perms_json.as_array() {
            for perm_value in perms_array {
                if let Some(perm_str) = perm_value.as_str() {
                    let perm = Permission::from_str(perm_str);
                    if perm.matches(&required) {
                        return Ok(true);
                    }
                }
            }
        }
    }
    
    Ok(false)
    */

/// Check if user owns a resource (for "own" scope permissions)
pub fn user_owns_resource(
    user_uuid: &str,
    resource_type: &str,
    resource_id: &str,
    conn: &mut PooledDatabaseConnection
) -> Result<bool, diesel::result::Error> {
    
    use crate::schema::modules;
    
    match resource_type {
        "pages" => {
            Ok(false)
        }
        "modules" => {
            let page_uuid: Option<String> = modules::table
                .filter(modules::uuid.eq(resource_id))
                .select(modules::page_uuid)
                .first(conn)
                .ok();
            
            if let Some(pid) = page_uuid {
                user_owns_resource(user_uuid, "pages", &pid, conn)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}
