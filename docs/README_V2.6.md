# FreeRadical v2.6 Documentation

This directory contains comprehensive documentation for FreeRadical CMS v2.6+ features, focusing on the MCP Server implementation and Newsletter Subscription System.

## 📚 Documentation Index

### Quick Start
- **[FEATURES_V2.6.md](./FEATURES_V2.6.md)** - Quick reference guide for all v2.6 features
  - MCP Server overview
  - Newsletter subscription system
  - TTL configuration
  - Testing examples

### MCP Server Documentation
- **[MCP_SERVER.md](./MCP_SERVER.md)** - Complete MCP server guide (485 lines)
  - Quick start and connection details
  - Authentication with JWT
  - Available resources and tools
  - AI agent integration examples
  - Development guide

- **[MCP_RBAC.md](./MCP_RBAC.md)** - Role-Based Access Control (306 lines)
  - User roles (Admin, Editor, Viewer)
  - JWT claims requirements
  - Permission matrix
  - Security best practices

- **[MCP_IMPLEMENTATION_DECISION.md](./MCP_IMPLEMENTATION_DECISION.md)** - Design rationale (123 lines)
  - "MCP as AI Interface Layer" pattern
  - Options analysis (async vs guidance)
  - Architectural trade-offs

### Newsletter & CRM Documentation
- **[CRM_VERIFICATION_FLOW.md](./CRM_VERIFICATION_FLOW.md)** - Email verification flow (306 lines)
  - Double opt-in flow diagram
  - Public API endpoints
  - Security features
  - Frontend integration examples
  - Admin configuration

### Other Documentation
- **[enable_oauth.md](./enable_oauth.md)** - OAuth integration guide

---

## 🎯 Feature Overview

### MCP Server (Port 9009)
AI agents can interact with FreeRadical conversationally via the Model Context Protocol.

**Key Features:**
- ✅ WebSocket-based JSON-RPC 2.0
- ✅ JWT authentication
- ✅ Role-Based Access Control (Admin, Editor, Viewer)
- ✅ Tenant isolation
- ✅ MCP Protocol v2024-11-05 compliant

**Quick Test:**
```bash
curl http://localhost:9009/health
```

### Newsletter Subscription System
Public-facing double opt-in email verification for newsletters and forms.

**Key Features:**
- ✅ Double opt-in email verification
- ✅ Configurable TTL (1-168 hours)
- ✅ Secure token generation
- ✅ Automatic cleanup
- ✅ Built on CRM APIs

**Quick Test:**
```bash
curl -X POST http://localhost:8000/v1/public/crm/customers \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","first_name":"Test","lifecycle_stage":"lead","tags":["newsletter_subscriber"],"source":"test"}'
```

---

## 🚀 Quick Start

### 1. Get a JWT Token
```bash
curl -X POST http://localhost:8000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"your-password"}'
```

### 2. Connect to MCP Server
```bash
wscat -c ws://localhost:9009/mcp \
  --header "Authorization: Bearer YOUR_TOKEN"
```

### 3. List Available Tools
```json
{"jsonrpc":"2.0","method":"tools/list","id":1}
```

### 4. Test Newsletter Subscription
```bash
curl -X POST http://localhost:8000/v1/public/crm/customers \
  -H "Content-Type: application/json" \
  -d '{
    "email": "subscriber@example.com",
    "first_name": "John",
    "lifecycle_stage": "lead",
    "tags": ["newsletter_subscriber"],
    "source": "website"
  }'
```

---

## 📖 Reading Guide

### For AI Agent Developers
1. Start with [FEATURES_V2.6.md](./FEATURES_V2.6.md)
2. Read [MCP_SERVER.md](./MCP_SERVER.md) for details
3. Review [MCP_RBAC.md](./MCP_RBAC.md) for security

### For Frontend Developers
1. Read [CRM_VERIFICATION_FLOW.md](./CRM_VERIFICATION_FLOW.md)
2. See frontend examples in the same document
3. Test with public API endpoints

### For System Administrators
1. Review [MCP_RBAC.md](./MCP_RBAC.md)
2. Configure TTL in [CRM_VERIFICATION_FLOW.md](./CRM_VERIFICATION_FLOW.md)
3. Set up production deployment from [FEATURES_V2.6.md](./FEATURES_V2.6.md)

### For Architecture Review
1. Start with [MCP_IMPLEMENTATION_DECISION.md](./MCP_IMPLEMENTATION_DECISION.md)
2. Review "MCP as AI Interface Layer" pattern
3. Understand security model in [MCP_RBAC.md](./MCP_RBAC.md)

---

## 🔒 Security

### MCP Server
- JWT authentication required
- Role-based access control
- Tenant isolation
- Audit logging

### Email Verification
- Secure token generation (UUID + 32 random chars)
- One-time use tokens
- Configurable expiration
- Automatic cleanup

---

## 🛠️ Development

### Adding MCP Tools
See [MCP_SERVER.md](./MCP_SERVER.md) "Development" section

### Adding MCP Resources
See [MCP_SERVER.md](./MCP_SERVER.md) "Development" section

### Configuring Email Templates
See [CRM_VERIFICATION_FLOW.md](./CRM_VERIFICATION_FLOW.md) "Configuration" section

---

## 📊 Architecture

```
FreeRadical CMS
├── REST API (port 8000)
│   ├── Public endpoints (/v1/public/*)
│   │   ├── Newsletter subscriptions
│   │   └── Email verification
│   └── Admin endpoints (/v1/api/*)
│       └── Verification settings
│
└── MCP Server (port 9009)
    ├── WebSocket (/mcp)
    ├── Health check (/health)
    ├── Resources (read configuration)
    └── Tools (AI-guided operations)
        ├── get_verification_settings (Editor+)
        └── update_verification_settings (Admin)
```

---

## 🧪 Testing

### MCP Server
```bash
# Health check
curl http://localhost:9009/health

# WebSocket connection
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

## 📝 API Reference

### Public Endpoints (No Auth)
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/public/crm/customers` | POST | Create subscription with verification |
| `/v1/public/crm/verify/{token}` | GET | Verify email token |

### Admin Endpoints (JWT Required)
| Endpoint | Method | Role | Purpose |
|----------|--------|------|---------|
| `/v1/api/verification/settings` | GET | Editor+ | List settings |
| `/v1/api/verification/settings/{type}` | PUT | Admin | Update settings |
| `/v1/api/verification/settings/{type}` | DELETE | Admin | Delete settings |

### MCP Endpoints
| Endpoint | Protocol | Purpose |
|----------|----------|---------|
| `ws://localhost:9009/mcp` | WebSocket | MCP connection |
| `http://localhost:9009/health` | HTTP | Health check |

---

## 🌐 Production Deployment

### Environment Variables
```bash
MCP_PORT=9009
CRM_VERIFY_URL=https://your-domain.com/v1/public/crm/verify
JWT_SECRET=your-secret-key
```

### Nginx Configuration
```nginx
location /mcp {
    proxy_pass http://localhost:9009/mcp;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
}
```

See [FEATURES_V2.6.md](./FEATURES_V2.6.md) for complete deployment guide.

---

## 📚 External Resources

- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [MCP SDK Documentation](https://github.com/anthropics/anthropic-sdk-typescript)
- [MCP Server Examples](https://github.com/modelcontextprotocol/servers)

---

## ✅ Status

- **MCP Server:** Production Ready ✅
- **Newsletter System:** Production Ready ✅
- **TTL Configuration:** Production Ready ✅
- **RBAC:** Fully Implemented ✅
- **Documentation:** Complete ✅

**Version:** v2.6.0+  
**Last Updated:** 2025-12-30
