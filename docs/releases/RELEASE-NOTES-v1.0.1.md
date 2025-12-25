# FreeRadical CMS v1.0.1 - Release Notes

**Release Date**: December 25, 2025  
**Version**: 1.0.1  
**Codename**: "Commerce & SDK Ecosystem"

---

## 🎉 Overview

FreeRadical CMS v1.0.1 is a **major milestone release** introducing complete e-commerce capabilities and a comprehensive SDK ecosystem. This release transforms FreeRadical from a powerful headless CMS into a **full-stack commerce platform**.

---

## ✨ Headline Features

### 1. Complete Commerce API

Build e-commerce applications with our new Product and Order Management APIs:

- **Product Catalog**: Full CRUD operations with SKU tracking and inventory management
- **Order Processing**: Multi-item orders with automatic total calculation
- **Payment Integration**: Seamless integration with Stripe, PayPal, and Square
- **Order Workflow**: Status management from pending → processing → completed

### 2. Multi-Language SDK Ecosystem

Official client libraries in three languages:

- **TypeScript/JavaScript** (`@freeradical/sdk` v1.0.1)
- **Python** (`freeradical-client` v1.0.1)
- **Go** (`github.com/cyberiums/freeradical-go-client` v1.0.1)

All SDKs are auto-generated from OpenAPI spec with full type safety.

### 3. Professional Documentation

Complete developer documentation organized in `docs/`:

- SDK Developer Guide
- Core Developer Guide  
- Publishing guides with CI/CD automation
- Architecture documentation
- Migration guides

---

## 📦 Commerce API Details

### Product Management (5 Endpoints)

```bash
# List products (public, paginated)
GET /products?page=0&per_page=20

# Get single product (public)
GET /products/{id}

# Create product (requires authentication)
POST /products
{
  "name": "Product Name",
  "price_cents": 5000,
  "currency": "USD",
  "sku": "PROD-001",
  "inventory_count": 100
}

# Update product (requires authentication)
PUT /products/{id}

# Delete product (soft delete, requires authentication)
DELETE /products/{id}
```

### Order Management (5 Endpoints)

```bash
# List user's orders (requires authentication)
GET /orders

# Get order details with items (requires authentication)
GET /orders/{id}

# Create order (requires authentication)
POST /orders
{
  "items": [
    {"product_id": 1, "quantity": 2},
    {"product_id": 3, "quantity": 1}
  ],
  "currency": "USD"
}

# Update order status (requires authentication)
PUT /orders/{id}/status
{
  "status": "completed"
}

# Link payment to order (requires authentication)
POST /orders/{id}/payment
{
  "payment_provider": "stripe",
  "payment_intent_id": "pi_xxx"
}
```

---

## 🛠️ SDK Usage Examples

### TypeScript

```typescript
import { FreeRadicalClient } from '@freeradical/sdk';

const client = new FreeRadicalClient({
  baseUrl: 'https://api.example.com',
  bearerToken: 'your-jwt-token'
});

// List products
const products = await client.products.list({ page: 0, per_page: 20 });

// Create order
const order = await client.orders.create({
  items: [
    { product_id: 1, quantity: 2 }
  ],
  currency: 'USD'
});
```

### Python

```python
from freeradical_client import ApiClient, Configuration, DefaultApi

config = Configuration(
    host="https://api.example.com",
    access_token="your-jwt-token"
)

with ApiClient(config) as client:
    api = DefaultApi(client)
    
    # List products
    products = api.products_get(page=0, per_page=20)
    
    # Create order
    order = api.orders_post({
        "items": [{"product_id": 1, "quantity": 2}],
        "currency": "USD"
    })
```

### Go

```go
import "github.com/cyberiums/freeradical-go-client"

client := freeradical.NewAPIClient(&freeradical.Configuration{
    Host: "api.example.com",
    Scheme: "https",
})

ctx := context.WithValue(context.Background(), 
    freeradical.ContextAccessToken, "your-jwt-token")

// List products
products, _, err := client.DefaultApi.ProductsGet(ctx).
    Page(0).PerPage(20).Execute()

// Create order
order, _, err := client.DefaultApi.OrdersPost(ctx).
    CreateOrderRequest(freeradical.CreateOrderRequest{
        Items: []freeradical.OrderItemInput{
            {ProductId: 1, Quantity: 2},
        },
        Currency: "USD",
    }).Execute()
```

---

## 🔧 Technical Improvements

### Database Schema

New tables for commerce:

- `products` - Product catalog with SKU and inventory
- `orders` - Order tracking
 with payment integration
- `order_items` - Line items for orders

Enhanced users table:
- `two_factor_secret` - For TOTP 2FA
- `two_factor_enabled` - 2FA status

### Authentication & Security

- JWT authentication enforced on all commerce endpoints
- User isolation for orders
- Payment data protected
- Soft delete for products

### Performance

- Pagination support (up to 100 items per page)
- Optimized database queries
- Efficient order total calculation

---

## 📚 Documentation Highlights

### For SDK Developers

- [SDK Developer Guide](docs/sdk/SDK_DEVELOPER_GUIDE.md)
- [SDK Overview](docs/sdk/sdk_overview.md)
- [Publishing Guide](docs/sdk/PUBLISHING_GUIDE.md)

### For Core Contributors

- [Core Developer Guide](docs/core/CORE_DEVELOPER_GUIDE.md)
- [Architecture Documentation](docs/architecture/)
- [Migration Guides](docs/migrations/)

### For Users

- [API Documentation](API-DOCS.md)
- [Deployment Guide](DEPLOYMENT.md)
- [Changelog](CHANGELOG.md)

---

## 🚀 Getting Started

### Install SDKs

```bash
# TypeScript/JavaScript
npm install @freeradical/sdk

# Python
pip install freeradical-client

# Go
go get github.com/cyberiums/freeradical-go-client@v1.0.1
```

### Deploy FreeRadical

```bash
# Clone repository
git clone https://github.com/cyberiums/freeradical.git
cd freeradical

# Build
cargo build --release

# Run
./target/release/freeradical
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed instructions.

---

## 🔄 Migration from v0.9.0

### Database Migrations

Apply the commerce schema:

```bash
# Run pending migrations
diesel migration run
```

Or apply manually:

```bash
mysql -u user -p database < migrations/2025-12-25-020000_add_commerce/up.sql
```

### API Changes

**New endpoints** (backwards compatible):
- All `/products/*` endpoints
- All `/orders/*` endpoints

**No breaking changes** to existing endpoints.

---

## ⚠️ Known Issues

None reported. This is a stable release.

---

## 🎯 What's Next: v1.1.0

Planned features:
- 2FA frontend UI
- Automated backup scheduling
- Integration tests
- Admin user guide
- Product categories
- Order search and filtering

---

## 🙏 Acknowledgments

Built with ❤️ by the FreeRadical team and contributors.

Special thanks to:
- OpenAPI Generator for SDK generation
- Diesel ORM for database management
- Actix-web for the web framework
- All our beta testers

---

## 📞 Support

- **Documentation**: https://github.com/cyberiums/freeradical/tree/main/docs
- **Issues**: https://github.com/cyberiums/freeradical/issues
- **Discussions**: https://github.com/cyberiums/freeradical/discussions
- **Email**: team@fastbuilder.ai

---

## 📄 License

MIT License - See [LICENSE](LICENSE) for details

---

**Download**: [GitHub Releases](https://github.com/cyberiums/freeradical/releases/tag/v1.0.1)

**Full Changelog**: https://github.com/cyberiums/freeradical/compare/v0.9.0...v1.0.1
