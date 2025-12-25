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

### Core CMS (v0.8.0)
- ✅ GraphQL API with Playground
- ✅ RESTful API
- ✅ Image optimization (auto WebP, resize)
- ✅ SEO optimization (sitemaps, schemas)
- ✅ Redis caching
- ✅ JWT authentication

### Admin Dashboard
- ✅ Modern React 18 UI
- ✅ TipTap WYSIWYG editor
- ✅ Media browser
- ✅ Dark/light mode
- ✅ SEO preview

### CLI Tool
- ✅ Project scaffolding
- ✅ Content export/import
- ✅ Database migrations
- ✅ Development server

### Multi-Language Support (Ready)
- Database schema complete
- Translation API ready
- Hreflang generation

### Enterprise Features (Ready)
- OAuth (Google, GitHub)
- Analytics tracking
- Multi-tenancy schema

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

## 🧪 Testing

```bash
# CMS tests
cargo test

# Admin tests
cd admin && npm test

# CLI tests
cd cli && cargo test
```

## 🚢 Deployment

See [DEPLOYMENT.md](DEPLOYMENT.md) for comprehensive deployment guide.

Quick deploy with Docker:
```bash
./scripts/deploy.sh
```

## 📊 Status

**v0.8.0**: 90% complete
- Phase 1 (Core): 100%
- Phase 2 (Admin): 92%
- Phase 3 (CLI): 100%
- Phase 4 (i18n): 85%
- Phase 5 (Enterprise): 75%

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
