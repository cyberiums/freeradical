# Release v1.0.1 - Commerce API & SDK Ecosystem

> **Released**: December 25, 2025  
> **Version**: 1.0.1  
> **Status**: Production Ready

---

## Overview

FreeRadical CMS v1.0.1 is a major milestone release that transforms the platform from a headless CMS into a complete **e-commerce platform** with comprehensive **multi-language SDK support**.

### Highlights

- 🛒 **Complete Commerce API** - Product catalog and order management
- 📦 **Multi-Language SDKs** - TypeScript, Python, and Go
- 📚 **Professional Documentation** - Comprehensive guides and references
- 🚀 **Automated Publishing** - CI/CD for all SDKs

---

## Commerce API

### Product Management

Five endpoints for complete product lifecycle management:

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/products` | GET | No | List products (paginated) |
| `/products/{id}` | GET | No | Get single product |
| `/products` | POST | Yes | Create product |
| `/products/{id}` | PUT | Yes | Update product |
| `/products/{id}` | DELETE | Yes | Soft delete product |

**Example**:
```bash
# List products
curl https://api.example.com/products?page=0&per_page=20

# Create product (requires JWT)
curl -X POST https://api.example.com/products \
  -H "Authorization: Bearer YOUR_JWT" \
  -d '{"name":"T-Shirt","price_cents":2999,"currency":"USD","sku":"TSHIRT-001"}'
```

### Order Management

Five endpoints for order processing and payment integration:

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/orders` | GET | Yes | List user's orders |
| `/orders/{id}` | GET | Yes | Get order details |
| `/orders` | POST | Yes | Create order |
| `/orders/{id}/status` | PUT | Yes | Update order status |
| `/orders/{id}/payment` | POST | Yes | Link payment to order |

**Example**:
```bash
# Create order
curl -X POST https://api.example.com/orders \
  -H "Authorization: Bearer YOUR_JWT" \
  -d '{"items":[{"product_id":1,"quantity":2}],"currency":"USD"}'
```

### Features

- ✅ Automatic total calculation
- ✅ Multi-item orders
- ✅ Product availability validation
- ✅ Payment provider integration (Stripe, PayPal, Square)
- ✅ Order status workflow
- ✅ User isolation & security

---

## SDK Ecosystem

### TypeScript/JavaScript SDK v1.0.1

**Package**: `@freeradical/sdk`

```bash
npm install @freeradical/sdk
```

**Usage**:
```typescript
import { FreeRadicalClient } from '@freeradical/sdk';

const client = new FreeRadicalClient({
  baseUrl: 'https://api.example.com',
  bearerToken: 'your-jwt-token'
});

const products = await client.products.list({ page: 0, per_page: 20 });
```

### Python SDK v1.0.1

**Package**: `freeradical-client`

```bash
pip install freeradical-client
```

**Usage**:
```python
from freeradical_client import ApiClient, Configuration, DefaultApi

config = Configuration(
    host="https://api.example.com",
    access_token="your-jwt-token"
)

with ApiClient(config) as client:
    api = DefaultApi(client)
    products = api.products_get(page=0, per_page=20)
```

### Go SDK v1.0.1

**Package**: `github.com/cyberiums/freeradical-go-client`

```bash
go get github.com/cyberiums/freeradical-go-client@go-v1.0.1
```

**Usage**:
```go
import "github.com/cyberiums/freeradical-go-client"

client := freeradical.NewAPIClient(&freeradical.Configuration{
    Host: "api.example.com",
    Scheme: "https",
})

ctx := context.WithValue(context.Background(), 
    freeradical.ContextAccessToken, "your-jwt-token")

products, _, err := client.DefaultApi.ProductsGet(ctx).Execute()
```

---

## Database Schema

### New Tables

**products**:
- Product catalog with SKU tracking
- Inventory management
- Soft delete support

**orders**:
- Order tracking with payment integration
- User association
- Status workflow

**order_items**:
- Line items for orders
- Product references
- Quantity tracking

### Enhanced Tables

**users**:
- `two_factor_secret` - For TOTP 2FA
- `two_factor_enabled` - 2FA status

---

## Documentation

### Developer Guides

- [SDK Developer Guide](../sdk/SDK_DEVELOPER_GUIDE.md)
- [Core Developer Guide](../core/CORE_DEVELOPER_GUIDE.md)
- [Publishing Guide](../sdk/PUBLISHING_GUIDE.md)
- [CI/CD Setup](../sdk/CICD_SETUP.md)

### API Reference

- [API Documentation](../API-DOCS.md)
- [Commerce API](../releases/RELEASE-NOTES-v1.0.1.md)

### Deployment

- [Deployment Guide](../../DEPLOYMENT.md)
- [Publication Checklist](PUBLICATION_CHECKLIST_v1.0.1.md)

---

## Migration from v0.9.0

### Database Migrations

Apply commerce schema:

```bash
diesel migration run
```

Or manually:

```bash
mysql -u user -p database < migrations/2025-12-25-020000_add_commerce/up.sql
```

### API Changes

**New endpoints** (backwards compatible):
- All `/products/*` endpoints
- All `/orders/*` endpoints

**No breaking changes** to existing endpoints.

---

## Installation

### Main Application

```bash
git clone https://github.com/cyberiums/freeradical.git
cd freeradical
cargo build --release
./target/release/freeradical
```

### SDKs

```bash
# TypeScript
npm install @freeradical/sdk

# Python
pip install freeradical-client

# Go
go get github.com/cyberiums/freeradical-go-client@go-v1.0.1
```

---

## What's Next: v1.1.0

Planned features:
- 2FA frontend UI
- Automated backup scheduling
- Integration test suite
- Product categories & tags
- Order search & filtering
- Admin dashboard enhancements

---

## Links

- [GitHub Repository](https://github.com/cyberiums/freeradical)
- [Release Notes](RELEASE-NOTES-v1.0.1.md)
- [Changelog](../../CHANGELOG.md)
- [Issues](https://github.com/cyberiums/freeradical/issues)
- [Discussions](https://github.com/cyberiums/freeradical/discussions)

---

**Download**: [v1.0.1](https://github.com/cyberiums/freeradical/releases/tag/v1.0.1)  
**Full Changelog**: [v0.9.0...v1.0.1](https://github.com/cyberiums/freeradical/compare/v0.9.0...v1.0.1)
