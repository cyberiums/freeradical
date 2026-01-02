# FreeRadical CMS

[![Version](https://img.shields.io/badge/version-2.6.2-blue.svg)](https://github.com/cyberiums/freeradical/releases/tag/v2.6.2)
![License](https://img.shields.io/badge/license-MIT%20%2F%20Enterprise-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A **blazing-fast**, **production-ready** headless CMS and **e-commerce platform** built with Rust. Designed for performance, scalability, and developer experience.

# Fastly built with <a href="https://fastbuilder.ai" target="_blank" class="text-cyan-400 hover:text-cyan-300 transition-colors">FastBuilder.AI</a>

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/cyberiums/freeradical.git
cd freeradical

# Option 1: Docker (recommended)
docker-compose up -d

# Option 2: Manual setup
cargo build --release        # Build CMS
cd admin && npm install      # Install admin UI
cd ../cli && cargo build     # Build CLI tool
```

## 📁 Project Structure

```
freeradical/
├── src/              # Main CMS (Rust/Actix-web)
├── admin/            # Admin UI (React/TypeScript)
├── cli/              # CLI tool (Rust)
├── migrations/       # Database migrations
├── uploads/          # Media files
└── docker-compose.yml
```

## 🎯 Features

### Core CMS
- ✅ Content management with pages, modules, and media
- ✅ Multi-language support (i18n)
- ✅ SEO optimization with sitemaps and metadata
- ✅ Multi-tenancy with tenant isolation

### E-Commerce
- ✅ Product catalog with variants
- ✅ Order management
- ✅ Payment processing (extensible handlers)
- ✅ Inventory tracking
- ✅ Shopping cart and checkout

### AI Capabilities
- ✅ AI content generation
- ✅ Sentiment analysis
- ✅ Fraud detection
- ✅ Price optimization
- ✅ Demand forecasting

### Enterprise Features
- ✅ OAuth (Google, GitHub, SAML 2.0)
- ✅ Multi-provider SSO
- ✅ Advanced CRM with customer segmentation
- ✅ Analytics and metrics tracking
- ✅ Audit logging
- ✅ Webhooks (HMAC signed)
- ✅ Role-based access control

### API
- ✅ **154 REST endpoints** fully documented
- ✅ OpenAPI 3.0 specification
- ✅ Interactive Swagger UI
- ✅ Beautiful ReDoc documentation
- ✅ Production URL: https://freeradical.dev

## 🛠️ Development

```bash
# Start CMS
cargo run

# Start admin UI (in admin/)
npm run dev

# Use CLI (in cli/)
cargo run -- init my-project
```

## 📚 Documentation

### API Documentation
- **[Swagger UI](http://localhost:8000/swagger-ui)** - Interactive API explorer
- **[ReDoc](http://localhost:8000/redoc)** - Beautiful API reference
- **[OpenAPI Spec](http://localhost:8000/api-docs/openapi.json)** - Machine-readable spec
- **Production**: https://freeradical.dev

### Guides
- [Deployment Guide](oxidly/docs/DEPLOYMENT.md)
- [Admin README](admin/README.md)
- [CLI README](cli/README.md)
- [Phase 5 Implementation](oxidly/docs/core/PHASE-5-IMPLEMENTATION.md)

## 🧪 Testing

```bash
# CMS tests
cargo test

# Integration tests
cargo test --test integration_tests

# Load tests
k6 run tests/load/scenarios.js
```

## 🚢 Deployment

See [DEPLOYMENT.md](oxidly/docs/DEPLOYMENT.md) for comprehensive deployment guide.

Quick deploy with Docker:
```bash
./scripts/deploy.sh
```

## 📊 Status

**v2.6.2**: Latest Release (OpenAPI Complete) - January 2, 2026
- ✅ **154 API Endpoints** fully documented
- ✅ Complete OpenAPI 3.0 specification
- ✅ Zero schema validation errors
- ✅ Real-time Swagger UI
- ✅ Production-ready ReDoc

**Previous Releases:**
- v2.6.1: Complete API Documentation
- v2.5.0: Enterprise Release (SSO, Audit, Webhooks)
- Phases 1-23: 100% complete

## Environment Variables

### Database Configuration

FreeRadical uses **PostgreSQL** for optimal performance and advanced features.

```bash
DATABASE_URL=postgres://freeradical:password@localhost:5432/freeradical
POSTGRES_USER=freeradical
POSTGRES_PASSWORD=password
POSTGRES_DB=freeradical
```

**Start the stack:**
```bash
docker-compose up -d
# Access at http://localhost:8000 (CMS)
# Access at http://localhost:3000 (Admin)
```

**Test connection:**
```bash
bash scripts/test_postgres.sh
```

**Why PostgreSQL?**
- ✅ **30% faster homepage** (1,605 req/s) - Proven in Apache Bench
- ✅ **129% faster API** (3,304 req/s - more than 2x!) - Proven in testing
- ✅ Superior concurrent request handling
- ✅ Advanced features (JSON, full-text search, arrays)
- ✅ Modern cloud infrastructure ready
- ✅ **Production recommended**

**Performance Benchmarks:**
- Homepage: **1,605 req/s** 🚀
- Pages API: **3,304 req/s** 🚀
- Median response: **2-4ms**

📖 **Full guide**: See [`oxidly/docs/core/databases.md`](./oxidly/docs/core/databases.md)

See `.env.sample` for all configuration options.

---

## 📚 Documentation

Comprehensive documentation is available in the [`oxidly/docs/`](./oxidly/docs/) directory:

### Developer Guides
- **[SDK Developer Guide](./oxidly/docs/core/sdk/SDK_DEVELOPER_GUIDE.md)** - Build client libraries
- **[Core Developer Guide](./oxidly/docs/core/core/CORE_DEVELOPER_GUIDE.md)** - Contribute to core
- **[SDK Overview](./oxidly/docs/core/sdk/sdk_overview.md)** - Compare available SDKs

### Architecture
- **[Payment System](./oxidly/docs/core/architecture/payment_architecture.md)** - Extensible payment handlers
- **[Feature Walkthrough](./oxidly/docs/core/walkthrough.md)** - Complete feature overview

### Historical Documentation
- **[Iterations](./oxidly/docs/core/iterations/)** - Development history by phase
- **[Roadmaps](./oxidly/docs/core/roadmaps/)** - Product planning & roadmaps
- **[Releases](./oxidly/docs/core/releases/)** - Version history & changelogs

**Start here**: [Documentation Index](./oxidly/docs/core/README.md)

---

## 🤝 Contributing

Contributions are welcome! Please see:
- [Core Developer Guide](./oxidly/docs/core/core/CORE_DEVELOPER_GUIDE.md) for backend/frontend development
- [SDK Developer Guide](./oxidly/docs/core/sdk/SDK_DEVELOPER_GUIDE.md) for SDK development

---

## 📜 License

FreeRadical CMS uses a **dual-license model**:

### Open Source (MIT License)
- ✅ **Content API** - Free for all
- ✅ **Internal API** - Free for all  
- ✅ **Customer API** - Free for companies < $20M revenue
- ✅ **Commerce API** - Free for companies < $20M revenue

### Enterprise License Required
- 🔒 **oxidly/** - Proprietary (all companies)
- 🔒 **marketplace/** - Proprietary (all companies)
- 🔒 **Customer API** - Companies ≥ $20M revenue
- 🔒 **Commerce API** - Companies ≥ $20M revenue

**Quick determination:**
- Revenue < $20M? → Use MIT License for all APIs ✅
- Revenue ≥ $20M? → Enterprise License required for Customer/Commerce APIs
- Using oxidly or marketplace? → Enterprise License required

📄 **License Files:**
- [LICENSE](./LICENSE) - Overview and component breakdown
- [LICENSE-MIT](./LICENSE-MIT) - Full MIT License text
- [LICENSE-ENTERPRISE](./LICENSE-ENTERPRISE) - Full Enterprise License text

💼 **Enterprise License:** enterprise@fastbuilder.ai

## 🙏 Credits

Built with:
- **Backend:** Rust + Actix-web
- **Frontend:** React + TypeScript + Vite  
- **Database:** PostgreSQL
- **Cache:** Redis
- **Editor:** TipTap, React Query, Tailwind CSS

---

**FreeRadical CMS** - Modern, fast, SEO-optimized headless CMS

## 🎨 Screenshots

### Homepage
![FreeRadical CMS Homepage](assets/freeradicalhomepage.png)

Modern, professional landing page built with Tailwind CSS showcasing the CMS features and technology stack.


### Security & Deployment
- **[HTTPS Setup Guide](./oxidly/docs/core/https-setup.md)** - Enable SSL/TLS for production

