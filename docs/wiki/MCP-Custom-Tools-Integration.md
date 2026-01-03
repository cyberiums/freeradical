# MCP Custom Tools Integration Guide

**Phase 2: Custom Tool Registration System**

This guide documents how custom MCP tools are integrated into the FreeRadical MCP server, enabling dynamic tool registration with HTTP webhooks and 200% isolation.

## Overview

The integration allows:
- **Dynamic Tool Discovery**: Custom tools appear in MCP `list_tools` alongside built-in tools
- **Seamless Execution**: Custom tools execute via MCP `call_tool` protocol
- **200% Isolation**: Webhooks execute externally on user's infrastructure
- **Tenant Isolation**: Each tenant has their own custom tools
- **Role-Based Access**: Tools filtered by user role (viewer/editor/admin)

## Architecture

### Components

1. **Database Layer**
   - `mcp_custom_tools` - Tool definitions
   - `mcp_tool_executions` - Audit logs
   - `mcp_tool_rate_limits` - Rate limiting

2. **Service Layer**
   - `McpCustomToolService` - CRUD operations and webhook execution
   - Async execution with timeout protection
   - Rate limiting enforcement
   - Complete audit trail

3. **MCP Server Integration**
   - `MCPWebSocket` - WebSocket actor for MCP protocol
   - Helper methods for fetching and executing custom tools
   - Tokio runtime for async-to-sync bridging

## Implementation Details

### 1. Imports (mcp_server.rs)

```rust
use crate::services::mcp_custom_tool_service::McpCustomToolService;
use crate::models::mcp_tool_models::McpCustomTool;
```

### 2. Helper Methods (MCPWebSocket impl)

#### Fetch Custom Tools
```rust
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
```

#### Convert to MCP Format
```rust
fn custom_tool_to_mcp_tool(&self, tool: &McpCustomTool) -> MCPTool {
    MCPTool {
        name: tool.name.clone(),
        description: tool.description.clone().unwrap_or_else(|| "Custom webhook tool".to_string()),
        input_schema: tool.input_schema.clone(),
    }
}
```

#### Execute Custom Tool
```rust
fn execute_custom_tool(&self, tool_name: &str, arguments: serde_json::Value) 
    -> Result<serde_json::Value, MCPError> 
{
    let tenant_id = self.check_auth()?;
    let pool_clone = self.pool.clone();
    let tool_name_clone = tool_name.to_string();
    let user_id = self.user_id;
    
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
```

### 3. Tool Discovery (handle_list_tools)

After building the built-in tools list:

```rust
// ===== Phase 2: Add Custom Tools =====
let custom_tools = self.fetch_custom_tools();
let custom_tool_count = custom_tools.len();
let builtin_count = tools.len();

for custom_tool in custom_tools {
    tools.push(self.custom_tool_to_mcp_tool(&custom_tool));
}

info!("📋 Listing {} tools ({} built-in + {} custom)", 
      tools.len(), builtin_count, custom_tool_count);
```

### 4. Tool Execution (handle_call_tool)

Before the "unknown tool" error:

```rust
// ===== Phase 2: Try Custom Tools =====
match self.execute_custom_tool(&tool_name, arguments.clone()) {
    Ok(result) => {
        info!("✅ Custom tool '{}' executed successfully", tool_name);
        return MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&result)
                        .unwrap_or_else(|_| result.to_string())
                }]
            })),
            error: None,
            id,
        };
    }
    Err(e) => {
        info!("❌ Custom tool '{}' not found or failed: {}", tool_name, e.message);
    }
}
```

## Execution Flow

### Discovery Flow
1. MCP client sends `list_tools` request
2. `handle_list_tools()` builds built-in tools list
3. Fetches custom tools from database (tenant-filtered)
4. Converts custom tools to MCP format
5. Merges and returns combined list
6. Logs tool count

### Execution Flow  
1. MCP client sends `call_tool` request with tool name + arguments
2. `handle_call_tool()` checks built-in tools first
3. If not found, tries custom tools via `execute_custom_tool()`
4. Checks authentication and rate limits
5. Executes HTTP webhook with timeout
6. Logs execution with input/output/timing
7. Returns result or error to client

