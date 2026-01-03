use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, ActorContext};
use serde::{Deserialize, Serialize};
use serde_json::json;
use log::{info, error};
use std::sync::Arc;

use crate::models::DbPool;
use crate::services::mcp_custom_tool_service::McpCustomToolService;
use crate::models::mcp_tool_models::McpCustomTool;

// ===== MCP Protocol Types =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// ===== Resource Definitions =====

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

// ===== Tool Specification for Data-Driven Routing =====

#[derive(Debug, Clone)]
struct ToolSpec {
    name: &'static str,
    required_role: &'static str,  // "viewer", "editor", "admin"
    http_method: &'static str,     // "GET", "POST", "PUT", "DELETE"
    endpoint: &'static str,        // REST API endpoint
    description: &'static str,     // Human-readable description
}

// Tool specifications for all platform tools
const TOOL_SPECS: &[ToolSpec] = &[
    // Content Management Tools
    ToolSpec {
        name: "list_content",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/content",
        description: "To list content (posts/pages), make this API call:",
    },
    ToolSpec {
        name: "create_content",
        required_role: "editor",
        http_method: "POST",
        endpoint: "/v1/api/content",
        description: "To create new blog post or page, make this API call:",
    },
    ToolSpec {
        name: "update_content",
        required_role: "editor",
        http_method: "PUT",
        endpoint: "/v1/api/content/{id}",
        description: "To update existing content, make this API call:",
    },
    ToolSpec {
        name: "delete_content",
        required_role: "admin",
        http_method: "DELETE",
        endpoint: "/v1/api/content/{id}",
        description: "To delete content, make this API call:",
    },
    ToolSpec {
        name: "generate_seo_metadata",
        required_role: "editor",
        http_method: "POST",
        endpoint: "/v1/ai/seo/generate",
        description: "To generate SEO metadata using AI, make this API call:",
    },
    
    // Customer Management Tools
    ToolSpec {
        name: "list_customers",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/customers",
        description: "To list all customers with filters, make this API call:",
    },
    ToolSpec {
        name: "get_customer_details",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/customers/{id}",
        description: "To get detailed customer information, make this API call:",
    },
    ToolSpec {
        name: "update_customer",
        required_role: "editor",
        http_method: "PUT",
        endpoint: "/v1/api/crm/customers/{id}",
        description: "To update customer information, make this API call:",
    },
    ToolSpec {
        name: "add_customer_note",
        required_role: "editor",
        http_method: "POST",
        endpoint: "/v1/api/crm/customers/{id}/notes",
        description: "To add a note to customer record, make this API call:",
    },
    ToolSpec {
        name: "log_customer_interaction",
        required_role: "editor",
        http_method: "POST",
        endpoint: "/v1/api/crm/interactions",
        description: "To log a customer interaction (email/call/meeting), make this API call:",
    },
    
    // Commerce Management Tools
    ToolSpec {
        name: "list_products",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/products",
        description: "To list products with inventory information, make this API call:",
    },
    ToolSpec {
        name: "update_inventory",
        required_role: "editor",
        http_method: "PUT",
        endpoint: "/v1/api/inventory/{product_id}",
        description: "To update product inventory levels, make this API call:",
    },
    ToolSpec {
        name: "list_orders",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/orders",
        description: "To list customer orders, make this API call:",
    },
    ToolSpec {
        name: "update_order_status",
        required_role: "editor",
        http_method: "PUT",
        endpoint: "/v1/api/orders/{id}/status",
        description: "To update order fulfillment status, make this API call:",
    },
    ToolSpec {
        name: "get_sales_analytics",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/analytics/sales",
        description: "To get sales and revenue analytics, make this API call:",
    },
    
    // Newsletter & Email Tools
    ToolSpec {
        name: "list_newsletter_subscribers",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/customers?tags=newsletter_subscriber",
        description: "To list all newsletter subscribers, make this API call:",
    },
    ToolSpec {
        name: "get_subscriber_stats",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/customers?tags=newsletter_subscriber&count=true",
        description: "To get newsletter subscription statistics, make this API call:",
    },
    ToolSpec {
        name: "create_newsletter_segment",
        required_role: "admin",
        http_method: "POST",
        endpoint: "/v1/api/crm/segments",
        description: "To create a targeted subscriber segment, make this API call:",
    },
    ToolSpec {
        name: "list_segments",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/segments",
        description: "To list all CRM segments, make this API call:",
    },
    ToolSpec {
        name: "list_campaigns",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/crm/campaigns",
        description: "To list email campaigns, make this API call:",
    },
    ToolSpec {
        name: "create_campaign",
        required_role: "editor",
        http_method: "POST",
        endpoint: "/v1/api/crm/campaigns",
        description: "To create a new email campaign, make this API call:",
    },
    ToolSpec {
        name: "list_pending_verifications",
        required_role: "editor",
        http_method: "GET",
        endpoint: "/v1/api/verification/pending",
        description: "To show pending email verifications, make this API call:",
    },
    ToolSpec {
        name: "cleanup_expired_verifications",
        required_role: "admin",
        http_method: "POST",
        endpoint: "/v1/api/verification/cleanup",
        description: "To manually cleanup expired verifications, make this API call:",
    },
];

