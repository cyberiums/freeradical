use actix_web::web;
use diesel::prelude::*;

use crate::models::DbPool;
use crate::services::errors_service::CustomHttpError;

/// AI Operation Scopes
#[derive(Debug, Clone, PartialEq)]
pub enum AIScope {
    ReadPublic,   // Published content only (guests)
    ReadOwn,      // Own content (authenticated users)
    WriteOwn,     // Generate content (authenticated users)
    Admin,        // Full access + provider management (admins only)
}

/// User context extracted from JWT
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: i32,         // 0 for guests
    pub username: String,     // "guest" for anonymous
    pub role: String,         // "guest", "user", "admin"
    pub permissions: Vec<String>,
    // Team-based permissions
    pub team_id: Option<i32>, // Team membership
    pub team_role: Option<String>, // Role within team: "member", "manager", "owner"
}

impl Default for UserContext {
    fn default() -> Self {
        Self {
            user_id: 0,
            username: "guest".to_string(),
            role: "guest".to_string(),
            permissions: vec!["read:public".to_string()],
            team_id: None,
            team_role: None,
        }
    }
}

/// Check if user has required AI scope
/// Includes team-based permission checking
pub fn check_ai_permission(
    user: &UserContext,
    required_scope: AIScope,
) -> Result<(), CustomHttpError> {
    match required_scope {
        AIScope::ReadPublic => {
            // Everyone can read published content
            Ok(())
        }
        AIScope::ReadOwn => {
            // Must be authenticated
            if user.user_id == 0 {
                return Err(CustomHttpError::Unauthorized(
                    "Authentication required".to_string()
                ));
            }
            Ok(())
        }
        AIScope::WriteOwn => {
            // Must be authenticated
            if user.user_id == 0 {
                return Err(CustomHttpError::Unauthorized(
                    "Authentication required for content generation".to_string()
                ));
            }
            
            // Check team-based MCP permissions if user is in a team
            if let Some(team_role) = &user.team_role {
                check_team_mcp_permission(user, "ai:generate")?;
            }
            
            Ok(())
        }
        AIScope::Admin => {
            // Must be admin OR team owner
            if user.role == "admin" {
                return Ok(());
            }
            
            // Team owners can manage MCP for their team
            if let Some(team_role) = &user.team_role {
                if team_role == "owner" {
                    return Ok(());
                }
            }
            
            Err(CustomHttpError::Forbidden(
                "Admin or team owner access required".to_string()
            ))
        }
    }
}

/// Team Role Hierarchy
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TeamRole {
    Member,   // Basic team member
    Manager,  // Can manage team members
    Owner,    // Full team control including MCP settings
}

impl TeamRole {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "owner" => TeamRole::Owner,
            "manager" => TeamRole::Manager,
            _ => TeamRole::Member,
        }
    }
}

/// Team MCP Settings (would be stored in database)
#[derive(Debug, Clone)]
pub struct TeamMCPSettings {
    pub team_id: i32,
    pub ai_enabled: bool,
    pub allowed_operations: Vec<String>, // e.g., ["ai:generate", "ai:search", "ai:metadata"]
    pub blocked_operations: Vec<String>, // Explicit denials
    pub allowed_users: Vec<i32>,         // Whitelist specific users
    pub blocked_users: Vec<i32>,         // Blacklist specific users
    pub min_role_for_ai: TeamRole,       // Minimum role to use AI
    pub monthly_budget_cents: Option<i32>,
}

/// Check team-based MCP permission
fn check_team_mcp_permission(
    user: &UserContext,
    operation: &str,
) -> Result<(), CustomHttpError> {
    // TODO: Load team MCP settings from database
    // For now, use default permissive settings
    
    // Simulated team settings check
    let team_role = user.team_role.as_ref()
        .map(|r| TeamRole::from_str(r))
        .unwrap_or(TeamRole::Member);
    
    // Team role hierarchy
    match team_role {
        TeamRole::Owner => Ok(()), // Owners can do anything
        TeamRole::Manager => {
            // Managers can use AI but not modify settings
            if operation.starts_with("ai:admin") {
                Err(CustomHttpError::Forbidden(
                    "Team owner permission required".to_string()
                ))
            } else {
                Ok(())
            }
        }
        TeamRole::Member => {
            // Members need explicit permission
            // TODO: Check against team's allowed_operations
            Ok(()) // Permissive for now
        }
    }
}

