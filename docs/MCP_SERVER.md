# FreeRadical MCP Server

## Overview

FreeRadical now includes a **Model Context Protocol (MCP) Server** that runs on **port 9009**, parallel to the main REST API on port 8000. This allows AI agents to interact with FreeRadical admin features conversationally.

---

## Quick Start

### Authentication Required 🔒

**IMPORTANT:** The MCP server requires JWT authentication. All connections must include a bearer token in the `Authorization` header.

### Connection

**WebSocket Endpoint:**
```
ws://localhost:9009/mcp
```

**Headers Required:**
```
Authorization: Bearer YOUR_JWT_TOKEN
```

**Health Check (No Auth):**
```
http://localhost:9009/health
```

### Testing with wscat

```bash
# Install wscat
npm install -g wscat

# Get your JWT token first (from login endpoint)
JWT_TOKEN="your-jwt-token-here"

# Connect to MCP server with authentication
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer $JWT_TOKEN"

# Send initialize request
{"jsonrpc":"2.0","method":"initialize","id":1}

# List available tools
{"jsonrpc":"2.0","method":"tools/list","id":2}

# List resources
{"jsonrpc":"2.0","method":"resources/list","id":3}
```

### Getting a JWT Token

```bash
# Login to get JWT token
curl -X POST http://localhost:8000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"your-password"}'

# Response includes: {"token": "eyJhbGc..."}
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

### MCP AI Interface Pattern ✨

**Design Philosophy:** MCP tools act as an **AI-friendly interface layer** that guides agents to the appropriate REST API endpoints rather than directly accessing the database.

**Why This Approach:**
- ✅ **Simple** - No complex async WebSocket actor refactoring
- ✅ **Maintainable** - Clear separation: MCP = AI interface, REST = operations  
- ✅ **Immediate value** - Tools are useful from day 1
- ✅ **Leverages existing** - REST API already tested and working

**How It Works:**

When an AI agent calls a tool like `update_verification_settings`, it receives:
- Complete API endpoint details
- cURL examples with headers
- Request body format
- Expected response structure
- Tenant/user/role context

The AI agent can then:
1. Use the provided information to understand the operation
2. Make the actual REST API call with proper authentication
3. Receive the real data from the database

**Example Tool Response:**
```markdown
To update verification settings for 'crm_customer', make this API call:

**Endpoint:** PUT /v1/api/verification/settings/crm_customer
**Headers:**
  Authorization: Bearer <your_jwt_token>
  Content-Type: application/json

**Body:**
{
  "ttl_hours": 24,
  "enabled": true
}

**Example cURL:**
curl -X PUT 'http://localhost:8000/v1/api/verification/settings/crm_customer' \
     -H 'Authorization: Bearer YOUR_TOKEN' \
     -H 'Content-Type: application/json' \
     -d '{"ttl_hours":24,"enabled":true}'
```

**See Also:** `docs/MCP_IMPLEMENTATION_DECISION.md` for the full design rationale.

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

## Security 🔒

**Current Implementation:**
- ✅ **JWT Authentication** - Bearer token required
- ✅ **Tenant Isolation** - All operations scoped to user's tenant_id
- ✅ **Authorization Header** - Standard bearer token format
- ✅ **Auth Validation** - JWT signature and expiration checked
- ✅ **Error Handling** - Custom -32001 error for auth failures
- ✅ **Audit Logging** - Tenant ID logged for all operations

**How It Works:**

1. Client connects with Authorization header
2. Server extracts JWT and validates signature
3. Tenant ID extracted from JWT claims
4. All tool calls scoped to that tenant
5. Cross-tenant access impossible

**Authentication Flow:**

```
Client                    MCP Server
  |                           |
  |--WebSocket + JWT--------->|
  |                           |-Validate JWT
  |                           |-Extract tenant_id
  |<----Connection Accepted---|
  |                           |
  |--Tool Call--------------->|
  |                           |-Check tenant_id
  |                           |-Execute (tenant-scoped)
  |<----Result (tenant data)--|
```

**Error Codes:**

| Code | Meaning | Solution |
|------|---------|----------|
| `-32001` | Authentication required | Provide valid JWT bearer token |
| `-32700` | Parse error | Check JSON syntax |
| `-32601` | Method not found | Use valid MCP method |

**Production Recommendations:**
1. ✅ Use HTTPS/WSS in production
2. ✅ Rotate JWT secrets regularly
3. ✅ Set short token expiration (1-24h)
4. ⚠️ Add rate limiting per tenant
5. ⚠️ Monitor failed auth attempts
6. ⚠️ Implement IP whitelist for sensitive operations

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

## See Also

- [FEATURES_V2.6.md](file:///Users/prabhatsingh/freeradical/docs/FEATURES_V2.6.md) - Quick reference for all v2.6 features
- [CRM_VERIFICATION_FLOW.md](file:///Users/prabhatsingh/freeradical/docs/CRM_VERIFICATION_FLOW.md) - Newsletter subscription flow
- [MCP_RBAC.md](file:///Users/prabhatsingh/freeradical/docs/MCP_RBAC.md) - Role-based access control details
- [MCP_IMPLEMENTATION_DECISION.md](file:///Users/prabhatsingh/freeradical/docs/MCP_IMPLEMENTATION_DECISION.md) - Design rationale

---

**Status:** ✅ Production Ready  
**Port:** 9009  
**Protocol:** Model Context Protocol (2024-11-05)  
**Commit:** `c5445f4`