// ===== WebSocket Actor =====

pub struct MCPWebSocket {
    pool: Arc<DbPool>,
    tenant_id: Option<i32>,  // Extracted from JWT
    user_id: Option<i32>,    // Extracted from JWT  
    role: Option<String>,     // User role (admin, editor, viewer)
    authenticated: bool,
}

impl MCPWebSocket {
    pub fn new(pool: Arc<DbPool>, tenant_id: Option<i32>, user_id: Option<i32>, role: Option<String>) -> Self {
        Self { 
            pool,
            tenant_id,
            user_id,
            role,
            authenticated: tenant_id.is_some(),
        }
    }
    
    fn check_auth(&self) -> Result<i32, MCPError> {
        self.tenant_id.ok_or_else(|| MCPError {
            code: -32001,
            message: "Authentication required. Please provide bearer token.".to_string(),
            data: None,
        })
    }
    
    fn check_role(&self, required_role: &str) -> Result<(), MCPError> {
        let user_role = self.role.as_deref().unwrap_or("viewer");
        
        match required_role {
            "admin" => {
                if user_role != "admin" {
                    return Err(MCPError {
                        code: -32002,
                        message: format!("Insufficient permissions. Required: admin, your role: {}", user_role),
                        data: Some(json!({"required": "admin", "actual": user_role})),
                    });
                }
            }
            "editor" => {
                if user_role != "admin" && user_role != "editor" {
                    return Err(MCPError {
                        code: -32002,
                        message: format!("Insufficient permissions. Required: editor or admin, your role: {}", user_role),
                        data: Some(json!({"required": "editor", "actual": user_role})),
                    });
                }
            }
            _ => {} // "viewer" has access by default
        }
        
        Ok(())
    }

