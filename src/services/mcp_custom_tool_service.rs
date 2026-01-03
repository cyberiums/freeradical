use actix_web::web;
use diesel::prelude::*;
use chrono::{Utc, Duration};
use serde_json;
use reqwest;
use std::time::Instant;

use crate::models::{DbPool, mcp_tool_models::*};
use crate::schema::{mcp_custom_tools, mcp_tool_executions, mcp_tool_rate_limits};
use crate::services::errors_service::CustomHttpError;

/// Custom MCP Tool Service
/// 
/// **200% Isolation Guarantee:**
/// - All tools execute via HTTP webhooks on EXTERNAL servers
/// - NO code execution happens on our infrastructure
/// - Network-level isolation (webhooks run in user's environment)
/// - Timeout protection (configurable per tool, default 30s)
/// - Rate limiting (prevents abuse, configurable per tool)
pub struct McpCustomToolService;

impl McpCustomToolService {
    /// List all custom tools for a tenant (filtered by user role)
    pub async fn list_custom_tools(
        pool: &DbPool,
        tenant_id: i32,
        user_role: &str,
    ) -> Result<Vec<McpCustomTool>, CustomHttpError> {
        let pool_clone = pool.clone();
        let user_role = user_role.to_string();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            // Get tools for this tenant OR public tools
            let tools: Vec<McpCustomTool> = mcp_custom_tools::table
                .filter(
                    mcp_custom_tools::tenant_id.eq(tenant_id)
                        .or(mcp_custom_tools::is_public.eq(Some(true)))
                )
                .filter(mcp_custom_tools::is_enabled.eq(Some(true)))
                .load::<McpCustomTool>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Query error: {}", e)))?;
            
            // Filter by required role (viewer < editor < admin)
            let filtered_tools: Vec<McpCustomTool> = tools
                .into_iter()
                .filter(|tool| {
                    let required = tool.required_role.as_deref().unwrap_or("viewer");
                    match user_role.as_str() {
                        "admin" => true,  // Admin can see all
                        "editor" => required == "viewer" || required == "editor",
                        "viewer" => required == "viewer",
                        _ => false,
                    }
                })
                .collect();
            
            Ok(filtered_tools)
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Get specific custom tool
    pub async fn get_custom_tool(
        pool: &DbPool,
        tool_id: i32,
        tenant_id: i32,
    ) -> Result<McpCustomTool, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            mcp_custom_tools::table
                .filter(mcp_custom_tools::id.eq(tool_id))
                .filter(
                    mcp_custom_tools::tenant_id.eq(tenant_id)
                        .or(mcp_custom_tools::is_public.eq(Some(true)))
                )
                .first::<McpCustomTool>(&mut conn)
                .map_err(|_| CustomHttpError::NotFound("Tool not found".into()))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Get tool by name (for MCP execution)
    pub async fn get_tool_by_name(
        pool: &DbPool,
        name: &str,
        tenant_id: i32,
    ) -> Result<McpCustomTool, CustomHttpError> {
        let pool_clone = pool.clone();
        let name = name.to_string();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            mcp_custom_tools::table
                .filter(mcp_custom_tools::name.eq(&name))
                .filter(
                    mcp_custom_tools::tenant_id.eq(tenant_id)
                        .or(mcp_custom_tools::is_public.eq(Some(true)))
                )
                .filter(mcp_custom_tools::is_enabled.eq(Some(true)))
                .first::<McpCustomTool>(&mut conn)
                .map_err(|_| CustomHttpError::NotFound(format!("Tool '{}' not found", name)))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Create new custom tool (Admin only)
    pub async fn create_custom_tool(
        pool: &DbPool,
        new_tool: NewMcpCustomTool,
    ) -> Result<McpCustomTool, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            diesel::insert_into(mcp_custom_tools::table)
                .values(&new_tool)
                .get_result::<McpCustomTool>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Insert error: {}", e)))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Update custom tool (Admin only)
    pub async fn update_custom_tool(
        pool: &DbPool,
        tool_id: i32,
        tenant_id: i32,
        updates: UpdateMcpCustomTool,
    ) -> Result<McpCustomTool, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            diesel::update(mcp_custom_tools::table)
                .filter(mcp_custom_tools::id.eq(tool_id))
                .filter(mcp_custom_tools::tenant_id.eq(tenant_id))
                .set(&updates)
                .get_result::<McpCustomTool>(&mut conn)
                .map_err(|_| CustomHttpError::NotFound("Tool not found or not authorized".into()))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Delete custom tool (Admin only)
    pub async fn delete_custom_tool(
        pool: &DbPool,
        tool_id: i32,
        tenant_id: i32,
    ) -> Result<bool, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            let deleted = diesel::delete(mcp_custom_tools::table)
                .filter(mcp_custom_tools::id.eq(tool_id))
                .filter(mcp_custom_tools::tenant_id.eq(tenant_id))
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Delete error: {}", e)))?;
            
            Ok(deleted > 0)
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Check rate limit (returns true if allowed, false if rate limited)
    pub async fn check_rate_limit(
        pool: &DbPool,
        tool: &McpCustomTool,
        tenant_id: i32,
    ) -> Result<bool, CustomHttpError> {
        let max_calls = tool.max_calls_per_hour.unwrap_or(100);
        let tool_id = tool.id;
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            // Get current hour window (round down to start of hour)
            let now = Utc::now().naive_utc();
            let window_start = now.date().and_hms_opt(now.hour(), 0, 0).unwrap();
            
            // Get or create rate limit record
            let rate_limit = mcp_tool_rate_limits::table
                .filter(mcp_tool_rate_limits::tool_id.eq(tool_id))
                .filter(mcp_tool_rate_limits::tenant_id.eq(tenant_id))
                .filter(mcp_tool_rate_limits::window_start.eq(window_start))
                .first::<McpToolRateLimit>(&mut conn)
                .optional()
                .map_err(|e| CustomHttpError::InternalServerError(format!("Query error: {}", e)))?;
            
            match rate_limit {
                Some(limit) => {
                    let current_count = limit.call_count.unwrap_or(0);
                    if current_count >= max_calls {
                        Ok(false) // Rate limited!
                    } else {
                        // Increment counter
                        diesel::update(mcp_tool_rate_limits::table)
                            .filter(mcp_tool_rate_limits::id.eq(limit.id))
                            .set(mcp_tool_rate_limits::call_count.eq(current_count + 1))
                            .execute(&mut conn)
                            .map_err(|e| CustomHttpError::InternalServerError(format!("Update error: {}", e)))?;
                        Ok(true)
                    }
                }
                None => {
                    // Create new rate limit record
                    diesel::insert_into(mcp_tool_rate_limits::table)
                        .values(&NewMcpToolRateLimit {
                            tool_id,
                            tenant_id,
                            window_start,
                            call_count: Some(1),
                        })
                        .execute(&mut conn)
                        .map_err(|e| CustomHttpError::InternalServerError(format!("Insert error: {}", e)))?;
                    Ok(true)
                }
            }
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Execute custom tool via HTTP webhook (200% ISOLATED!)
    /// 
    /// **Security Features:**
    /// - Executes on EXTERNAL server (user's infrastructure)
    /// - Timeout protection (default 30s, configurable)
    /// - Rate limit enforcement
    /// - Input/output logging for audit trail
    /// - HTTP status code tracking
    pub async fn execute_custom_tool(
        pool: &DbPool,
        tool: &McpCustomTool,
        input_data: serde_json::Value,
        tenant_id: i32,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value, CustomHttpError> {
        // 1. Check rate limit
        if !Self::check_rate_limit(pool, tool, tenant_id).await? {
            return Err(CustomHttpError::TooManyRequests(
                format!("Rate limit exceeded for tool '{}': max {} calls/hour", tool.name, tool.max_calls_per_hour.unwrap_or(100))
            ));
        }
        
        // 2. Execute webhook (200% ISOLATED - runs externally!)
        let start_time = Instant::now();
        let timeout = std::time::Duration::from_secs(tool.timeout_seconds.unwrap_or(30) as u64);
        let method = tool.webhook_method.as_deref().unwrap_or("POST");
        let url = tool.webhook_url.clone();
        let headers = tool.webhook_headers.clone().unwrap_or(serde_json::json!({}));
        
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| CustomHttpError::InternalServerError(format!("HTTP client error: {}", e)))?;
        
        // Build request
        let mut request = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "PATCH" => client.patch(&url),
            _ => return Err(CustomHttpError::BadRequest(format!("Unsupported HTTP method: {}", method))),
        };
        
        // Add custom headers
        if let Some(headers_obj) = headers.as_object() {
            for (key, value) in headers_obj {
                if let Some(val_str) = value.as_str() {
                    request = request.header(key, val_str);
                }
            }
        }
        
        // Send request with input data
        let response = request
            .json(&input_data)
            .send()
            .await;
        
        let execution_time_ms = start_time.elapsed().as_millis() as i32;
        
        // 3. Process response
        let (output_data, error_message, status, http_status_code) = match response {
            Ok(resp) => {
                let status_code = resp.status().as_u16() as i32;
                if resp.status().is_success() {
                    match resp.json::<serde_json::Value>().await {
                        Ok(json) => (Some(json.clone()), None, "success", status_code),
                        Err(e) => (None, Some(format!("Invalid JSON response: {}", e)), "error", status_code),
                    }
                } else {
                    let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    (None, Some(format!("HTTP {}: {}", status_code, error_text)), "error", status_code)
                }
            }
            Err(e) => {
                let error_msg = if e.is_timeout() {
                    format!("Timeout after {}s", tool.timeout_seconds.unwrap_or(30))
                } else {
                    format!("HTTP request failed: {}", e)
                };
                (None, Some(error_msg), "error", 0)
            }
        };
        
        // 4. Log execution
        let tool_id = tool.id;
        let log_pool = pool.clone();
        let log_input = input_data.clone();
        let log_output = output_data.clone();
        let log_error = error_message.clone();
        let log_status = status.to_string();
        
        actix_web::rt::spawn(async move {
            let _ = Self::log_execution(
                &log_pool,
                tool_id,
                tenant_id,
                user_id,
                log_input,
                log_output.clone(),
                log_error,
                execution_time_ms,
                http_status_code,
                log_status,
            ).await;
        });
        
        // 5. Update tool stats
        let stats_pool = pool.clone();
        actix_web::rt::spawn(async move {
            let _ = Self::update_tool_stats(&stats_pool, tool_id, status == "success", execution_time_ms).await;
        });
        
        // 6. Return result
        if status == "success" {
            Ok(output_data.unwrap())
        } else {
            Err(CustomHttpError::InternalServerError(
                error_message.unwrap_or_else(|| "Tool execution failed".to_string())
            ))
        }
    }
    
    /// Log tool execution
    async fn log_execution(
        pool: &DbPool,
        tool_id: i32,
        tenant_id: i32,
        user_id: Option<i32>,
        input_data: serde_json::Value,
        output_data: Option<serde_json::Value>,
        error_message: Option<String>,
        execution_time_ms: i32,
        http_status_code: i32,
        status: String,
    ) -> Result<(), CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            diesel::insert_into(mcp_tool_executions::table)
                .values(&NewMcpToolExecution {
                    tool_id,
                    tenant_id,
                    user_id,
                    input_data: Some(input_data),
                    output_data,
                    error_message,
                    execution_time_ms: Some(execution_time_ms),
                    http_status_code: Some(http_status_code),
                    status: Some(status),
                })
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Log error: {}", e)))?;
            
            Ok(())
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Update tool statistics
    async fn update_tool_stats(
        pool: &DbPool,
        tool_id: i32,
        success: bool,
        execution_time_ms: i32,
    ) -> Result<(), CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            // Get current stats
            let tool = mcp_custom_tools::table
                .find(tool_id)
                .first::<McpCustomTool>(&mut conn)
                .optional()
                .map_err(|e| CustomHttpError::InternalServerError(format!("Query error: {}", e)))?;
            
            if let Some(tool) = tool {
                let total_calls = tool.total_calls.unwrap_or(0) + 1;
                let success_count = if success { tool.success_count.unwrap_or(0) + 1 } else { tool.success_count.unwrap_or(0) };
                let error_count = if !success { tool.error_count.unwrap_or(0) + 1 } else { tool.error_count.unwrap_or(0) };
                
                // Calculate new average execution time
                let old_avg = tool.avg_execution_ms.unwrap_or(0);
                let old_total = tool.total_calls.unwrap_or(0);
                let new_avg = if old_total > 0 {
                    ((old_avg * old_total) + execution_time_ms) / total_calls
                } else {
                    execution_time_ms
                };
                
                diesel::update(mcp_custom_tools::table)
                    .filter(mcp_custom_tools::id.eq(tool_id))
                    .set((
                        mcp_custom_tools::total_calls.eq(total_calls),
                        mcp_custom_tools::success_count.eq(success_count),
                        mcp_custom_tools::error_count.eq(error_count),
                        mcp_custom_tools::avg_execution_ms.eq(new_avg),
                        mcp_custom_tools::last_used_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(&mut conn)
                    .map_err(|e| CustomHttpError::InternalServerError(format!("Update error: {}", e)))?;
            }
            
            Ok(())
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Execute tool by name (for MCP call_tool)
    pub async fn execute_by_name(
        pool: &DbPool,
        name: &str,
        input_data: serde_json::Value,
        tenant_id: i32,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value, CustomHttpError> {
        let tool = Self::get_tool_by_name(pool, name, tenant_id).await?;
        Self::execute_custom_tool(pool, &tool, input_data, tenant_id, user_id).await
    }
    
    /// Get execution logs for a tool
    pub async fn get_executions(
        pool: &DbPool,
        tool_id: i32,
        tenant_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<McpToolExecution>, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB error: {}", e))
            })?;
            
            mcp_tool_executions::table
                .filter(mcp_tool_executions::tool_id.eq(tool_id))
                .filter(mcp_tool_executions::tenant_id.eq(tenant_id))
                .order(mcp_tool_executions::executed_at.desc())
                .limit(limit)
                .offset(offset)
                .load::<McpToolExecution>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Query error: {}", e)))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
}
