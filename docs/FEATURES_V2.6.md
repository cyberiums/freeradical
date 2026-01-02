# FreeRadical v2.6+ Feature Summary

## What's New in v2.6

### 🤖 Native MCP Server (Port 9009)

AI agents can now interact with FreeRadical conversationally through the Model Context Protocol.

**Features:**
- WebSocket-based JSON-RPC 2.0
- JWT authentication with bearer tokens
- Role-Based Access Control (Admin, Editor, Viewer)
- Tenant isolation
- MCP Protocol v2024-11-05 compliant

**Quick Start:**
```bash
# Get JWT token
curl -X POST http://localhost:8000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"password"}'

# Connect to MCP
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer YOUR_TOKEN"

# Health check (no auth required)
curl http://localhost:9009/health
```

**Documentation:** [MCP_SERVER.md](file:///Users/prabhatsingh/freeradical/docs/MCP_SERVER.md)

---

### 📧 Newsletter Subscription System

Public-facing double opt-in email verification system for newsletters, contact forms, and waitlists.

**Features:**
- Double opt-in email verification (prevents spam)
- Configurable TTL (1-168 hours, default 12h)
- Secure token generation (UUID + 32 random chars)
- Automatic cleanup of expired verifications
- Built on CRM APIs

**Public API (No Auth Required):**
```bash
# Subscribe
curl -X POST http://localhost:8000/v1/public/crm/customers \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "first_name": "John",
    "lifecycle_stage": "lead",
    "tags": ["newsletter_subscriber"],
    "source": "website_signup"
  }'

# Response
{
  "message": "Verification email sent. Please check your inbox.",
  "expires_in_hours": 12
}

# User clicks link in email to verify
# GET /v1/public/crm/verify/{token}
```

**Documentation:** [CRM_VERIFICATION_FLOW.md](file:///Users/prabhatsingh/freeradical/docs/CRM_VERIFICATION_FLOW.md)

---

### ⚙️ Configurable Verification TTL

Admins can configure email verification expiration times per tenant and verification type.

**Admin API (JWT Required):**
```bash
# Get settings
curl http://localhost:8000/v1/api/verification/settings \
  -H "Authorization: Bearer TOKEN"

# Update TTL to 24 hours
curl -X PUT http://localhost:8000/v1/api/verification/settings/crm_customer \
  -H "Authorization: Bearer TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"ttl_hours": 24, "enabled": true}'
```

**Hierarchical Resolution:**
1. Tenant-specific settings
2. Global defaults
3. Hardcoded fallback (12 hours)

---

## Architecture

```
FreeRadical CMS
├── REST API (port 8000)
│   ├── Public endpoints (/v1/public/*)
│   │   └── Newsletter subscriptions
│   └── Admin endpoints (/v1/api/*)
│       └── Verification settings
│
└── MCP Server (port 9009)
    ├── WebSocket (/mcp)
    ├── Health check (/health)
    ├── Resources (read configuration)
    └── Tools (AI-guided operations)
```

---

## Security

### MCP Server
- ✅ JWT Authentication required
- ✅ Role-Based Access Control (Admin, Editor, Viewer)
- ✅ Tenant isolation (multi-tenant safe)
- ✅ Audit logging

### Email Verification
- ✅ Secure tokens (UUID + 32 random chars)
- ✅ One-time use (invalidated after verification)
- ✅ Auto-expiration (configurable TTL)
- ✅ Automatic cleanup (hourly background job)

---

## Database Tables

### `pending_verifications`
Stores unverified email subscriptions with tokens and expiration.

### `verification_settings`
Configurable TTL and email templates per tenant and verification type.

### `crm_customers`
Verified subscribers stored with tags (e.g., `newsletter_subscriber`).

---

## Use Cases

### Newsletter Subscriptions
1. User submits email via web form
2. System creates pending verification
3. User receives verification email
4. User clicks link to verify
5. User added to CRM as subscriber
6. Welcome email sent

### AI-Powered CRM Management
1. AI agent connects to MCP server with JWT
2. Agent asks: "Set newsletter TTL to 24 hours"
3. MCP provides API guidance with cURL example
4. Agent calls REST API to update settings
5. Settings updated with audit log

---

## Testing

### MCP Server
```bash
# Health check
curl http://localhost:9009/health

# WebSocket test
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer TOKEN"
```

### Newsletter Flow
```bash
# Subscribe
curl -X POST http://localhost:8000/v1/public/crm/customers \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","first_name":"Test","lifecycle_stage":"lead","tags":["newsletter_subscriber"],"source":"test"}'

# Verify (replace TOKEN with actual token from email)
curl http://localhost:8000/v1/public/crm/verify/TOKEN
```

---

## Documentation Index

| Document | Purpose |
|----------|---------|
| [MCP_SERVER.md](file:///Users/prabhatsingh/freeradical/docs/MCP_SERVER.md) | MCP server quick start and API reference |
| [MCP_RBAC.md](file:///Users/prabhatsingh/freeradical/docs/MCP_RBAC.md) | Role-based access control details |
| [MCP_IMPLEMENTATION_DECISION.md](file:///Users/prabhatsingh/freeradical/docs/MCP_IMPLEMENTATION_DECISION.md) | Design rationale and architecture decisions |
| [CRM_VERIFICATION_FLOW.md](file:///Users/prabhatsingh/freeradical/docs/CRM_VERIFICATION_FLOW.md) | Email verification flow and integration |
| **[FEATURES_V2.6.md](file:///Users/prabhatsingh/freeradical/docs/FEATURES_V2.6.md)** | **This file - Quick reference** |

---

## AI Agent Integration

### Claude Desktop

**Config:** `~/Library/Application Support/Claude/claude_desktop_config.json`
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

**Natural Language Commands:**
- "Set newsletter verification TTL to 24 hours"
- "Show me current verification settings"
- "Enable email verification for user registration"

---

## Production Deployment

### Environment Variables
```bash
# MCP Server
MCP_PORT=9009

# Email Verification
CRM_VERIFY_URL=https://your-domain.com/v1/public/crm/verify
SMTP_HOST=smtp.example.com
SMTP_FROM=noreply@your-domain.com

# JWT
JWT_SECRET=your-secret-key
JWT_EXPIRATION_HOURS=24
```

### Nginx WebSocket Proxy
```nginx
location /mcp {
    proxy_pass http://localhost:9009/mcp;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
}
```

---

## Status

- ✅ **MCP Server:** Production Ready
- ✅ **Newsletter System:** Production Ready
- ✅ **TTL Configuration:** Production Ready
- ✅ **RBAC:** Fully Implemented
- ✅ **Documentation:** Complete

**Version:** v2.6.0+  
**Last Updated:** 2025-12-30
