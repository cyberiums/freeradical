use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use utoipa::ToSchema;

/// Custom MCP Tool - Webhooks execute externally for 200% isolation
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crate::schema::mcp_custom_tools)]
pub struct McpCustomTool {
    pub id: i32,
    pub tenant_id: i32,
    
    // Tool Metadata
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    
    // MCP Tool Definition
    pub input_schema: serde_json::Value,
    
    // HTTP Webhook (executes externally - 200% isolation!)
    pub webhook_url: String,
    pub webhook_method: Option<String>,
    pub webhook_headers: Option<serde_json::Value>,
    
    // Security & Isolation
    pub required_role: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub max_calls_per_hour: Option<i32>,
    
    // Status
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    
    // Metadata
    pub created_by: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_used_at: Option<NaiveDateTime>,
    pub total_calls: Option<i32>,
    pub success_count: Option<i32>,
    pub error_count: Option<i32>,
    pub avg_execution_ms: Option<i32>,
}

/// Create new custom tool
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crate::schema::mcp_custom_tools)]
pub struct NewMcpCustomTool {
    pub tenant_id: i32,
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
    pub created_by: Option<i32>,
}

/// Update existing tool
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::mcp_custom_tools)]
pub struct UpdateMcpCustomTool {
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
    pub is_public: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Default for UpdateMcpCustomTool {
    fn default() -> Self {
        Self {
            description: None,
            version: None,
            input_schema: None,
            webhook_url: None,
            webhook_method: None,
            webhook_headers: None,
            required_role: None,
            timeout_seconds: None,
            max_calls_per_hour: None,
            is_enabled: None,
            is_public: None,
            updated_at: None,
        }
    }
}

/// Tool Execution Log (audit trail)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::mcp_tool_executions)]
pub struct McpToolExecution {
    pub id: i32,
    pub tool_id: i32,
    pub tenant_id: i32,
    pub user_id: Option<i32>,
    pub input_data: Option<serde_json::Value>,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i32>,
    pub http_status_code: Option<i32>,
    pub status: Option<String>,
    pub executed_at: Option<NaiveDateTime>,
}

/// Log new execution
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::mcp_tool_executions)]
pub struct NewMcpToolExecution {
    pub tool_id: i32,
    pub tenant_id: i32,
    pub user_id: Option<i32>,
    pub input_data: Option<serde_json::Value>,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i32>,
    pub http_status_code: Option<i32>,
    pub status: Option<String>,
}

/// Rate Limit Tracking
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::mcp_tool_rate_limits)]
pub struct McpToolRateLimit {
    pub id: i32,
    pub tool_id: i32,
    pub tenant_id: i32,
    pub window_start: NaiveDateTime,
    pub call_count: Option<i32>,
}

/// Track rate limit
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::mcp_tool_rate_limits)]
pub struct NewMcpToolRateLimit {
    pub tool_id: i32,
    pub tenant_id: i32,
    pub window_start: NaiveDateTime,
    pub call_count: Option<i32>,
}
