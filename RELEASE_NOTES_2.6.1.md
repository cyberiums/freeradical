# FreeRadical CMS v2.6.1 Release Notes

**Release Date**: January 2, 2026  
**Type**: Feature Release

---

## 🎉 Major Features

### Complete API Documentation (130 Endpoints)
✅ **100% OpenAPI/Swagger Coverage** - All 130 public API endpoints now professionally documented

**Coverage Breakdown**:
- **Customer & Authentication** (31): Complete auth flow, 2FA, user management, CRM suite
- **Content Management** (31): Pages, modules, categories, media, SEO, i18n, relationships, revisions
- **Commerce** (19): Products, orders, payments, billing & subscriptions
- **Marketplace** (6): Themes and plugins ecosystem
- **System & Admin** (43): Tenants, webhooks, sites, surveys, SSO, dashboard, metrics, backup

**Features**:
- Interactive Swagger UI at `/swagger-ui/`
- Complete request/response schemas
- Authentication flow documentation
- Try-it-now functionality
- Client SDK generation ready

---

## 🔧 Improvements

### Security
- **CSP Enhancement**: Updated Content Security Policy to properly allow API connections
  - Added `connect-src` directive for localhost:8000 and localhost:8080
  - Fixes frontend API connection blocking issues

### Configuration
- **Port Configuration**: Backend CMS now defaults to port 8000
  - `APP_BIND_PORT`: 8000
  - `APP_BASE_URL`: http://127.0.0.1:8000
  - Maintains compatibility with existing deployments

---

## 📝 API Documentation Details

### New Endpoint Categories
- **i18n** (3): Multi-language support
- **Relationships** (3): Content linking system
- **Revisions** (3): Version history and rollback
- **Dashboard** (4): Analytics and metrics
- **Backup** (2): Data management
- **Search** (1): Content search

### Documentation Enhancements
- All endpoints have `#[utoipa::path]` annotations
- Complete schema definitions with `ToSchema` derives
- Comprehensive request/response examples
- Security requirements documented
- Organized into 15 logical categories

---

## 🚀 Deployment

**Swagger UI Access**: http://localhost:8000/swagger-ui/  
**OpenAPI JSON**: http://localhost:8000/api-docs/openapi.json

**Requirements**:
- Rust 1.70+
- PostgreSQL 12+
- No breaking changes from v2.6.0

---

## 📊 Statistics

- **Total Endpoints**: 130
- **Documentation Coverage**: 100%
- **Controller Files**: 29
- **Schema Models**: 50+
- **API Categories**: 15
- **Build Status**: ✅ Zero errors

---

## 🔗 Links

- **Documentation**: http://localhost:8000/swagger-ui/
- **Repository**: https://github.com/yourusername/freeradical
- **Issues**: https://github.com/yourusername/freeradical/issues

---

## 🙏 Acknowledgments

This release represents a major milestone in API documentation, providing complete coverage of all FreeRadical CMS endpoints with professional-grade OpenAPI specifications.
