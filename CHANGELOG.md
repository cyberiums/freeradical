# FreeRadical CMS - Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.1] - 2025-12-25

### 🎉 Major Release: Commerce & SDK Ecosystem

This release marks a major milestone with complete e-commerce capabilities and a comprehensive SDK ecosystem.

### Added

#### Commerce API
- **Product Management**
  - `GET /products` - List products with pagination
  - `GET /products/{id}` - Get single product
  - `POST /products` - Create product (authenticated)
  - `PUT /products/{id}` - Update product (authenticated)
  - `DELETE /products/{id}` - Soft delete product (authenticated)
  
- **Order Management**
  - `GET /orders` - List user's orders (authenticated)
  - `GET /orders/{id}` - Get order details with items (authenticated)
  - `POST /orders` - Create order with line items (authenticated)
  - `PUT /orders/{id}/status` - Update order status (authenticated)
  - `POST /orders/{id}/payment` - Link payment to order (authenticated)
  
- **Commerce Features**
  - Automatic order total calculation
  - Product availability validation
  - Multi-item order support
  - Order-payment integration
  - Order status workflow (pending → processing → completed)

#### SDK Ecosystem
- **TypeScript/JavaScript SDK v1.0.1**
  - Full API coverage
  - TypeScript type definitions
  - Comprehensive documentation
  - npm package ready

- **Python SDK v1.0.1**
  - Auto-generated from OpenAPI spec
  - Type hints included
  - PyPI package ready
  
- **Go SDK v1.0.1**
  - Module path: `github.com/cyberiums/freeradical-go-client`
  - Go 1.21+ compatibility
  - GitHub release ready

#### Infrastructure
- **Database Schema**
  - `products` table with SKU, inventory tracking
  - `orders` table with payment integration
  - `order_items` table for line items
  - `two_factor_secret` and `two_factor_enabled` columns on users table

- **Documentation**
  - SDK Developer Guide
  - Core Developer Guide
  - Publishing Guide with automated CI/CD
  - Complete API documentation
  - Organized docs/ folder structure

### Changed
- Reorganized all documentation into `docs/` folder structure
- Moved release notes to `docs/releases/`
- Moved upgrade documentation to `docs/migrations/`
- Moved benchmarks to `docs/benchmarks/`

### Fixed
- Resolved Diesel schema conflicts
- Fixed CustomHttpError enum usage
- Corrected payment controller error handling

### Security
- JWT authentication enforced on all mutation endpoints
- User isolation for orders (users only see their own)
- Payment data protected with proper authorization

---

## [0.9.0] - 2025-12-24

### Added
- Payment provider system (Stripe, PayPal, Square)
- Plugin infrastructure
- Multi-engine template support (Handlebars + Liquid)
- GraphQL API foundation
- 2FA backend (TOTP)
- Automated backup service

### Changed
- Upgraded to Diesel 2.x
- Performance optimizations

---

## [0.8.0] - 2025-12-23

### Added
- SEO features (meta tags, sitemap, robots.txt)
- Media optimization
- i18n support
- OAuth integrations

---

## [0.7.0] - 2025-12-20

### Added
- Admin UI foundation
- Content versioning
- Webhook system

---

[1.0.1]: https://github.com/cyberiums/freeradical/compare/v0.9.0...v1.0.1
[0.9.0]: https://github.com/cyberiums/freeradical/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/cyberiums/freeradical/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/cyberiums/freeradical/releases/tag/v0.7.0

## [1.0.3] - 2025-12-25

### Changed
- Upgraded Rust from 1.90.0 to 1.92.0 stable
- Updated README badge to rust-1.92+

### Fixed
- Fixed GitHub Actions workflow paths (cli/ and admin/)
- Fixed analytics_service test type mismatches
- Fixed /admin route serving admin portal chooser
- Improved CI error handling and debugging

### Added
- Better CI workflow existence checks
- Admin portal chooser at /admin

