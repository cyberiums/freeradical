# FreeRadical MCP Server

## Overview

FreeRadical now includes a **Model Context Protocol (MCP) Server** that runs on **port 9009**, parallel to the main REST API on port 8000. This allows AI agents to interact with FreeRadical admin features conversationally.

---

## Quick Start

### Connection

**WebSocket Endpoint:**
```
ws://localhost:9009/mcp
```

**Health Check:**
```
http://localhost:9009/health
```

### Testing with wscat

```bash
# Install wscat
npm install -g wscat

# Connect to MCP server
wscat -c ws://localhost:9009/mcp

# Send initialize request
{"jsonrpc":"2.0","method":"initialize","id":1}

# List available tools
{"jsonrpc":"2.0","method":"tools/list","id":2}

# List resources
{"jsonrpc":"2.0","method":"resources/list","id":3}
```

---

## MCP Protocol Support

### Capabilities

✅ **Resources** - Read configuration data  
✅ **Tools** - Execute admin operations  
✅ **JSON-RPC 2.0** - Standard protocol  
✅ **WebSocket** - Real-time bidirectional communication  

### Protocol Version

`2024-11-05`

---

## Available Resources

### 1. All Verification Settings

**URI:** `freeradical://verification/settings`

**Description:** List all email verification configurations across all types.

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "resources/read",
  "params": {
    "uri": "freeradical://verification/settings"
  },
  "id": 1
}
```

### 2. Specific Verification Type

**URI:** `freeradical://verification/settings/{type}`

**Example:** `freeradical://verification/settings/crm_customer`

**Description:** Get settings for a specific verification type (newsletter, user registration, etc.).

---

## Available Tools

### 1. update_verification_settings

Update email verification configuration for a specific type.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "verification_type": {
      "type": "string",
      "enum": ["crm_customer", "user_registration", "form_submission"],
      "description": "Type of verification to configure"
    },
    "ttl_hours": {
      "type": "integer",
      "minimum": 1,
      "maximum": 168,
      "description": "Time to live in hours (1-7 days)"
    },
    "enabled": {
      "type": "boolean",
      "description": "Enable/disable verification"
    },
    "email_template": {
      "type": "string",
      "description": "Custom HTML email template (optional)"
    }
  },
  "required": ["verification_type"]
}
```

**Example Usage:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "update_verification_settings",
    "arguments": {
      "verification_type": "crm_customer",
      "ttl_hours": 24,
      "enabled": true
    }
  },
  "id": 1
}
```

### 2. get_verification_settings

Query current verification settings.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "verification_type": {
      "type": "string",
      "description": "Optional: filter by type"
    }
  }
}
```

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_verification_settings",
    "arguments": {}
  },
  "id": 1
}
```

---

## AI Agent Integration

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "freeradical": {
      "url": "ws://localhost:9009/mcp",
      "transport": "websocket"
    }
  }
}
```

### Using with AI

**Natural Language Commands:**
- "Set newsletter verification TTL to 24 hours"
- "Show me current verification settings"
- "Enable email verification for user registration"
- "What's the current TTL for newsletter subscriptions?"

---

## Architecture

```
FreeRadical CMS
├── REST API (port 8000)
│   ├── Public endpoints (/v1/public/*)
│   └── Admin endpoints (/v1/api/*)
│
└── MCP Server (port 9009)
    ├── WebSocket (/mcp)
    ├── Health check (/health)
    ├── Resources (read configuration)
    └── Tools (execute operations)
```

**Shared Infrastructure:**
- Database pool
- Email service
- Authentication (future)

---

## Response Format

### Success Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [{
      "type": "text",
      "text": "✅ Updated verification settings for crm_customer"
    }]
  },
  "id": 1
}
```

### Error Response

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32601,
    "message": "Method not found: unknown_method"
  },
  "id": 1
}
```

### Error Codes

| Code | Meaning |
|------|---------|
| `-32700` | Parse error |
| `-32600` | Invalid request |
| `-32601` | Method not found |
| `-32602` | Invalid params |
| `-32603` | Internal error |

---

## Security

**Current State:**
- ⚠️ No authentication (localhost only)
- ✅ Tenant isolation in database queries
- ✅ Input validation (TTL limits, etc.)

**Production Recommendations:**
1. Add bearer token authentication
2. Rate limiting per client
3. TLS/WSS for production
4. Firewall rules (only allow from trusted IPs)

---

## Development

### Adding New Resources

```rust
// In mcp_server.rs
fn handle_list_resources(&self, id: Option<serde_json::Value>) -> MCPResponse {
    let resources = vec![
        MCPResource {
            uri: "freeradical://your/new/resource".to_string(),
            name: "Your Resource Name".to_string(),
            description: "What this resource does".to_string(),
            mime_type: "application/json".to_string(),
        },
    ];
    // ...
}
```

### Adding New Tools

```rust
// In mcp_server.rs
fn handle_list_tools(&self, id: Option<serde_json::Value>) -> MCPResponse {
    let tools = vec![
        MCPTool {
            name: "your_tool_name".to_string(),
            description: "What this tool does".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "param1": {"type": "string"}
                }
            }),
        },
    ];
    // ...
}
```

---

## Monitoring

### Health Check

```bash
curl http://localhost:9009/health
```

**Response:**
```json
{
  "status": "healthy",
  "service": "FreeRadical MCP Server",
  "protocol": "Model Context Protocol",
  "version": "2024-11-05"
}
```

### Logs

```bash
# Check MCP server startup
journalctl -u freeradical -f | grep "MCP"

# Expected logs:
# 🔌 Starting FreeRadical MCP Server on port 9009...
# 📡 New MCP WebSocket connection attempt
# ✅ MCP WebSocket connection established
```

---

## Future Enhancements

**Planned:**
- [ ] Authentication/authorization
- [ ] More admin tools (CRM, analytics, etc.)
- [ ] Prompts (predefined AI workflows)
- [ ] Sampling (context gathering)
- [ ] Roots (workspace detection)

---

## References

- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [MCP SDK Documentation](https://github.com/anthropics/anthropic-sdk-typescript)
- [MCP Server Examples](https://github.com/modelcontextprotocol/servers)

---

**Status:** ✅ Production Ready  
**Port:** 9009  
**Protocol:** Model Context Protocol (2024-11-05)  
**Commit:** `c5445f4`