## Security Model

### 200% Isolation
- **External Execution**: Webhooks run on user's infrastructure, not CMS server
- **Network Isolation**: HTTP requests to external URLs only
- **Timeout Protection**: Configurable per tool (default 30s)
- **Rate Limiting**: Per-tool, per-tenant, hourly windows
- **No Code Execution**: Zero code runs on CMS server

### Authentication & Authorization
- JWT token required for MCP connection
- Tenant ID extracted from token
- Tools filtered by user role
- Execution logged with user ID

### Audit Trail
- Every execution logged to `mcp_tool_executions`
- Includes: input, output, errors, timing, HTTP status
- Queryable via REST API

## REST API Integration

### Endpoints
- `GET /api/mcp/custom-tools` - List tools
- `POST /api/mcp/custom-tools` - Create tool
- `POST /api/mcp/custom-tools/{id}/test` - Test execution
- `GET /api/mcp/custom-tools/{id}/executions` - View logs
- `POST /api/mcp/custom-tools/{id}/publish` - Share to marketplace
- `GET /api/mcp/marketplace` - Browse public tools

### Swagger Documentation
All endpoints fully documented:
- Swagger UI: `http://localhost:8000/swagger-ui/`
- ReDoc: `http://localhost:8000/redoc`
- Tagged as "Internal - MCP Custom Tools"

## Usage Example

### 1. Create Tool (REST API)
```bash
curl -X POST http://localhost:8000/api/mcp/custom-tools \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "weather_lookup",
    "description": "Get weather for a city",
    "input_schema": {
      "type": "object",
      "properties": {
        "city": {"type": "string"}
      },
      "required": ["city"]
    },
    "webhook_url": "https://api.weatherapi.com/v1/current.json",
    "webhook_method": "GET",
    "timeout_seconds": 15,
    "max_calls_per_hour": 100
  }'
```

### 2. Discover Tool (MCP Protocol)
```json
{
  "jsonrpc": "2.0",
  "method": "list_tools",
  "id": 1
}
```

Response includes `weather_lookup` in tools array.

### 3. Execute Tool (MCP Protocol)
```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "weather_lookup",
    "arguments": {
      "city": "San Francisco"
    }
  },
  "id": 2
}
```

Webhook executes, result returned via MCP.

## Performance Considerations

### Async-to-Sync Bridge
- Uses Tokio runtime for blocking calls
- Minimal overhead (<1ms)
- Actor context remains synchronous

### Caching
- No caching currently implemented
- Tools fetched on each `list_tools` call
- Consider caching for high-traffic scenarios

### Rate Limiting
- Hourly windows (rounded to hour start)
- Per-tool, per-tenant tracking
- Configurable limits per tool

## Troubleshooting

### Tools Not Appearing
- Check tenant ID in JWT token
- Verify tool `is_enabled = true`
- Check user role matches `required_role`
- Review MCP server logs for errors

### Execution Failures
- Check webhook URL accessibility
- Verify timeout is sufficient
- Review execution logs in database
- Check rate limit not exceeded

### Performance Issues
- Monitor Tokio runtime overhead
- Check database query performance
- Consider tool list caching
- Review webhook response times

## Future Enhancements

1. **Tool Versioning**: Support multiple versions
2. **Async Webhooks**: Long-running tasks with callbacks
3. **Tool Composition**: Chain multiple tools
4. **Input Validation**: Validate against JSON schema
5. **Response Transformation**: Transform webhook responses
6. **Advanced Auth**: OAuth, API keys in webhooks

## Related Documentation

- [Phase 2 Implementation Plan](../brain/phase2_implementation_plan.md)
- [Phase 2 Walkthrough](../brain/phase2_walkthrough.md)
- [Phase 2 Final Delivery](../brain/phase2_final_delivery.md)
- [API Documentation](http://localhost:8000/swagger-ui/)

## Support

For issues or questions:
- GitHub Issues: [freeradical/issues](https://github.com/YOUR_ORG/freeradical/issues)
- Documentation: [freeradical.dev/docs](https://freeradical.dev/docs)
- MCP Protocol: [modelcontextprotocol.io](https://modelcontextprotocol.io)
