use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::DbPool;
use crate::services::errors_service::CustomHttpError;

/// User context extracted from JWT or session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: i32,
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
}

/// Authorization scopes for AI features
#[derive(Debug, Clone)]
pub enum AIScope {
    ReadPublic,      // Can read published content
    ReadOwn,         // Can read own content
    WriteOwn,        // Can generate content
    Admin,           // Full access to AI config
}

/// Extract user context from request
pub fn extract_user_context(req: &HttpRequest) -> Result<UserContext, CustomHttpError> {
    // TODO: Extract from JWT token or session
    // For now, return a default context
    // In production, parse Authorization header and validate JWT
    
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                // Parse JWT and extract user info
                // For now, mock a user context
                return Ok(UserContext {
                    user_id: 1,
                    username: "user".to_string(),
                    role: "user".to_string(),
                    permissions: vec!["read:public".to_string()],
                });
            }
        }
    }
    
    // Return anonymous/guest context
    Ok(UserContext {
        user_id: 0,
        username: "guest".to_string(),
        role: "guest".to_string(),
        permissions: vec!["read:public".to_string()],
    })
}

/// Check if user has required scope
pub fn check_scope(user: &UserContext, required_scope: AIScope) -> Result<(), CustomHttpError> {
    match required_scope {
        AIScope::ReadPublic => {
            // Everyone can read public content
            Ok(())
        }
        AIScope::ReadOwn => {
            // Must be authenticated
            if user.user_id == 0 {
                return Err(CustomHttpError::Unauthorized("Authentication required".to_string()));
            }
            Ok(())
        }
        AIScope::WriteOwn => {
            // Must be authenticated
            if user.user_id == 0 {
                return Err(CustomHttpError::Unauthorized("Authentication required".to_string()));
            }
            Ok(())
        }
        AIScope::Admin => {
            // Must be admin
            if user.role != "admin" && user.role != "administrator" {
                return Err(CustomHttpError::Forbidden("Admin access required".to_string()));
            }
            Ok(())
        }
    }
}

/// Filter pages by user permissions
pub fn filter_pages_by_permission(
    user: &UserContext,
    pool: &web::Data<DbPool>,
) -> Result<Vec<i64>, diesel::result::Error> {
    use crate::schema::pages;
    
    let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::Unknown,
        Box::new("Connection error".to_string())
    ))?;
    
    if user.role == "admin" || user.role == "administrator" {
        // Admins see everything - return empty vec as placeholder (uuid conversion needed)
        Ok(vec![])
    } else if user.user_id > 0 {
        // Authenticated users see published + their own - placeholder
        Ok(vec![])
    } else {
        // Guests see only published - placeholder
        Ok(vec![])
    }
}

/// Get accessible page IDs for search/recommendations
pub async fn get_accessible_page_ids(
    user: &UserContext,
    pool: web::Data<DbPool>,
) -> Result<Vec<i64>, CustomHttpError> {
    web::block({let user = user.clone(); move || filter_pages_by_permission(&user, &pool)})
        .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))
}

/// Verify user can access specific page
pub async fn verify_page_access(
    user: &UserContext,
    page_id: i64,
    pool: web::Data<DbPool>,
) -> Result<bool, CustomHttpError> {
    use crate::schema::pages;
    
    let user_id = user.user_id;
    let user_role = user.role.clone();
    
    let has_access = web::block(move || -> Result<bool, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        if user_role == "admin" || user_role == "administrator" {
            // Admins can access everything
            return Ok(true);
        }
        
        // Note: pages table uses uuid as PK, not i64 id
        // This needs refactoring to work with String UUIDs
        // For now, return false as placeholder
        Ok(false)
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(has_access)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guest_scope() {
        let guest = UserContext {
            user_id: 0,
            username: "guest".to_string(),
            role: "guest".to_string(),
            permissions: vec![],
        };
        
        assert!(check_scope(&guest, AIScope::ReadPublic).is_ok());
        assert!(check_scope(&guest, AIScope::ReadOwn).is_err());
        assert!(check_scope(&guest, AIScope::WriteOwn).is_err());
        assert!(check_scope(&guest, AIScope::Admin).is_err());
    }
    
    #[test]
    fn test_user_scope() {
        let user = UserContext {
            user_id: 1,
            username: "user".to_string(),
            role: "user".to_string(),
            permissions: vec![],
        };
        
        assert!(check_scope(&user, AIScope::ReadPublic).is_ok());
        assert!(check_scope(&user, AIScope::ReadOwn).is_ok());
        assert!(check_scope(&user, AIScope::WriteOwn).is_ok());
        assert!(check_scope(&user, AIScope::Admin).is_err());
    }
    
    #[test]
    fn test_admin_scope() {
        let admin = UserContext {
            user_id: 1,
            username: "admin".to_string(),
            role: "admin".to_string(),
            permissions: vec![],
        };
        
        assert!(check_scope(&admin, AIScope::ReadPublic).is_ok());
        assert!(check_scope(&admin, AIScope::ReadOwn).is_ok());
        assert!(check_scope(&admin, AIScope::WriteOwn).is_ok());
        assert!(check_scope(&admin, AIScope::Admin).is_ok());
    }
}
