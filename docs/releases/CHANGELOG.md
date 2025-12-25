# Changelog

All notable changes to FreeRadical CMS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2025-12-24

### Added - Enterprise Features & Stability
- **OAuth Infrastructure**: Callbacks for Google/GitHub, secure token storage, session management
- **Analytics Dashboard**: Real-time React widget with charts (Chart.js), visitor tracking, referrer analysis
- **Load Testing**: k6 test scenarios covering 100-1000 concurrent users
- **Integration Tests**: Automated flow testing for API and OAuth
- **API Improvements**: Typed Analytics API client in Admin UI

### Changed
- Refactored `admin/src/lib/api.ts` for namespace clarity
- Updated Admin UI dependencies to include Chart.js

### Status
- **Phase 5 Complete**: 99% (only production OAuth testing remains)
- **Overall Completion**: 99.8%

## [0.3.0] - 2025-12-24

### Added

#### Iteration 1: Advanced SEO Features
- Breadcrumb structured data (BreadcrumbList JSON-LD schema)
- Article schema support (Article, BlogPosting, NewsArticle types)
- Author metadata with Person schema
- Image sitemap endpoint (`/image-sitemap.xml`)
- Sitemap enhancements (gzip compression helper, X-Sitemap-Count header)
- Dynamic robots.txt configuration via database
- `robots_rules` table for per-user-agent directives
- Article metadata fields (`author`, `article_type`, `featured_image`, `word_count`, `reading_time`)

#### Iteration 2: Performance & Analytics
- Redis caching infrastructure (optional, disabled by default)
- Composite database indexes for query optimization
- Built-in analytics system with privacy-first design
- SHA256 IP hashing for visitor tracking (GDPR compliant)
- `page_views` and `analytics_summary` tables
- Admin dashboard API with 4 endpoints
- Time-based analytics queries (today, week, month)
- Unique visitor tracking
- Referrer analysis
- SEO health check endpoint

### Changed
- SEO score improved from 93/100 to 97/100
- Version bumped from 0.2.0 to 0.3.0
- Database schema: +3 tables, +5 fields to pages, +2 composite indexes
- Enhanced sitemap controller with scalability improvements

### Performance
- Maintained >2,000 req/s homepage throughput
- Analytics tracking overhead <1ms per request
- Dashboard API response times <100ms
- All database queries <10ms

### Dependencies
- Added `redis` 0.21 with tokio-comp features
- Added `sha2` 0.9 for IP hashing
- Updated `flate2` 1.0 for compression

### Documentation
- Added `RELEASE_NOTES_v0.3.0.md`
- Added `docs/breadcrumb-implementation.md`
- Added `docs/iteration-2-enhancements.md`
- Added `ITERATION-1-COMPLETE.md`
- Added `ITERATION-2-COMPLETE.md`
- Updated `README.md` with v0.3.0 features
- Updated `SEO-readiness.md` (97/100 score)
- Updated `PERFORMANCE.md` with latest benchmarks

### Security & Privacy
- Privacy-compliant analytics (no PII collection)
- IP address hashing (SHA256)
- GDPR compliance ensured
- Input validation on SEO fields

## [0.2.0] - 2025-12-23

### Added
- SEO foundation with 93/100 score
- XML sitemap generation
- Robots.txt support
- Meta tags with validation
- Canonical URLs
- Structured data (WebSite, Organization, WebPage)
- Open Graph & Twitter Cards
- Database optimization (5 indexes)
- Dual-layer validation (server + database)

### Performance
- Homepage: 2,159 req/s (+79% from baseline)
- Sitemap: 2,278 req/s
- All queries <2ms

## [0.1.0] - Initial Release

### Added
- Core CMS functionality
- Page management
- Module system
- Category system
- User management
- JWT authentication
- MySQL database integration
- Handlebars templating
- Rate limiting
- Basic SEO support

---

**Note**: For detailed migration guides and breaking changes, see `RELEASE_NOTES_v0.X.X.md` files.