    fn handle_request(&self, req: MCPRequest) -> MCPResponse {
        info!("🔧 MCP Request: method={}, id={:?}", req.method, req.id);

        match req.method.as_str() {
            "initialize" => self.handle_initialize(req.id),
            "resources/list" => self.handle_list_resources(req.id),
            "resources/read" => self.handle_read_resource(req.params, req.id),
            "tools/list" => self.handle_list_tools(req.id),
            "tools/call" => self.handle_call_tool(req.params, req.id),
            _ => MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Method not found: {}", req.method),
                    data: None,
                }),
                id: req.id,
            },
        }
    }

    fn handle_initialize(&self, id: Option<serde_json::Value>) -> MCPResponse {
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "FreeRadical CMS MCP Server",
                    "version": "1.0.0"
                },
                "capabilities": {
                    "resources": {
                        "subscribe": false,
                        "listChanged": false
                    },
                    "tools": {
                        "listChanged": false
                    }
                }
            })),
            error: None,
            id,
        }
    }

    fn handle_list_resources(&self, id: Option<serde_json::Value>) -> MCPResponse {
        let resources = vec![
            MCPResource {
                uri: "freeradical://verification/settings".to_string(),
                name: "Verification Settings".to_string(),
                description: "Email verification configuration for all types".to_string(),
                mime_type: "application/json".to_string(),
            },
            MCPResource {
                uri: "freeradical://verification/settings/crm_customer".to_string(),
                name: "Newsletter Verification Settings".to_string(),
                description: "Settings for newsletter/CRM customer verification".to_string(),
                mime_type: "application/json".to_string(),
            },
        ];

        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({ "resources": resources })),
            error: None,
            id,
        }
    }

    fn handle_read_resource(&self, params: Option<serde_json::Value>, id: Option<serde_json::Value>) -> MCPResponse {
        let uri = params
            .and_then(|p| p.get("uri").and_then(|u| u.as_str()).map(String::from))
            .unwrap_or_default();

        info!("📖 Reading resource: {}", uri);

        // For now, return example data
        // In production, this would query the database
        let content = match uri.as_str() {
            "freeradical://verification/settings" => json!({
                "settings": [
                    {
                        "type": "crm_customer",
                        "ttl_hours": 12,
                        "enabled": true
                    }
                ]
            }),
            _ => json!({"error": "Resource not found"}),
        };

        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": serde_json::to_string_pretty(&content).unwrap()
                }]
            })),
            error: None,
            id,
        }
    }

    fn handle_list_tools(&self, id: Option<serde_json::Value>) -> MCPResponse {
        let mut tools = vec![
            // ===== Verification Tools =====
            MCPTool {
                name: "update_verification_settings".to_string(),
                description: "Update email verification settings for a specific type".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "verification_type": {
                            "type": "string",
                            "description": "Type of verification (e.g., crm_customer, user_registration)",
                            "enum": ["crm_customer", "user_registration", "form_submission"]
                        },
                        "ttl_hours": {
                            "type": "integer",
                            "description": "Time to live in hours (1-168)",
                            "minimum": 1,
                            "maximum": 168
                        },
                        "enabled": {
                            "type": "boolean",
                            "description": "Whether verification is enabled"
                        },
                        "email_template": {
                            "type": "string",
                            "description": "Custom HTML email template (optional)"
                        }
                    },
                    "required": ["verification_type"]
                }),
            },
            MCPTool {
                name: "get_verification_settings".to_string(),
                description: "Get current verification settings".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "verification_type": {
                            "type": "string",
                            "description": "Optional: filter by verification type"
                        }
                    }
                }),
            },
            
            // ===== Content Management Tools =====
            MCPTool {
                name: "list_content".to_string(),
                description: "List blog posts, pages, or all content with filters".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "content_type": {
                            "type": "string",
                            "description": "Type of content to list",
                            "enum": ["post", "page", "all"]
                        },
                        "status": {
                            "type": "string",
                            "description": "Filter by status",
                            "enum": ["draft", "published", "scheduled"]
                        },
                        "search": {
                            "type": "string",
                            "description": "Search query for title/content"
                        },
                        "page": {
                            "type": "integer",
                            "description": "Page number",
                            "minimum": 1
                        },
                        "per_page": {
                            "type": "integer",
                            "description": "Results per page",
                            "minimum": 1,
                            "maximum": 100
                        }
                    }
                }),
            },
            MCPTool {
                name: "create_content".to_string(),
                description: "Create new blog post or page".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "Content title"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content body (HTML or Markdown)"
                        },
                        "content_type": {
                            "type": "string",
                            "description": "Type of content",
                            "enum": ["post", "page"]
                        },
                        "status": {
                            "type": "string",
                            "description": "Publication status",
                            "enum": ["draft", "published"]
                        },
                        "tags": {
                            "type": "array",
                            "description": "Content tags",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["title", "content", "content_type"]
                }),
            },
            MCPTool {
                name: "update_content".to_string(),
                description: "Update existing blog post or page".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "content_id": {
                            "type": "integer",
                            "description": "ID of content to update"
                        },
                        "title": {
                            "type": "string",
                            "description": "Content title"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content body"
                        },
                        "status": {
                            "type": "string",
                            "description": "Publication status",
                            "enum": ["draft", "published"]
                        }
                    },
                    "required": ["content_id"]
                }),
            },
            MCPTool {
                name: "delete_content".to_string(),
                description: "Delete blog post or page (Admin only)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "content_id": {
                            "type": "integer",
                            "description": "ID of content to delete"
                        }
                    },
                    "required": ["content_id"]
                }),
            },
            MCPTool {
                name: "generate_seo_metadata".to_string(),
                description: "Generate SEO meta tags for content using AI".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "Content title"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content body excerpt"
                        },
                        "keywords": {
                            "type": "array",
                            "description": "Target keywords",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["title", "content"]
                }),
            },
            
            // ===== Customer Management Tools =====
            MCPTool {
                name: "list_customers".to_string(),
                description: "List all customers with filters".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "lifecycle_stage": {
                            "type": "string",
                            "description": "Filter by lifecycle stage",
                            "enum": ["lead", "prospect", "customer", "advocate"]
                        },
                        "min_health_score": {
                            "type": "integer",
                            "description": "Minimum health score (0-100)"
                        },
                        "churn_risk": {
                            "type": "string",
                            "description": "Filter by churn risk",
                            "enum": ["low", "medium", "high"]
                        },
                        "tags": {
                            "type": "array",
                            "description": "Filter by tags",
                            "items": {"type": "string"}
                        },
                        "page": {"type": "integer", "minimum": 1},
                        "per_page": {"type": "integer", "minimum": 1, "maximum": 100}
                    }
                }),
            },
            MCPTool {
                name: "get_customer_details".to_string(),
                description: "Get detailed customer information including interactions and notes".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "customer_id": {
                            "type": "integer",
                            "description": "Customer ID"
                        }
                    },
                    "required": ["customer_id"]
                }),
            },
            MCPTool {
                name: "update_customer".to_string(),
                description: "Update customer information".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "customer_id": {"type": "integer"},
                        "lifecycle_stage": {
                            "type": "string",
                            "enum": ["lead", "prospect", "customer", "advocate"]
                        },
                        "health_score": {
                            "type": "integer",
                            "minimum": 0,
                            "maximum": 100
                        },
                        "tags": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["customer_id"]
                }),
            },
            MCPTool {
                name: "add_customer_note".to_string(),
                description: "Add notes to customer record".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "customer_id": {"type": "integer"},
                        "note": {"type": "string", "description": "Note content"}
                    },
                    "required": ["customer_id", "note"]
                }),
            },
            MCPTool {
                name: "log_customer_interaction".to_string(),
                description: "Log customer interactions (email, call, meeting)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "customer_id": {"type": "integer"},
                        "interaction_type": {
                            "type": "string",
                            "enum": ["email", "call", "meeting", "chat"]
                        },
                        "description": {"type": "string"},
                        "outcome": {"type": "string"}
                    },
                    "required": ["customer_id", "interaction_type", "description"]
                }),
            },
            
            // ===== Commerce Management Tools =====
            MCPTool {
                name: "list_products".to_string(),
                description: "List all products with inventory information".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "category": {"type": "string"},
                        "in_stock": {"type": "boolean"},
                        "min_price": {"type": "number"},
                        "max_price": {"type": "number"},
                        "page": {"type": "integer", "minimum": 1},
                        "per_page": {"type": "integer", "minimum": 1, "maximum": 100}
                    }
                }),
            },
            MCPTool {
                name: "update_inventory".to_string(),
                description: "Update product inventory levels".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "product_id": {"type": "integer"},
                        "quantity": {"type": "integer", "minimum": 0},
                        "operation": {
                            "type": "string",
                            "enum": ["set", "add", "subtract"],
                            "description": "How to update: set=replace, add=increase, subtract=decrease"
                        }
                    },
                    "required": ["product_id", "quantity", "operation"]
                }),
            },
            MCPTool {
                name: "list_orders".to_string(),
                description: "List customer orders".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "status": {
                            "type": "string",
                            "enum": ["pending", "processing", "shipped", "delivered", "cancelled"]
                        },
                        "customer_id": {"type": "integer"},
                        "date_from": {"type": "string", "format": "date"},
                        "date_to": {"type": "string", "format": "date"},
                        "page": {"type": "integer", "minimum": 1},
                        "per_page": {"type": "integer", "minimum": 1}
                    }
                }),
            },
            MCPTool {
                name: "update_order_status".to_string(),
                description: "Update order fulfillment status".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "order_id": {"type": "integer"},
                        "status": {
                            "type": "string",
                            "enum": ["pending", "processing", "shipped", "delivered", "cancelled"]
                        },
                        "tracking_number": {"type": "string"}
                    },
                    "required": ["order_id", "status"]
                }),
            },
            MCPTool {
                name: "get_sales_analytics".to_string(),
                description: "Get sales and revenue analytics".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "date_from": {"type": "string", "format": "date"},
                        "date_to": {"type": "string", "format": "date"},
                        "group_by": {
                            "type": "string",
                            "enum": ["day", "week", "month"],
                            "description": "How to group the data"
                        }
                    }
                }),
            },
            
            // ===== Newsletter & Email Tools =====
            MCPTool {
                name: "list_newsletter_subscribers".to_string(),
                description: "List all newsletter subscribers".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "lifecycle_stage": {"type": "string"},
                        "page": {"type": "integer", "minimum": 1},
                        "per_page": {"type": "integer", "minimum": 1, "maximum": 100}
                    }
                }),
            },
            MCPTool {
                name: "get_subscriber_stats".to_string(),
                description: "Get newsletter subscription statistics".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "date_from": {"type": "string", "format": "date"},
                        "date_to": {"type": "string", "format": "date"}
                    }
                }),
            },
            MCPTool {
                name: "create_newsletter_segment".to_string(),
                description: "Create targeted subscriber segment (Admin only)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "description": {"type": "string"},
                        "criteria": {
                            "type": "object",
                            "description": "Segment criteria (e.g., tags, lifecycle_stage)"
                        }
                    },
                    "required": ["name", "criteria"]
                }),
            },
            MCPTool {
                name: "list_segments".to_string(),
                description: "List all CRM segments".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "page": {"type": "integer", "minimum": 1},
                        "per_page": {"type": "integer", "minimum": 1}
                    }
                }),
            },
            MCPTool {
                name: "list_campaigns".to_string(),
                description: "List email campaigns".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "status": {
                            "type": "string",
                            "enum": ["draft", "scheduled", "sent"]
                        },
                        "page": {"type": "integer", "minimum": 1}
                    }
                }),
            },
            MCPTool {
                name: "create_campaign".to_string(),
                description: "Create new email campaign".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "segment_id": {"type": "integer"},
                        "subject": {"type": "string"},
                        "content": {"type": "string", "description": "HTML email content"},
                        "schedule_date": {"type": "string", "format": "date-time"}
                    },
                    "required": ["name", "segment_id", "subject", "content"]
                }),
            },
            MCPTool {
                name: "list_pending_verifications".to_string(),
                description: "Show pending email verifications".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "verification_type": {"type": "string"},
                        "page": {"type": "integer", "minimum": 1}
                    }
                }),
            },
            MCPTool {
                name: "cleanup_expired_verifications".to_string(),
                description: "Manually cleanup expired verifications (Admin only)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dry_run": {
                            "type": "boolean",
                            "description": "If true, shows what would be deleted without deleting"
                        }
                    }
                }),
            },
        ];

        // ===== Phase 2: Add Custom Tools =====
        let custom_tools = self.fetch_custom_tools();
        let custom_tool_count = custom_tools.len();
        let builtin_count = tools.len();
        
        for custom_tool in custom_tools {
            tools.push(self.custom_tool_to_mcp_tool(&custom_tool));
        }
        
        info!("📋 Listing {} tools ({} built-in + {} custom)", tools.len(), builtin_count, custom_tool_count);

        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({ "tools": tools })),
            error: None,
            id,
        }
    }

    fn handle_call_tool(&self, params: Option<serde_json::Value>, id: Option<serde_json::Value>) -> MCPResponse {
        // Check authentication first
        let tenant_id = match self.check_auth() {
            Ok(id) => id,
            Err(error) => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(error),
                    id,
                };
            }
        };
        
        let tool_name = params
            .as_ref()
            .and_then(|p| p.get("name").and_then(|n| n.as_str()))
            .unwrap_or("");

        let arguments = params
            .as_ref()
            .and_then(|p| p.get("arguments"))
            .cloned()
            .unwrap_or(json!({}));

        info!("🔧 Calling tool: {} for tenant_id={}", tool_name, tenant_id);

        // Special case: legacy verification tools with custom logic
        match tool_name {
            "update_verification_settings" => {
                if let Err(error) = self.check_role("admin") {
                    return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(error),
                        id,
                    };
                }
                
                let v_type = arguments.get("verification_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("crm_customer");
                let body = json!({
                    "ttl_hours": arguments.get("ttl_hours").and_then(|v| v.as_i64()).unwrap_or(12),
                    "enabled": arguments.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true)
                }).to_string();
                
                return self.build_api_guidance(
                    "PUT",
                    &format!("/v1/api/verification/settings/{}", v_type),
                    Some(body),
                    &format!("To update verification settings for '{}', make this API call:", v_type),
                    tenant_id,
                    id,
                );
            }
            "get_verification_settings" => {
                if let Err(error) = self.check_role("editor") {
                    return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(error),
                        id,
                    };
                }
                
                let v_type_filter = arguments.get("verification_type").and_then(|v| v.as_str());
                let query_params = v_type_filter.map(|v| format!("?verification_type={}", v)).unwrap_or_default();
                let description = if let Some(vtype) = v_type_filter {
                    format!("To retrieve verification settings for type '{}', make this API call:", vtype)
                } else {
                    "To retrieve verification settings, make this API call:".to_string()
                };

                return self.build_api_guidance(
                    "GET",
                    &format!("/v1/api/verification/settings{}", query_params),
                    None,
                    &description,
                    tenant_id,
                    id,
                );
            }
            _ => {}
        }

        // Data-driven routing for platform tools
        if let Some(spec) = TOOL_SPECS.iter().find(|s| s.name == tool_name) {
            // Check RBAC
            if let Err(error) = self.check_role(spec.required_role) {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(error),
                    id,
                };
            }
            
            // Build endpoint with path parameters and query parameters
            let endpoint = Self::build_endpoint(spec.endpoint, &arguments);
            
            // Build request body for POST/PUT requests
            let body = match spec.http_method {
                "POST" | "PUT" => Some(arguments.to_string()),
                _ => None,
            };
            
            return self.build_api_guidance(
                spec.http_method,
                &endpoint,
                body,
                spec.description,
                tenant_id,
                id,
            );
        }

        // ===== Phase 2: Try Custom Tools =====
        // If no built-in tool matched, try custom tools
        match self.execute_custom_tool(&tool_name, arguments.clone()) {
            Ok(result) => {
                info!("✅ Custom tool '{}' executed successfully", tool_name);
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string())
                        }]
                    })),
                    error: None,
                    id,
                };
            }
            Err(e) => {
                // Custom tool failed or doesn't exist - fall through to error
                info!("❌ Custom tool '{}' not found or failed: {}", tool_name, e.message);
            }
        }

        // Unknown tool
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(MCPError {
                code: -32601,
                message: format!("Unknown tool: {}", tool_name),
                data: None,
            }),
            id,
        }
    }
    
    // Helper: Build endpoint from template + arguments
    fn build_endpoint(template: &str, arguments: &serde_json::Value) -> String {
        let mut endpoint = template.to_string();
        
        // Replace path parameters like {id}, {product_id}
        if let Some(obj) = arguments.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{}}}", key);
                if endpoint.contains(&placeholder) {
                    let val_str = match value {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        _ => value.to_string(),
                    };
                    endpoint = endpoint.replace(&placeholder, &val_str);
                }
            }
            
            // Add query parameters for GET requests (skip path params already used)
            if !template.contains('?') {
                let query_params: Vec<String> = obj.iter()
                    .filter(|(k, _)| !template.contains(&format!("{{{}}}", k)))
                    .filter_map(|(k, v)| {
                        match v {
                            serde_json::Value::Null => None,
                            serde_json::Value::String(s) if s.is_empty() => None,
                            serde_json::Value::String(s) => Some(format!("{}={}", k, s)),
                            serde_json::Value::Number(n) => Some(format!("{}={}", k, n)),
                            serde_json::Value::Bool(b) => Some(format!("{}={}", k, b)),
                            serde_json::Value::Array(_) | serde_json::Value::Object(_) => None,
                        }
                    })
                    .collect();
                
                if !query_params.is_empty() {
                    endpoint.push('?');
                    endpoint.push_str(&query_params.join("&"));
                }
            }
        }
        
        endpoint
    }
    
    // Helper function to build API guidance responses
    fn build_api_guidance(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<String>,
        description: &str,
        tenant_id: i32,
        id: Option<serde_json::Value>,
    ) -> MCPResponse {
        let headers_section = if body.is_some() {
            "**Headers:**\n  Authorization: Bearer <your_jwt_token>\n  Content-Type: application/json\n"
        } else {
            "**Headers:**\n  Authorization: Bearer <your_jwt_token>\n"
        };
        
        let body_section = if let Some(ref body_str) = body {
            format!("\n**Body:**\n```json\n{}\n```\n", body_str)
        } else {
            String::new()
        };
        
        let curl_example = if let Some(body_str) = body {
            format!(
                "curl -X {} 'http://localhost:8000{}' \\\n     -H 'Authorization: Bearer YOUR_TOKEN' \\\n     -H 'Content-Type: application/json' \\\n     -d '{}'",
                method, endpoint, body_str.replace('\n', "")
            )
        } else {
            format!(
                "curl -X {} 'http://localhost:8000{}' \\\n     -H 'Authorization: Bearer YOUR_TOKEN'",
               method, endpoint
            )
        };
        
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "{}\\n\\n**Endpoint:** {} {}\\n{}{}\\n**Example cURL:**\\n{}\\n\\n**Context:**\\n  Tenant: {}\\n  User: {}\\n  Role: {}",
                        description,
                        method,
                        endpoint,
                        headers_section,
                        body_section,
                        curl_example,
                        tenant_id,
                        self.user_id.unwrap_or(0),
                        self.role.as_deref().unwrap_or("viewer")
                    )
                }]
            })),
            error: None,
            id,
        }
    }
    
    // ===== Phase 2: Custom Tool Integration =====
    
    /// Fetch custom tools from database
    fn fetch_custom_tools(&self) -> Vec<McpCustomTool> {
        if let Some(tenant_id) = self.tenant_id {
            let role = self.role.as_deref().unwrap_or("viewer");
            let pool_clone = self.pool.clone();
            let tenant_id_clone = tenant_id;
            let role_clone = role.to_string();
            
            // Blocking call to async function
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                McpCustomToolService::list_custom_tools(&pool_clone, tenant_id_clone, &role_clone)
                    .await
                    .unwrap_or_else(|e| {
                        error!("Failed to fetch custom tools: {}", e);
                        vec![]
                    })
            })
        } else {
            vec![]
        }
    }
    
    /// Convert custom tool to MCPTool format
    fn custom_tool_to_mcp_tool(&self, tool: &McpCustomTool) -> MCPTool {
        MCPTool {
            name: tool.name.clone(),
            description: tool.description.clone().unwrap_or_else(|| "Custom webhook tool".to_string()),
            input_schema: tool.input_schema.clone(),
        }
    }
    
    /// Execute custom tool
    fn execute_custom_tool(&self, tool_name: &str, arguments: serde_json::Value) -> Result<serde_json::Value, MCPError> {
        let tenant_id = self.check_auth()?;
        let pool_clone = self.pool.clone();
        let tool_name_clone = tool_name.to_string();
        let user_id = self.user_id;
        
        // Blocking call to async function
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            McpCustomToolService::execute_by_name(
                &pool_clone,
                &tool_name_clone,
                arguments,
                tenant_id,
                user_id,
            )
            .await
            .map_err(|e| MCPError {
                code: -32000,
                message: format!("Custom tool execution failed: {}", e),
                data: None,
            })
        })
    }
}

