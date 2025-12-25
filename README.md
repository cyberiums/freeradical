# FreeRadical CMS

Open-source headless CMS built with Rust, featuring GraphQL API, SEO optimization, and modern admin interface.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/freeradical.git
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

### Core CMS (v0.9.0)
- ✅ GraphQL API with Playground
- ✅ RESTful API
- ✅ Image optimization (auto WebP, resize)
- ✅ SEO optimization (sitemaps, schemas)
- ✅ Redis caching
- ✅ JWT authentication

### Admin Dashboard (Ready)
- ✅ Modern React 18 UI
- ✅ TipTap WYSIWYG editor
- ✅ Media browser
- ✅ Dark/light mode
- ✅ SEO preview
- ✅ Analytics Dashboard 🆕

### CLI Tool (Ready)
- ✅ Project scaffolding
- ✅ Content export/import
- ✅ Database migrations
- ✅ Development server
- ✅ Production builds

### Multi-Language Support (Ready)
- ✅ Database schema complete
- ✅ Translation API ready
- ✅ Hreflang generation
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

**v0.9.0**: 99.8% complete (Production Beta)
- Phase 1 (Core): 100%
- Phase 2 (Admin): 100%
- Phase 3 (CLI): 100%
- Phase 4 (i18n): 100%
- Phase 5 (Enterprise): 99%

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines.

## 📄 License

MIT License - see LICENSE file

## 🙏 Credits

Built with:
- Rust + Actix-web
- React + TypeScript + Vite
- MySQL + Redis
- TipTap, React Query, Tailwind CSS

---

**FreeRadical CMS** - Modern, fast, SEO-optimized headless CMS
