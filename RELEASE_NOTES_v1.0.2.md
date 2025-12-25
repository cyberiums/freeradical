# Release Notes - FreeRadical CMS v1.0.2

**Release Date:** December 25, 2024  
**Type:** Maintenance & Feature Release  
**Status:** ✅ Production Ready

---

## 🎯 Overview

Version 1.0.2 brings critical dependency upgrades, modern UI enhancements, comprehensive HTTPS documentation, and improved admin interface clarity. All 6 planned dependency upgrades completed successfully with zero breaking changes for end users.

---

## 🔒 Security & Dependency Upgrades

### Critical Updates (6/6 Complete)

**1. jsonwebtoken: 7.0 → 10.2.0**
- ✅ Modern cryptographic backend (`aws_lc_rs` feature)
- ✅ Enhanced security for JWT token generation
- ✅ 3 major version jump with full compatibility

**2. argon2: 0.2 → 0.5**
- ✅ Stable password hashing algorithm
- ✅ Migrated to `password-hash` crate ecosystem
- ✅ Improved security for user authentication

**3. diesel: 2.2 → 2.3.5**
- ✅ Database ORM stability improvements
- ✅ Minor version update (backward compatible)
- ✅ Enhanced MySQL query performance

**4. handlebars: 3.5.2 → 6.3.2**
- ✅ Template engine modernization
- ✅ Updated helper signatures and API
- ✅ 3 major version upgrade

**5. async-graphql: 3.0 → 7.0.17**
- ✅ GraphQL API improvements
- ✅ 4 major version jump
- ✅ Enhanced query performance

**6. redis + deadpool: 0.24 → 0.32 + 0.22**
- ✅ Caching layer optimization
- ✅ Connection pooling improvements
- ✅ deadpool-redis compatibility updates

---

## 🎨 UI/UX Enhancements

### New Landing Pages

**Homepage (/):**
- Professional Tailwind CSS design
- Gradient hero section
- Performance statistics showcase
- Feature cards highlighting tech stack

**API Documentation Portal (/static/api.html):**
- GraphQL Playground access
- REST API reference
- Tech stack information
- Quick links to admin portals

**Admin Portal Chooser (/static/admin-portal.html):**
- Side-by-side interface comparison
- Clear feature differentiation
- Visual selection cards

### Admin Interfaces

**Simple Admin (/static/admin-login.html):**
- Lightweight API access
- Animated gradient background
- Direct backend authentication
- Minimal, fast interface

**Full Admin Interface (http://localhost:3000/login):**
- Rich React-based UI
- Complete content management
- Visual editor integration
- User management dashboard

**Key Improvements:**
- ✅ Consistent branding (indigo/purple gradients)
- ✅ Professional typography
- ✅ Responsive mobile design
- ✅ Security badges (JWT + Argon2)
- ✅ Clear interface titles

---

## 📚 Documentation

### HTTPS Setup Guide

**New Documentation:**
- `docs/https-setup.md` - Comprehensive deployment guide
- GitHub Wiki article published
- README link added

**Coverage:**
- Caddy reverse proxy (recommended)
- Nginx + Certbot setup
- Native Actix-Web OpenSSL
- Docker deployment with SSL
- Security best practices
- Troubleshooting guide
- Production checklist

**Resources:**
- Certificate management
- Auto-renewal setup
- Security headers configuration
- Performance optimization tips

---

## 🐛 Bug Fixes

- ✅ Fixed `/api` link 404 error on homepage
- ✅ Resolved Docker static directory missing issue
- ✅ Corrected React admin login design issues
- ✅ Fixed handlebars DirectorySourceOptions API
- ✅ Resolved redis ConnectionLike trait compatibility

---

## 🔧 Technical Details

### Build Status
- **Compile Time:** ~1.5 minutes (release mode)
- **Errors:** 0
- **Warnings:** 156 (non-critical, mostly deprecations)
- **Binary Size:** ~15MB

### Breaking Changes
**None for end users**

**Developer Notes:**
- `handlebars` v6 requires API updates (see upgrade guide)
- `argon2` v0.5 uses new `password-hash` crate
- `redis` v0.32 paired with deadpool-redis v0.22

### Dependencies Added
```toml
password-hash = "0.5"
openssl = "0.10" (optional - for native HTTPS)
```

### Docker Updates
- Static directory now included in image
- Landing pages accessible in production
- Single rebuild required for deployment

---

## 📦 Files Changed

**New Files:**
- `static/index.html` - Homepage
- `static/api.html` - API documentation
- `static/admin-login.html` - Simple admin login
- `static/admin-portal.html` - Admin chooser
- `docs/https-setup.md` - HTTPS guide
- `docs/wiki/HTTPS-Setup-Guide.md` - Wiki article

**Modified Files:**
- `Cargo.toml` - All dependency versions
- `src/services/auth_service.rs` - argon2 v0.5 API
- `src/helpers/default.rs` - handlebars v6 helpers
- `src/services/template_service.rs` - DirectorySourceOptions
- `src/watch.rs` - Template directory registration
- `admin/src/pages/Login.tsx` - React admin styling
- `Dockerfile` - Static directory inclusion
- `README.md` - Screenshots and HTTPS link

---

## 🚀 Deployment

### Upgrading from v1.0.1

```bash
# Pull latest changes
git pull origin main

# Rebuild Docker images
docker-compose down
docker-compose build
docker-compose up -d

# Or rebuild locally
cargo build --release
cd admin && npm run build
```

**No database migrations required.**

---

## 🧪 Testing

All components tested and verified:
- ✅ Dependency compilation successful
- ✅ All unit tests passing
- ✅ Integration tests green
- ✅ Docker build successful
- ✅ Landing pages responsive
- ✅ Admin authentication working
- ✅ API endpoints functional

---

## 📊 Performance

**No regressions detected:**
- Request throughput: 4,300+ req/s maintained
- P95 latency: <5ms maintained
- Memory usage: Stable
- Build performance: Improved (parallel compilation)

---

## 🙏 Credits

**Built with:**
- Rust 1.70+ & Actix-Web v4.12
- React 18 & TypeScript
- MySQL 8.0 & Redis 0.32
- Tailwind CSS 3.0
- Docker & docker-compose

---

## 📝 Migration Notes

### For Developers

**If using custom handlebars helpers:**
```rust
// Old (v3)
fn call(&self, h: &Helper, ...) -> Result<Option<ScopedJson>, RenderError>

// New (v6)
fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'rc>, ...) -> Result<ScopedJson<'rc>, RenderError>
```

**If using argon2 directly:**
```rust
// Old (v0.2)
use argon2::hash_password_simple;

// New (v0.5)
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
```

---

## 🔗 Links

- **GitHub:** https://github.com/cyberiums/freeradical
- **Wiki:** https://github.com/cyberiums/freeradical/wiki
- **HTTPS Guide:** https://github.com/cyberiums/freeradical/wiki/HTTPS-Setup-Guide
- **Issues:** https://github.com/cyberiums/freeradical/issues

---

## 📅 What's Next?

**Planned for v1.0.3:**
- GraphQL API enhancements
- Additional payment gateway integrations
- Performance optimizations
- Extended documentation

---

**Questions or Issues?** Open an issue on GitHub or check the wiki for guides.

**Enjoy FreeRadical CMS v1.0.2!** 🚀
