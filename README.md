# FreeRadical CMS

[![Version](https://img.shields.io/badge/version-1.4.0-blue.svg)](https://github.com/cyberiums/freeradical/releases/tag/v1.4.0)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A **blazing-fast**, **production-ready** headless CMS and **e-commerce platform** built with Rust. Designed for performance, scalability, and developer experience.

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

- ✅ Language management

### Enterprise Features (Ready)
- ✅ OAuth (Google, GitHub)
- ✅ Analytics tracking
- ✅ Multi-tenancy schema
- ✅ Load testing suite

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

- [Deployment Guide](DEPLOYMENT.md)
- [Admin README](admin/README.md)
- [CLI README](cli/README.md)
- [API Documentation](https://docs.freeradical.dev)
- [Phase 5 Implementation](PHASE-5-IMPLEMENTATION.md)

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

See [DEPLOYMENT.md](DEPLOYMENT.md) for comprehensive deployment guide.

Quick deploy with Docker:
```bash
./scripts/deploy.sh
```

## 📊 Status

**v1.0.3**: 99.8% complete (Production Beta)
- Phase 1 (Core): 100%
- Phase 2 (Admin): 100%
- Phase 3 (CLI): 100%
- Phase 4 (i18n): 100%
- Phase 5 (Enterprise): 99%

## Environment Variables

### Database Configuration

FreeRadical supports **both PostgreSQL and MySQL** with runtime switching. **PostgreSQL is recommended** for maximum performance (30-129% faster in benchmarks).

#### PostgreSQL (Default - Recommended 🏆)

```bash
DATABASE_URL=postgres://freeradical:password@localhost:5432/freeradical
POSTGRES_USER=freeradical
POSTGRES_PASSWORD=password
POSTGRES_DB=freeradical
```

Start PostgreSQL stack (default):
```bash
docker-compose up -d
# Access at http://localhost:8000 (CMS)
# Access at http://localhost:3000 (Admin)
```

Test PostgreSQL:
```bash
bash scripts/test_postgres.sh
```

#### MySQL (Alternative)

```bash
DATABASE_URL=mysql://freeradical:password@localhost:3306/freeradical
APP_MYSQL_USERNAME=freeradical
APP_MYSQL_PASSWORD=password
APP_MYSQL_DATABASE=freeradical
```

Start MySQL stack:
```bash
docker-compose -f docker-compose.mysql.yml up -d
# Access at http://localhost:8000
```

Test MySQL:
```bash
bash scripts/test_mysql.sh
```

**No rebuild required!** Switch databases by changing the `DATABASE_URL` environment variable.

📖 **Full guide**: See [`docs/databases.md`](./docs/databases.md) for comprehensive database configuration, migration, and troubleshooting.

See `.env.sample` for all configuration options.

### Which Database Should You Choose?

Based on professional Apache Bench load testing ([detailed results](./docs/performance_benchmarks.md)):

#### Choose PostgreSQL for Maximum Performance 🏆

- ✅ **30% faster homepage** (1,605 vs 1,231 req/s) - **Proven in Apache Bench**
- ✅ **129% faster API** (3,304 vs 1,442 req/s - more than 2x!) - **Proven**
- ✅ Better concurrent request handling
- ✅ Advanced features (JSON, full-text search)
- ✅ Modern cloud infrastructure ready
- ✅ **Recommended for production**

**Apache Bench Results:** 
- Homepage: **1,605 req/s** 🚀
- Pages API: **3,304 req/s** 🚀
- Median response: **2-4ms**

#### Choose MySQL for Simplicity

- ✅ Team familiarity preferred
- ✅ Traditional hosting environments  
- ✅ Simpler operations
- ✅ **Still excellent performance** (1,231 req/s)

**Apache Bench Results:**
- Homepage: 1,231 req/s
- Pages API: 1,442 req/s
- Median response: 2-5ms

#### Deployment Scenarios (Updated with Benchmarks)

| Use Case | Recommended Database | Performance | Why |
|----------|---------------------|-------------|-----|
| 📝 Blog / Content Site | **PostgreSQL** 🏆 | 1,605 req/s | 30% faster, proven in tests |
| 🛒 E-commerce Platform | **PostgreSQL** 🏆 | 3,304 req/s | 129% faster API, complex queries |
| 🔌 API Backend | **PostgreSQL** 🏆 | 3,304 req/s | Proven 2x faster API responses |
| 📊 Analytics Dashboard | **PostgreSQL** 🏆 | Best | Superior aggregations + speed |
| 📱 Mobile App Backend | **PostgreSQL** 🏆 | 3,304 req/s | Fast APIs critical |
| 🏢 Enterprise CMS | **PostgreSQL** 🏆 | Best | Performance + features |

💡 **Pro Tip:** PostgreSQL delivers 30-129% better performance - use it unless you have specific MySQL requirements!

📈 **See Apache Bench results:** [`docs/benchmarks/apache_bench_v1.0.3.md`](./docs/benchmarks/apache_bench_v1.0.3.md)

---

## 📚 Documentation

Comprehensive documentation is available in the [`docs/`](./docs/) directory:

### Developer Guides
- **[SDK Developer Guide](./docs/sdk/SDK_DEVELOPER_GUIDE.md)** - Build client libraries
- **[Core Developer Guide](./docs/core/CORE_DEVELOPER_GUIDE.md)** - Contribute to core
- **[SDK Overview](./docs/sdk/sdk_overview.md)** - Compare available SDKs

### Architecture
- **[Payment System](./docs/architecture/payment_architecture.md)** - Extensible payment handlers
- **[Feature Walkthrough](./docs/walkthrough.md)** - Complete feature overview

### Historical Documentation
- **[Iterations](./docs/iterations/)** - Development history by phase
- **[Roadmaps](./docs/roadmaps/)** - Product planning & roadmaps
- **[Releases](./docs/releases/)** - Version history & changelogs

**Start here**: [Documentation Index](./docs/README.md)

---

## 🤝 Contributing

Contributions are welcome! Please see:
- [Core Developer Guide](./docs/core/CORE_DEVELOPER_GUIDE.md) for backend/frontend development
- [SDK Developer Guide](./docs/sdk/SDK_DEVELOPER_GUIDE.md) for SDK development

---

## 📜 License

MIT
 License - see LICENSE file

## 🙏 Credits

Built with:
- **Backend:** Rust + Actix-web
- **Frontend:** React + TypeScript + Vite  
- **Databases:** MySQL / PostgreSQL (runtime switchable)
- **Cache:** Redis
- **Editor:** TipTap, React Query, Tailwind CSS

---

**FreeRadical CMS** - Modern, fast, SEO-optimized headless CMS

## 🎨 Screenshots

### Homepage
![FreeRadical CMS Homepage](assets/freeradicalhomepage.png)

Modern, professional landing page built with Tailwind CSS showcasing the CMS features and technology stack.


### Security & Deployment
- **[HTTPS Setup Guide](./docs/https-setup.md)** - Enable SSL/TLS for production

