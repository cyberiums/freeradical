use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::DbPool;
use crate::schema::{users, audit_logs};
use crate::services::errors_service::CustomHttpError;

/// System health response
#[derive(Debug, Serialize, ToSchema)]
pub struct SystemHealthResponse {
    pub status: String,
    pub database: String,
    pub redis: String,
    pub uptime_seconds: u64,
}

/// User list response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserInfo>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// User info for admin view
#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub uuid: String,
    pub two_factor_enabled: bool,
}

/// Update role request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// Clear cache request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ClearCacheRequest {
    pub cache_type: String, // "all", "pages", "api", etc.
}

/// Clear cache response
#[derive(Debug, Serialize, ToSchema)]
pub struct ClearCacheResponse {
    pub message: String,
    pub cache_type: String,
    pub cleared: bool,
}

/// Log entry
#[derive(Debug, Serialize, ToSchema)]
pub struct LogEntry {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub user_id: i32,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
}

/// List all users (admin only)
#[utoipa::path(
    get,
    path = "/v1/admin/users",
    tag = "Internal - Admin",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 50)")
    ),
    responses(
        (status = 200, description = "List of users", body = UserListResponse),
        (status = 403, description = "Forbidden - admin only"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, CustomHttpError> {
    let page = query.get("page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(1);
    let per_page = query.get("per_page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(50)
        .min(100);
    
    let offset = (page - 1) * per_page;
    
    let (user_list, total) = web::block(move || -> Result<(Vec<UserInfo>, i64), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        let total = users::table.count().get_result::<i64>(&mut conn)?;
        
        let users_data = users::table
            .select((
                users::id,
                users::username,
                users::uuid,
                users::two_factor_enabled,
            ))
            .limit(per_page)
            .offset(offset)
            .load::<(i32, String, String, bool)>(&mut conn)?;
        
        let user_list = users_data.into_iter().map(|(id, username, uuid, two_factor_enabled)| {
            UserInfo {
                id,
                username,
                uuid,
                two_factor_enabled,
            }
        }).collect();
        
        Ok((user_list, total))
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(UserListResponse {
        users: user_list,
        total,
        page,
        per_page,
    }))
}

/// Update user role (admin only) - Placeholder for future role system
#[utoipa::path(
    put,
    path = "/v1/admin/users/{id}/role",
    tag = "Internal - Admin",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated"),
        (status = 403, description = "Forbidden - admin only"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_user_role(
    _pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    payload: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = user_id.into_inner();
    
    // TODO: Implement actual role assignment when role system is ready
    // For now, just return success as placeholder
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": id,
        "role": payload.role,
        "message": "Role update queued (placeholder - role system pending)"
    })))
}

/// System health check
#[utoipa::path(
    get,
    path = "/v1/admin/health",
    tag = "Internal - Admin",
    responses(
        (status = 200, description = "System health status", body = SystemHealthResponse)
    )
)]
pub async fn system_health(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let db_status = web::block(move || -> Result<String, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        diesel::sql_query("SELECT 1").execute(&mut conn)?;
        Ok("connected".to_string())
    })
    .await
    .unwrap_or_else(|_| Err(diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::Unknown,
        Box::new("Disconnected".to_string())
    )))
    .unwrap_or_else(|_| "disconnected".to_string());
    
    let redis_status = "not_configured".to_string();
    let uptime_seconds = 3600;
    
    let overall_status = if db_status == "connected" {
        "healthy"
    } else {
        "degraded"
    };
    
    Ok(HttpResponse::Ok().json(SystemHealthResponse {
        status: overall_status.to_string(),
        database: db_status,
        redis: redis_status,
        uptime_seconds,
    }))
}

/// Clear cache
#[utoipa::path(
    post,
    path = "/v1/admin/cache/clear",
    tag = "Internal - Admin",
    request_body = ClearCacheRequest,
    responses(
        (status = 200, description = "Cache cleared", body = ClearCacheResponse),
        (status = 403, description = "Forbidden - admin only"),
        (status = 500, description = "Cache error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn clear_cache(
    payload: web::Json<ClearCacheRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let cache_type = &payload.cache_type;
    
    // TODO: Integrate with actual cache service when available
    let cleared = match cache_type.as_str() {
        "all" | "pages" | "api" => true,
        _ => false,
    };
    
    Ok(HttpResponse::Ok().json(ClearCacheResponse {
        message: format!("Cache '{}' cleared successfully (placeholder)", cache_type),
        cache_type: cache_type.clone(),
        cleared,
    }))
}

/// View system logs
#[utoipa::path(
    get,
    path = "/v1/admin/logs",
    tag = "Internal - Admin",
    params(
        ("limit" = Option<i64>, Query, description = "Number of logs to return (default: 100)")
    ),
    responses(
        (status = 200, description = "System logs"),
        (status = 403, description = "Forbidden - admin only"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn view_logs(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, CustomHttpError> {
    let limit = query.get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(100)
        .min(1000);
    
    let logs = web::block(move || -> Result<Vec<LogEntry>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        let log_data = audit_logs::table
            .select((
                audit_logs::id,
                audit_logs::tenant_id,
                audit_logs::user_id,
                audit_logs::action,
                audit_logs::resource_type,
                audit_logs::resource_id,
            ))
            .order(audit_logs::created_at.desc())
            .limit(limit)
            .load::<(i32, Option<i32>, i32, String, String, Option<String>)>(&mut conn)?;
        
        let logs = log_data.into_iter().map(|(id, tenant_id, user_id, action, resource_type, resource_id)| {
            LogEntry {
                id,
                tenant_id,
                user_id,
                action,
                resource_type,
                resource_id,
            }
        }).collect();
        
        Ok(logs)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "logs": logs,
        "total": logs.len()
    })))
}
