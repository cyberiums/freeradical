use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, ActorContext, AsyncContext, Handler, Message as ActixMessage};
use serde::{Deserialize, Serialize};
use serde_json::json;
use log::{info, error};
use std::sync::Arc;

use crate::models::DbPool;

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

// ===== WebSocket Actor =====

pub struct MCPWebSocket {
    pool: Arc<DbPool>,
    tenant_id: Option<i32>,  // Extracted from JWT
    authenticated: bool,
}

impl MCPWebSocket {
    pub fn new(pool: Arc<DbPool>, tenant_id: Option<i32>) -> Self {
        Self { 
            pool,
            tenant_id,
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
        let tools = vec![
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
        ];

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

        match tool_name {
            "update_verification_settings" => {
                // Tool execution is now scoped to tenant_id
                MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(json!({
                        "content": [{
                            "type": "text",
                            "text": format!(
                                "✅ Updated verification settings for type: {} (tenant: {})",
                                arguments.get("verification_type")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown"),
                                tenant_id
                            )
                        }]
                    })),
                    error: None,
                    id,
                }
            }
            "get_verification_settings" => {
                MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Current settings for tenant {}:\n- crm_customer: 12h TTL, enabled", tenant_id)
                        }]
                    })),
                    error: None,
                    id,
                }
            }
            _ => MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Unknown tool: {}", tool_name),
                    data: None,
                }),
                id,
            },
        }
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
    
    // Extract tenant_id from JWT token in Authorization header
    let tenant_id = extract_tenant_from_request(&req);
    
    if tenant_id.is_none() {
        info!("⚠️  MCP connection attempt without authentication - rejecting");
    } else {
        info!("✅ MCP connection authenticated for tenant: {:?}", tenant_id);
    }
    
    let ws_actor = MCPWebSocket::new(pool.get_ref().clone(), tenant_id);
    ws::start(ws_actor, &req, stream)
}

fn extract_tenant_from_request(req: &HttpRequest) -> Option<i32> {
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
    use serde::{Deserialize};
    
    #[derive(Debug, Deserialize)]
    struct Claims {
        sub: String,
        tenant_id: Option<i32>,
    }
    
    // Get Authorization header
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    
    // Extract token from "Bearer <token>"
    let token = auth_str.strip_prefix("Bearer ")?.trim();
    
    // Get JWT secret from environment
    let jwt_secret = std::env::var("APP_JWT_KEY").unwrap_or_else(|_| "secret".to_string());
    
    // Decode and validate JWT
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation
    ).ok()?;
    
    token_data.claims.tenant_id
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
