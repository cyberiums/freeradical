use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use log::{info, error, warn};

/// MCP (Model Context Protocol) Client
/// Implements the Model Context Protocol for AI agent communication
/// Supports WebSocket connections, tool discovery, and message routing
#[derive(Debug, Clone)]
pub struct MCPClient {
    server_url: String,
    tools: HashMap<String, MCPTool>,
    connected: bool,
}

/// MCP Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// MCP Request format
#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub method: String,
    pub params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// MCP Response format
#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// MCP Error type
#[derive(Debug, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl MCPClient {
    /// Create a new MCP client
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            tools: HashMap::new(),
            connected: false,
        }
    }

    /// Connect to MCP server via WebSocket
    pub async fn connect(&mut self) -> Result<(), String> {
        info!("Connecting to MCP server at: {}", self.server_url);
        
        match connect_async(&self.server_url).await {
            Ok((ws_stream, _)) => {
                info!("✅ Successfully connected to MCP server");
                self.connected = true;
                
                // Perform handshake and tool discovery
                self.discover_tools_from_stream(ws_stream).await?;
                Ok(())
            },
            Err(e) => {
                error!("❌ Failed to connect to MCP server: {}", e);
                Err(format!("Connection failed: {}", e))
            }
        }
    }

    /// Discover available tools from the MCP server
    async fn discover_tools_from_stream(
        &mut self,
        mut ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>
    ) -> Result<(), String> {
        // Send tools/list request
        let request = MCPRequest {
            method: "tools/list".to_string(),
            params: serde_json::json!({}),
            id: Some("discovery-1".to_string()),
        };

        let message = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        ws_stream.send(Message::Text(message)).await
            .map_err(|e| format!("Failed to send discovery request: {}", e))?;

        // Wait for response
        if let Some(Ok(Message::Text(text))) = ws_stream.next().await {
            let response: MCPResponse = serde_json::from_str(&text)
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            if let Some(result) = response.result {
                // Parse tools from response
                if let Some(tools_array) = result.get("tools").and_then(|t| t.as_array()) {
                    for tool_value in tools_array {
                        if let Ok(tool) = serde_json::from_value::<MCPTool>(tool_value.clone()) {
                            info!("Discovered tool: {}", tool.name);
                            self.tools.insert(tool.name.clone(), tool);
                        }
                    }
                    info!("✅ Discovered {} tools", self.tools.len());
                } else {
                    warn!("⚠️  No tools found in response");
                }
            } else if let Some(error) = response.error {
                error!("❌ Tool discovery error: {}", error.message);
                return Err(format!("Tool discovery failed: {}", error.message));
            }
        }

        Ok(())
    }

    /// Execute a tool via MCP
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        _parameters: serde_json::Value
    ) -> Result<serde_json::Value, String> {
        if !self.connected {
            return Err("Not connected to MCP server".to_string());
        }

        if !self.tools.contains_key(tool_name) {
            return Err(format!("Unknown tool: {}", tool_name));
        }

        info!("Executing tool: {}", tool_name);

        // For now, return a placeholder
        // In production, this would send a request and wait for response
        Ok(serde_json::json!({
            "status": "success",
            "tool": tool_name,
            "result": "Tool execution would happen here"
        }))
    }

    /// Get list of available tools
    pub fn get_tools(&self) -> Vec<&MCPTool> {
        self.tools.values().collect()
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Disconnect from MCP server
    pub async fn disconnect(&mut self) {
        if self.connected {
            info!("Disconnecting from MCP server");
            self.connected = false;
            self.tools.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client_creation() {
        let client = MCPClient::new("ws://localhost:3000/mcp".to_string());
        assert!(!client.is_connected());
        assert_eq!(client.get_tools().len(), 0);
    }

    #[test]
    fn test_mcp_request_serialization() {
        let request = MCPRequest {
            method: "test".to_string(),
            params: serde_json::json!({"foo": "bar"}),
            id: Some("test-1".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("foo"));
    }
}