/// Get list of page IDs that user can access
/// Used for scoping semantic search and recommendations
/// TODO: Re-enable when pages table name is confirmed in schema
/*
pub async fn get_accessible_page_ids(
    user: &UserContext,
    pool: &web::Data<DbPool>,
) -> Result<Vec<i32>, CustomHttpError> {
    use crate::schema::pages;
    
    let mut conn = pool.get()
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let accessible_ids: Vec<i32> = if user.role == "admin" {
        // Admins can access all pages
        pages::table
            .select(pages::id)
            .load(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?
    } else if user.user_id == 0 {
        // Guests can only access published pages  
        pages::table
            .filter(pages::status.eq("published"))
            .select(pages::id)
            .load(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?
    } else {
        // Authenticated users can access published pages + their own
        pages::table
            .filter(
                pages::status.eq("published")
                .or(pages::created_by.eq(user.user_id))
            )
            .select(pages::id)
            .load(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?
    };
    
    Ok(accessible_ids)
}
*/

/// Log AI usage for tracking and billing
/// TODO: Re-enable when ai_usage_log table is in schema
/*
pub async fn log_ai_usage(
    pool: &web::Data<DbPool>,
    user_id: i32,
    operation: &str,
    provider_type: Option<&str>,
    tokens_used: i32,
    cost_cents: i32,
) -> Result<(), CustomHttpError> {
    use crate::models::ai_provider_models::NewAIUsageLog;
    use crate::schema::ai_usage_log;
    
    let mut conn = pool.get()
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let log_entry = NewAIUsageLog {
        user_id: Some(user_id),
        operation: operation.to_string(),
        provider_type: provider_type.map(|s| s.to_string()),
        tokens_used: Some(tokens_used),
        cost_cents: Some(cost_cents),
    };
    
    diesel::insert_into(ai_usage_log::table)
        .values(&log_entry)
        .execute(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(())
}

/// Get user's AI usage for the current month
/// TODO: Re-enable when ai_usage_log table is in schema  
pub async fn get_user_monthly_usage(
    pool: &web::Data<DbPool>,
    user_id: i32,
) -> Result<(i32, i32), CustomHttpError> {
    use crate::schema::ai_usage_log::dsl;
    use chrono::{Utc, Datelike};
    
    let mut conn = pool.get()
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    // Get first day of current month
    let now = Utc::now();
    let month_start = now
        .date_naive()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    
    // Sum tokens and costs for this month
    let results: Vec<(Option<i32>, Option<i32>)> = dsl::ai_usage_log
        .filter(dsl::user_id.eq(user_id))
        .filter(dsl::created_at.ge(month_start))
        .select((dsl::tokens_used, dsl::cost_cents))
        .load(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let total_tokens: i32 = results.iter()
        .filter_map(|(t, _)| *t)
        .sum();
    
    let total_cost: i32 = results.iter()
        .filter_map(|(_, c)| *c)
        .sum();
    
    Ok((total_tokens, total_cost))
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guest_permissions() {
        let guest = UserContext::default();
        
        // Guests can read public
        assert!(check_ai_permission(&guest, AIScope::ReadPublic).is_ok());
        
        // Guests cannot read own
        assert!(check_ai_permission(&guest, AIScope::ReadOwn).is_err());
        
        // Guests cannot write
        assert!(check_ai_permission(&guest, AIScope::WriteOwn).is_err());
        
        // Guests cannot admin
        assert!(check_ai_permission(&guest, AIScope::Admin).is_err());
    }
    
    #[test]
    fn test_user_permissions() {
        let user = UserContext {
            user_id: 123,
            username: "testuser".to_string(),
            role: "user".to_string(),
            permissions: vec!["read:own".to_string(), "write:own".to_string()],
            team_id: None,
            team_role: None,
        };
        
        // Users can read public
        assert!(check_ai_permission(&user, AIScope::ReadPublic).is_ok());
        
        // Users can read own
        assert!(check_ai_permission(&user, AIScope::ReadOwn).is_ok());
        
        // Users can write
        assert!(check_ai_permission(&user, AIScope::WriteOwn).is_ok());
        
        // Users cannot admin
        assert!(check_ai_permission(&user, AIScope::Admin).is_err());
    }
    
    #[test]
    fn test_admin_permissions() {
        let admin = UserContext {
            user_id: 1,
            username: "admin".to_string(),
            role: "admin".to_string(),
            permissions: vec!["*".to_string()],
            team_id: None,
            team_role: None,
        };
        
        // Admins have all permissions
        assert!(check_ai_permission(&admin, AIScope::ReadPublic).is_ok());
        assert!(check_ai_permission(&admin, AIScope::ReadOwn).is_ok());
        assert!(check_ai_permission(&admin, AIScope::WriteOwn).is_ok());
        assert!(check_ai_permission(&admin, AIScope::Admin).is_ok());
    }
}
