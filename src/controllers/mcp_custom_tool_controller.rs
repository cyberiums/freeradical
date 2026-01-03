use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{DbPool, mcp_tool_models::*, user_models::User};
use crate::services::{mcp_custom_tool_service::McpCustomToolService, errors_service::CustomHttpError};
use crate::helpers::tenant_helper::resolve_tenant_id;

// Helper to extract user from request (uses JWT middleware)
fn get_user_from_request(req: &HttpRequest) -> Result<User, CustomHttpError> {
        .get::<User>()
        .cloned()
        .ok_or_else(|| CustomHttpError::Unauthorized("User not authenticated".into()))
}

// ===== Request/Response DTOs =====

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCustomToolRequest {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub input_schema: serde_json::Value,
    pub webhook_url: String,
    pub webhook_method: Option<String>,
    pub webhook_headers: Option<serde_json::Value>,
    pub required_role: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub max_calls_per_hour: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCustomToolRequest {
    pub description: Option<String>,
    pub version: Option<String>,
    pub input_schema: Option<serde_json::Value>,
    pub webhook_url: Option<String>,
    pub webhook_method: Option<String>,
    pub webhook_headers: Option<serde_json::Value>,
    pub required_role: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub max_calls_per_hour: Option<i32>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CustomToolListResponse {
    pub tools: Vec<McpCustomTool>,
    pub total: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecutionLogResponse {
    pub executions: Vec<McpToolExecution>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ===== Handlers =====

/// List all custom tools for current tenant
/// GET /api/mcp/custom-tools
#[utoipa::path(
    get,
    path = "/api/mcp/custom-tools",
    responses(
        (status = 200, description = "List of custom tools", body = CustomToolListResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_custom_tools(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let _user = get_user_from_request(&req)?;
    // TODO: Extract role from JWT claims properly
    let user_role = "admin"; // Temp: all authenticated users are admin
    
    let tools = McpCustomToolService::list_custom_tools(&pool, tenant_id, user_role).await?;
    let total = tools.len();
    
    Ok(HttpResponse::Ok().json(CustomToolListResponse { tools, total }))
}

/// Get specific custom tool
/// GET /api/mcp/custom-tools/{id}
#[utoipa::path(
    get,
    path = "/api/mcp/custom-tools/{id}",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    responses(
        (status = 200, description = "Custom tool details", body = McpCustomTool),
        (status = 404, description = "Tool not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_custom_tool(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let tool_id = path.into_inner();
    
    let tool = McpCustomToolService::get_custom_tool(&pool, tool_id, tenant_id).await?;
    
    Ok(HttpResponse::Ok().json(tool))
}

/// Create new custom tool (Admin only)
/// POST /api/mcp/custom-tools
#[utoipa::path(
    post,
    path = "/api/mcp/custom-tools",
    request_body = CreateCustomToolRequest,
    responses(
        (status = 201, description = "Tool created successfully", body = McpCustomTool),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "Admin role required"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_custom_tool(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    body: web::Json<CreateCustomToolRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    
    // TODO: Check admin role properly (extract from JWT)
    // For now, all authenticated users can create tools
    
    let new_tool = NewMcpCustomTool {
        tenant_id,
        name: body.name.clone(),
        description: body.description.clone(),
        version: body.version.clone(),
        input_schema: body.input_schema.clone(),
        webhook_url: body.webhook_url.clone(),
        webhook_method: body.webhook_method.clone(),
        webhook_headers: body.webhook_headers.clone(),
        required_role: body.required_role.clone(),
        timeout_seconds: body.timeout_seconds,
        max_calls_per_hour: body.max_calls_per_hour,
        created_by: Some(user.id),
    };
    
    let tool = McpCustomToolService::create_custom_tool(&pool, new_tool).await?;
    
    Ok(HttpResponse::Created().json(tool))
}

/// Update custom tool (Admin only)
/// PUT /api/mcp/custom-tools/{id}
#[utoipa::path(
    put,
    path = "/api/mcp/custom-tools/{id}",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    request_body = UpdateCustomToolRequest,
    responses(
        (status = 200, description = "Tool updated successfully", body = McpCustomTool),
        (status = 403, description = "Admin role required"),
        (status = 404, description = "Tool not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_custom_tool(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateCustomToolRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    let tool_id = path.into_inner();
    
    // TODO: Check admin role properly (extract from JWT)
    // For now, all authenticated users can update tools
    
    let updates = UpdateMcpCustomTool {
        description: body.description.clone(),
        version: body.version.clone(),
        input_schema: body.input_schema.clone(),
        webhook_url: body.webhook_url.clone(),
        webhook_method: body.webhook_method.clone(),
        webhook_headers: body.webhook_headers.clone(),
        required_role: body.required_role.clone(),
        timeout_seconds: body.timeout_seconds,
        max_calls_per_hour: body.max_calls_per_hour,
        is_enabled: body.is_enabled,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let tool = McpCustomToolService::update_custom_tool(&pool, tool_id, tenant_id, updates).await?;
    
    Ok(HttpResponse::Ok().json(tool))
}

/// Delete custom tool (Admin only)
/// DELETE /api/mcp/custom-tools/{id}
#[utoipa::path(
    delete,
    path = "/api/mcp/custom-tools/{id}",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    responses(
        (status = 204, description = "Tool deleted successfully"),
        (status = 403, description = "Admin role required"),
        (status = 404, description = "Tool not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_custom_tool(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    let tool_id = path.into_inner();
    
    // TODO: Check admin role properly (extract from JWT)
    // For now, all authenticated users can delete tools
    
    let deleted = McpCustomToolService::delete_custom_tool(&pool, tool_id, tenant_id).await?;
    
    if deleted {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(CustomHttpError::NotFound("Tool not found".into()))
    }
}

/// Test custom tool execution
/// POST /api/mcp/custom-tools/{id}/test
#[utoipa::path(
    post,
    path = "/api/mcp/custom-tools/{id}/test",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Tool executed successfully"),
        (status = 400, description = "Execution failed"),
        (status = 429, description = "Rate limit exceeded"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn test_custom_tool(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    let tool_id = path.into_inner();
    
    // Get the tool
    let tool = McpCustomToolService::get_custom_tool(&pool, tool_id, tenant_id).await?;
    
    // Execute it
    let result = McpCustomToolService::execute_custom_tool(
        &pool,
        &tool,
        body.into_inner(),
        tenant_id,
        Some(user.id),
    ).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// Get execution logs for a tool
/// GET /api/mcp/custom-tools/{id}/executions
#[utoipa::path(
    get,
    path = "/api/mcp/custom-tools/{id}/executions",
    params(
        ("id" = i32, Path, description = "Tool ID"),
        ("limit" = Option<i64>, Query, description = "Number of records to return"),
        ("offset" = Option<i64>, Query, description = "Number of records to skip"),
    ),
    responses(
        (status = 200, description = "Execution logs", body = ExecutionLogResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_tool_executions(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let tool_id = path.into_inner();
    
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    
    let executions = McpCustomToolService::get_executions(&pool, tool_id, tenant_id, limit, offset).await?;
    let total = executions.len();
    
    Ok(HttpResponse::Ok().json(ExecutionLogResponse { executions, total }))
}

/// Publish tool to marketplace (make it public)
/// POST /api/mcp/custom-tools/{id}/publish
#[utoipa::path(
    post,
    path = "/api/mcp/custom-tools/{id}/publish",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    responses(
        (status = 200, description = "Tool published to marketplace"),
        (status = 403, description = "Admin role required"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn publish_to_marketplace(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    let tool_id = path.into_inner();
    
    // TODO: Check admin role properly (extract from JWT)
    // For now, all authenticated users can publish tools
    
    let updates = UpdateMcpCustomTool {
        is_public: Some(true),
        updated_at: Some(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    
    let tool = McpCustomToolService::update_custom_tool(&pool, tool_id, tenant_id, updates).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Tool published to marketplace successfully",
        "tool": tool
    })))
}

/// Unpublish tool from marketplace
/// POST /api/mcp/custom-tools/{id}/unpublish
#[utoipa::path(
    post,
    path = "/api/mcp/custom-tools/{id}/unpublish",
    params(
        ("id" = i32, Path, description = "Tool ID")
    ),
    responses(
        (status = 200, description = "Tool removed from marketplace"),
        (status = 403, description = "Admin role required"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn unpublish_from_marketplace(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool)?;
    let user = get_user_from_request(&req)?;
    let tool_id = path.into_inner();
    
    // TODO: Check admin role properly (extract from JWT)
    // For now, all authenticated users can unpublish tools
    
    let updates = UpdateMcpCustomTool {
        is_public: Some(false),
        updated_at: Some(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    
    let tool = McpCustomToolService::update_custom_tool(&pool, tool_id, tenant_id, updates).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Tool removed from marketplace successfully",
        "tool": tool
    })))
}

/// List marketplace tools (public tools from all tenants)
/// GET /api/mcp/marketplace
#[utoipa::path(
    get,
    path = "/api/mcp/marketplace",
    responses(
        (status = 200, description = "List of public marketplace tools", body = CustomToolListResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_marketplace_tools(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let pool_clone = pool.clone();
    
    let tools = web::block(move || {
        let mut conn = pool_clone.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB error: {}", e))
        })?;
        
        use crate::schema::mcp_custom_tools;
        
        mcp_custom_tools::table
            .filter(mcp_custom_tools::is_public.eq(Some(true)))
            .filter(mcp_custom_tools::is_enabled.eq(Some(true)))
            .load::<McpCustomTool>(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(format!("Query error: {}", e)))
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?;
    
    let total = tools.len();
    
    Ok(HttpResponse::Ok().json(CustomToolListResponse { tools, total }))
}

// Route initialization
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/mcp")
            .route("/custom-tools", web::get().to(list_custom_tools))
            .route("/custom-tools", web::post().to(create_custom_tool))
            .route("/custom-tools/{id}", web::get().to(get_custom_tool))
            .route("/custom-tools/{id}", web::put().to(update_custom_tool))
            .route("/custom-tools/{id}", web::delete().to(delete_custom_tool))
            .route("/custom-tools/{id}/test", web::post().to(test_custom_tool))
            .route("/custom-tools/{id}/executions", web::get().to(get_tool_executions))
            .route("/custom-tools/{id}/publish", web::post().to(publish_to_marketplace))
            .route("/custom-tools/{id}/unpublish", web::post().to(unpublish_from_marketplace))
            .route("/marketplace", web::get().to(list_marketplace_tools))
    );
}