impl Actor for MCPWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("✅ MCP WebSocket connection established");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("🔌 MCP WebSocket connection closed");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MCPWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<MCPRequest>(&text) {
                    Ok(request) => {
                        let response = self.handle_request(request);
                        let response_text = serde_json::to_string(&response).unwrap();
                        ctx.text(response_text);
                    }
                    Err(e) => {
                        error!("Failed to parse MCP request: {}", e);
                        let error_response = MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            result: None,
                            error: Some(MCPError {
                                code: -32700,
                                message: format!("Parse error: {}", e),
                                data: None,
                            }),
                            id: None,
                        };
                        ctx.text(serde_json::to_string(&error_response).unwrap());
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Client closed connection: {:?}", reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

// ===== HTTP Routes =====

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<Arc<DbPool>>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("📡 New MCP WebSocket connection attempt");
    
    // Extract auth info from JWT token in Authorization header
    let (tenant_id, user_id, role) = extract_auth_from_request(&req);
    
    if tenant_id.is_none() {
        info!("⚠️  MCP connection attempt without authentication");
    } else {
        info!("✅ MCP authenticated - tenant: {:?}, user: {:?}, role: {:?}", tenant_id, user_id, role);
    }
    
    let ws_actor = MCPWebSocket::new(pool.get_ref().clone(), tenant_id, user_id, role);
    ws::start(ws_actor, &req, stream)
}

