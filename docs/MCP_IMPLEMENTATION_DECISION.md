# MCP Admin Tools - Implementation Decision

## Current State

We have an MCP server with RBAC (Role-Based Access Control) that includes:
- ✅ Authentication (JWT with tenant_id, user_id, role)
- ✅ Authorization (admin/editor/viewer roles)
- ✅ Two tools defined:
  - `update_verification_settings` (admin-only)
  - `get_verification_settings` (editor+)

## Problem

MCP tools currently return **placeholder messages**. To make them functional:

**Attempted:** Direct database queries in MCP tools
**Issue:** MCP handler `handle_call_tool()` is synchronous, but database operations are async

**Error:**
```
error[E0728]: `await` is only allowed inside `async` functions and blocks
```

## Options

### Option 1: MCP as AI Interface Layer (Recommended) ⭐

**Approach:**
- Keep MCP tools lightweight
- Tools provide AI-friendly descriptions
- AI agents use REST API for actual operations
- MCP tools could internally call REST endpoints (via HTTP)

**Pros:**
- Simple, maintainable
- Clear separation of concerns
- MCP = AI interface, REST = operations
- Already have working REST API

**Cons:**
- MCP tools don't directly access database
- Slight overhead (MCP → REST → DB instead of MCP → DB)

**Implementation:**
```rust
// MCP tool returns guidance
"update_verification_settings"  => {
    MCPResponse {
        result: Some(json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "To update settings, call:\n\
                     PUT /v1/api/verification/settings/{}\n\
                     Headers: Authorization: Bearer <token>\n\
                     Body: {{\"ttl_hours\": 24}}",
                    verification_type
                )
            }]
        }))
    }
}
```

### Option 2: Async MCP Tool Execution (Complex)

**Approach:**
- Refactor WebSocket actor to support async tool handlers
- Use `actix::AsyncContext` for async operations
- Tools directly query database

**Pros:**
- MCP tools fully functional
- Direct database access
- Fewer HTTP requests

**Cons:**
- Significant refactoring (WebSocket actor, message passing)
- More complex error handling
- Harder to maintain

**Implementation:**
Requires refactoring entire `MCPWebSocket` Actor structure to use async message handlers.

### Option 3: Hybrid Approach (Middle Ground)

**Approach:**
- Keep synchronous placeholder messages for now
- Document that AI agents should use REST API
- Future: Add async support when needed

**Pros:**
- Ship quickly
- Proven architecture (REST API works)
- Can be enhanced later

**Cons:**
- MCP tools less useful initially
- Documentation burden on AI agents

## Recommendation

**Go with Option 1** - MCP as AI Interface Layer

**Why:**
1. **Faster to ship** - minimal code changes
2. **Clearer architecture** - MCP for AI interaction, REST for operations
3. **Easier to maintain** - less coupling
4. **Proven** - REST API already works and is tested

**Next Steps:**
1. Update MCP tool responses to provide clear API guidance
2. Document MCP → REST workflow
3. Optional: Make tools call REST endpoints internally (future enhancement)

## Decision

**Waiting for user input:**
- Option 1: MCP as AI interface (simple) ✅ Recommended
- Option 2: Full async implementation (complex)
- Option 3: Hybrid approach
- Other?
