# Role-Based Access Control (RBAC) for MCP Server

## Overview

The FreeRadical MCP Server implements **Role-Based Access Control (RBAC)** to ensure that users can only execute operations appropriate to their permission level.

---

## User Roles

### 1. **Viewer** (Read-Only)
- Can view resources
- Cannot modify settings
- Cannot execute admin/editor tools
- Default role if not specified

### 2. **Editor** (Read-Write)
- All viewer permissions
- Can view verification settings
- Can execute read operations
- Cannot modify critical settings

### 3. **Admin** (Full Access)
- All editor permissions
- Can update verification settings
- Can modify system configuration
- Can execute all tools

---

## JWT Claims Required

Your JWT token must include the following claims:

```json
{
  "sub": "user@example.com",
  "tenant_id": 1,
  "user_id": 42,
  "role": "admin"  
}
```

**Claims:**
- `tenant_id` (required) - Tenant isolation
- `user_id` (required) - User identification
- `role` (optional) - User role ("admin", "editor", "viewer")
  - Default: "viewer" if not specified

---

## Tool Permissions

| Tool | Required Role | Description |
|------|--------------|-------------|
| `get_verification_settings` | **editor** | View settings (read-only) |
| `update_verification_settings` | **admin** | Modify settings (write) |

---

## Permission Checks

### Authentication Flow

```
1. Client connects with JWT bearer token
2. Server extracts: tenant_id, user_id, role
3. Connection established

4. Client calls tool
5. Server checks authentication (tenant_id)
6. Server checks authorization (role)
7. Tool executes if permissions granted
```

### Error Responses

**Missing Authentication (-32001):**
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32001,
    "message": "Authentication required. Please provide bearer token."
  },
  "id": 1
}
```

**Insufficient Permissions (-32002):**
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32002,
    "message": "Insufficient permissions. Required: admin, your role: editor",
    "data": {
      "required": "admin",
      "actual": "editor"
    }
  },
  "id": 1
}
```

---

## Usage Examples

### Example 1: Admin User

```bash
# Admin JWT includes: {"role": "admin"}
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer ADMIN_TOKEN"

# Can update settings ✅
{
  "jsonrpc":"2.0",
  "method":"tools/call",
  "params":{
    "name":"update_verification_settings",
    "arguments":{"verification_type":"crm_customer","ttl_hours":24}
  },
  "id":1
}
```

### Example 2: Editor User

```bash
# Editor JWT includes: {"role": "editor"}
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer EDITOR_TOKEN"

# Can view settings ✅
{
  "jsonrpc":"2.0",
  "method":"tools/call",
  "params":{"name":"get_verification_settings","arguments":{}},
  "id":1
}

# Cannot update settings ❌ (returns -32002 error)
{
  "jsonrpc":"2.0",
  "method":"tools/call",
  "params":{
    "name":"update_verification_settings",
    "arguments":{"verification_type":"crm_customer","ttl_hours":24}
  },
  "id":2
}
```

### Example 3: Viewer User

```bash
# Viewer JWT includes: {"role": "viewer"} or no role
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer VIEWER_TOKEN"

# Cannot view settings ❌ (returns -32002 error)
# Cannot update settings ❌ (returns -32002 error)
```

---

## Best Practices

### 1. Principle of Least Privilege
- Grant minimum required permissions
- Most users should be "viewer" or "editor"
- Reserve "admin" for trusted administrators

### 2. JWT Token Management
- Include `role` claim in all JWTs
- Set appropriate token expiration
- Rotate tokens regularly

### 3. Audit Logging
All operations log:
- Tenant ID
- User ID
- Role
- Tool name
- Timestamp

Example log:
```
🔧 Calling tool: update_verification_settings for tenant_id=1, user_id=42, role=admin
```

### 4. Permission Escalation Prevention
- Roles are read from JWT (server-side validation)
- Users cannot self-promote roles
- Role changes require new JWT issuance

---

## Extending RBAC

### Adding New Roles

```rust
// In mcp_server.rs
fn check_role(&self, required_role: &str) -> Result<(), MCPError> {
    let user_role = self.role.as_deref().unwrap_or("viewer");
    
    match required_role {
        "superadmin" => {
            if user_role != "superadmin" {
                return Err(MCPError {
                    code: -32002,
                    message: "Required: superadmin".to_string(),
                    data: None,
                });
            }
        }
        // ... existing roles
    }
    Ok(())
}
```

### Adding New Permissions

```rust
// Example: Create a custom permission check
fn check_permission(&self, permission: &str) -> Result<(), MCPError> {
    // Fine-grained permission logic
    match permission {
        "settings:write" => self.check_role("admin"),
        "settings:read" => self.check_role("editor"),
        _ => Err(MCPError {
            code: -32002,
            message: format!("Unknown permission: {}", permission),
            data: None,
        })
    }
}
```

---

## Security Considerations

### ✅ Implemented
- JWT signature validation
- Role extraction from trusted claims
- Per-tool permission checks
- Tenant isolation
- Audit logging

### ⚠️ Recommendations
1. **Token Expiration:** Set short JWT expiration (1-12 hours)
2. **Token Refresh:** Implement refresh token rotation
3. **IP Whitelisting:** Restrict admin access by IP
4. **Rate Limiting:** Limit requests per user/role
5. **MFA for Admins:** Require 2FA for admin role

---

## Error Code Reference

| Code | Meaning | Solution |
|------|---------|----------|
| `-32001` | No authentication | Provide valid JWT bearer token |
| `-32002` | Insufficient permissions | Contact admin to upgrade role |
| `-32601` | Method not found | Check tool name spelling |
| `-32700` | Parse error | Check JSON syntax |

---

## Testing RBAC

```bash
# 1. Create test JWTs with different roles
# (Use your auth system to generate these)

# 2. Test viewer permissions
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer VIEWER_JWT"

# Expected: Connection OK, tools return -32002

# 3. Test editor permissions  
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer EDITOR_JWT"

# Expected: Can get_verification_settings, cannot update

# 4. Test admin permissions
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer ADMIN_JWT"

# Expected: All operations succeed
```

---

**Status:** ✅ Fully Implemented  
**Roles:** viewer, editor, admin  
**Permission Model:** Role-Based (RBAC)  
**Enforcement:** JWT claims + server-side validation  