fn extract_auth_from_request(req: &HttpRequest) -> (Option<i32>, Option<i32>, Option<String>) {
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
    use serde::{Deserialize};
    
    #[derive(Debug, Deserialize)]
    struct Claims {
        sub: String,
        tenant_id: Option<i32>,
        user_id: Option<i32>,
        role: Option<String>,  // "admin", "editor", "viewer"
    }
    
    // Get Authorization header
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return (None, None, None);
    }
    
    let auth_str = match auth_header.unwrap().to_str() {
        Ok(s) => s,
        Err(_) => return (None, None, None),
    };
    
    // Extract token from "Bearer <token>"
    let token = match auth_str.strip_prefix("Bearer ") {
        Some(t) => t.trim(),
        None => return (None, None, None),
    };
    
    // Get JWT secret from environment
    let jwt_secret = std::env::var("APP_JWT_KEY").unwrap_or_else(|_| "secret".to_string());
    
    // Decode and validate JWT
    let validation = Validation::new(Algorithm::HS256);
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation
    ) {
        Ok(data) => data,
        Err(_) => return (None, None, None),
    };
    
    (
        token_data.claims.tenant_id,
        token_data.claims.user_id,
        token_data.claims.role
    )
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "FreeRadical MCP Server",
        "protocol": "Model Context Protocol",
        "version": "2024-11-05"
    }))
}

// ===== Server Startup =====

pub async fn start_mcp_server(pool: DbPool) -> std::io::Result<()> {
    let pool = Arc::new(pool);
    let bind_addr = "0.0.0.0:9009";

    info!("🚀 Starting FreeRadical MCP Server on {}", bind_addr);
    info!("📡 WebSocket endpoint: ws://{}/mcp", bind_addr);
    info!("🏥 Health check: http://{}/health", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/mcp", web::get().to(ws_index))
            .route("/health", web::get().to(health))
    })
    .bind(bind_addr)?
    .run()
    .await
}
